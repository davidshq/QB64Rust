# Design

## Context

See `proposal.md` for why. The rulings are `study\39` §2, and the switch is `DECISIONS.md` 2026-10-26 ("Fixes of
QB64pe's kept behaviours go behind a switch"). What shapes the approach:

- The front end runs in one place, `driver` `check_program` (`crates\driver\src\lib.rs`). The CLI, the language
  server's three hooks (`check_for_server`, `format_for_server`, `index_for_server`) and the debugger's build all go
  through it, and each knows the main file's path on disk when there is one. `sema` takes its settings from
  `check_with(map, parsed, fold)` into `Settings` (`crates\sema\src\check\mod.rs`). `Program` carries them to the
  IR, and the IR to `codegen-cpp`.
- `driver` `build.rs` `build_key` hashes the fragments `codegen-cpp` writes, so a fix that changes the C++ already
  changes the key. A fix that only adds a compile error never reaches a build.
- The test runner (`tools\legacy_tests\run_legacy_tests.py`) starts `qb64rust` in two places, with `-x -q -m`
  (lines near 1668 and 2091 today).
- The language server has no handler for files changed on disk. The extension's `LanguageClientOptions`
  (`vscode\src\languageServer.ts`) declares no file watcher.
- RESUME in a SUB is written as a bare `return` (`codegen-cpp` `stmt.rs`, `Op::Resume`). `RUN` from a procedure
  calls `QBMAIN(NULL)` inside it (`stmt.rs`, `Op::Run`). A library's `STRING` argument is `(char*)(place)->chr`
  (`library.rs` `library_args`).

## Goals / Non-Goals

**Goals:**
- One fix set per build, worked out once in the driver, seen in the same way by every later stage and by the
  language server.
- A fix that is off leaves the generated C++ byte for byte as it is today. The call-site check, the suites and the
  build cache then need nothing new for the off state.
- The always-on fixes and the switched ones are built with the same mechanisms. "Always" is a fix with no name.

**Non-Goals:**
- The libqb rows (2, 5, 9, 11, 12, 17, 18): the second change, which reuses D1 to D4 unchanged.
- A per-file or per-region fix (a metacommand such as `$FIX:`). The decision puts fixes in the project's file and
  on the command line only.
- Settings in `qb64rust.toml` other than `[fixes]`. The file is read for that table alone, and other tables are
  ignored so a later change can add some.

## Decisions

### D1: The fix set is a type of `base`, filled by the driver

`qb64rust_base::fixes` holds `enum Fix` (one variant per name), a table of name and one-line description per
variant (for `--list-fixes` and the error messages), and `struct Fixes` (a set, `Copy`, with `Fixes::OFF`,
`Fixes::ALL` and `is_on(Fix)`). `sema`, `ir` and `codegen-cpp` already depend on `base`, so none of them gains a
dependency.

The driver's new module `fixes.rs` does three things. It finds the file by walking up from the main file's folder
(`find_settings(path) -> Option<PathBuf>`). It reads the file (`read_settings(path) -> Result<Fixes, SettingsError>`,
where the error carries file, line and message). It applies the command line (`apply_cli(base, &[CliFix]) ->
Fixes`: `--fixes` replaces, then each `--fix` sets one). `check_program` takes a `Fixes` argument. `main.rs` parses
the two options and `--list-fixes`, resolves the set before the front end runs, and on a `SettingsError` prints
`<file>:<line>: <message>` and exits with status 1.

`sema` `Settings` gains `fixes: Fixes`, and `check_with` takes `fixes` beside `fold` (which stays its own
argument, since it is a test switch). `Program` gains `fixes: Fixes`. The lowering copies it to `ir::Program`,
where the emitter reads it.

*Alternatives:* (a) An environment variable read deep in each stage, as `QB64RUST_NO_FOLD` is. Rejected: the
language server is one process for many programs, so the set must travel with the program. (b) Passing the set to
each stage's entry point separately. Rejected: three signatures to keep in step, while `Program` already carries
`sema`'s other settings (console, `$ASSERTS`) the same way.

### D2: `qb64rust.toml` read with the `toml` crate

The file is TOML. A hand-written reader of `[fixes]` and `key = bool` would accept and reject a different
language from the one users and editors know (quoted keys, comments after values, inline tables, dotted keys). The
`toml` crate (with `serde`'s derive left out: the file is read as a `toml::Table` and walked) gives exact errors
with line and column. It goes into `[workspace.dependencies]` and is used by `driver` only. A `DECISIONS.md` row
records the dependency, as `sha2`'s did.

Rules, as in spec `compiler/fixes`: only `[fixes]` is read; `all` is the master; each other key must be a fix name;
every value must be a boolean. A file found but unreadable (permissions) is an error too, not "off", so a user who
turned the fixes on never gets them off by accident.

*Alternative:* our own reader of the subset. Rejected for the reason above. The crate is small, and its licence
(MIT/Apache-2.0) is among those `tools\licenses\` already accepts.

### D3: The language server: errors on line 1, and a watched file

`check_for_server`, `format_for_server` and `index_for_server` call `fixes::for_program(main_path)`, which finds
and reads the file with no command line. A `SettingsError` becomes an error diagnostic at line 1, column 1 of the
main file, with the message `qb64rust.toml:<line>: <message>`. The front end then runs with the fixes off, so the
program's other diagnostics still show. A document that is not a file gets `Fixes::OFF`.

The server registers, in `initialize`'s answer, nothing new. The extension's `LanguageClientOptions` gains
`synchronize.fileEvents: workspace.createFileSystemWatcher("**/qb64rust.toml")`, and `crates\lsp` handles
`workspace/didChangeWatchedFiles` by dropping every cached check result and index and queueing a check of each
open document that has been checked, as after an edit. The settings are not cached in the server: each check reads
the file again. That costs one small file read per check, and the watcher is only a trigger to recheck. If the
watcher misses an event, the next edit still sees the new file.

*Alternative:* the server watches the file system itself (`notify` crate). Rejected: another dependency, and VS
Code's watcher already covers the workspace.

### D4: The build cache and the runner

`build_key` writes the fix set's bits after its version tag, which goes to `"qb64rust build cache 7"`. Every fix
here that changes run-time behaviour changes a fragment anyway; the explicit bits keep the key honest for a later
fix that might change only a make flag or a libqb define (the second change's `bit-number-check` may).

The runner passes `--fixes off` in every `qb64rust` command it builds, and its first line names it ("qb64rust,
fixes off"). For a verification program with `tests\verification\fixed\<name>.out.txt`, it builds the program a
second time into `<name>.fixed\` with `--fixes on`, runs it as the first run was run, and compares it with the
sidecar by the same rules (`.normalize` applied, folder masked). It reports `<name> [fixes on]` as a result of its
own. A `fixed\` sidecar is written by hand from the fix's specified behaviour and checked against QuickBASIC 4.5
where `study\39` measured it. It is never recorded by `qb64pe.exe`. `pass.list` and `known.list` stay one line per
program. A failing `[fixes on]` run fails the program's line, and the runner's summary says which half failed.
Tier 1's `verification_lists_are_exact` also checks that every `fixed\` sidecar names a program of the suite.

**Deterministic builds** (added during the close, `DECISIONS.md` 2026-10-27): the close's first runs skipped none of
their programs although most programs' C++ was unchanged, because each build wrote a timestamp into the program's
objects and executable (and `__DATE__`/`__TIME__` into every program), so the new key above rebuilt every executable
with new bytes and the runner's pass cache, which keys on them, missed. The program makefile now gives the program's
objects `-mno-incremental-linker-compatible` and the link `-Wl,--no-insert-timestamp`, and `main.txt` uses the
build's date and time only in a program that calls `_COMPILEDATE$` or `_COMPILETIME$` (never cached); the key goes to
version 8. Two builds of a program with no cache are then the same bytes (CLI test `builds_are_deterministic`).

### D5: Always-on fixes in the emitter

Each of the following is written once and used by every statement that needs it. Every one has a
`QB64PE-DIVERGENCES.md` row, and where the program ends up on a path QB64pe never takes, the divergence trace
(`qbr_diverged(n)`, `m3-divergence-trace`) reports it.

- **Row 3, a count below zero** (`codegen-cpp` `arrays.rs` `allocate_with`): before the allocation, when a
  dimension's upper bound is less than its lower bound minus one, `error(5)` is raised and nothing is allocated. A
  count of exactly zero keeps today's path unless `dim-empty-error` is on (D6).
- **Row 4, `STATIC` beside a shared static array** (`sema` `check\decl.rs`, where `STATIC name(bounds)` resolves
  the name against the shared arrays): a static shared array of that name and type no longer captures the
  `STATIC`. The procedure gets its own static array, as when the types differ. The answer is stored in the
  procedure's scope, so every later use, and the language server's index, takes it.
- **Row 7, a whole array with no storage** (`codegen-cpp` `arrays.rs`, one new helper `whole_data(array)` that
  every whole-array use calls in place of reading the descriptor's data pointer: `PRINT a()`, `READ`, `INPUT #`,
  `LINE INPUT #`, `SWAP`, `FIELD`, `LSET`, `RSET`, whole-array `GET`/`PUT`). It tests the descriptor as
  `copy_whole` already does (not allocated, or data equal to `nothingvalue`), raises `error(9)`, and the statement
  then does nothing. The test is new C++ only where a whole array is used, so the call-site check is untouched.
- **Row 8, `ERASE` of a dynamic array** (`arrays.rs` `release_array`): the lock renewal `REDIM` writes moves into a
  helper that both call, so `ERASE` invalidates the array's blocks. The other release paths (`CLEAR`, `RUN`, a
  SUB's local dynamic array at its end) are measured first (task 1). Each that fails the same way calls the helper
  too.
- **Row 10, `STRING * n` to `char*`** (`library.rs` `library_args`): for an `Arg::Ref` whose place is a member or
  an element of fixed-length string type, the call is wrapped. The place's address is taken once into a temporary
  pointer, n + 1 bytes are copied into a buffer with a NUL, the buffer is passed, and the n bytes are copied back
  after the call. The C++ statement is a block `{ … }` around the call, so a FUNCTION call in an expression uses a
  lambda written the way `value.rs` already writes statement-expressions. A variable of its own keeps
  `(char*)(s)->chr`.
- **Row 13, `RESUME` leaving a SUB** (`stmt.rs` `Op::Resume`): in a procedure (`self.k != 0`), `return;` becomes
  `goto <exit label>;`, the label that `EXIT SUB` jumps to, so the procedure's exit code runs (strings freed, locks
  released).
