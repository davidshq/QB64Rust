# Design

## Context

For the motivation and the list of 51 forms, see `proposal.md`; for the behaviour, the delta specs. How the parts
stand in this compiler (2026-10-24, on `m4-finish`):

- **The built-in tables.** `crates\sema\src\builtins.rs` holds `SUPPORTED` (each function with its `Rule`),
  `STATEMENTS` (each statement with its `StmtRule`) and `NEVER_RUN` (the built-ins no test runs). A name in none of
  them reaches `unsupported(…)` in `check\builtins.rs`. The coverage check of tier 1 wants each `SUPPORTED` name
  called in a `slice.list` program and a `typed` front-end test, each `STATEMENTS` name in a `slice.list` program
  and an `ir` test, and each `NEVER_RUN` name compiled and not reached in a slice program and in a call-site
  program (`tests\callsite\`).
- **What the table already gives.** `qb64rust_builtins` reads QB64pe's `subs_functions.bas`: each entry's libqb
  call, slots, optional mask, template and result type. `Rule::Plain` and `StmtRule::Plain` compile an entry from
  that alone; `_READBIT` (`m4-qb64pe-builtins`) is such a row.
- **The `CONST` evaluator** (`sema\src\consteval.rs`) computes `_D2G`, `_G2D`, `_R2G` and `_G2R` already
  (`m3-constant-evaluation`); an expression outside a `CONST` does not.
- **Line tracking.** `codegen-cpp\src\procs.rs` writes `error_track_line(0,0,NULL);` once at the start of the main
  module, as the old compiler does (`qb64pe.bas` 3272), so that libqb's critical-error message says "Enable
  $ErrorLocation:ON". Under `$ERRORLOCATION:ON` the old compiler writes `error_track_line(line,incline,"file");`
  before each statement (`qb64pe.bas` 6201–6209), and at two other places (5260, 5877).
- **Auto-included files and rounds.** `crates\syntax\src\program.rs` `parse_round` parses the colour file before the
  main tree and `afterlastline.bm` after it; a setting that changes the flags (`$COLOR`, `$CONSOLE`) starts another
  round. Since `m4-statements-metacommands` `sema` reads the trees as one stream of events (`check\walk.rs`).
- **The compiler root** is `--include-root`, else the folder of `qb64rust.exe` (`crates\driver\src\main.rs`
  `include_root`); `FileLoader` (`crates\driver\src\lib.rs`) looks up included files there.
- **The old compiler's `$USELIBRARY`** (`qb64pe.bas` 1915–1976): the quoted name, the descriptor
  `libraries/descriptors/<name>.ini` read with `ReadSetting$` (`source\utilities\ini-manager\ini.bm`), the files
  under `libraries/includes/<name>/`, a list of libraries kept across a restart of the compile (`do_recompile`),
  and the files included at the top in reverse order, after `aftermain.bas` (an `END`) in order, and before
  `afterlastline.bm` in order (1740–1789). The 4.7.0 release has no `libraries\` folder.

## Goals / Non-Goals

**Goals:**

- The probe of every table entry (`proposal.md`) finds no mark, and a tier 1 test keeps it so.
- Each new form's C++ call is the old compiler's (call-site check), and each value or effect a test can see is
  held to a `v58` recording.

**Non-Goals:**

- `SUB _GL` and the OpenGL calls in it: step 5.
- The 9 programs steps 2 and 3 left marked.
- Running what acts outside the program (the `NEVER_RUN` additions of D3).
- Libraries from a QB64pe release that ships them: none does; the format is the 4.7.0 compiler's.

## Decisions

### D1. The measurement family `v58` comes first

`verification\v58_*` records, before any code:

- **Functions** (`f*`): each new function's values over a few arguments of each type, its result type (`LEN` of
  `MKS$` of it, as `v53` does), a value out of its domain, and its rejected forms (an argument too many, too few, a
  string for a number). `_SETBIT`, `_RESETBIT`, `_TOGGLEBIT` over each integer type and a bit beyond the width or
  negative. The MBF functions' bytes for 0, 1.5, -2.25, a large and a tiny value, and a short string.
- **Statements** (`s*`): `OUT`/`INP` of the palette, `WAIT &H3DA`, ports not emulated and out of range; `FILES`
  of a pattern in a folder of the family's support files, of nothing, under `$CONSOLE:ONLY`; `DATE$ =`, `TIME$ =`
  and their rejected forms; the console statements under `$CONSOLE:ONLY`; the display statements' forms and their
  rejected forms; the lock-key statements compiled only (their rejected forms recorded).
- **`INSTR` with its first argument left out** (`i*`): the result, with the start left out, beside `INSTR(1, …)`
  and `INSTR(2, …)`, read with `-z` for the call.
- **`$ERRORLOCATION`** (`e*`): a stack overflow with `ON`, `OFF` and neither, in the main file, a SUB and an
  included file; `ERL` and `_ERRORLINE` under it; `ON` twice; `ON` and `OFF` in each order and across an included
  file and a `$IF`; `$ERRORLOCATION` bare, `:MAYBE`, with a blank.
- **`$USELIBRARY`** (`u*`, with `v58_u*.root`, D6): each kind of file alone and all three; two libraries with
  output that shows the order; one named twice, from one line and from two; a library that uses another; a value in
  quotes and a comment in the descriptor; the three errors; and without the sidecar, the release's own error.
- **A neighbour**: a built-in written with its result's suffix (`_UFONTHEIGHT&`, `_COLORCHOOSERDIALOG&`), which this
  compiler rejects as a reserved name; the probe of `proposal.md` met it.

*Alternative:* take the values from libqb's source. Rejected: the slot conversions and result types are where the
old compiler's own rules show, and those are not in libqb.

### D2. Most names are rows of the tables

Each name the table compiles as it stands becomes a `Rule::Plain` or `StmtRule::Plain` row, the call written from
the table: the math functions, `_SETBIT` and its kin (their table entries are `_READBIT`'s shape), the MBF
functions, `LPOS`, `INP`, `_COMPILERVERSION$`, `_ENVIRONCOUNT`, `_UPTIME`, the console, display and dialog
functions and statements, `OUT`, `WAIT`, `FILES`, `_BLINK`, the lock keys and the function `_DISPLAY`. Where `v58`
shows the old compiler special-casing a name (a result type, a slot), the existing rule that fits takes it
(`ResultOfArg`, `Fixed`, `FloatByArg`); a new `Rule` variant is added only when none fits, and named in this
design when it is. Code: `sema\src\builtins.rs`.

The `CONST` evaluator is left as it is. `v58_f02`–`f08` asked whether the expression and the evaluator agree:
they need not. The evaluator does not know the three hyperbolic functions (nor `_SETBIT` and kin, `MKSMBF$`,
`MKDMBF$`, `_COMPILERVERSION$`, `_ENVIRONCOUNT`: "Unexpected element", `x107`–`x111`; `sema`'s evaluator must
reject them the same way), and its `_D2G(90)` is 99.99999999900001 where the expression's is 100 (`f05`): the
expression takes libqb's `func_deg2grad` and the C library's `std::asinh` and kin, as the table says, and the
`CONST` keeps the evaluator's value (`m3-constant-evaluation`).

The test of D10 found one more entry the probe of `proposal.md` had missed: the function `INPUT$` (`func_input`,
two LONG slots, the second optional). It is a plain row too; the parser takes the `#` the old compiler drops before
its second argument alone (`qb64pe.bas` 20546), and nothing after the `#` is its "Invalid expression" (measured,
`v58_f16`, `x112`–`x116`).

### D3. Never run, held by the call-site check

`NEVER_RUN` takes `_SCREENCLICK`, `_SCREENPRINT`, `_NOTIFYPOPUP`, `_COLORCHOOSERDIALOG`, `_INPUTBOX$`, the
statements `_CAPSLOCK`, `_NUMLOCK` and `_SCROLLLOCK`, and `_CONSOLEFONT`, each with its reason in a comment, as the
list's others have. The functions `_CAPSLOCK` and kin are run: they only read. Each gets its call-site programs and
a slice program where it is compiled and not reached.

### D4. Statements written as assignments: `DATE$ =` and `TIME$ =`

The parser reads `DATE$ = x` by the table's template `=?` of the SUB entry `Date` (`statement_templates` takes a
SUB entry whose `musthave` suffix the name carries), as a `BuiltinStmt`, so the rows `DATE$` and `TIME$` of
`STATEMENTS` are all it takes: the plain entry `sub_date`/`sub_time` with its one string slot is the call, and a
number is "String required for sub" by the slot's rule (`v58_x70`–`x75`). The assignment checker of `check\io.rs`
is not touched (the design first thought it was: the `-z` read of `v58_s03` and the IR dump showed the template
path).

