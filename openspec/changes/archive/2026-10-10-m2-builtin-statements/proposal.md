# Proposal

## Why

The compiler has no built-in **statement**: the IR's operations are stores, `PRINT`, procedure calls and jumps, so
`OPEN`, `CLOSE`, `PRINT #`, `KILL`, `READ`, `SWAP`, `RANDOMIZE`, `INPUT` and every other built-in statement is "not
supported yet". This is step 9 of `STATUS.md` "Next" (`study\27` §3, accepted 2026-10-08), and it is what the corpus
waits for. Counted 2026-10-09 with the release build: 74 corpus programs with a recorded output are not on
`slice.list`; their marks are `KILL` 23, `OPEN` and `CLOSE` 22 each, `PRINT #` 16, `DATA` and `READ` 10 each, `LINE
INPUT` 9, then `RESTORE`, `INPUT`, `WRITE`, `MKDIR`, `RMDIR`, `RND`, `RANDOMIZE` 3 each and about 50 names once or
twice. About 60 of the 74 carry only marks this change removes (an upper bound: a statement's first mark can hide a
later one, and each program must still print what `qb64pe.exe` prints).

The change also holds the trial of the **call-site check** (`DECISIONS.md` 2026-10-09, `study\28` §3.2): whether
comparing the libqb call the new compiler emits with the old compiler's is stable enough to become the standard way
a plain built-in comes in.

## What Changes

- **Measurements before design**: `verification\v22_*` programs run with `qb64pe.exe`, and the old compiler's C++
  read (`qb64pe -z`), for every statement below: argument conversion, the order of evaluation, which points test
  for a pending error, the error numbers, and the forms the old compiler rejects.
- **A built-in statement operation in the IR**, the statement twin of a built-in function call: it names the
  built-in, with one slot per template argument (a value, a place, or absent) and the template words chosen. `sema`
  checks it with the table and a closed rule set, as it does functions; the emitter writes the call by its rule.
- **Sequential file I/O**: `OPEN` (both forms, every mode, access and lock word), `CLOSE`, `PRINT #`, `WRITE` (to a
  file and to the console), `INPUT #`, `LINE INPUT #`, `SEEK`; the functions `EOF`, `LOF`, `LOC`, `SEEK`,
  `FREEFILE`, `_FILEEXISTS`, `_DIREXISTS`, `_CWD$`; and `KILL`, `NAME`, `MKDIR`, `RMDIR`, `CHDIR`.
- **`DATA`, `READ`, `RESTORE`**: the program's data as the old compiler stores it, `READ` into every kind of place,
  `RESTORE` with and without a label.
- **Other statements and functions the corpus and Civil War Strategy name**: `SWAP`, the `MID$` statement,
  `RANDOMIZE` (with `USING`), `RND`, `TIMER`, `SHELL` (statement forms and function), `COMMAND$`, `ENVIRON$` and
  the `ENVIRON` statement.
- **Console `INPUT` and `LINE INPUT`** under `$CONSOLE:ONLY`, with literal prompts, as the old compiler. The corpus
  runner gets a `<name>.stdin` sidecar so that such a program can be recorded and checked.
- **Plain functions by demand**: after the groups above the corpus blockers are counted again, and a function a
  corpus, upstream or real program names is added when an existing rule fits it and its measurements agree. The
  candidates today: the trigonometric and hyperbolic family (`_ACOS`, `_ASIN`, `_SINH`, `_COSH`, `_TANH`, `_COT`,
  `_CSC`, `_SEC`, `_D2R`, `_R2D`), `_CEIL`, `_STRCMP`, `_STRICMP`, `CSRLIN`, `POS`.
- **The call-site check, as a trial**: a tool that compiles a one-statement program with both compilers (`-z`),
  takes the statement's libqb calls from each and compares them after normalising names and spacing. Built for the
  file I/O statements first; the change ends with a recorded verdict (kept as the standard way in, or dropped).