- **Row 14, `RUN` from a procedure** (`stmt.rs` `Op::Run`, `procs.rs`): the old `QBMAIN(NULL)` from inside the
  procedure is replaced by an unwind. `RUN` in a procedure stores its target in `qbr_run_target` (0 for the top, or
  the `runline` number), sets `qbr_unwinding = 1` and `qbevent = 1`, and goes to the procedure's exit label. At the
  event check of each statement boundary, after each statement below `$CHECKING:OFF` (which has no event check),
  and at each procedure's start (so that a FUNCTION called after the one that ran `RUN`, in the same expression,
  leaves at once; added by the review, `verification\v62_g4`, `g5`), the emitter adds `if (qbr_unwinding) goto
  <exit label>;` in a procedure, and in the main module a jump to a new block that runs the
  `RUN` as the main module already does (`sub_run_init(); sub_clear(…); goto` the target). This is the path `END`
  from a procedure already takes, if it unwinds (checked in task 1; if `END` does not unwind, this adds the
  mechanism and `END` is left as it is). *Read in task 1.3:* `END` does not unwind; `Op::End` writes `sub_end();`,
which ends in libqb's `end()` and sleeps there until the process exits, so no procedure is left and this change
adds the unwind. **`RUN` in a `$CONSOLE:ONLY` program**: after `sub_run_init()`, which sets
  screen 0, the emitter adds the same selection of the console as output that the program's start makes (read from
  the C++ of the start in task 1; expected `sub__dest(func__console())` and `sub__source(…)`).