### D5. `INSTR` with its first argument left out

The checker's omitted-argument rule (`check\builtins.rs`, from `m4-statements-metacommands` D11) lets the first
argument of `INSTR` be absent when two follow, and passes it as the old compiler's C++ does (read with `-z` in task
2.2, `v58_i01`): `func_instr(NULL,a,b,0)`, the start `NULL` and the passed mask 0, where the two-argument form
passes the same and `INSTR(1, a, b)` passes `func_instr( 1 ,a,b,0|1)`. So the omitted first argument is the table
entry's optional first slot left empty, nothing more. The search starts at 1 (`v58_i01`).

### D6. `$ERRORLOCATION`: a program setting, and the call before each statement

`Checker::meta` records each `$ERRORLOCATION:ON` or `:OFF` line in file order (the included files where they are
included, as the old compiler's first pass reads them). The program's setting is the last line's; the first line
whose setting differs from it is the old compiler's error "$ErrorLocation is a whole-program option and cannot be
changed within the program" (measured, `v58_e08`–`e12`: the first pass takes the last setting, the main pass stops
at the first line that disagrees, so the error is at the program's first `$ERRORLOCATION` line in every order
tried). The IR gets the setting on the program (`ir::Program`), and codegen writes the old compiler's
`error_track_line(n,0,NULL);` (main file) or `error_track_line(n,m,"file");` (included file, its name without its
folder, as the old compiler writes it) before each statement where the old compiler writes it: `n` is the
statement's line as the IR has it (a joined line's last physical line, `v58_e23`). Task 2.2 reads the old
compiler's C++ (`-z`) for `v58_e26_on_kinds` (each kind of statement and block, a joined line, a single-line `IF`,
an included file) to place the call exactly; the call-site check compares the calls. Code: `check\mod.rs` (`meta`),
`ir`, `codegen-cpp\src\stmt.rs`, `procs.rs`.

