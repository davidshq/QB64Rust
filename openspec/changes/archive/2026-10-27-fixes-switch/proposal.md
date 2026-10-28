# Proposal

## Why

On 2026-10-26 the user decided that fixes of QB64pe's kept behaviours go behind a switch, and ruled the 19
crash-class rows of "QB64pe behaviours to review" one at a time (`DECISIONS.md` 2026-10-26, the fixes switch;
`study\39`). Nothing of it is built. Until it is, a program can still end with no handler, hang, or write into
memory it does not own, in each of the ways those rows describe. A user who wants QuickBASIC's stricter
behaviour also has no way to ask for it.

The rulings fall into two halves by where the code changes: the compiler (`sema`, the IR, `codegen-cpp`) or
libqb (edits through `tools\libqb\edits`). This change builds the switch and the compiler half. The libqb half
(rows 2, 5, 9, 11, 12, 17 and 18) is a second change, which adds its switched fix (`bit-number-check`) to the
switch built here.

## What Changes

- **The fixes switch.** Each fix has a name. A master switch turns them all on or off, and a setting per fix
  overrides it. They are set in `qb64rust.toml`, the first one found walking up from the `.bas` file's folder:

  ```toml
  [fixes]
  all = true
  resume-after-block = false
  ```

  The command line overrides the file: `--fixes on|off` and `--fix <name>=on|off`. A program with no
  `qb64rust.toml`, or with no `[fixes]` table, builds with every fix off. An unknown fix name, or a value other
  than `true`/`false`, is an error that names the file and the known names. `qb64rust --list-fixes` prints each
  name with a line on what it does.
- **The language server reads the same file**, through the driver's check, so the compile errors a fix adds (rows
  6, 13 and 14) show while editing. It checks again when a `qb64rust.toml` changes. **The extension** gets a
  command, "QB64: New Project", which writes a folder with `main.bas` and a `qb64rust.toml` that turns the fixes on
  (a new project defaults to on, an existing program to off).
- **The test runner builds with `--fixes off`**, so a `qb64rust.toml` in the tree never changes a suite's result.
  It runs a program that has a `fixed\<name>.out.txt` sidecar a second time with `--fixes on` and compares that run
  with the sidecar, so each switched fix has a program recorded both ways.
- **Always-on fixes from the compiler** (no switch). Each gets a `QB64PE-DIVERGENCES.md` row and is measured under
  `qb64pe.exe` first, where `study\39` §3 asks:
  - row 3: `DIM`/`REDIM` with a count below zero raises error 5 instead of "Out of memory", which no handler
    receives;
  - row 4: `STATIC a(5)` in a SUB beside a shared static array of that name and type declares the SUB's own
    array, instead of "Duplicate definition" at start;
  - row 7: a whole array with no storage used whole (`READ a()`, `SWAP`, `PRINT a()`, whole-array `GET`/`PUT`,
    `INPUT #`) raises error 9 instead of writing over libqb's `nothingvalue`;
  - row 8: `ERASE` of a dynamic array invalidates its `_MEM` blocks, as `REDIM` does;
  - row 10: a `STRING * n` member or array element passed to a library's `char*` is copied in with a NUL and
    copied back;
  - row 13: `RESUME` in a SUB leaves it through its normal exit path, so its strings and locks are released;
  - row 14: `RUN` from a SUB or FUNCTION leaves the active procedures first, and `RUN label` in a
    `$CONSOLE:ONLY` program keeps its output on the console;
  - row 16: `_CAST` from a float gives what the same conversion of a variable gives, constants included.
- **Switched fixes** (off unless turned on), each a named fix:
  - `resume-after-block` (row 1): `RESUME NEXT` after an error in a block's condition goes on after the block;
  - `dim-empty-error` (row 3): a count of zero, or a bound that raises, is error 5 with nothing allocated;
  - `static-local-dynamic` (row 4): `STATIC a(5)` beside a shared dynamic array is the SUB's own;
  - `whole-array-value` (row 6): a whole array `a()` used as a value is a compile error;
  - `resume-in-procedure` (row 13): `RESUME` in a SUB or FUNCTION is a compile error;
  - `run-in-procedure` (row 14): `RUN` in a SUB or FUNCTION is a compile error with a hint;
  - `checking-on-service` (row 19): an error raised under `$CHECKING:OFF` is serviced before the first checked
    statement, so `RESUME NEXT` goes on at it.
- **A new dependency**, the `toml` crate, to read `qb64rust.toml`.

Not in this change: the libqb rows (2, 5, 9, 11, 12, 17, 18) and their fix `bit-number-check`; the candidates of
`study\39` §4; a settings UI for the fixes in the extension; fixes from the other rows of "QB64pe behaviours to
review".

## Capabilities

### New Capabilities
- `compiler/fixes`: the fixes switch: the names, `qb64rust.toml` and how it is found, the command-line options
  and their precedence, the defaults, `--list-fixes`, the errors for a bad file.
- `editor/new-project`: the extension's "QB64: New Project" command and the `qb64rust.toml` it writes.

### Modified Capabilities
- `compiler/cli`: the options `--fixes`, `--fix` and `--list-fixes`.
- `editor/language-server`: the check reads `qb64rust.toml` and runs again when it changes.
- `language/error-handling`: `resume-after-block`, `resume-in-procedure`, and `RESUME` in a SUB leaving through
  the exit path.
- `language/control-flow`: `RUN` from a procedure unwinds first; `RUN label` in a `$CONSOLE:ONLY` program keeps
  its output; `run-in-procedure`.
- `language/run-time-checks`: `checking-on-service`.
- `language/arrays-and-types`: rows 3, 4, 6 and 7 (counts below one, `STATIC` beside a shared array, a whole
  array as a value, a whole array with no storage).
- `language/mem-blocks`: `ERASE` of a dynamic array invalidates its blocks.
- `language/external-libraries`: `STRING * n` members and elements passed to `char*`.
- `language/builtin-functions`: `_CAST` from a float.
- `testing/verification-suite`: `--fixes off` for every build, and the `fixed\` recordings run with the fixes on.

## Impact

- `crates\driver`: finding and reading `qb64rust.toml`, the new options, `--list-fixes`; the fix set goes to
  `sema` through `check_program`, so the CLI, the language server and the debugger's build all see it. The build
  cache key gets the fix set.
- `crates\sema`: the fix set in its settings; the compile errors of `whole-array-value`, `resume-in-procedure` and
  `run-in-procedure`; `STATIC` name resolution (row 4); `_CAST` folding (row 16). `Program` carries the set to the
  later stages.
- `crates\ir`: the lowering of `resume-after-block` and `checking-on-service`; the module comment's statement of
  the error rules gains the two switched variants.
- `crates\codegen-cpp`: the helpers of rows 3, 7, 8, 10, 13, 14 and 16.
- `crates\lsp`: watching `qb64rust.toml`.
- `vscode\`: the New Project command; `package.json` contributes it.
- `tools\legacy_tests\run_legacy_tests.py`: `--fixes off`, the `fixed\` sidecars.
- New verification programs for the always-on fixes and the `qb64pe.exe` measurements of `study\39` §3, recorded
  before the closing runs. `QB64PE-DIVERGENCES.md` gets one row per always-on fix and a table of the switched
  fixes.
- `Cargo.toml`: the dependency `toml`.