- **Row 16, `_CAST` from a float** (`sema` `check\mem.rs` `cast`, the constant folder; `codegen-cpp` `value.rs`): a
  helper per target width, `qbr_cast_f_to_i32(double)` and so on, in the generated prelude, does what x86-64's
  `cvttsd2si` gives a variable (out of range: the "integer indefinite" value, the narrow types through the low bits
  of a 32-bit conversion, as `v49_a_cast` measures). Every `_CAST` from a float calls it, so a constant argument no
  longer reaches undefined C++. The folder computes the same value in Rust, so folding and `QB64RUST_NO_FOLD=1`
  agree.

### D6: Switched fixes

Each switched fix is an `if fixes.is_on(Fix::…)` at one place:

- `resume-after-block` (`ir` `lower.rs`, where block headers are lowered): the header's statement gets a
  resume-next target, the block's end label. `Resume::Next` serviced at that statement goes there instead of to the
  statement after the header. The IR names it as a field of the statement boundary (`next: Option<Label>`), and the
  emitter writes the resume's jump to it. The module comment of `crates\ir\src\lib.rs` states the rule with the fix
  on, beside the measured one.
- `dim-empty-error` (`arrays.rs` `allocate_with`): the row-3 test becomes "upper below lower", the zero count
  included. A statement that raised while its bounds were evaluated tests `is_error_pending()` before allocating,
  and allocates nothing then.
- `static-local-dynamic` (`sema` `check\decl.rs`, the same place as row 4): a shared dynamic array no longer
  captures the `STATIC` either.
- `whole-array-value` (`sema` `check\whole.rs`, where a whole array is accepted as a value): a compile error at the
  `a()`, with the message "a whole array is not a value; name an element, as in a(0)". `LEN(a())` and the variable
  places of the statements stay accepted.
- `resume-in-procedure`, `run-in-procedure` (`sema` `check\flow.rs`): a compile error at the statement inside a
  SUB or FUNCTION. The note of `run-in-procedure` says "set a flag and RUN from the main module".
- `checking-on-service` (`ir` `lower.rs` and `codegen-cpp` `stmt.rs`, where `$CHECKING:ON` regions start): the
  first checked statement after unchecked ones gets an error check written before its body. That check services a
  pending error as at the boundary of the statement before, so `RESUME NEXT` goes on at the checked statement and
  `RESUME` runs it.