A `DYNAMIC` library is loaded and its functions found before any statement runs, so the old compiler writes the
call inside the failure's braces, with the `DECLARE` line before error 259 and the `SUB`/`FUNCTION` line before
error 260 (`qb64pe.bas` 5253–5274, 5874–5895); codegen does the same from each one's place (`sema::Track`,
`codegen-cpp\src\library.rs`). Measured: `v58_e28` (a function the library lacks: the message names the
`FUNCTION` line) and `v58_e29` (a statement of a `$USELIBRARY` top file: the line in that file and its name).
Error 259 itself is not measured: a library that does not exist is rejected when compiled ("DYNAMIC LIBRARY not
found", `v58_e27`), and one found then but gone at run time is beyond the runner; its call rests on the old
compiler's source and the C++ snapshot `errorlocation_cpp`.

### D7. `$USELIBRARY`: a list of libraries kept across rounds

- **Reading.** The parser meets `$USELIBRARY` as a metacommand line. It reads the name between the quotes by the
  old compiler's rule (the first two `'`, trimmed), asks the loader for the descriptor text
  (`Loader::library(name)`, a new method; `FileLoader` reads `<root>\libraries\descriptors\<name>.ini`), and reads
  the three keys of `[LIBRARY INCLUDES]` by `ReadSetting$`'s rules (`ini.bm`: sections, `key=value`, quotes, `;`
  and `'` comments; read in full and followed, nothing copied: it is QB64pe's MIT code, but a reader of three keys
  is a few lines of Rust). A missing descriptor or file is the error at the line. Code: `syntax\src\parser\meta.rs`,
  a new `syntax\src\library.rs`, `syntax\src\program.rs` (`parse_round`).
- **Including.** A library not yet in the list (by name and the file and line that name it, as the old compiler
  keys it: `v58_u04`, `u05` show a library named twice included twice) is added and starts another round
  (`Round::again`, a parse with the same flags), as a new flag setting does; the list lives across the rounds of
  `parse_settled`. Each round parses the `IncAtTop` files after the colour file and before the main tree, last
  library first (`u03`: `b;a;`; `u06`: a dependency registered from a top file comes first); the `IncAfterMain`
  files after the main module, in order; the `IncAtBottom` files after the main tree and before
  `afterlastline.bm`, in order. `ParsedProgram` holds the three lists of trees and the libraries.
- **After the main module.** The old compiler's `aftermain.bas` is an `END` with `$INCLUDEONCE`: it is copied into
  the builtins crate as `afterlastline.bm` is (`tools\builtins\extract_builtins.py`) and parsed as a tree of its
  own before the first `IncAfterMain` tree, when there is one. `sema`'s list of top-level statements
  (`check\mod.rs`, `top`) takes the trees in the order above: the colour file, the top files, the main file, the
  `END` and the after-main files, the bottom files, `afterlastline.bm`. The main body's statements thus end with
  the `END` before a library's labels and handlers (`u01`, `u19`, `u20`), wherever the program's own procedures
  stand. A program with no library is unchanged.
