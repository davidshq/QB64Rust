# Design

## Context

Motivation, scope and what was measured: `proposal.md` (`verification\v36_*`, 58 programs, 2026-10-15). Where
whole arrays stand in this compiler (2026-10-15):

- **`sema`** `check\mod.rs` `assign` marks `x.v() = y.v()` and `x.v() = a()` "whole-array assignment of member
  arrays"; any other `name()` in a value reaches `check\places.rs` (lines 831, 987) and is "the whole array
  `name()`" (not supported yet). `check\proc.rs` already resolves a whole array for an array argument:
  `whole_array` (a name with empty parentheses, by `find_array`) and `member_array` (`check\places.rs` 1151: a
  chain of members ending in a member array, as an `ArrayRef::Member`). The number of dimensions of an array
  parameter is solved from facts (`DimFact::Count`, `DimFact::Same`, `solve_dims`).
- **`ArrayRef`** (`sema\lib.rs` 657) is `Var(VarId)` or `Member { base: Box<Place>, member }`; the IR uses it for
  `Allocate`, `Clear`, `Release` and array arguments.
- **`codegen-cpp`**: an array variable's descriptor is `self.name(id)` (a parameter's is the caller's
  descriptor). A whole member array as an argument gets a descriptor of its own (`value.rs`
  `member_array_arg`: a `passN` of the body, flags allocated and static, the member's bounds, its block set where
  the place is found once); a dynamic member array is passed as its own descriptor
  (`owner::dyn_fn(id, m, "desc")(slot)`). Element copies exist: `stmt.rs` `copy_parts` (a user type part by part,
  `qbs_set` for a string, `memmove` for bytes) and `owner.rs` `copy_owner` (an owner value, deep).
- **The old compiler** (`qb64pe.bas` 17343 `assign`): only `name() = name()` or member chains ending in `()`,
  each name passing `validname` (so a name with a suffix never enters: `v36_x24`, `x38`–`x44`) and resolved as an
  array, take the path; `EmitAssignWhole` checks the types, sizes and dimensions, writes the member descriptors
  (target, then source: `x49`) and calls `AppendWholeArrayAssign` (plain and inline user types) or
  `AppendDynUDTArrayAssign` (arrays of owner types). Every other form goes on as an ordinary assignment, whose
  guard (`CheckAsgRHS`) gives the two messages of the proposal when either side is an array without an index.

## Goals / Non-Goals

**Goals:**

- `a() = b()` and the member forms compile and run as the old compiler's program does, for every element type
  and place, with its compile-time and run-time errors.
- One IR operation; the checks and the copy are code generation.

**Non-Goals:**

- Following two behaviours of the old compiler: a copy made after an index on the way to a member array raised
  error 9 (D-004 widened), and the copy of an array of an owner type made after its bounds check failed (a read
  past the source's block, `v36_x52`; `DIVERGENCES.md` D-022). Both copy nothing here.
- Suffixed names (`a%() = b%()`): "not supported yet" (user, 2026-10-15).
- Giving the target new bounds, `_ARRAYCOPY`, a plain scalar beside an array typed by `DIM … AS`.

## Decisions

### D1. The form, in `assign`

`assign` takes the whole-array path when **both** sides are whole: an `ast::Expr::Call` with an empty argument
list, or an `ast::Expr::Field` (member chain) ending in one; the source may be wrapped in `ast::Expr::Paren`
(any depth, `v36_x14`). Then:

1. **A suffix on either name** (the last name of a chain): both suffixed: "not supported yet" ("whole-array
   assignment of arrays named with a type suffix"); one suffixed: a compile error at the other side, "`name()` is a
   whole array; the other side is not" (the old compiler: "Expected array index after" the unsuffixed name,
   `x41`, `x42`).
2. **Each side resolved as an `ArrayRef`**: a plain name by `find_array` (no array of that name: the error of the
   table in D2), a chain by `member_array`. The target first, then the source, so the `names` of the language
   server keep the source order.
3. **The checks** (D2), then `StmtKind::CopyArray { from: ArrayRef, to: ArrayRef }`.

When only one side is whole, or neither, the statement goes on as today, with D2's table applied where today's
code would reach "the whole array `name()`" in an assignment, so the eight `.err` programs get real errors.

*Alternative considered*: resolving `x()` as a general expression kind. Rejected: nowhere else is a whole array
a value, and the old compiler's other uses (element 0, `x37`) stay marked.

### D2. Compile-time checks and the error table

- **Element types** equal as `Ty` (kind, width, sign, `STRING * n` length, `_BIT * n` width and sign, user type
  id): else "cannot assign an array of `T1` to an array of `T2`" (old: "Cannot assign arrays of different types",
  or "… with different element sizes" for two lengths of `STRING * n`). `_MEM` arrays are user types and pass
  (`x31`).
- **Dimensions**: both known and different: an error (`x08`). One unknown (an array parameter, `COMMON a()`,
  `STATIC a()`): a `DimFact::Count` with the other's number; both unknown: a `DimFact::Same` (as an argument
  passed to a parameter). `solve_dims` then reports a conflict as it does for an argument (`x45`).
