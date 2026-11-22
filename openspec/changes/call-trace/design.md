# Design

## Context

Motivation: `proposal.md`, "Why". The user chose the shadow stack on 2026-11-21, with the panel's view
(`DECISIONS.md` 2026-11-21). What the code does today:

- **The message** is built in libqb's `fix_error` (`libqb\internal\c\libqb\src\error_handle.cpp`): the format
  `"Line: %u (in %s)\n%s%s"`, the last `%s` being `"\nContinue?"`. The dialog and the stderr text under
  `QB64PE_NOPROMPT` both get that string. A critical error (300 to 315, and error 256 from `libqb_check_stack`)
  goes through `critical_error_message`, which formats into a static buffer and allocates nothing.
- **When it is built.** An error is reported at the failing statement's event check, inside the procedure that
  raised it. Every caller's C++ frame is still on the stack then.
- **A procedure** is one C++ function (`Emitter` in `crates\codegen-cpp\src\procs.rs`). Its start is the
  `PROLOGUE` lines, and every way out passes the label `exit_subfunc`.
- **`GOSUB`** pushes a number onto libqb's `return_point` stack (`Op::Gosub` in `stmt.rs`). The number `G` is
  unique in the program and names one `GOSUB` statement. `RETURN` pops it. So the pending `GOSUB`s are already
  recorded at run time, at no cost.
- **An event** (`events.rs`) runs a SUB by a plain call, or a label by pushing return point 0 and calling
  `QBMAIN(NULL)` again. An `ON ERROR` handler is also run by `fix_error` calling `QBMAIN(NULL)` again.
- **No current line is kept.** A statement passes its line to `evnt(...)` only when an event or error is
  pending. `error_track_line` keeps one only under `$ERRORLOCATION:ON`.
