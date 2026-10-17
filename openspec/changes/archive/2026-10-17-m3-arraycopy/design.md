# Design

## Context

Motivation, scope and what was measured: `proposal.md` (`verification\v41_*`, 87 programs, 2026-10-17); the
behaviour: the delta specs. Where the parts stand in this compiler (2026-10-17):

- **Parser**: `syntax\parser\io.rs` `array_copy_stmt` reads `_ARRAYCOPY` and two `DimItem`s around `TO`, in the
  shape of a `REDIM` item (`redim_items`: a member path `a(1).m(5)`, each part's parentheses an `ArrayBounds`).
  `ast::DimItem::path` gives the parts, `ArrayBounds::ranges` each item as `(lower, upper)`. The tree does not
  change; one message does (D2).
- **`sema`**: `check\mod.rs` reports `ArrayCopyStmt` "not supported yet" (1022). Whole arrays are resolved for
  whole-array assignment (`check\whole.rs` `copy_array`: `find_array`, `member_array` in `check\places.rs` 1161)
  and for `REDIM` of a member array (`places.rs` `redim_member`, which walks a `DimItem` path). The number of
  dimensions of an array parameter is solved at the end from facts (`DimFact`, `solve_dims`, `check\mod.rs` 864).
- **IR**: `ArrayRef` (`Var` or `Member { base, member }`) and `Op::CopyArray` (`ir\lib.rs` 500).
- **`codegen-cpp`**: `arrays.rs` `whole_desc` gives a side's descriptor (an array variable's, a dynamic member's
  `desc(slot)`, or one made by `value.rs` `member_array_arg`, set where the place is found); `copy_dims`.
- **libqb**: `array-copy.cpp` (`qb64_array_copy_{1d,nd}_{whole,whole_at,range_at}_{fixed,string}`) is built with
  libqb and declared in `qbx.cpp`, which this compiler's output includes. It checks every bound at run time.
- **The old compiler** (`qb64pe.bas` 10520, `source\utilities\arrcpy.bm`): it picks the form by trying six
  shapes of the two sides in turn, resolves the source then the destination, checks the pair by the form, compiles
  the ranges and starts, and writes one block per statement that calls libqb.

## Goals / Non-Goals

**Goals:**

- The six forms, with the old compiler's checks, evaluation order and libqb call, so that the call-site check
  compares this compiler's statement with the old one's.
- One IR operation; the choice of entry point and the element size are code generation.

**Non-Goals:**

- The two faults of the proposal (D-004 widened, D-025): both copy what the program names.
- New runtime code: libqb's checks and copies are used as they are.

## Decisions

### D1. The form, in a module `sema\check\arraycopy.rs`

`Checker::array_copy(stmt)` (called from `check\mod.rs` in place of the "not supported yet") reads each side's last
parentheses as the old compiler's `ParseArrayCopyRef` does, and picks the form in its order:

| Side | Shape | Old `ref_kind` |
|---|---|---|
| source | `()` | 1, whole |
| source | one item with `TO` | -1, a range |
| source | two or more items (each must then have `TO`, D2) | 3, ranges |
| destination | `()` | 1, whole |
| destination | items without `TO` (one or more) | 0 or 2, starts |

The six pairs map to `ArrayCopyForm` (`RangeAt`, `Whole`, `WholeAt`, `RangeWhole`, `RangesAt`, `RangesWhole`); any
other pair, a side without parentheses, an `AS` clause on a side and a source in parentheses (`x67`) are the
form error. A side's path before its last parentheses is resolved as an array: a plain name by `find_array`
(no suffix, `x44`, `x45`: "… must be an array"), a member path by the walk `redim_member` uses, moved into a helper
`member_path_array` (`places.rs`) that both call. Source first, then destination, each once.

*Alternative considered*: a new tree node per form in the parser. Rejected: the old compiler decides the form
from the same tokens, and errors such as `m(1 TO 2, 3)` (a range count right, one `TO` missing) need the item
list, not a parse failure.

### D2. The checks, in the old compiler's order, and the table

