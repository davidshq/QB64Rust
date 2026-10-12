# Proposal

## Why

The IR names a place only as a scalar variable (`Op::Assign { place: VarId }`, `Arg::Ref(VarId)`,
`ValueKind::Var`), and `Ty` has no user types. The fourth review moved a minimal arrays-and-`TYPE` slice ahead of
the type table, unsigned types and the built-ins, because the upstream yardstick is array-heavy (209 of 404
programs dimension arrays, 193 use `TYPE`) and those steps should target a stable type and place model
(`study\24` §2). The IR review kept the IR and left two questions to this change: how a place other than a
variable is expressed, with the old compiler's store rule **measured per place** (its code guards an element store
against a pending error, a member store not), and whether the IR's value tree (`ir::Value`, a copy of
`sema::Expr`) stays (`study\25` §2.2, §3).

Counted 2026-10-07 with the release build: 17 corpus programs and 11 upstream programs outside `deferred.list`
get only array or `TYPE` diagnostics today (an upper bound: some need array parameters or `REDIM`, which stay
out). Today 113 slice programs pass, the full corpus 127, upstream 23 of 279.

## What Changes

- **Tier-1 safety net first** (`study\25` §2.3): `cargo test` lowers and emits every accepted input (corpus,
  upstream, snippets, clone sets), without compiling C++, and checks each IR with a new `ir::validate`.
- **Measurements before design** (`study\25` §3): `verification\v18_*` programs run with `qb64pe.exe` for the store
  rule per place, the error `ERR` reports when an index and a value both raise, the evaluation order of an element
  passed by reference, out-of-range and float indexes, `LBOUND`/`UBOUND` forms and type, array and member naming
  and scope.
- **Arrays**: `DIM a(n)`, `DIM a(l TO u)`, several dimensions, with constant bounds, in the main module (also
  `DIM SHARED`), of the slice's numeric types, of `STRING` and of a user type; element read and write; an element passed by
  reference; `LBOUND`/`UBOUND` with and without the dimension. The parser gains array bounds in `DIM` (taken from
  `m2-parser-breadth` task 7.2).
- **`TYPE`**: a type with members of the slice's numeric types and of other user types; variables of a user type
  in every storage class scalars have; member read and write (`p.x`, `p.a.b`, `a(i).x`); a member passed by
  reference. `sema` decides between a dotted name and member access as the old compiler does
  (`m2-parser-breadth` M8).
- **IR**: a `Place` (variable, element, member) in stores, loads and by-reference arguments; `Ty` gains
  `User(TypeId)`; the store rule per place, as measured, in the IR's error rule. Index checks stay the emitter's
  (`array_check` is libqb's ABI).
- **Emitter**: array descriptors and static allocation, element and member addressing, the measured store guards,
  `LBOUND`/`UBOUND`.
- **Last task**: decide whether `ir::Value` is replaced by `sema::Expr`, by the criterion of `study\25` §2.2.
- **Corpus**: new `tests\corpus\slice\` programs for each construct and store rule, recorded with `qb64pe.exe`;
  `slice.list` gains them and every corpus program this change makes pass; upstream programs that pass go into
  `tests\upstream\pass.list`.

## Capabilities

### New Capabilities

- `language/arrays-and-types`: static arrays (bounds, elements, `LBOUND`/`UBOUND`, errors), user types (members,
  nesting, dotted names), places passed by reference.

### Modified Capabilities

- `compiler/pipeline`: the ABI-neutral IR names places (variable, element, member), states the store rule per
  place, and has user types.
- `testing/compiler-tests`: tier 1 lowers and emits every accepted input and validates the IR.

## Impact

- Changed: `crates\syntax` (bounds in `DIM`; `ast.rs`), `crates\sema` (arrays, user types, places, `LBOUND`/
  `UBOUND`), `crates\ir` (`Place`, `Ty::User`, `validate`), `crates\codegen-cpp`, `crates\driver\tests\inputs.rs`,
  `tests\corpus\slice.list`, `tests\upstream\pass.list`, the three shrink-only lists in `tests\`; new
  `tests\corpus\slice\s20…`, `tests\frontend\*.bas` files, measurements `verification\v18_*`.
- `m2-parser-breadth` (paused) loses array bounds in `DIM` from task 7.2.
- No change to the build path, the CLI or the reference clone.
- Not in this change: `REDIM`, dynamic arrays (non-constant bounds, arrays in procedures, `$DYNAMIC`), `OPTION
  BASE`, implicit arrays (used without `DIM`), array parameters (`a()`), `SHARED a()`, `ERASE`, member arrays,
  whole-array and whole-`TYPE` assignment, `TYPE` parameters, fixed-length strings (`STRING * n`), unsigned and
  `_BIT` members, `STRING` members (`SOMEDAY.md` or later steps). String arrays are in (decided in task 2.1).
- `STATUS.md`, `crates\README.md`, `tests\corpus\README.md`, `tests\upstream\README.md`, `DIVERGENCES.md` (if a
  divergence is decided), `DECISIONS.md` updated when done.
