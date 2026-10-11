# Proposal

## Why

Step 10 of the order of work (`STATUS.md`; `study\26` §4, `study\29` §3). The compiler knows one kind of array:
a static one, declared with `DIM` in the main module with constant bounds. Everything else that makes or changes
an array, and the default-type statements, is still "not supported yet", and that is what the three yardsticks
wait on most.

Counted 2026-10-10 with the release build (`qb64rust --report unsupported`, per program):

| Yardstick | Held back | By this change's constructs alone | Programs that name one of them |
|---|---|---|---|
| Corpus (316 accepted) | 56 (29 are screen programs, M3) | 8, and 2 more with `STRING` members | `DEFINT` 7, implicit arrays 4, `REDIM` 3, `ERASE` 2, arrays in procedures 2, dynamic arrays 2, `OPTION BASE` 1, member arrays 1 |
| Upstream outside `deferred.list` (279) | 196 | 58, and 5 more with `STRING` members, a `DIM AS type` list, a `_BIT` array or a whole-`TYPE` assignment | implicit arrays 70, member arrays 59, `REDIM` 48, `ERASE` 14, `OPTION BASE` 7, `DEFxxx` 5, arrays in procedures 4 |
| Civil War Strategy | 1,740 marked errors | none (it draws) | `COMMON` 12 uses, `hit(...)` 12, `summ(...)` 3, arrays in a procedure 3, `DEFINT` 1 |

The counts are upper bounds: a program's first unsupported declaration hides what follows it (most "implicit
array" messages are arrays whose `REDIM` or member declaration was marked). Upstream stands at 50 of 279; no
other step moves it by this much, and the 54 `_DEST` programs of step 11 mostly need these arrays too.

## What Changes

- **Measurements before design**: `verification\v24_*` programs run with `qb64pe.exe` for every rule below that
  `verification\v09_dim.bas` and `v18_*` did not settle (`design.md` D1). Wrong code found in the old compiler is
  put to the user before it is designed.
- **`DEFINT`, `DEFLNG`, `DEFSNG`, `DEFDBL`, `DEFSTR` and `_DEFINE`**: the default type of a name without a suffix
  or `AS` clause, by its first letter, from the statement on (position-dependent, `study\00` §4).
- **`COMMON` and `COMMON SHARED`** as declarations: `COMMON` declares main-module variables and arrays as `DIM`
  does, `COMMON SHARED` as `DIM SHARED`. `CHAIN`, which gives `COMMON` its other meaning, stays "not supported
  yet" (M4, `study\00` §10).
- **Dynamic arrays**: `DIM` with bounds that are not constant, `REDIM`, `REDIM _PRESERVE`, `'$DYNAMIC` and
  `'$STATIC`, `DIM a()`, with errors 5, 9 and 10 as measured; `ERASE` of a static and of a dynamic array.
- **Arrays in procedures**: `DIM` and `REDIM` in a SUB or FUNCTION (new on each call), `STATIC a(n)`, `STATIC a()`
  followed by `DIM`, `SUB … STATIC`, `SHARED a()`, and array parameters (`SUB s (a())`, called with `s x()`).
- **`OPTION BASE 0` and `1`**, including the rule that `DIM a(0)` stays `0 TO 0`.
- **Implicit arrays**: `a(i)` and `LBOUND(a)` for a name that is no array make an array `0 TO 10` per dimension
  (`OPTION BASE` to 10), as measured; `OPTION _EXPLICITARRAY` rejects them.
- **`_BIT` arrays**, if 1.1 measures them as plain packed storage (one upstream program waits only on them).
- **Plain `TYPE` member arrays** (`DECISIONS.md` 2026-10-07): `TYPE t: v(0 TO 3) AS LONG: END TYPE`, also as
  `AS LONG v(3), w`; element access through any chain (`x(0).n(1).q(2)`), `LBOUND`/`UBOUND` of a member array,
  `REDIM` and `ERASE` of one. The `_STATIC`/`_DYNAMIC` markers, `$UNSTABLE:TYPEFIELDS`, `_ARRAYCOPY`, whole-array
  assignment and `REDIM _RETAIN` stay in `SOMEDAY.md`. This group is last and has a decision point after its
  measurement (`design.md` D8): what a `REDIM` of an unmarked member array does is not known yet.
- **The rest of `TYPE`**, added by Claude to the scope of step 10 because programs counted above need them and
  member arrays may share their per-variable set-up (to be confirmed; they are task group 8, which no other group
  needs, `design.md` D9 and Open Questions): variable-length
  `STRING` members, assigning a whole `TYPE` value (`q = p`), and a `TYPE` variable as argument and parameter.
- **`DIM AS type a, b(3)`** lists in `sema` (parsed since `m2-parser-breadth`; 16 upstream programs name them).
- **IR and emitter**: an array is static or dynamic; statements that allocate, re-allocate, clear and free arrays;
  a member array as a place. Descriptors, allocation and release stay the emitter's.
- **Tests**: `tests\frontend\` programs per form and error, `tests\corpus\slice\` programs recorded with
  `qb64pe.exe` per measured rule, call-site programs where a libqb call is written; `slice.list`,
  `tests\upstream\pass.list` and the shrink-only lists follow.

## Capabilities

### New Capabilities

- `language/default-types`: `DEFxxx` and `_DEFINE`: which names they type, from where in the file, in the main
  module and in procedures.

### Modified Capabilities

- `language/arrays-and-types`: arrays stop being static-only (dynamic arrays, `REDIM`, `ERASE`, `OPTION BASE`,
  implicit arrays, arrays in procedures, `COMMON`); `LBOUND`/`UBOUND` take implicit and member arrays; user types
  gain member arrays, `STRING` members and whole-value assignment.
- `language/procedures`: a parameter may be an array or a user type; the scopes of a procedure include its arrays.
- `compiler/pipeline`: the IR distinguishes static and dynamic arrays, has the array statements and member-array
  places; `$STATIC` and `$DYNAMIC` in a comment take effect.

## Impact

- Changed: `crates\sema` (`check\decl.rs`, `check\places.rs`, `check\proc.rs`, `check\mod.rs`, `lib.rs`: default
  types, array kinds, the new statements, member arrays), `crates\ir` (`lib.rs`, `lower.rs`, `validate.rs`,
  `dump.rs`), `crates\codegen-cpp` (`decl.rs`, `place.rs`, `procs.rs`, `stmt.rs`: dynamic descriptors, allocation
  and release, array arguments, member-array offsets, string members). `crates\syntax` only if a measured form
  does not parse yet (the statements all parse today).
- New: `verification\v24_*`, `tests\frontend\*.bas`, `tests\corpus\slice\s5x_*`, `tests\callsite\` programs.
- Lists and documents updated when done: `tests\corpus\slice.list`, `tests\upstream\pass.list`, the shrink-only
  lists, `study\00` §5 (findings), `STATUS.md`, `ROADMAP.md`, `crates\README.md`, `tests\corpus\README.md`,
  `tests\upstream\README.md`, `SOMEDAY.md`, `DECISIONS.md` and `DIVERGENCES.md` (if a divergence is decided).
- No change to the build path, the CLI, the test runner or the reference clone.
- Not in this change: `CHAIN` and what `COMMON` passes to it; `_STATIC`/`_DYNAMIC` member markers and
  `$UNSTABLE:TYPEFIELDS`; `_ARRAYCOPY`; whole-array assignment; `REDIM _RETAIN`; `_MEM` over arrays; `TYPE` blocks
  inside a SUB or FUNCTION; `DEF SEG`; `$CONSOLE`, `_DEST` and `$SCREENHIDE` (step 11).
