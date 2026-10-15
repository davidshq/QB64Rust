# Design

## Context

Motivation, scope and what was measured: `proposal.md`. The measured facts are in `verification\v35_*` (69
programs); the old compiler's code is `qb64pe.bas` 2180–2460 (the markers in a `TYPE` block), 8890–8990 (`REDIM`
of a member), 15654–16520 (descriptors, guards, indexes), 17075–17300 (files) and `utilities\type.bas` 10–2140
(layout, set-up, release, copy), at the reference clone's commit. Where this compiler stands (2026-10-14):

- **`syntax`** reads a member's leading `_DYNAMIC`/`_STATIC` as a token of the `TypeField` and one after the
  name or bounds as a token of the `FieldName` (`TypeField::modifier`, `FieldName::modifier`); one after the type
  is not read. `$UNSTABLE` is a metacommand the parser does not know: "not supported yet".
- **`sema`** (`check\places.rs`, the `TYPE` block) reports any marker "not supported yet". `Member { name, ty,
  bounds }`; `OPTION BASE` at a block comes from `type_bases`, filled by a walk over the source in order through
  the branches taken. `ArrayRef::Member { base, member }` names a member array; its bounds are constants and its
  elements lie in the value.
- **The IR**: `Op::Allocate { array: VarId, bounds, mode }`, `Op::Release { array: VarId }`, `Op::Clear { array:
  ArrayRef }` (the only operation on a member array), `Op::CopyWhole { from, to }`.
- **`codegen-cpp`**: a user type is packed, its members' sizes added (`decl.rs` `ty_size`, `member_offset`); a
  value is set up, released, cleared and copied by its *parts* (`parts`: a `STRING` is a part, any other member
  its bytes); `allocate_with` (`arrays.rs`) writes `REDIM` on a descriptor named by a C expression `n`, given a
  `Var` for its type; a member array is passed to an array parameter as a static descriptor made on the spot
  (`value.rs` `member_array_arg`); a local's strings are freed at each return (`procs.rs`).

## Goals / Non-Goals

**Goals:**

- Programs with `_DYNAMIC` members behave as the old compiler's do, by output and runtime errors, apart from the
  three memory misuses the user decided to fix (D-004 widened, D-019, D-020).
- Types without a `_DYNAMIC` member, in programs with or without the flag, compile to the same C++ as today.
- The memory layout of an owner type is the old compiler's, so `_MEM` of it and the offsets of its members agree.

**Non-Goals:**

- Writing the old compiler's C++ line for line: its helpers (`udt_dyn_desc_N`, `udt_root_data_guard_N`, the
  per-statement clone loops) are not copied; no call-site program covers these forms (they are statements of the
  language, not rows of the built-in table). What must match is behaviour, layout, and the order and number of
  evaluations of indexes.
- Whole-array assignment and `_ARRAYCOPY` (deferred); `$UNSTABLE` flags other than the three.
- `_MEMGET`/`_MEMPUT` and the other `_MEM` statements on an owner value beyond what the existing `_MEM` code does
  with its address and size (no test needs more; a form that needs more stays "not supported yet").

## Decisions

### D1. `$UNSTABLE` is read where `OPTION BASE` at a block is read

The parser takes `$UNSTABLE` as a metacommand with the text after it, as it takes `$EXEICON`. The flag is not a
precompiler flag: the old compiler sets it while reading lines in order, before its first pass (`qb64pe.bas`
2071–2090), and a `TYPE` block is read in that same walk. So the walk that fills `type_bases` (the `OPTION BASE`
in effect at each block) also records, per block, whether `$UNSTABLE:TYPEFIELDS` stood above it in a branch that
is taken: `type_flags: HashMap<NodeKey, bool>` beside `type_bases`. `$UNSTABLE` with any other flag, an empty one
or no colon gives the old compiler's error at the metacommand; `MIDI` and `HTTP` give the warning with `-w`, as
`$DEBUG`'s warning is given.

*Alternative:* a program-wide flag, set if the metacommand stands anywhere. Rejected: `v35_x45` shows the
position counts, and the block walk already exists.

### D2. The markers in the syntax tree and in `Member`

