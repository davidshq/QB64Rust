# Proposal

## Why

The next step of M3 is the user's choice (`STATUS.md`; user, 2026-10-16: "lets do arraycopy and versioninfo";
`m3-versioninfo` came first and is archived). `_ARRAYCOPY` is the first line of the demand report and what is
left in `tests\upstream\deferred.list`: four upstream programs need it, `arrays/ArrayCopyTest` (strings),
`ArrayCopyTest_1` (to a member array of an element), `ArrayCopyTest_2` and `ArrayCopyTest_3` (multidimensional
ranges, `_DYNAMIC` members, `REDIM _RETAIN`). The parser already reads the statement (`ArrayCopyStmt`); `sema`
reports it "not supported yet". The runtime is QB64pe's libqb (`array-copy.cpp`, twelve entry points), which this
compiler already builds and links; nothing in libqb changes.

Measured before this proposal (`verification\v41_*`, 87 programs, compiled and run with `qb64pe.exe`; the
clone's `git status --porcelain` empty before and after; the facts go into `study\00` §5 in task 1.1; the old
compiler's code: `qb64pe.bas` 10520–10600 and `source\utilities\arrcpy.bm`):

- **Six forms**, chosen by the shape of each side's last parentheses: `s() TO d()`, `s(lo TO hi) TO d(start)`,
  `s() TO d(start, …)`, `s(lo TO hi) TO d()`, `s(lo TO hi, …) TO d(start, …)` and `s(lo TO hi, …) TO d()`
  (`v41_a`, `v41_c`). A range copies its elements from `lo` to `hi`; to a start they go from there on; to `d()` a
  range keeps its indexes, and a whole one-dimensional source keeps its indexes (`d` must hold them); a whole
  source to a start goes in its order of elements; in several dimensions each coordinate is mapped by itself.
  Overlap and an array copied to itself are safe (`v41_a` 7.–10.).
- **Element types**: every number type, `_OFFSET`, `STRING * n` and `STRING` (deep: the copy owns its strings,
  `v41_b`); not user types, `_MEM`, `_BIT`; both sides of one type, `STRING * n` of one length (`v41_x15`–`x24`,
  `x68`, `x69`).
- **Places**: arrays of every place (`v41_f`), member arrays of a plain `TYPE` (also `STRING` and `STRING * n`,
  nested, of an element: `v41_d`, `v41_k`) and `_DYNAMIC` members below `$UNSTABLE:TYPEFIELDS` (`v41_e`). A name
  with a type suffix is "source must be an array" (`x44`, `x45`).
- **Evaluation**: the places on the way to member arrays (source, then destination), then every lower bound, then
  every upper bound, then the starts, each once; a bound is rounded as an index is (`v41_h`, `v41_d` 9., 10.).
- **Run-time errors**, raised by libqb and leaving the destination unchanged: error 9 for a range or start outside
  its array, low above high, a destination too small, an array not allocated, a whole multidimensional copy with
  other bounds (`v41_g`; also under `$CHECKING:OFF`, `x62`).
- **Compile errors**: 22 messages over the 60 rejected forms of `v41_x01`–`x76` (16 are accepted), among them a
  form the six do not cover, a side that is not an array, types, dimensions, counts of ranges and starts. **An
  array parameter whose number of dimensions nothing but the `_ARRAYCOPY` gives** is "cannot determine the array
  dimensions" (`x75`): the statement says nothing about it.
- **Two faults of the old compiler**:
  - **After an index outside its bounds on the way to a member array** (`tt(3).b()` for `tt(1 TO 2)`) it raises
    error 9 and still copies, through the member of the first element, numbers as well as strings (`v41_j`).
  - **`_FLOAT` arrays** (`v41_b`, `v41_i`): it passes 32 as the element size, but an element of a `_FLOAT` array
    is 16 bytes from the next (each block holds 32 per element); a copy that does not start at the first element
    reads and writes the wrong elements (`_ARRAYCOPY g1(2 TO 3) TO g3(0)` puts `g1(3)` and `g1(4)` there). Both
    stay inside the blocks.

## What Changes

- **`_ARRAYCOPY source TO destination`** compiles in its six forms, for every place of an array and member arrays,
  with the old compiler's compile-time checks, its order of evaluation and the libqb call it writes.
- **Two divergences, by the rules the user set for wrong elements (D-004, 2026-10-07) and memory misuses
  (2026-10-14), for the user to confirm**: after an index outside its bounds on the way to a member array nothing
  is copied (`DIVERGENCES.md` D-004 widened again); a `_FLOAT` array is copied by the distance between its
  elements, so the elements the program names are copied (new D-025).
- **Tests**: front-end tests of every rejected form and of the IR and C++; call-site programs of each form; two
  slice programs recorded with `qb64pe.exe`; the four upstream programs added to `tests\upstream\pass.list`
  and taken out of `deferred.list` (which is then empty but for `t001`).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `language/arrays-and-types`: requirements for `_ARRAYCOPY` (its forms, its places, its run-time errors, its
  checks); "Member arrays" names it beside the other uses of a member array with empty parentheses.
- `compiler/pipeline`: the IR's array operations include copying a part of an array into another.

## Impact

- Changed: `crates\sema` (a module `check\arraycopy.rs`: the forms, the checks, the checks left until the
  dimensions of parameters are solved; `lib.rs`: a `StmtKind`), `crates\ir` (an `Op`, its lowering, validation and
  dump), `crates\codegen-cpp` (`arrays.rs`: the libqb call); new `tests\frontend\arraycopy_*.bas`,
  `tests\callsite\arraycopy_*.bas`, slice programs `tests\corpus\slice\s131_*` and `s132_*`; CLI tests for the two
  divergences; `tests\upstream\pass.list` and `deferred.list`; the demand report and the shrink-only lists
  regenerated.
- `crates\syntax`: an unexpected `TO` in an `_ARRAYCOPY` item is a plain syntax error, not "not supported yet"
  (the tree does not change).
- No change to libqb, the Makefile or the reference clone.
- `DIVERGENCES.md`: D-004 widened, D-025 new.
- Not in this change: a plain scalar beside an array typed by `DIM … AS` (`t001`, `DECISIONS.md` 2026-10-15).
- `STATUS.md`, `ROADMAP.md`, `study\00` §5, `SOMEDAY.md` (the `_ARRAYCOPY` row goes; the questionable behaviours
  kept: a whole one-dimensional source keeps its indexes but a whole multidimensional one must have the
  destination's bounds; a range to a start of a two-dimensional array is the message of a "TYPE-member step"),
  `verification\README.md` (the `v41` family) and `DECISIONS.md` updated.
