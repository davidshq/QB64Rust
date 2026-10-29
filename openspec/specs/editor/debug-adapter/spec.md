# editor/debug-adapter Specification

## Purpose
The debug adapter `qb64rust dap`: a Debug Adapter Protocol server in front of the toolchain's `lldb-dap` that
shows a debug build of this compiler (spec `compiler/debug-info`) in BASIC terms: its threads, stack, stepping,
variables, evaluation, set value, data breakpoints, set next line and the stop at a runtime error. What it does
not rewrite is `lldb-dap`'s as it is.

## Requirements

### Requirement: Adapter start and backend
`qb64rust dap` SHALL run a Debug Adapter Protocol server over stdin and stdout. It SHALL start the `lldb-dap` of
the toolchain a build would use (on Windows `bin\lldb-dap.exe` of the toolchain folder, spec `compiler/toolchain`;
on Linux `bin/lldb-dap` of a toolchain folder given by option or variable, else the `lldb-dap-<n>` on `PATH` with the
highest `<n>` of at least 20, else `lldb-dap` on `PATH`) as its
backend and pass every message it does not rewrite to the other side unchanged but for sequence numbers. An
`lldb-dap-<n>` older than 20 SHALL NOT be started. When
`lldb-dap` cannot be started, the adapter SHALL answer `initialize` with an error naming the path it looked for, or,
with no toolchain found, the message of "No toolchain found". When the backend ends, the adapter SHALL send
`terminated` and end.

#### Scenario: Initialize
- **WHEN** a client sends `initialize`
- **THEN** the adapter answers with `lldb-dap`'s capabilities plus set next line (`supportsGotoTargetsRequest`),
  exception information, and one exception filter, "BASIC runtime errors"

#### Scenario: No lldb-dap
- **WHEN** `qb64rust dap --toolchain-root <dir>` names a folder without `bin\lldb-dap.exe` and a client sends
  `initialize`
- **THEN** the answer is an error whose message names `<dir>\bin\lldb-dap.exe`

#### Scenario: No toolchain
- **WHEN** `qb64rust dap` finds no toolchain and a client sends `initialize`
- **THEN** the answer is an error whose message names `qb64rust toolchain install`

#### Scenario: A versioned lldb-dap on Linux
- **WHEN** `qb64rust dap` runs on Ubuntu where only `/usr/bin/lldb-dap-20` exists, and a client sends `initialize`
- **THEN** the adapter starts `lldb-dap-20` and answers with its capabilities and the additions above