The parser reads a marker in each place the old compiler reads one (`qb64pe.bas` 2213–2290: before the name,
after the name or its bounds, after the type; in an `AS type` list before a name or after its bounds) as a
`FieldMarker` node of the field or of the name, so `TypeField::markers` and `FieldName::markers` give every marker
of a member (any number; `sema` counts them). `AS _DYNAMIC LONG` is a compile error given by the parser. A
marker before an `AS type` list (`_STATIC AS LONG v(1)`) is a compile error given by `sema`, checked after the
flag (`v35_x60`: the old compiler reads `AS` as the member's name).
`Member`
gains `dynamic: bool`. In the `TYPE` block, in this order: a marker without the block's flag is the error that
names the flag; two markers (equal or not) "a member takes one of `_DYNAMIC` and `_STATIC`"; a marker on a member
with no bounds "`_DYNAMIC` and `_STATIC` mark member arrays only". `_STATIC` sets nothing. A `CONST` in a bound is
"not supported yet" for a `_DYNAMIC` member as for any other (the rule of "Member arrays"; `v35_x17` shows the old
compiler takes it, and the constants are read after the blocks).

### D3. Owner types and their layout

`owns_desc(types, t)` (in `sema`, beside `holds_strings`): a user type with a `dynamic` member, or a member (also
a member array) of a user type that owns one. For an owner type `codegen-cpp` uses the old compiler's
`BuildUDTDynLayoutM` (`type.bas` 801–880): each member in order; a `_DYNAMIC` member takes 8 bytes, and it and a
member of an owner type (also an inline member array of one) start at a multiple of 8; the type's size is
rounded up to 8; an inline member array of an owner type takes its count times the element type's size. Other
types keep their packed layout, so nothing changes for programs without `_DYNAMIC` (Goals). `ty_size` and
`member_offset` give the memory layout; a new `file_layout(t)` gives the declared one for files (D8).

*Alternative:* align every type once the flag is on. Rejected: the old compiler keeps the packed layout for types
without `_DYNAMIC` (`v35_x53`), and `LEN` of them is measured.

### D4. A `_DYNAMIC` member is a dynamic-array descriptor that the value points to

The 8 bytes hold a `ptrszint*` to a descriptor of the shape libqb's dynamic arrays have (`decl.rs` `dim_slot`,
`lock_slot`), allocated with `malloc` for its number of dimensions, with its own `_MEM` lock. So everything that
works on a dynamic array's descriptor works on a member's: `allocate_with` (`REDIM`, `_PRESERVE`, `_RETAIN`),
`release`, the bounds, an array argument (passed as the descriptor itself: a `REDIM` in the procedure is seen by
the caller, `v35_d`). The number of dimensions may change with a `REDIM` (`v35_a` 9.), so it is kept in the
descriptor (slot 3, as the old compiler's guard reads it) and checked at each use against the number of indexes
written: a use with another number raises error 9 (`v35_g` 9.); a `REDIM` to another number makes a new
descriptor. A null pointer (an erased static parent, `v35_b` 9.) reads as "not allocated": every use raises error
9, and `REDIM` makes a descriptor (`x51`).

`sema`: an element of a `_DYNAMIC` member is an `Element { array: ArrayRef::Member { .. } }` as now, with no
check of the number of indexes against the declared bounds (the runtime checks it); `LBOUND`/`UBOUND` of it and
the member as an array argument evaluate the place (`x59`). `REDIM` of it gives `Allocate` (D5), `ERASE` gives
`Release`; the forms of `REDIM` are those of a member array (`lower TO upper` or `AS` the type) with the old
compiler's error for another type after `AS` (`x18`, `x19`, `x32`, `x33`). The forms the old compiler rejects
for an owner value (`LEN`, `_OFFSET`, `COMMON`, `PUT`/`GET` of one value; `x20`, `x39`, `x46`, `x50`, `x54`) are
compile errors in `sema`.

### D5. The IR: `Allocate` and `Release` take an `ArrayRef`

`Op::Allocate { array: ArrayRef, bounds, mode }` and `Op::Release { array: ArrayRef }`; a `Member` there must be
`dynamic` (validation). Their documentation gains: the place is found once, its indexes evaluated before the
bounds, an index outside raises error 9 and the element at position 0 stands in (`x58`); a static-array check
(error 10) does not apply. The set-up, release and copy of the members of an owner value are not operations:
they follow from the type, where a value is made or goes (spec "Member arrays in the IR"). The dumps print
`Allocate`/`Release` of a member as `Clear` prints one.

*Alternative:* new operations `AllocateMember`/`ReleaseMember`. Rejected: they would repeat `Allocate`'s modes,
order and errors, which are the same.

### D6. The life of an owner value: one C++ function per owner type and use

For each owner type `codegen-cpp` writes, once, `static` functions that take the value's address:
`udt_new_N(char*)` (each `_DYNAMIC` member a descriptor at its declared bounds with zero or empty elements, each
`STRING` an empty string, nested owner members and elements recursively), `udt_free_N(char*)` (the reverse:
elements' strings and owners, blocks, descriptors and their locks; a null pointer is skipped) and
`udt_clone_N(char* dst, char* src)` (deep). These are functions, not inline text, because the walk is recursive
and runs per element of arrays. They are called where the parts of a value with strings are handled today:

| Where | Today (`STRING` parts) | An owner value |
|---|---|---|
| A variable made (`alloc`), a `STATIC` local once | `new_strings` | `udt_new_N` |
| A local at each return (`procs.rs`) | `free_strings` | `udt_free_N` |
| An array allocated, its new elements (`alloc_array`, `allocate_with`) | `new_strings` per element | `udt_new_N` per element |
| An array released (`release`), plain `REDIM` of it | `free_strings` per element | `udt_free_N` per element |
| `ERASE` of a static array (`clear_elements`) | strings emptied | `udt_free_N`, slots left null (`v35_b` 9.; listed for review) |
| `_PRESERVE`/`_RETAIN` of an array, a kept element | `qbs_set` of each part, then the old freed | the element's bytes moved (descriptors with them), the old slot zeroed so the old block's release frees nothing twice |
| `CopyWhole` | each part, places found again per part | D7 |
| `SWAP` | bytes exchanged | bytes exchanged (`x28`) |

*Alternative:* inline text per use, as the string parts are. Rejected: an owner type nested in an inline member
array of an owner type needs loops in loops; one function per type keeps each use one call.

### D7. `CopyWhole` of an owner type evaluates the places as the old compiler does

The old compiler writes the copy as lines that each name both places, so their indexes are evaluated once per
line (`v35_x56`: four pairs for a type of a LONG, a `_DYNAMIC` member and a LONG; twelve evaluations, not all in
pairs, for one with strings). Its lines come from `CopyDynUDTExpr` (`type.bas` 2093: one line per member that is
not `_DYNAMIC`, a `STRING` as `qbs_set`) and `AppendDynUDTDescCopy` (`type.bas` 10: the descriptor clones, which
name the places in their own way). Task 3.3 reads `AppendDynUDTDescCopy` and writes the same sequence: per part
as `copy_whole` does for strings, then the descriptor clones as that function names the places; `v35_x56` is the
check, a `typed` or `cpp` test fixes it. Listed for review in `SOMEDAY.md` with the string case.

### D8. Files: a pack and unpack function per owner type

`PUT`/`GET` of a whole array of an owner type (not of a type with a `STRING`, which stays "UDT must have fixed
size") write `udt_pack_N(uint8* dst, char* src)` and `udt_unpack_N(char* dst, uint8* src)` for the declared
layout (`file_layout`): members packed, a `_DYNAMIC` member's elements at their declared bounds. Before `PUT`,
every element's `_DYNAMIC` members are checked against their declared bounds; a mismatch raises error 9 and
nothing is written (`v35_e` 5.). `GET` reads into a buffer of the declared size, then unpacks element by element;
a member with other bounds raises error 9 and is skipped, its elements left as they were (D-019; the old compiler
copies the declared bytes into the smaller block). The buffer is a `static` one per type, grown with `realloc`,
as the old compiler's is.

### D9. Element places and the parent's place

An element of a `_DYNAMIC` member is reached by: the parent's place (its `bytes_of`, indexes evaluated once,
outside raises error 9 and element 0 stands in, as for an array element now), the descriptor pointer read from
the slot through a guard (`null` or not allocated: error 9, a static dummy descriptor and data block of zero bytes
and empty strings, so the expression can finish as the old compiler's guard lets it), then the flat index from
the descriptor as for a dynamic array (`flat_index`'s descriptor form). A store whose parent index or member index
raised stores nothing (D-004, now also for the parent's index; the old compiler writes into `p(0).v`, `x58`).
`REDIM` and `ERASE` through a parent index outside act on element 0's member, as the old compiler does (`x58` 1.,
5.); not a misuse, so not a divergence.

### D10. Limits

A `REDIM` of a member whose size the machine cannot hold ends the program with "Out of memory" through the same
size check `allocate_with` writes for every array (D-020; the old compiler's member path has no such check,
`v35_g3`). Reversed bounds at run time end it the same way (`v35_g2`), as they do already.

### D11. Tests

- **Front end**: `unstable_forms.bas` (`check-ok`/`check-fail`: D1's forms), `typefields_markers.bas`
  (`check-fail`: D2's errors), `typefields_rejected.bas` (`check-fail`: the forms of D4's last sentence and
  `x48`), `typefields_ir.bas` (`ir`: `Allocate`/`Release` of a member, `CopyWhole`), `typefields_cpp.bas` (`cpp`:
  the layout, the functions of one owner type with every kind of part, a `REDIM`, the file pack);
  `member_arrays_errors.bas` loses its marker lines.
- **Corpus**: console slice programs from `v35_a`–`v35_g` and `x56`–`x58` (each recorded with `qb64pe.exe`; a
  divergent line recorded from a program without it, as for D-004), on `slice.list`.
- **Upstream**: the 93 programs leave `deferred.list`; those that pass join `pass.list`, the `.err` programs among
  them by their error.

## Risks / Trade-offs

- [Memory errors in the life functions (a descriptor freed twice, a leak) show as crashes far from their cause] →
  every slice program runs to its end and prints after each step; the upstream programs (`t2xx`) exercise every
  parent operation; a program that ends early or with an exit code other than its recorded one fails its
  comparison.
- [The number of index evaluations of D7 is read from the old compiler's code and may be wrong for a nesting not
  measured] → `x56` covers the flat case and one with strings; an owner type nested in another is measured in
  task 3.3 before it is written, and a case that does not match is put to the user.
- [A type's layout changes when one of its members gains `_DYNAMIC`] → only under the flag, as in the old
  compiler; the `cpp` snapshot of a packed type with `_STATIC` members is unchanged (`x53`).
- [`_MEM` of an owner value gives a block over pointers] → as in the old compiler (`x22`, `x52`); `_MEMGET` of
  such a block reads the pointer bytes. Not on any test list.

## Open Questions

None. The three choices that change behaviour were put to the user before this design (`proposal.md`).
