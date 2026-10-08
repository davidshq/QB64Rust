# Proposal

## Why

The front end parses every program the old compiler accepts, but `sema` compiles only two built-in functions
(`INSTR`, `CHR$`), checked by hand one by one, and marks `SELECT CASE` and `ON … GOTO/GOSUB` "not supported yet".
The fifth review put a core built-ins tranche next, ahead of the language server and the type table (`study\26`
§6, `STATUS.md` step 6): the string and math built-ins need no unsigned types and no type table, and the value
model they target is stable since D10 of `m2-arrays-and-types`. The review also asked that `sema`'s checking of
built-ins become table-driven before more of them arrive (§8).

Counted 2026-10-07 with the release build (every program that does not pass yet, by its "not supported yet"
marks): of the 131 corpus programs still marked, **58 carry only marks this change removes** (string and math
built-ins, `SELECT CASE`, `ON … GOTO/GOSUB`), of the upstream programs **11** (`VAL`, `_TOSTR$`, `LEN`). These are
upper bounds: a statement's first mark can hide a later one, and each program must still print what `qb64pe.exe`
prints. Today: `slice.list` 131, full corpus 147 pass, upstream 34 of 279.

## What Changes

- **Measurements before design**: `verification\v20_*` programs run with `qb64pe.exe` for each built-in's typing,
  argument conversion, edge values and runtime errors, for which forms the old compiler rejects, and for
  `SELECT CASE` and `ON … GOTO/GOSUB` (selector evaluation, float cases against integer selectors, errors in
  selectors and cases, out-of-range `ON` values).
- **Table-driven built-ins in `sema`**: one checker for every supported built-in function, reading names, required
  suffixes, argument slots, optional slots and plain return types from the built-in table, with a closed set of
  typing and argument rules for the functions the old compiler special-cases (`LEN`, `VAL`, `ABS`, `INT`, `HEX$`,
  ...). `INSTR` and `CHR$` move onto it.
- **String built-ins**: `LEN`, `LEFT$`, `RIGHT$`, `MID$` (function), `ASC` (one and two arguments), `STR$`, `VAL`
  (also with a type argument, for the slice's types), `STRING$`, `SPACE$`, `LTRIM$`, `RTRIM$`, `_TRIM$`, `UCASE$`,
  `LCASE$`, `HEX$`, `OCT$`, `_BIN$`, `_TOSTR$`.
- **Math built-ins**: `ABS`, `SGN`, `INT`, `FIX`, `SQR`, `SIN`, `COS`, `TAN`, `ATN`, `LOG`, `EXP`, `CINT`, `CLNG`,
  `CSNG`, `CDBL`, `_ROUND`, `_PI`, `_ATAN2`, `_HYPOT`.
- **`SELECT CASE`** and **`SELECT EVERYCASE`** with numeric and string selectors, `CASE` lists, `a TO b`, `IS op
  e`, `CASE ELSE`; lowered to the IR's flat jumps under the existing pending-error rule.
- **`ON n GOTO` and `ON n GOSUB`** to labels (line-number targets stay marked with line numbers).
- **Emitter**: `codegen-cpp\src\lib.rs` split by concern before the built-ins land (no change in output), then
  the emission of each special-cased built-in.
- **Corpus and lists**: new `tests\corpus\slice\` programs recorded with `qb64pe.exe`; `slice.list` gains them and
  every corpus program that passes; upstream programs that pass go into `tests\upstream\pass.list`; the shrink-only
  lists of tier 1 are regenerated.

## Capabilities

### New Capabilities

- `language/builtin-functions`: built-in functions in expressions: which are compiled, how each argument is
  converted to its slot, the type of each result, required suffixes, optional arguments, and the runtime errors
  they raise.

### Modified Capabilities

- `language/control-flow`: adds `SELECT CASE` / `SELECT EVERYCASE` and `ON n GOTO` / `ON n GOSUB`.
- `compiler/pipeline`: the scenario of "The whole language parses" that uses `SELECT CASE` as its example of a
  construct not compiled yet needs another example.
- `testing/compiler-tests`: every built-in function `sema` compiles is covered by a slice program and a `typed`
  front-end test, checked in tier 1.

## Impact

- Changed: `crates\sema` (a built-ins module replacing the hand checks in `check\expr.rs`; `check\blocks.rs` for
  `SELECT`; `check\mod.rs` for `ON … GOTO`), `crates\ir` (lowering of `SELECT` and `ON … GOTO/GOSUB` to the existing
  operations, design D7, D8), `crates\codegen-cpp` (split into modules; built-in emission), `crates\builtins` (helpers
  for slot types, if needed), `crates\driver\tests\inputs.rs`, `tests\corpus\slice.list`, `tests\upstream\pass.list`,
  the shrink-only lists in `tests\`; new `tests\corpus\slice\s25…`, `tests\frontend\*.bas`, `verification\v20_*`.
- No change to the build path, the CLI, the parser's tree or the reference clone.
- Not in this change: `_IIF`, `_MIN`, `_MAX`, `_CLAMP` (type-generic special forms; `_IIF` evaluates only one
  branch), `TAB`/`SPC` (`PRINT`-only), `PRINT USING`, the `MID$` statement, `RND`/`RANDOMIZE`/`TIMER`, `_CAST`,
  `CVx`/`MKx$`, file and screen built-ins, unsigned and `_BIT` arguments, built-ins inside `CONST` (still marked),
  and the "keep" decisions of `study\00` §6 (step 8 of `STATUS.md`). The remaining ~250 built-ins are step 9.
- `STATUS.md`, `crates\README.md`, `tests\corpus\README.md`, `tests\upstream\README.md`, `study\00` §5 (measured
  facts), `DIVERGENCES.md` and `DECISIONS.md` (if a divergence is decided) updated when done.