- **`SUB _GL`** runs on libqb's display thread, beside the main program's thread.
- **Recorded outputs.** 45 files under `tests\` hold `Runtime error: Line`, so a longer message in every build
  would change recordings. This is why the call trace is not on by default.
- **The models** are `--coverage` (`coverage.rs`) and `--trace-values` (`trace.rs`, `trace_rt.h`): the driver
  passes a value down, the emitter adds calls, and a fixed C++ block goes into the program.
- libqb's own stack walker (`logging\stacktrace.cpp`) gives C++ symbol names with no BASIC lines, and nothing
  on wasm. It is not used here.

## Goals / Non-Goals

**Goals:**
- Every row is exact: a procedure's name, and the line of the call that is still running.
- The same text on Windows, Linux and wasm.
- A build without the call trace writes the same C++ as today.
- The report of error 256 allocates nothing, since the stack is nearly full.
- The cost is per call, not per statement.

**Non-Goals:**
- A BASIC function that returns the call stack (user, 2026-11-21).
- The call stack in `_LOGERROR`'s output or in the SIGSEGV report. Both use libqb's own walker.
- The arguments or variables of a frame. The debugger shows those.
- The line where an event interrupted the program. It would need a store per statement.

## Decisions

### D1. A frame per call, pushed by the procedure itself

The program keeps a stack of frames. A frame is three 32-bit numbers:
- `proc`: the procedure's number in a table of names;
- `site`: the place of the call, a number in a table of files and lines (D2);
- `gosubs`: the value of `next_return_point` when the procedure started (D4).

A procedure pushes its frame after the `PROLOGUE` lines and pops it at `exit_subfunc`. Both are one line of
C++ each: `qbr_ct_push(<proc>);` and `qbr_ct_pop();`. The push comes before the test
`if (is_error_pending()) goto exit_subfunc;`, so that every path that reaches the pop has pushed.

Code: a new module `crates\codegen-cpp\src\call_trace.rs`, used from `Emitter::proc` in `procs.rs`. The
run-time part is `crates\codegen-cpp\src\call_trace_rt.h`, taken in with `include_str!` as `trace_rt.h` is.

*Why the procedure and not the call site:* a procedure has one start and one end. Calls sit inside
expressions, and there are many more of them.

*Alternative considered:* walk the C++ stack with libqb's `_Unwind_Backtrace`. Rejected: no call lines, nothing
on wasm, and an optimized build inlines frames away.

### D2. The line of a call is stored once per statement that holds a call

A statement that holds a call of a `SUB` or `FUNCTION` of the program stores its site number before it runs:
`qbr_ct.site=<n>;`. The push copies that number into the new frame. The pop writes the frame's number back, so
that the caller's next call in the same statement (`f(1) + g(2)`) still has the statement's site, whatever
`f` stored inside.

The emitter knows which statements hold a call: it writes each `Op::Call` and `ExprKind::CallProc` of a program
procedure, and notes it there (`Sites::called`), as `trace.rs` records its after-call lines.

A site is a file and a line. Sites are numbered while the bodies are emitted, as `coverage::Counters` numbers
its statements. Two numbers are fixed: 0 is "not known" and 1 is "event" (D5).

The store goes outside the statement's `do{…}while(r);`, where the coverage counter goes
(`Emitter::body` in `procs.rs`).

*Why not a store at every statement:* a loop that calls nothing would pay for it. Only the line of a call is
needed. The line of the error itself comes from libqb (D6).

### D3. The stack grows, and belongs to its thread

The frames are a `thread_local` array that starts at 256 frames and doubles with `realloc` when it is full. If
`realloc` fails, pushes are only counted from then on, and the block says how many frames are missing. The
state (`site`, the depth, the array) is one `thread_local` struct, `qbr_ct`.

*Why growing:* a recursive program can be tens of thousands of calls deep before error 256. A fixed array or a
ring would lose the frames between its ends after the program came back up from a deeper point. A frame is 12
bytes, so 100,000 calls are 1.2 MB.

*Why `thread_local`:* `SUB _GL` and the procedures it calls run on the display thread. With one shared stack,
their pushes and pops would be mixed into the main program's. An error is reported on the thread that raised
it, and the hook (D6) reads that thread's stack.

*Alternative considered:* leave `SUB _GL` and everything it can reach without pushes. Rejected: the call graph
does not say what an event or a `DECLARE LIBRARY` callback reaches.

### D4. A pending GOSUB is read from libqb's return stack

Nothing is added to `GOSUB` or `RETURN`. The program gets a table from the number `G` of each `GOSUB` to its
site (`Op::Gosub` in `stmt.rs` gives `G`; the table is written by `call_trace.rs`). When the block is written,
the entries of `return_point` from a frame's `gosubs` up to the next frame's `gosubs` are that procedure's
pending `GOSUB`s. The main module's start at 0. An entry 0 is an event's (D5).

*Why this is safe to read:* `return_point` and `next_return_point` are plain globals of libqb that the
generated C++ already reads and writes.

### D5. Events and error handlers

- **A SUB run by an event** (`Case::Call` in `events.rs`): the generated case stores site 1 before the call and
  the old site after it. The SUB's frame then carries site 1, and the row below it reads `(event)`.
- **A label run by an event** (`Case::Label`): the case does the same around its `QBMAIN(NULL)`. The return
  point 0 it pushes ends the list of `GOSUB` rows above it. The interrupted procedure's row reads `(event)`.
- **An `ON ERROR` handler** runs in the main module while the frames of the failing procedure are still on the
  stack. libqb knows this state (`error_handling`) and passes it to the hook (D6). The block then starts with
  `Error handler (<file>:<line>)`, and the innermost frame's row has no line, because its position is the first
  error's, which only `ERL` holds. The callers' rows keep their lines.

A handled error prints nothing, and pushes and pops stay paired: the handler returns into the procedure
(`RESUME`), or the program ends.

### D6. One hook in libqb, and the block's text

libqb gets one edit, `tools\libqb\edits\call-trace-error-hook.edit`, to `error_handle.cpp` and its header:

```
const char *(*libqb_error_call_stack)(uint32_t line, uint32_t include_line,
                                      const char *include_file, int in_handler) = NULL;