#### Scenario: A versioned lldb-dap before the plain one
- **WHEN** `qb64rust dap` runs on Linux with both `lldb-dap` (the distribution's default) and `lldb-dap-21` on
  `PATH`
- **THEN** the adapter starts `lldb-dap-21`

#### Scenario: Only an older lldb-dap
- **WHEN** `qb64rust dap` runs on Linux where only `lldb-dap-19` is on `PATH`, and a client sends `initialize`
- **THEN** the answer is an error that names the path of `lldb-dap-19`, its version, that 20 or later is needed, and
  the package that provides it on the distribution; no `lldb-dap` is started

#### Scenario: No lldb-dap on Linux
- **WHEN** `qb64rust dap` runs on Linux with no `lldb-dap` or `lldb-dap-<n>` on `PATH`, and a client sends
  `initialize`
- **THEN** the answer is an error that names `lldb-dap` and the package that provides it on the distribution

### Requirement: Launch
`launch` SHALL take the executable of a debug build (`program`) and read its symbol file `<program>.debug.json`;
it SHALL refuse an executable without one, an executable modified later than its symbol file (built again by
another build, which left the symbol file of an earlier one), or one with a symbol file of a version it does not
read, with an error message that says to build with `--debug`. It SHALL pass `args`, `cwd` (by default the
executable's folder), `stopOnEntry` and `console` to `lldb-dap`, and set `QB64PE_NOPROMPT` in the program's
environment when `noPrompt` is true. With `stopOnEntry` the program SHALL stop at its first BASIC line, not inside
libqb's start.

#### Scenario: Not a debug build
- **WHEN** `launch` names an executable built without `--debug`
- **THEN** the answer is an error that names the missing `<program>.debug.json` and `--debug`, and no process
  is started

#### Scenario: Built again by QB64pe
- **WHEN** `launch` names an executable that QB64pe built after a debug build of the same program left its
  `<program>.debug.json`
- **THEN** the answer is an error that says the executable is newer than its debug symbol file and names
  `--debug`, and no process is started

#### Scenario: Stop on entry
- **WHEN** `launch` has `stopOnEntry: true` for a program whose first statement is on line 3
- **THEN** the first `stopped` event is on line 3 of the main file, in the main module's frame

### Requirement: One thread, the program's
`threads` SHALL list one thread, the one that runs the BASIC program (the main module's C function and the
procedures it calls), named "Program". A stop on any other thread (a pause) SHALL be reported on the program's
thread.

#### Scenario: Pause
- **WHEN** a program loops forever with `DO: x = x + 1: LOOP` and the client sends `pause`
- **THEN** a `stopped` event names the program's thread, and its stack's top frame is the loop's line

### Requirement: The stack in BASIC terms
`stackTrace` SHALL list only the frames of BASIC code: the main module as "Main module", a SUB as `SUB <name>`,
a FUNCTION as `FUNCTION <name>` (names as declared), each at its file and line. The frames of libqb, of the thread
wrapper, of libqb's `evnt` and of any C++ without a file the symbol file lists SHALL be left out. An error
handler's frame SHALL be shown above the frame of the line that raised the error.

#### Scenario: Stopped in a SUB
- **WHEN** the program stops on a breakpoint in `SUB show`, called from line 3 of the main module
- **THEN** `stackTrace` gives two frames: `SUB show` at the breakpoint's line, then "Main module" at line 3

#### Scenario: Stopped in a handler
- **WHEN** an `ON ERROR` handler on line 8 is running for an error raised on line 4
- **THEN** the frames are "Main module" at line 8, then "Main module" at line 4

### Requirement: Stepping by BASIC line
`next`, `stepIn` and `stepOut` SHALL end only on a line of a file the symbol file lists and within that file's
lines. A stop elsewhere (a C++ fragment such as `data1.txt`, an inline function of libqb's headers, a line past
the file's end, `evnt`) SHALL be stepped past (out of a frame that is not BASIC code, on by line otherwise). A
`stepIn` into a procedure SHALL end at its first statement, not at its header line. After a return to the caller,
or out of an inline function of libqb's, a stop on the line at a column other than 1 (the rest of a `CALL`, of the
line) SHALL be stepped past. Breakpoints and pauses SHALL stop where they stop.

#### Scenario: Step into a SUB
- **WHEN** the program is stopped on `CALL show(x)` and the client sends `stepIn`
- **THEN** one `stopped` event follows, at the first statement of `SUB show`

#### Scenario: Step into a FUNCTION of an included file
- **WHEN** the program is stopped on a line that calls `twice&`, defined in `lib.bi`, and the client sends
  `stepIn`
- **THEN** one `stopped` event follows, at the statement of `twice&` in `lib.bi`

#### Scenario: Step out of a SUB
- **WHEN** the program is stopped in `SUB show`, called from line 3, and the client sends `stepOut`
- **THEN** one `stopped` event follows, at line 4 of the main module

#### Scenario: Next past a handler's end
- **WHEN** the program is stopped on `RESUME` in a handler and the client sends `next` twice
- **THEN** no stop is on a line past the main file's end or in `qbx.cpp`

### Requirement: Variables
`scopes` SHALL give, for a frame of BASIC code, "Locals" (for a procedure: its parameters in order, then its
locals, statics and result, by BASIC name; for the main module: empty) and "Main module" (the variables of main
storage), and nothing of `lldb-dap`'s own scopes. Each variable SHALL show its BASIC name with the type as BASIC
spells it, and a value read from the program's memory through the symbol file: a number in decimal, a `STRING`
or `STRING * n` quoted, an array as its bounds with its elements as children (by index, within the bounds its
descriptor holds when read, at most 1000 per level, then a note), a user type as its members. A variable whose
storage is not allocated yet (a local before its procedure's first statement, a dynamic array not yet
dimensioned) SHALL show that rather than an error.

#### Scenario: Each kind
- **WHEN** the program is stopped in a SUB with a `LONG` parameter, a `SINGLE` local and a `STATIC INTEGER`, and
  the main module has `DIM SHARED total AS LONG`, a `STRING`, `DIM a(1 TO 3) AS SINGLE` and a `TYPE` variable
- **THEN** "Locals" lists the parameter, the local and the static with their values, and "Main module" lists
  `TOTAL`, the string quoted, `A` expanding to `A(1)` to `A(3)`, and the `TYPE` variable expanding to its members

#### Scenario: A frame further down
- **WHEN** the program is stopped in a SUB called from another SUB, and the client asks for the caller's "Locals"
- **THEN** the caller's locals are listed with their values in that frame

### Requirement: Evaluate in BASIC terms
`evaluate` in the contexts `hover`, `watch` and `variables` SHALL take a BASIC variable name (any case, with or
without its suffix), an array element with integer-constant subscripts, or a member path (`p.x`, `a(2).nm`), seen
from the given frame (the procedure's own variables first, then the main module's), and answer as `variables`
would show it. A name of neither SHALL be answered "not a variable here". A suffix `,h`, `,b` or `,o`, or DAP's `format.hex`, SHALL show an integer in hex,
binary or octal. In the `repl` context an expression that is not such a name SHALL go to `lldb-dap` as typed, so
that `lldb`'s own commands work in the Debug Console.

#### Scenario: Hover
- **WHEN** the program is stopped in `SUB show` and the client evaluates `total` for a hover
- **THEN** the answer is the value of the main module's `TOTAL`, unless `SUB show` has a variable `TOTAL` of its
  own, whose value it is then

#### Scenario: Hex
- **WHEN** `total` holds 255 and the client evaluates `total,h`
- **THEN** the answer is `&HFF`

### Requirement: Set value
`setVariable` and `setExpression` SHALL write a number of the variable's type (refusing a value out of the type's
range or a text that is not a number), and a `STRING * n` in place (padded with spaces or cut to `n`). A
variable-length `STRING` SHALL be written through libqb's own string assignment if the backend can call it;
otherwise the request SHALL be refused with a message saying so.

#### Scenario: A number
- **WHEN** the client sets `total` (a `LONG`) to `42` and continues
- **THEN** the program reads 42 from `total`

#### Scenario: Out of range
- **WHEN** the client sets an `INTEGER` to `40000`
- **THEN** the request is refused with a message, and the value is unchanged

### Requirement: Data breakpoints
`dataBreakpointInfo` SHALL offer a data breakpoint for a scalar variable, an array element or a member of at most
8 bytes, and `setDataBreakpoints` SHALL set it as a hardware watchpoint on its bytes (write, read or both as
asked). A larger value (a `STRING`'s text, a `_FLOAT`, a `STRING * n` longer than 8) SHALL get no data
breakpoint, with a description saying why.

#### Scenario: Watch a variable
- **WHEN** the client sets a data breakpoint on `total` and continues
- **THEN** the program stops at the line that changes `total`, with reason `data breakpoint`

### Requirement: Set next line
`gotoTargets` SHALL offer a line of the current frame's procedure (between its header and its `END`, or a line of
the main module for the main module); `goto` SHALL move the program's next statement there, and a `stopped` event
SHALL show it. A line outside the procedure SHALL get no target.

#### Scenario: Jump back
- **WHEN** the program is stopped on line 6 of the main module and the client sends `goto` to line 4
- **THEN** the program is stopped at line 4, and `next` runs line 4

### Requirement: Stop on a runtime error
With the exception filter "BASIC runtime errors" set, a runtime error SHALL stop the program before it is handled
(also when an `ON ERROR` handler would handle it), with a `stopped` event of reason `exception` on the raising line
and the error's number and text in its description; `exceptionInfo` SHALL give the same. Without the filter, no
such stop. The client's own function breakpoints SHALL be kept apart from the one the filter sets.

#### Scenario: Error 5
- **WHEN** the filter is set and the program runs `x$ = CHR$(-1)` on line 3
- **THEN** it stops with reason `exception`, the description names error 5, and the top frame is line 3

#### Scenario: Filter not set
- **WHEN** the filter is not set and the program has an `ON ERROR` handler for the same error
- **THEN** the program runs on through its handler without a stop