- **The table**, by the old compiler's `CheckAsgRHS` (state of a side: 0 no array, 1 an array named bare, 2 an
  array with empty parentheses, 3 an element): a 2 on either side with a nonzero state on the other is "both sides
  of a whole-array assignment need empty parentheses" at the statement (`x02`, `x26`; `t026`, `t027`); a 2 against
  a 0 is "`name()` is a whole array; the other side is not" at the 2 (`x01`, `x03`, `x04`, `x13`; `t024`, `t025`,
  `t037`, `t038`, `t040`, `t041`). Undeclared names and a `CONST` on the other side (`x15`, `x16`, `x29`) are
  measured as the first message; a FUNCTION (`x18`) as "Expected (...)". The member forms (`t028`–`t044`) keep
  the errors they get today.
- **Owner types**: no "storage layouts incompatible" error: every pair measured is accepted (`x46`, `x47`), and
  this compiler has one layout per type.

### D3. The IR operation

`Op::CopyArray { from: ArrayRef, to: ArrayRef }`, documented in `ir\lib.rs`: `to`'s place is found, then
`from`'s, each once (an `ArrayRef::Member` with indexes on the way; an index outside its bounds raises error 9);
**if anything raised so far, nothing is copied**; else both arrays must be allocated and have, in every dimension,
the same lower bound and number of elements, else error 9 and nothing copied; the same block on both sides copies
nothing; else every element is copied deep, by kind. Always may raise. Validation: equal element types and number
of dimensions (where known), `to` not a constant array. Dumped as `CopyArray <from> -> <to>`.

### D4. The C++ of the copy

`codegen-cpp\arrays.rs` `copy_array`, the old compiler's `AppendWholeArrayAssign` (17272) with its local names:

```cpp
{
ptrszint *whole_arr_dst=<to>;
ptrszint *whole_arr_src=<from>;
int32 whole_arr_ok=1;
uint64 whole_arr_total=1;
if (!(whole_arr_dst[2]&1)||!(whole_arr_src[2]&1)||(!whole_arr_dst[0])||(!whole_arr_src[0])||(whole_arr_dst[0]==(ptrszint)nothingvalue)||(whole_arr_src[0]==(ptrszint)nothingvalue)){error(9); whole_arr_ok=0;}
if (whole_arr_ok){ for each dimension: lower and count equal, else error(9), ok=0; count < 0: error(9); overflow: error(257); total*=count }
if (whole_arr_ok&&(whole_arr_dst[0]!=whole_arr_src[0])){ <the copy by kind> }
}
```

- `<to>`, `<from>`: an array variable's descriptor; a member array's from `member_array_arg` (a `passN` set where
  the place is found) or a dynamic member's `desc(slot)`, target first. When either side has an `ArrayRef::Member`
  whose place may raise, `whole_arr_ok` starts as `!is_error_pending()` (D-004 widened).
- **By kind**: numbers, `_OFFSET`, `STRING * n`, `_BIT` (`width/8+1` bytes, as the allocation gives an element),
  `_MEM` and user types that hold no string: one `memmove` of `total * size` bytes under `if
  (!is_error_pending())`, as the old compiler writes it; `STRING`: `qbs_set` element by element; a user type that
  holds strings: `copy_parts` per element; an owner type: `copy_owner` per element, in place, for a `DIM` and a
  `REDIM` target alike.
- **Owner types, not the old compiler's replace-copy**: it builds a new block for a `REDIM` target and frees the
  old one; the result is the same values (`v36_e`), and `_MEMEXISTS` of a block on a member of an element is true
  after either (`x51`); in place no block of the program moves. The bounds check is the one above, so a failed
  check copies nothing (`DIVERGENCES.md` D-022; the old compiler copies `total` of the target's elements,
  `x52`).

### D5. Tests

- `tests\frontend\`: `whole_array_errors.bas` (`check-fail`: every rejected form of `v36_x*` and the four
  member forms), `whole_array_ir.bas` (`ir`: variable to variable, member to variable, a dynamic member),
  `whole_array_cpp.bas` (`cpp`: one array of each kind, a member array, an owner type).
- `tests\corpus\`: `s119_whole_array` (numbers, strings, `STRING * n`, `_BIT`, user types, places, the
  run-time errors) and `s120_whole_array_members` (member arrays, the index outside, owner types below the flag),
  recorded with `qb64pe.exe` apart from the two divergences, whose lines are left out of the slice programs and
  are tested by CLI tests with this compiler's output.
- `verification\v36_*` built and run with this compiler: the same output, apart from the two divergences.

## Risks / Trade-offs

- **[A wrong copy is silent]** → the slice programs print every element after each copy, and `v36_*` is
  compared whole.
- **[Facts on parameter dimensions]** → a mismatch found by `solve_dims` reports at the assignment, with a
  front-end test.
- **[Two more divergences]** → both are memory or store-through-element-0 behaviours of the kind the user chose to
  fix (2026-10-14); listed in `DIVERGENCES.md` and put to the user with the change.