With every fix off, no `if` changes the output. Task 2 checks this by building the corpus's C++ before and after
with `-z` and comparing it byte for byte.

### D7: The extension's New Project command

`vscode\src\newProject.ts`: `qb64rust.newProject` shows `window.showOpenDialog` (folders only), refuses when
`main.bas` or `qb64rust.toml` exists there (a message names the file), writes both, and opens `main.bas`. The
`qb64rust.toml` it writes:

```toml
# qb64rust settings for this program (fixes: qb64rust --list-fixes).
[fixes]
all = true
```

`main.bas` is `PRINT "Hello, world!"`. The command is contributed in `package.json` and works in an untrusted
workspace (it builds nothing). Tests: an extension test with the dialog stubbed, for an empty folder and a folder
holding `main.bas`.

### D8: Tests

- `base` unit tests: the name table round-trips, and `Fixes` is set as expected.
- `driver` unit tests of `fixes.rs`: walking up, the nearest file, no file, no `[fixes]`, `all` plus an override, a
  misspelt name (its message names the known fixes and the line), a non-boolean, invalid TOML, the command line
  over the file.
- CLI tests (`crates\driver\tests\cli.rs`): `--list-fixes`, `--fixes`/`--fix` errors, a dump with a fix (spec
  `compiler/cli`), the build cache with a changed fix.
- The language server's tests: an error in `qb64rust.toml` shown on line 1, a fix's compile error shown, and the
  recheck after `didChangeWatchedFiles`.
- Each always-on fix: a new verification program per row (family `v62`, "the fixes", recorded with `qb64pe.exe`
  first to keep QB64pe's side on record; the program then goes to `known.list` as `D-0nn`, or passes with its
  `.divergences` sidecar where the trace is the only difference). The rows' existing programs (`v54_d_whole_erased`,
  `v32_d29_erase_dynamic`, `v37_x66`, `v49_x18`, `v24_x272`, `v57_x01`–`x04`, `v57_r04`) move from `pass.list`
  to `known.list` with their `D-0nn` reason, or keep passing with a `.divergences` sidecar.
- Each switched fix: a `fixed\<name>.out.txt` sidecar for at least one program (D4): `v17_*` block headers for
  `resume-after-block`, `v24_b_dynamic` for `dim-empty-error`, `v55_d_static_shared` for `static-local-dynamic`,
  `v50_c_checking` for `checking-on-service`. The three compile-error fixes get CLI tests instead (a sidecar records
  output, not a compile failure).
- The call-site check, the corpus, upstream and qbasic suites: unchanged with the fixes off. That is the evidence
  for D6's "byte for byte".

## Risks / Trade-offs

- [The unwind of row 14 adds a test at every procedure's start and after every statement below `$CHECKING:OFF`]
  → It is one `if` on a global, written only in programs that contain `RUN` inside a procedure. Programs without
  one get no new code, which also keeps the call-site check's programs unchanged.
- [The statement whose expression called the procedure that ran `RUN` ends before the unwind reaches the main
  module] → As after an error raised in a FUNCTION: `PRINT f(1) + g(2)` still prints, with `g` left at its start
  (D-046, `verification\v62_g4`). Stopping it would split every expression at its calls; a test after only a SUB's
  `CALL` let an unchecked loop that calls a FUNCTION run forever (`v62_g5`), which is why the test stands after
  each unchecked statement.
- [Row 10's copy breaks a C function that keeps the pointer after it returns] → It never worked for a member
  (QB64pe passes the place's own bytes, and nothing about them after the call is defined). The divergence row says
  so.
- [A `qb64rust.toml` in a parent folder the user forgot about turns fixes on] → `qb64rust` prints nothing about
  the file on a normal build, but `qb64rust -w` names the file used (a warning-level line, "fixes from <file>"),
  so a surprise has a visible cause.
- [The `toml` crate's errors are worded by the crate] → Our message wraps them with the file and line. The wording
  table of old errors is not involved, since QB64pe has no such file.
- [Two runs of each switched-fix program in the verification suite] → Four or five programs, a few seconds per
  full run.

## Migration Plan

Nothing to migrate. A program with no `qb64rust.toml` builds as today, apart from the always-on fixes, which only
change programs that crashed, hung, or wrote memory they did not own. Each such change is a `QB64PE-DIVERGENCES.md`
row, and `vscode\CHANGELOG.md` gets a line on the fixes and the New Project command.

## Open Questions

- Whether `END` from a procedure already unwinds (D5, row 14). Task 1 reads it, and the answer changes only where
  the shared code lives, not the behaviour.
- The exact error number that a block of an array gives after `REDIM` (row 8). It is measured in task 1 and goes
  into the scenario's recording, not into any choice made here.
