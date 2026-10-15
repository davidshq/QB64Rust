# Design

## Context

Motivation, scope and what was measured: `proposal.md`. Where `REDIM` stands in this compiler (2026-10-14):

- **`sema`** reads the words after `REDIM` in `check\decl.rs` `redim` as `(preserve, shared)` and gives
  `Declarer::Redim { preserve }`; anything with `_RETAIN` is "not supported yet". `check\places.rs` turns the
  declarer into `AllocMode::{Dim, Redim, Preserve}` (`lib.rs` 563) on `StmtKind::Allocate`, and `redim_member`
  takes `preserve: bool` (with it, a member array is left alone and the indexes of its path are not evaluated).
- **The IR** re-exports `AllocMode` (`ir\lib.rs` 102); `Op::Allocate` documents what each mode does; the dumps
  print the mode with `{:?}` (`ir\dump.rs` 258, `sema\dump.rs` 366).
- **`codegen-cpp\arrays.rs` `allocate_with`** writes the old compiler's allocation (`qb64pe.bas` 15193–15531) with
  locals in place of its `static` working variables. Under `REDIM` it captures the old block and count in an
  `if (n[2]&1)` block before the bounds, builds the new descriptor in `alloc_new_desc`, then for an allocated
  array makes the new block, copies by flat position under `Preserve` (strings by `qbs_set`, `_BIT` by
  `setbits`/`getubits`, user types with strings part by part, `memcpy` for the rest), frees the old block, and
  writes the descriptor last. The descriptor's slots: `dim_slot(dims, k)` for dimension `k` (the first
  dimension, the fastest, at the highest slot), `lock_slot(dims)` after them.
- **The old compiler's `_RETAIN`** (`redimoption = 3`): `static int64 preserve_old_desc[…]`, `preserve_lo`,
  `preserve_hi`, `preserve_idx` of one entry per dimension, `preserve_old_off`, `preserve_new_off`,
  `preserve_dim`, `preserve_any`; the old descriptor's slots 4 to `4 * nume + 3` copied where the old block is
  captured (15243–15247), `preserve_any=0` when the array is not allocated; after the size checks
  `preserve_any=(preserve_old_total!=0)` and per dimension the overlap `lo = max(old lower, new lower)`, `hi =
  min(old upper, new upper)`, `preserve_any=0` when `hi < lo` (15286–15296); then `AppendRetainCoordinateWalk`
  (14782–14812): the offsets of the element in both blocks from the indexes, the payload, and an odometer over
  the indexes, the first dimension fastest.

## Goals / Non-Goals

**Goals:**

- `_RETAIN` writes the old compiler's copy for every kind of element, in the same order of statements, so a
  program's output, its runtime errors and the `_MEM` lock behave as the old compiler's.
- The word orders are rejected where the old compiler rejects them.

**Non-Goals:**

- `$UNSTABLE:TYPEFIELDS` arrays (descriptor-layout user types, the `preserve_old_flags&8` paths): the next change.
- Matching the old compiler's crash when a FUNCTION in a bound gives the same array new bounds (`v34_d`).

## Decisions

### D1. `AllocMode::Retain`, and the declarer carries the mode