`check_pair(form, source, destination)` follows `arrcpy.bm`: per form, `CheckArrayCopy1DPair` (range forms of one
dimension), `CheckArrayCopyWholePair` / `RangeNDPair` (the others), with `STRING` first (same type, then rank),
then the element pair (one type as `Ty`; `STRING * n` of one length; user types, `_MEM`, `_BIT` refused), then
"at least two dimensions" for the range forms of several, then the range count, then the start count. The ranges
and starts are then checked as values (a number, else "Illegal string-number conversion"; `x49`, `x50`). The
messages are this compiler's, the old wording comes as the note after the error (`tools\diagnostics\old_wording.py`
pairs them from the recorded `v41_x*` rejections). Some forms stop in the parser (`x04` an unclosed `(`, `x35` two
`TO` in one item, `x36`, `x37` an empty item, `x67` a source in parentheses): compile errors still, with the old
wording note. **Two of them are "not supported yet" today** (`x04`, `x35`: the parser's path for an unexpected
word, "not supported yet: word `TO`"); in an `_ARRAYCOPY` item an unexpected `TO` becomes a plain syntax error
(`syntax\parser\io.rs` `array_copy_stmt`, through a parser flag beside `redim_items`), so a program the old
compiler rejects gets a real error.

### D3. Dimensions that are not known yet

An array parameter's number of dimensions may be known only after `solve_dims`. When a side's number is not known
at the statement, `array_copy` pushes the statement's pair check onto `self.copy_checks` and `solve_dims` runs
them after solving, with an unknown number as 0, so the old compiler's messages fall out of the same checks
("cannot determine the array dimensions" in the whole-pair path, `x75`; the dimension mismatch, `x76`).
**The statement adds no `DimFact`**: the old compiler learns nothing from it (`x75`'s `never` is rejected though
its own form says two dimensions).

### D4. The IR operation

`Op::ArrayCopy { from: ArrayRef, to: ArrayRef, ranges: Vec<(Value, Value)>, starts: Vec<Value> }` (`ir\lib.rs`,
lowered from `sema::StmtKind::ArrayCopy` in `ir\lower.rs`), every value an `I64` converted as an index is
(rounded, `v41_h` 4., 5.). Documented: `from`'s place is found, then `to`'s, each once; then every lower bound,
every upper bound, every start, left to right (`v41_h` 2., 3.); **if finding a place raised, nothing is copied**;
else the runtime checks of the spec; empty `ranges` is the whole source, empty `starts` the whole destination (a
range then keeps its indexes). Always may raise. Validation (`ir\validate.rs`): one element type, of a kind the
operation takes; `ranges` and `starts` empty or one per dimension where known (the IR has no constant arrays, so
there is no check that `to` is not one; task 2.3). Dumped as
`ArrayCopy <from>(<lo> TO <hi>, …) -> <to>(<start>, …)`.

### D5. The C++, `codegen-cpp\arrays.rs` `array_copy`

The old compiler's block, with its local names, so the call-site check compares it:

```cpp
{
<the source's place, then the destination's: a member descriptor set where it is found>
int32 array_copy_ok=!is_error_pending();          // only when a side is a member whose place may raise
int32 array_copy_dims=array_copy_dst[3]==<n>;     // only for a _DYNAMIC member side (each such side, by &&)
if (!array_copy_dims) error(9);
int64 array_copy_src_low=(int64)(<lo>);           // several dimensions: int64 array_copy_src_low[n]={…};
int64 array_copy_src_high=(int64)(<hi>);
int64 array_copy_dst_start=(int64)(<start>);
ptrszint *array_copy_src=<from>;
ptrszint *array_copy_dst=<to>;
if (array_copy_ok) qb64_array_copy_<entry>(array_copy_src,array_copy_dst,…,(uint64)<element bytes>); else qbr_diverged_on(4);
// with array_copy_dims: if (array_copy_dims){if (array_copy_ok) qb64_array_copy_…; else qbr_diverged_on(4);}
}
```

