# Proposal

## Why

Control flow is the hardest back-end question left, and the IR has not met it: the IR is a 1:1 mirror of the typed
tree, a label is "a position before statement N of a flat list" and "the IR has no jumps"
(`crates\ir\src\lib.rs`). That cannot express a label inside an `IF` or `FOR` body, a `GOSUB`, or what the old
compiler does after an error in a block header (`study\23` §2.1). The third review moved this slice ahead of the
rest of parser breadth so that the IR is tested before more is built on it, and the IR review is held right after
this change (`study\23` §4, step 3).

The block nodes and their accessors exist since `m2-parser-breadth` group 6; `sema` only marks them. Counted
2026-10-05 with the release build over the corpus (275) and the upstream programs outside `deferred.list` (265
with an expected result): **53 corpus and 12 upstream programs are blocked only by the constructs of this change**
(including all operators). Today 54 corpus programs and 10 of 279 upstream programs pass. Adding `SYSTEM n` or line
numbers would unblock none more, so both stay out; `SELECT CASE` would add 6 corpus programs and stays in step 8 of
the order of work.

## What Changes

- **Operators**: the comparisons `=`, `<>`, `<`, `>`, `<=`, `>=` (numbers and strings), `NOT`, `AND`, `OR`,
  `XOR`, `EQV`, `IMP`, `_ANDALSO`, `_ORELSE`, `_NEGATE`, `\`, `MOD` and `^`, with the old compiler's types,
  rounding of float operands and results (`study\02` §1.4; probed 2026-10-05).
- **Blocks compiled**: `IF … THEN … ELSE` on one line (including `IF c GOTO label`), block `IF` with `ELSEIF` and
  `ELSE`, `FOR … NEXT` with `STEP` (the old compiler's temporaries, evaluated once, wider than the variable),
  `DO … LOOP` with `WHILE`/`UNTIL` at either end, `WHILE … WEND`, `EXIT FOR`, `EXIT DO`, `EXIT WHILE`. The check
  of `NEXT` variables against their `FOR` (left to this change by `m2-parser-breadth` D4).
- **Jumps**: `GOTO label`, `GOSUB label`, `RETURN`, `RETURN label`; labels inside blocks and inside procedures
  (per body, as in the old compiler).
- **Errors in block headers**: what happens after an error in an `IF`/`ELSEIF`/`WHILE`/`DO`/`LOOP` condition or a
  `FOR` header, for `RESUME`, `RESUME NEXT` and an untrapped error, as measured (an error in an `IF` or `WHILE`
  condition enters the body; in `LOOP UNTIL` it leaves the loop; in a `FOR` header it enters the body without
  setting the variable).
- **`CONST`**: in the main module and in procedures, with the old compiler's own constant evaluator (64-bit
  integers, right-associative `^`, a suffix on the name rounds), visible from its line on; functions in a `CONST`
  expression stay "not supported yet". The parser gains the `CONST` statement (moved here from `m2-parser-breadth`
  task 7.2).
- **`OPTION _EXPLICIT`** (and `_EXPLICITARRAY`, which allows implicit scalars): an implicit variable becomes a
  compile error. The parser gains the `OPTION` statement; `OPTION BASE` stays "not supported yet" (arrays).
- **IR**: bodies stay flat; blocks lower to conditional and plain jumps to labels the lowering creates, hidden
  temporaries get a storage class, and `GOSUB`/`RETURN` are operations. The IR's error rule is restated as what
  the emitter already does (a pending error and placeholder values, checked only at named points; "skips the
  rest of the statement" was true only of `PRINT`) and, with the lowering, gives the measured behaviour in block
  headers.
- **Emitter**: jumps, `GOSUB` return points (`retK.txt`), `FOR` temporaries, conditions with the old compiler's
  error-pending rule, labels in procedures.
- **Corpus**: new `tests\corpus\slice\` programs for each construct and for errors in headers, recorded with
  `qb64pe.exe`; `slice.list` gains them and every corpus program this change makes pass; upstream programs that
  pass are added to `tests\upstream\pass.list`.

## Capabilities

### New Capabilities

- `language/control-flow`: `IF` in both forms, `FOR`, `DO`, `WHILE`, `EXIT` of loops, `GOTO`, `GOSUB`/`RETURN`,
  labels in blocks and procedures, `NEXT` variable checks.
- `language/constants`: `CONST` (scope, visibility from its line, typing, the constant evaluator) and `OPTION
  _EXPLICIT` / `_EXPLICITARRAY`.

### Modified Capabilities

- `language/numeric-semantics`: comparison, logical, integer-division, `MOD` and `^` operators.
- `language/error-handling`: labels inside procedures; errors raised in block headers; `RETURN` without `GOSUB`.
- `compiler/pipeline`: the ABI-neutral IR requirement allows jumps (labels stay positions; the lowering adds its
  own), states the hidden-temporary storage class and `GOSUB`/`RETURN`.

## Impact

- Changed: `crates\syntax` (`CONST`, `OPTION` statements; `parser\decl.rs`, `ast.rs`), `crates\sema` (operators,
  block checking instead of marking, labels per procedure, constants, `OPTION _EXPLICIT`; `check.rs` split by
  family as `study\23` §3 asks), `crates\ir` (jumps, temporaries, `GOSUB`), `crates\codegen-cpp`,
  `tests\corpus\slice.list`, `tests\upstream\pass.list`, the three shrink-only lists in `tests\`; new
  `tests\corpus\slice\s13…`, `tests\frontend\*.bas` files, measurements `verification\v17_*`.
- `m2-parser-breadth` (paused) loses `CONST` and `OPTION` from task 7.2, and its pipeline delta's example of an
  unsupported block changes from `FOR` to `SELECT CASE`.
- No change to the build path, the CLI or the reference clone.
- Not in this change: `SELECT CASE`, `ON … GOTO/GOSUB`, event `ON …` forms, `DEFxxx`, line numbers (so `ERL` stays
  0 and `THEN 100` stays "not supported yet"), `SYSTEM n`, `STOP`, `_CONTINUE`, `RESUME` inside a procedure,
  functions in `CONST` expressions, the auto-included constants (`_TRUE`, colour names), arrays.
- After this change: the IR review (`study\20` §3.4: keep the IR, or merge it into the typed tree). It judges the
  jump and error model; how the IR names a place other than a scalar variable (array element, `TYPE` member)
  stays open until arrays and `TYPE` (step 8).
- `STATUS.md`, `crates\README.md`, `tests\corpus\README.md`, `tests\upstream\README.md`, `DIVERGENCES.md` (if a
  divergence is decided), `CLAUDE.md` (decisions) updated when done.