```

`fix_error` and `critical_error_message` call it when it is set, and put its text between the error's text and
`Continue?`. With the hook unset, both write the bytes they write today. A call-trace program sets the hook in
a static initializer.

The hook formats into a static buffer of 4 KB and returns it. It allocates nothing, so that error 256 can be
reported with the stack nearly full. The rows:

- The first row is the innermost frame's procedure with the error's line. The line is `include_line` in
  `include_file` when that is not 0, else `line` in the main file. With no line known, the row has no
  parentheses.
- Then, per frame from the innermost out: its pending `GOSUB` rows (D4), then the caller's row with the frame's
  site.
- A path is the file's path from the main file's folder with `/`, as `coverage_files` in
  `crates\driver\src\lib.rs` gives it.
- A name is the procedure's as declared, as the debug symbol file writes it (`debug.rs`).
- More than 24 rows are cut as the spec says. The hook counts the rows first, then writes.

*Why a hook and not text in libqb:* the tables and the stack are the program's. libqb is shared with builds
that have no call trace, and this keeps its part to a pointer and two calls.

*Alternative considered:* print the block from the program at exit. Rejected: the dialog is shown, and the
stderr text written, before any exit code of the program runs.

### D7. The options

- `crates\driver\src\main.rs`: `--call-trace` and `--no-call-trace`, for a compile line and for `test`.
- `crates\driver\src\settings.rs`: `call_trace` in `BuildTable` and `Build`, checked in the function that
  checks the `[build]` table.
- The value is decided in one function in `main.rs`: the command line, else the settings file, else `--debug`.
- `compile.rs` gets a field `call_trace: bool`, and `lib.rs` passes it to the emitter beside `coverage` and
  `trace`.
- A wasm build takes the option. The run-time part is plain C++.
- The build cache: task 1.3 checks whether it keys on the C++ text, as `value-trace` did (its design D7).

The extension needs no change: its debug launch builds with `--debug`, so it gets the call trace.

*Why `[build]` and not a new table:* it is a property of how the project is built, as `output` and `copy` are.

### D8. Tests that compare a debug build with a recording

A `--debug` build's error message is now longer, so a test that builds a recorded program with `--debug` and
compares stderr would fail on the 45 programs that end in a runtime error. Those tests pass
`--no-call-trace`, or compare with the block cut out. Task 4.3 finds them.

## Found while building (2026-11-22)

- **D2 had a wrong fact.** I wrote that `trace.rs` walks a statement for its calls. It does not: it records a call
  where the emitter writes it (`trace_call` in `value.rs` and `stmt.rs`). So the call trace does the same: the two
  places that write a call of a program procedure set `Sites::called`, and `Emitter::body` stores the site before
  the statement when it is set. No walk was moved or written.
- **The handler's frame.** D5 first gave the hook an `in_handler` flag. That could not place a SUB called from a
  handler. So `mainerr.txt` pushes a frame of its own (procedure 1, "Error handler", site 0), and `RESUME` takes it
  off (`qbr_ct_resume`). `RESUME label` in the main module drops every frame (`qbr_ct_reset`), since the code that
  raised the error is left for good. A label run by an event gets a frame of the main module's with site 1, so
  the row of the interrupted code reads `(event)`. The return point 0 that the event pushes is skipped.
- **The file of the error's line.** libqb gives an included file by its base name only (`evnt`'s argument). The
  hook finds the file whose path ends in that name. Two included files with one base name in different folders
  would show the first one's path. Not seen in a program here.
- **Three edit files, not one.** An edit holds one old and new pair, so the hook is
  `call-trace-a-error-hook-header.edit`, `call-trace-b-critical-message.edit` and `call-trace-c-fix-error.edit`.
  They are listed in `libqb\SOURCE.md`. `UPSTREAM.md` gets no entry: it lists QB64pe's bugs.
- **`RESUME` in a procedure.** A `RESUME` that runs inside a SUB called from a handler pops nothing, since the
  handler's frame is not on top. The handler's frame then stays. A later error's block would list it. Not
  tested; no program here does this.
- **The cost, measured on 2026-11-22 on the development machine (Windows).** A loop of 10 million calls of an empty
  SUB, 10 runs of each build, medians. The default build has no `-O2`, so push and pop are not inlined.

  | Build | Without the option | With `--call-trace` | A call costs |
  |---|---|---|---|
  | Default | 0.211 s | 0.365 s | about 15 ns more |
  | Default, the state not `thread_local` (an experiment, not kept) | 0.211 s | 0.300 s | about 9 ns more |
  | `-f:OptimizeCppProgram=true` | 0.090 s | 0.123 s | about 3 ns more |

  About 6.5 ns of the default build's 15 is `thread_local`. The rest is the calls of push and pop. Put to the
  user, who chose to write push and pop as macros and go on (2026-11-22). With the macros, the same loop, again
  10 runs of each build taken in turn:

  | Build | Without the option | With `--call-trace`, push and pop as macros | A call costs |
  |---|---|---|---|
  | Default | 0.155 s | 0.255 s | about 10 ns more |
  | `-f:OptimizeCppProgram=true` | 0.094 s | 0.116 s | about 2 ns more |

  The first macros made the VM's run of the CLI tests fail four debugger tests (2026-11-22). A step into a SUB
  stopped in the function the push called to grow the stack, since the program's C++ has line information
  throughout. And a breakpoint on `END SUB` got two places, so a step from it stopped again on the same line: the
  pop's variables made a lexical block, and `lldb` places a breakpoint once per block of a line. So the final macros
  call no function of the program and declare no variable: a procedure takes the thread's state once, as a
  reference at its start (`qbr_ct_state &qbr_ct_s=qbr_ct;`), and the push and the pop work on it. Measured again
  with them, 10 runs of each build in turn: Windows 0.125 s and 0.195 s in the default build (about 7 ns a call),
  0.082 s and 0.132 s with `-O2` (about 5 ns); Linux 0.104 s and 0.170 s (about 7 ns), 0.077 s and 0.095 s with
  `-O2` (about 2 ns). The tables above are of the earlier macros.

  The machine's own speed moved between the two sessions (the default build without the option took 0.211 s,
  then 0.155 s). Each pair was run in turn in one loop, so each difference holds. On Linux (this tree's copy in
  WSL on the same machine, `g++`), with the macros: 0.112 s and 0.166 s in the default build (about 5 ns a call),
  0.078 s and 0.094 s with `-O2` (about 2 ns). `g12_game`'s scripts are not measured: they run `SHELL`, which
  shows a window on the host (`CLAUDE.md` working rule 6), and the screen runner takes no `--call-trace`.

## Risks / Trade-offs

- [A procedure is left without its pop] → every way out goes through `exit_subfunc` (`EXIT SUB`, the end, the
  `RUN` unwind, a pending error). A CLI test runs each and checks that a later error's block is right.
- [`RUN` restarts the main module with frames left] → `RUN` in a procedure unwinds through `exit_subfunc`.
  The `RUN` block of the main module also sets the depth to 0, for a `RUN` inside a label run by an event.
- [`thread_local` is slow with MinGW] → task 3.4 measures a recursive loop of 10 million calls with and without
  the option, on Windows and Linux. The number goes into the manual. If a call costs more than about 10 ns,
  the design is put to the user again before the tests of group 4.
- [The block is too long for the dialog] → 24 rows at most, and the hook's buffer is 4 KB. A row that does not
  fit is cut at the buffer's end, with the terminating byte kept.
- [A row's line is stale] → a row's site is written only by a call statement or by an event case, and the pop
  restores it. The innermost row's line is libqb's. No row uses a site that was not set for it.
- [The message differs from QB64pe's] → only in a build that asked for it. A `DIVERGENCES-QB64PE.md` row says
  so.
- [A call-trace build of a recorded program changes output] → only the error message. Task 4.4 runs the corpus
  programs that hold a runtime error with the option and compares the rest of their output.

## Open Questions

- Whether the call trace becomes the default for every build. It waits for the cost measured in task 3.4, and
  it would need the 45 recordings handled. It changes nothing in this change.
