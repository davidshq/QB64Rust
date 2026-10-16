# Proposal

## Why

The user took whole-array assignment out of `SOMEDAY.md` after `m3-typefields` ("do whole-array assignment",
2026-10-15): it was the first line of the upstream demand report, and 16 programs of
`tests\upstream\deferred.list` wait on it (12 accepted programs for it alone, 4 `.err` programs, and
`t001_whole_array_core`, which also needs a plain scalar beside an array typed by `DIM … AS`). `_ARRAYCOPY` is
not part of it and stays deferred.

Read and measured before this proposal (`qb64pe.bas` at the reference clone's commit: `assign` 17343,
`EmitAssignWhole` 16675, `AppendWholeArrayAssign` 17272, `AppendDynUDTArrayAssign` 16855, `CheckAsgRHS` 16809;
`verification\v36_*`, 58 programs recorded with `qb64pe.exe`, 2026-10-15):

- **The form is `target() = source()`**, each a name with empty parentheses or a chain of members ending in one
  (`x.v()`, `e(1).v()`), outer parentheses around the source allowed (`(b())`, `v36_x14`), `LET` allowed, blanks
  inside the parentheses allowed (`x20`). A name with a type suffix on either side takes another path (below).
- **Checked at compile time**: the same element type (`LONG` and `INTEGER`, signed and unsigned, `SINGLE` and
  `DOUBLE`, `_BIT * 2` and `* 3`, `_OFFSET` and `_INTEGER64`, two user types of one layout: "Cannot assign arrays
  of different types"; `STRING * 4` and `* 8`: "… with different element sizes"), the same number of dimensions
  ("… with different dimensions"; an array parameter's number comes from its uses, `x17`, `x39`, `x45`)
  (`x05`–`x12`, `x19`, `x21`–`x23`, `x34`, `x35`). A target or source that is no array, an index on one side,
  an expression, a literal, a FUNCTION, a `CONST`, an undeclared name: "Expected array index after …" or
  "Whole-array assignment requires empty parentheses on both sides", by `CheckAsgRHS`'s table (`x01`–`x04`,
  `x13`, `x15`, `x16`, `x18`, `x26`, `x29`, `x32`; the upstream `.err` programs `t024`–`t027`, `t037`, `t038`,
  `t040_scalar_equals_whole_array`, `t041`).
- **At run time** (`v36_f`): both arrays must be allocated and have the same lower bound and the same number of
  elements in every dimension, else error 9 and nothing is copied; the same block on both sides (`a() = a()`)
  copies nothing. A static array after `ERASE` is still allocated. The target is not given new bounds.
- **The copy is deep and by kind** (`v36_a`, `v36_b`): numbers, `_OFFSET`, `_BIT` and `STRING * n` as one move of
  the bytes (a `_BIT * n` element has `n/8+1` bytes of its own, as the allocation gives it); `STRING` element by
  element; a user type element by element as a whole-value copy, strings and member arrays included; `_MEM` as
  bytes.
- **Every place an array may be** (`v36_c`): `SHARED`, local, `STATIC`, array parameters on either side,
  `COMMON SHARED` in conventional memory, `$DYNAMIC`, used without `DIM`, a FUNCTION.
- **Member arrays, without the flag** (`v36_d`): member to member, member to array, array to member, members of
  array elements, strings and user types; a member's bounds that differ from the array's are error 9. The target's
  place is found before the source's (`x49`). **An index outside its bounds on the way** raises error 9; numbers
  are then not copied, strings are copied through the member of element 0 (`x48`).
- **Owner types under `$UNSTABLE:TYPEFIELDS`** (`v36_e`, `x46`, `x47`): each element copied deep with its
  members' own bounds, a DIM or a REDIM target, a nested owner type; a `_DYNAMIC` member array to another or to a
  plain array and back, with error 9 for other bounds (the target is not given new bounds). **But an array of an
  owner type with other bounds** raises error 9 and is copied anyway, as many elements as the target has: with a
  larger target the old program reads past the source's block (`x52`). `_MEMEXISTS` of a block on a member of an
  element stays true after the copy (`x51`).
- **A suffix on both names** (`a%() = b%()`, `a&() = b%()`, `a$() = b$()`): the old compiler copies element 0 to
  element 0, converting, with no type check (`x24`, `x38`, `x40`, `x43`, `x44`); a suffix on one side only is an
  error (`x41`, `x42`).

## What Changes

- **`target() = source()` compiles** for arrays of every element type, with the old compiler's compile-time
  checks and messages and its run-time checks (error 9), as a deep copy by kind; for array variables of every
  place, member arrays (plain and `_DYNAMIC`) on either side, and arrays of owner types.
- **The forms the old compiler rejects are compile errors** here, each with its own message: the eight upstream
  `.err` programs of the feature on `tests\known_unsupported_rejections.list` are then rejected by a real error.
- **A suffix on both names stays "not supported yet"** (user, 2026-10-15, as `DECISIONS.md` 2026-10-13 (3) for a
  whole array in `FIELD`): listed in `SOMEDAY.md` with the old compiler's element-0 copy.
- **An index outside its bounds on the way to a member array copies nothing**, for every kind (the old compiler
  copies strings through element 0's member): `DIVERGENCES.md` D-004 widened. For the user to confirm.
- **An array of an owner type with other bounds copies nothing** after error 9, as every other array does (the
  old compiler copies anyway and reads past a block): `DIVERGENCES.md` D-022, as the user chose for the memory
  misuses of 2026-10-14. For the user to confirm.
- **The IR gets one operation**, a whole-array copy from an array to an array (an array variable or a member
  array), with the order and errors above.
- **Tests and lists**: `verification\v36_*`; front-end tests (each rejected form, the IR operation, the C++ of
  each kind); console slice programs on `tests\corpus\slice.list`; the 16 programs leave
  `tests\upstream\deferred.list`, `t001` named there for its other feature; the 12 accepted programs that pass
  and the eight `.err` programs join `pass.list` (290 to about 310), and the eight leave
  `known_unsupported_rejections.list`.

Out of scope: `_ARRAYCOPY` (still deferred); a plain scalar beside an array typed by `DIM … AS` (user,
2026-10-15), so `t001_whole_array_core` stays waiting; giving the target the source's bounds (the old compiler
does not).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `language/arrays-and-types`: a new requirement "Whole-array assignment"; requirements "Member arrays" and
  "Dynamic member arrays" (a whole member array with empty parentheses is now also a side of a whole-array
  assignment, where it was only an array argument).
- `compiler/pipeline`: requirement "Array storage in the IR" (the copy operation).

## Impact

- `crates\sema` (`check\mod.rs` `assign`: the form, the checks and messages; `check\places.rs`: the whole array
  `x()` resolved as an `ArrayRef`; `lib.rs`: the statement), `crates\ir` (the operation, its dump and validation),
  `crates\codegen-cpp` (`arrays.rs` or a new `whole.rs`: the checks and the copy by kind, member arrays through
  the descriptor an array argument already gets, owner types through `copy_owner`).
- Tests: `tests\frontend\` (new programs and snapshots), `tests\corpus\` (new slice programs and their recorded
  output), `tests\upstream\deferred.list`, `pass.list`, `tests\known_unsupported_rejections.list`.
- Documents: `SOMEDAY.md` (the whole-array row goes; the suffix form is added), `DIVERGENCES.md` (D-004, D-022),
  `DECISIONS.md`, `study\00` §5, `CLAUDE.md`'s layout table (`v36*`), `crates\README.md`,
  `tests\upstream\README.md`, `STATUS.md`, `ROADMAP.md`.