`AllocMode` gains `Retain` ("new bounds; the elements whose indexes lie in both the old and the new bounds keep
their values at the same indexes"). `Declarer::Redim { preserve: bool }` becomes `Declarer::Redim { mode:
AllocMode }` (`Redim`, `Preserve` or `Retain`), so `places.rs` takes the mode as it is and `redim_member` asks
"keeps the elements" (`mode != Redim`) where it asked `preserve`. The IR's dump prints `Retain` with no other
change; validation has nothing new to check (the mode puts no condition on the bounds or the array).

*Alternative:* a second bool `retain` beside `preserve`. Rejected: three states in two bools allow a fourth that
means nothing, and every match on the declarer would have to rule it out.

### D2. The words after `REDIM`: one of `_PRESERVE` and `_RETAIN`, then `SHARED`

`redim` matches the words as now, with `["_RETAIN"]` and `["_RETAIN", "SHARED"]` added. Every other sequence is a
compile error at the second word. The messages say what is wrong (this compiler's errors name the rule; the old
compiler's "Invalid variable name" does not): `SHARED` before either word keeps its message, worded for both
(`` `_RETAIN` comes before `SHARED` in a `REDIM` ``); both words, in either order, "a `REDIM` takes one of
`_PRESERVE` and `_RETAIN`"; any other repeat, "`X` stands twice in a `REDIM`". `REDIM _RETAIN` alone and `DIM
_RETAIN` keep the parser's errors they get today (`v34_x01`, `x07`: "expected a variable name", "expected the end
of the statement"); `x = _RETAIN` keeps "names starting with `_` are reserved" (`x14`). Task 2.2 adds the forms
to a `check-fail` test.

### D3. The coordinate copy in `allocate_with`

Under `Retain` the statement declares, beside `_PRESERVE`'s locals, `int64 preserve_old_desc[L]` (`L` =
`lock_slot(dims)`), `int64 preserve_lo[d]`, `preserve_hi[d]`, `preserve_idx[d]`, `int64 preserve_old_off`,
`preserve_new_off`, `ptrszint preserve_dim` and `int32 preserve_any=0`, and copies the slots 4 to `L - 1` into
`preserve_old_desc` inside the capture block. After the size checks it writes `preserve_any=(preserve_old_total!=0);`
and, for each walk index `w` from 0 (the first dimension) to `d - 1`, the old compiler's four lines with slot
`dim_slot(dims, w)`. The copy is a helper `retain_walk(dims, payload)` that writes `AppendRetainCoordinateWalk`'s
lines; the payload for each kind is `_PRESERVE`'s copy of one element with `preserve_new_off` and
`preserve_old_off` in place of `tmp_long`:

| Kind | Payload |
|---|---|
| `STRING` | `qbs_set(((qbs**)(preserve_new_ptr))[preserve_new_off],((qbs**)(preserve_old_ptr))[preserve_old_off]);` |
| `_BIT * n` | `setbits(n,(uint8*)(preserve_new_ptr),preserve_new_off,(int64)getubits(n,(uint8*)(preserve_old_ptr),preserve_old_off));` |
| User type with strings | each part: `qbs_set` or `memcpy` at `+preserve_new_off*size+offset` and `+preserve_old_off*size+offset` |
| Everything else | `memcpy(((uint8*)(preserve_new_ptr))+preserve_new_off*size,((uint8*)(preserve_old_ptr))+preserve_old_off*size,size);` |

It stands where `Preserve`'s flat copy stands, so the order of the new block, the new strings, the copy, the
release of the old strings and block, and the descriptor written last stays as it is. A `_BIT` array's offsets
count elements, as the old compiler's do (its `_PRESERVE` copy counts them the same way). Task 3.1 compares
the C++ of each kind with the old compiler's (`-z`, `v34_a`, `v34_b`), line by line apart from `static`.

*Alternative:* read the old bounds from the array's own descriptor during the copy (it is written last, so
they are still there) and skip `preserve_old_desc`. Rejected: a FUNCTION in a bound may give the array new
bounds before the copy (`v34_d`), and the old compiler's order is the one measured; the copy costs a few lines.

### D4. Tests

- **Front end**: `redim_retain_errors.bas` (`check-fail`: the rejected word orders, D2), `redim_retain_ir.bas`
  (`ir`: the mode), `redim_retain_cpp.bas` (`cpp`: one array of each kind of D3, and one with three
  dimensions); `redim_unsupported.bas` loses its two `_RETAIN` lines and its snapshot is redone.
- **Corpus**: slice programs `s112_redim_retain` (numbers: `v34_a`'s cases) and `s113_redim_retain_kinds`
  (`v34_b`'s and `v34_c`'s), recorded with `qb64pe.exe`, on `slice.list`.
- **Upstream**: the seven `_RETAIN`-only programs leave `deferred.list`; those that pass go on `pass.list`;
  `array_elements_change` leaves `tests\known_unsupported_rejections.list` (its error "Cannot change the number
  of elements" is real now).
- No call-site program: `REDIM` is a statement of the language, not a row of the built-in table.

## Risks / Trade-offs

- [The walk is written once per `REDIM _RETAIN`, with as many lines as the old compiler's] → it is the old
  compiler's code, and only programs that use `_RETAIN` get it.
- [`v34_d`: a FUNCTION in a bound that gives the same array new bounds] → the old compiler's program ends with
  exit 127; this compiler's reads the block captured before the bounds, as its `_PRESERVE` does, and may end
  the same way or not. Not on any test list; listed in `SOMEDAY.md` ("QB64pe behaviours to review") with the
  `_PRESERVE` case, which has the same order.
- [The 21 `TYPEFIELDS` programs with `_RETAIN` stay deferred] → they need descriptor-layout user types, the next
  change; this one writes nothing for them that it would have to undo.

## Open Questions

None.