- **Corpus and lists**: new `tests\corpus\slice\s35…` programs recorded with `qb64pe.exe`; `slice.list` gains them
  and every corpus program that passes; upstream programs that pass go into `tests\upstream\pass.list`; the
  shrink-only lists of tier 1 are regenerated.

## Capabilities

### New Capabilities

- `language/builtin-statements`: built-in statements in general (how one is recognised and checked, argument
  conversion, the pending-error rule of a statement call) and the ones that belong to no narrower capability:
  `SWAP`, the `MID$` statement, `RANDOMIZE`, `SHELL`, `ENVIRON`, console `INPUT` and `LINE INPUT`.
- `language/file-io`: files by number: `OPEN`, `CLOSE`, `PRINT #`, `WRITE`, `INPUT #`, `LINE INPUT #`, `SEEK`, the
  file functions, and the file-system statements (`KILL`, `NAME`, `MKDIR`, `RMDIR`, `CHDIR`), with their runtime
  errors.
- `language/data-read`: `DATA`, `READ` and `RESTORE`.
- `testing/call-site-check`: the comparison of the libqb calls the two compilers emit for one statement.

### Modified Capabilities

- `language/builtin-functions`: the list of supported functions grows (`RND`, `TIMER`, `COMMAND$`, `ENVIRON$`, the
  file functions, `SHELL`, the by-demand functions); more functions are callable without parentheses.
- `compiler/pipeline`: the IR gains the built-in statement operation and the data of the program, with their error
  rules; the "whole language parses" scenario that uses `SWAP` as its example of a statement not compiled yet
  needs another example.
- `testing/compiler-tests`: the built-in coverage check also covers built-in statements.
- `testing/golden-corpus`: a `<name>.stdin` sidecar gives a program its standard input.

## Impact

- Changed: `crates\sema` (a statement side of `builtins.rs`; new checks for the I/O statements, `DATA`/`READ`,
  `SWAP`; the marks in `check\mod.rs` go away one by one), `crates\syntax` (`ast.rs` accessors for the I/O
  statements and `BuiltinStmt`, which have none yet; no change to the tree), `crates\ir` (new operations, the
  program's data, lowering, `validate`, `dump`), `crates\codegen-cpp` (statement calls, the data fragment, the
  file `PRINT`/`INPUT` forms), `crates\driver\tests`, `tools\legacy_tests\run_legacy_tests.py` (the `.stdin`
  sidecar), a new `tools\callsite\` script, `tests\corpus\slice.list`, `tests\upstream\pass.list`, the shrink-only
  lists; new `tests\corpus\slice\s35…`, `tests\frontend\*.bas`, `verification\v22_*`.
- No change to the build path, the CLI, the parser's tree or the reference clone (`qb64pe -z` writes only into the
  clone's git-ignored `internal\temp`).
- Not in this change: random and binary file access (`GET`, `PUT`, `FIELD`, `LSET`, `RSET`, `LOCK`, `UNLOCK`; `OPEN`
  accepts their modes, the statements stay marked), `INPUT$`, `PRINT USING`, `LPRINT`, `WIDTH`, `TAB`/`SPC`,
  `CHAIN`, `RUN`, `FILES`, `_IIF`, `_MIN`, `_MAX`, `_CLAMP`, `_SHL`/`_SHR`/`_ROL`/`_ROR`, `EXIT SELECT`, line-number
  targets (`RESTORE 100`), `READ` or `INPUT` into a whole array or `TYPE`, expression prompts for `INPUT`
  (`SOMEDAY.md`), `INPUT` in a program without `$CONSOLE:ONLY`, every screen, sound and window built-in (after
  step 9a), `DEFxxx` and the rest of arrays (step 11).
- `STATUS.md`, `crates\README.md`, `tests\corpus\README.md`, `tests\upstream\README.md`, `study\00` §5 (measured
  facts), `DECISIONS.md` (the verdict on the call-site check; any divergence decided), `DIVERGENCES.md` and
  `SOMEDAY.md` updated when done.