- **Lines.** The old compiler's C++ of `v58_u01` and `u19` (task 2.2) gives a library file's statements the
  main-file line a `$INCLUDE` would give them: 0 for a top file (`evnt(0,2,"three_top.bi")`), the main file's
  number of lines for `aftermain.bas`, an after-main file and a bottom file (`evnt(12,2,"three_bottom.bm")` in a
  file of 12 lines; `_ERRORLINE` gives it, `_INCLERRORLINE` the file's own line). `sema` gives the library trees
  these `include_line`s. The old compiler includes `aftermain.bas` in every program (its `sub_end()` with
  `evnt(n,20,"aftermain.bas")` before the final `sub_end(); return;`): this compiler keeps its one `sub_end()` and
  parses the `END` tree only when a library has an after-main file; the call-site check never sees that line.
- **The errors.** "Expected $USELIBRARY:'author/library'" (no two quotes, an empty name), "Library descriptor for
  '…' not found", "Library source for '…' not found (IncAtTop)" with the key's name, each at the line, as parse
  errors (the loader is the parser's); the line's forms the old compiler takes are taken (`u09`–`u11`, `u21`,
  `u25`).

*Alternative:* splice the files' text into the main file. Rejected for the reason D8 of `m4-statements-metacommands`
gives: a tree is one file.

### D8. Recording with a copy of the release

The runner's verification suite, for a program with a sidecar `<name>.root` (one line, a folder of
`verification\`), records with `qb64pe.exe` in `target\qb64pe-root\<folder>\`: a copy of the clone's
`qb64pe.exe`, a directory junction `internal` to the clone's `internal\`, and a copy of the named folder's
`libraries\`. `qb64pe.exe` finds `internal\` and `libraries\` beside itself, so nothing is written to the clone but
what every recording writes into `internal\temp`, which the clone's lock already guards; the runner takes it. The
copy is made again when the clone's `qb64pe.exe` or the folder changes (a stamp of their hashes). The new
compiler's compile gets `--include-root` of the copied folder in the run's own folder. Code:
`tools\legacy_tests\run_legacy_tests.py`, with a test for the sidecar in `test_run_legacy_tests.py`.

*Alternative:* a full copy of the release. Rejected: the toolchain under `internal\c\c_compiler` is large, and the
copy would have to be kept to the clone's version by hand.

### D9. The examples of "not supported yet"

The scenarios and front-end tests that use a built-in or metacommand of this change as their example of a mark
take a form left marked: `FOR` over an element of a member array (`v56_f3`), `VARPTR` of a `TYPE` that holds a
`STRING` in conventional memory (`v56_a5`), or `SUB _GL`. Affected: `tests\frontend\unsupported.bas` (`_SETBIT`,
`_TOGGLEBIT`), the parser test of `DO: FILES: LOOP` (`crates\syntax`), the snapshot `unsupported.check-fail.snap`,
and the metacommand test of `$ERRORLOCATION:ON`.

### D10. A tier 1 test over the whole table

`crates\sema\tests\every_builtin.rs` builds, for every function and statement entry of `qb64rust_builtins` that is
not a stub, one line from its syntax with arguments of its slot types (the probe of `proposal.md`, in Rust), and
checks the front end gives it no "not supported yet". The entries whose syntax needs a place of a kind (an array
for `_WAVE`) get the line by a small table in the test. A name that a later step takes (`SUB _GL`) is listed in
the test with its reason, so the list can only shrink.

## Risks / Trade-offs

- [`$ERRORLOCATION` places a call where the old compiler does not] → The call-site check compares the calls for a
  program of every statement kind; the slice programs compare the messages.
- [A critical error's message differs by the machine's stack] → The recorded line is the recursive call's, which
  does not depend on depth; the runner's normalisation masks the rest if `v58` shows it varies.
- [The junction of D8 fails on a file system without it] → The runner stops with a message naming the folder; no
  fallback copies the toolchain.
- [`$USELIBRARY` starts a round per library] → Rounds are cheap (`$COLOR` uses them); a library list of three
  files per library adds three trees.
- [`FILES` prints the free space of the disk] → The runner's normalisation masks the free-space line, recorded with
  a `.normalize` sidecar of the program.
- [A value that depends on the machine (`_DESKTOPREFRESHRATE`, `_UPTIME`, the lock keys)] → The program prints a
  comparison (`> 0`, `= 0 OR = -1`), not the value.