- **Entry point**, by the form and the source's dimensions: `Whole` 1 → `1d_whole`, else `nd_whole(n)`;
  `RangeAt` → `1d_range_at`; `WholeAt` 1 → `1d_whole_at` (first start), else `nd_whole_at`; `RangeWhole` →
  `1d_range_at` with the low as the start; `RangesAt` → `nd_range_at`; `RangesWhole` → `nd_range_at` with the lows
  as the starts. `_string` for `STRING`, else `_fixed` with the element size.
- **Element size**: the width of a number, `_OFFSET` 8, `STRING * n` n; **`_FLOAT` `sizeof(long double)`**, the
  distance between its elements (the old compiler passes 32, its `LEN`; D-025).
- **The guard** `array_copy_ok` (D-004 widened): captured after the places and before the bounds, so the bounds
  are still evaluated and an error raised in a bound does not stop the copy, as in the old compiler (`v41_g` 24.,
  `v41_h` 11.); a FUNCTION in a bound after an error on the way is entered with the error pending and runs no line,
  in both compilers (found while writing the CLI test, task 3.3). Without a member side the line and the `if` are not written, which keeps the
  call-site programs (top-level arrays) equal to the old compiler's.
- **A `_DYNAMIC` member side with another number of dimensions** (found by the change's review, measured
  `v41_l`): a `REDIM` may give a dynamic member another number of dimensions than declared, and libqb reads the
  bounds by the number it is given without checking the descriptor's slot 3 (past the descriptor when it has
  fewer). The old compiler's guard `UDTDynArrayDescGuard` (`qb64pe.bas` 16234: a null descriptor or another
  number of dimensions) raises error 9 there and gives a stand-in that is not allocated, so nothing is copied;
  here `array_copy_dims` compares slot 3 with the declared number, as `copy_array` does for a whole-array copy
  (`v38_c`), raises error 9 after the places (in both compilers after any FUNCTION in a plain member side's
  indexes) and keeps the copy from running. It is not a divergence, so it takes no D-004 trace, also when the
  other side's place raised: the old compiler copies nothing then either (read from its C++, not run: the
  stand-in never sets slot 2, so libqb's descriptor check raises error 9).

### D6. Tests

- `tests\frontend\`: `arraycopy_errors.bas` (`check-fail`: every rejected form of `v41_x*`), `arraycopy_ir.bas`
  (`ir`: each form, a member, a parameter), `arraycopy_cpp.bas` (`cpp`: each entry point, `STRING`, `_FLOAT`, a
  member with the guard).
- `tests\callsite\arraycopy_*.bas`: one program per form and kind (`LONG`, `STRING`, `STRING * 4`), a 3D range,
  a `SINGLE` bound; `_FLOAT` none (D-025 differs by design).
- `tests\corpus\slice\`: `s131_arraycopy` (the forms, the kinds but `_FLOAT`, several dimensions, the places, the
  run-time errors, the order), `s132_arraycopy_members` (plain, nested, `STRING` and `_DYNAMIC` members) and
  `s133_arraycopy_dynamic_dims` (`_DYNAMIC` members with another number of dimensions, `v41_l`), recorded with
  `qb64pe.exe`; the lines of the two divergences are left out and tested by CLI tests with this
  compiler's output (`arraycopy_through_an_index_outside`, `arraycopy_of_float_arrays`).
- `verification\v41_*` built and run with this compiler: the same output but for `v41_b`'s `_FLOAT` line,
  `v41_i` and `v41_j`. Upstream `ArrayCopyTest`, `_1`, `_2`, `_3` pass and move from `deferred.list` to
  `pass.list`.

## Risks / Trade-offs

- **[A member descriptor of another layout than the old compiler's]** (a `STRING` member array of a plain `TYPE`
  holds `qbs*` slots there, which libqb's `_string` entry points read) → `v41_k` and `s132` compare the strings
  after each copy; upstream `ArrayCopyTest_1`–`_3` cover `_DYNAMIC` members.
- **[Checks left until the dimensions are solved report out of statement order]** → only for a statement whose
  sides' dimensions are not known there; its first error still comes from the old compiler's sequence (D3).
- **[Two more divergences]** → both of the kind the user chose to fix (writing elements the program did not
  name); listed in `DIVERGENCES.md` and put to the user with the change.
