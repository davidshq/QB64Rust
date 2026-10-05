# Proposal

## Why

The first slice (`m2-workspace-and-slice`) proved that bytes → lossless tree → typed tree → ABI-neutral IR →
C++ works for a main module of straight-line statements. Two parts of the IR design are still untested, and both
are about the riskiest bet (`study\15` §2):

- **Procedures and by-reference arguments.** The IR has no procedures, so nothing yet shows that it can describe
  pointer parameters, by-value temporaries, per-call locals, `STATIC` and `SHARED` storage, and string parameter
  copies without naming C types or libqb symbols, while the emitter still produces what `qbx.cpp` expects
  (`mainK.txt`, `dataK.txt`, `freeK.txt`, `regsf.txt` prototypes).
- **Raise and resume.** The IR states "errors are handled per statement; `RESUME` re-runs, `RESUME NEXT` continues
  after, the statement", but nothing in the slice raises, so the rule has only a `cpp` snapshot behind it. The
  pipeline spec's scenario "Error inside a PRINT" is still not pinned end to end (task 1.4 of the last change).

The corpus already has 18 programs that need exactly these features (12 with SUBs, 6 with `ON ERROR`), and 17
programs that pass today but are not in `slice.list` yet (`STATUS.md`).

## What Changes

- **`slice.list` grows** from 14 to 31 programs (the 17 that already pass), then to every corpus program this
  change makes pass; the tier-1 count follows the list instead of a hard-coded 14.
- **Procedures**: `SUB name [(params)]` … `END SUB` and `FUNCTION name[suffix] [(params)]` … `END FUNCTION` with
  scalar parameters (`name[suffix] [AS type]` for the slice's types); `CALL name[(args)]` and the call without
  `CALL` (`name args`); function calls in expressions (with and without an argument list); assignment to the
  function's name for its result; `EXIT SUB`, `EXIT FUNCTION`; `DECLARE SUB/FUNCTION` lines (parsed, otherwise
  ignored, as the old compiler does); local variables (`DIM` and implicit), `STATIC name AS type` inside a procedure,
  `SHARED name AS type` inside a procedure, `DIM SHARED` in the main module. Arguments are by reference when they
  are a plain variable of exactly the parameter's type, otherwise a by-value temporary (no copy-back), as measured.
- **Error handling**: labels (`name:` at the start of a statement, main module only), `ON ERROR GOTO label`,
  `ON ERROR GOTO 0`, `RESUME`, `RESUME 0`, `RESUME NEXT`, `RESUME label`, `ERROR n`, and the functions `ERR` and
  `ERL`; `ON ERROR GOTO` inside a procedure naming a main-module label. One more built-in, `CHR$`, because it raises (error 5 outside 0–255): it lets the slice show retry and
  skip end to end without control flow.
- **Reserved names**: a variable, parameter or procedure may not be named after a built-in or keyword (the old
  compiler rejects `SUB s (name AS STRING)`, corpus 48); the exact rule as measured (`verification\v14_res_*`).
- **Symbol table and typed tree accessors** (`study\20` §3.5, added 2026-10-04): `sema` records every variable,
  parameter, procedure and label with its definition and reference spans, so a later language server can answer
  hover, go-to-definition and references without a retrofit; `sema` reads the syntax tree through thin typed
  accessors instead of child positions. Added here because this change introduces scopes and triples `sema`.
- **IR**: procedures, storage classes, call arguments by reference or by value, procedure exit, labels, the error
  handler, raise and the three resume forms, all without C or libqb names.
- **Emitter**: `mainK.txt`, `dataK.txt`, `freeK.txt`, `regsf.txt` prototypes, `mainerr.txt` dispatch, labels and the
  resume code, as measured from `qb64pe -z`.
- **Corpus**: new `tests\corpus\slice\` programs for by-reference rules, functions, scopes, error handling and
  errors inside a PRINT, recorded with `qb64pe.exe`.
- **Tier 1** also requires every corpus program with an `.err` file to get at least one diagnostic, so supporting
  more of the language can never turn a rejected program into an accepted one unnoticed.

## Capabilities

### New Capabilities

- `language/procedures`: SUB and FUNCTION definitions and calls, argument passing (by reference, by-value
  temporaries), scopes (`DIM`, implicit, `STATIC`, `SHARED`, `DIM SHARED`), function results, `EXIT`, `DECLARE`,
  reserved names.
- `language/error-handling`: labels, `ON ERROR GOTO`, the `RESUME` forms, `ERROR`, `ERR`, `ERL`; what is retried or
  skipped; errors raised inside procedures; errors that cannot be trapped.

### Modified Capabilities

- `compiler/pipeline`: the ABI-neutral IR requirement covers procedures and call arguments; "Error inside a PRINT"
  becomes an end-to-end scenario; new requirement: the symbol table.
- `testing/compiler-tests`: tier 1 checks that `.err` programs stay rejected; the slice-list count is not fixed.

## Impact

- Changed: `crates\syntax` (block parsing, new statements, labels, typed accessors in `ast.rs`), `crates\sema`
  (procedure table, scopes, calls, symbol table), `crates\ir`, `crates\codegen-cpp`, `crates\driver\tests\corpus.rs`, `tests\corpus\slice.list`; new
  `tests\corpus\slice\s08…`, `tests\frontend\*.bas` files, measurements in `verification\`.
- No change to the build path, the CLI or the reference clone.
- Not in this change: `GOTO`, `GOSUB`, `IF` and other control flow; numeric line labels (so `ERL` is always 0);
  labels inside procedures; arrays and array parameters; `BYVAL`, `OPTIONAL`, `STATIC` on a procedure header;
  `DEF FN`; `$ERRORLOCATION`; `_ERRORLINE` and the other error functions; any built-in beyond `INSTR`, `CHR$`, `ERR`, `ERL`,
  `ERROR`.
- `STATUS.md`, `crates\README.md`, `tests\corpus\README.md`, `CLAUDE.md` (layout, decisions) updated when done.
