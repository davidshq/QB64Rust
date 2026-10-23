# Design

## Context

For the motivation, scope and the findings before this proposal, see `proposal.md`. For the behaviour, see the
delta specs. Where the four parts stand in this compiler (2026-10-20):

- **Names and types.** A scope (`sema\check\mod.rs` `Scope`) keeps scalars in `vars` keyed by (name, type) and
  arrays in `arrays` keyed the same way. The type a plain (suffix-less) name means comes from `plain` for a scalar
  declared with `AS`, from `array_plain` for an array declared with `AS`, and otherwise from the default type
  (`check\decl.rs` `var_key`, `check\places.rs` `array_key_ty`). A procedure also sees the main module's `DIM
  SHARED` names (`dim_shared_plain`, `dim_shared_array_plain`). Two marks stand where the two kinds of name meet.
  One is `check\decl.rs` `variable`: a plain scalar that is not found, where `array_plain` has the name. The other
  is `check\places.rs` `declare_array` (about line 535): an array without `AS` beside a typed scalar or array, or
  one typed otherwise than a scalar of the name.
- **`REDIM` without parentheses.** `check\decl.rs` `declare_each` (about line 622) marks a `REDIM` item without
  bounds in a procedure. The same item in the main module, and `DIM name AS type` in a procedure, go on to the
  scalar declaration below that check.
- **`TYPE` blocks.** Pass 1 (`check\mod.rs` `check`, about line 150) calls `declare_types` on the main module's
  top-level statements only (`check\places.rs`). Pass 2 meets every other block in `type_block`, which marks it
  and starts the follow-on rule. `collect_block_settings` (`check\mod.rs`), which runs just before, already walks
  every block and procedure in file order (into included files, past inactive `$IF` code). It records the
  `OPTION BASE` and the `$UNSTABLE:TYPEFIELDS` flag at each `TYPE` block it meets, nested or not.
- **`LEN` of a whole array.** `check\builtins.rs` `len` marks a whole member array. A whole array `a()` reaches
  `check\places.rs` `element`, which marks it ("the whole array `a()`"). `_MEM(a())` already names a whole array as
  `Reference::Array(ArrayRef)` (`check\mem.rs` `reference_or_value`). `bytes_reference` holds the checks the old
  compiler's byte helpers make: the `_BIT` error, the string-array error, and marks for a `TYPE` with a `STRING`
  member, a dynamic member array and the paths not measured. `codegen-cpp\src\mem.rs` `bytes_count` writes the
  old compiler's "−5" size expression for a `Reference`: an array's bytes from its first element (`all_bytes`),
  or a member array's bytes.

## Goals / Non-Goals

**Goals:**

- The marks above are gone wherever `v48` shows that the old compiler's behaviour is the one the delta specs
  describe. Each pairing it does not cover stays marked, and has its row in `SOMEDAY.md`.
- One reading of a name: the lookup that already serves `a(1)` and `a` serves the new pairings. No second path.

**Non-Goals:**

- A whole array as a value (`PRINT a()`, `SWAP x(), y()`, `READ a()`), and the old compiler's errors for names
  that are no array or no SUB (`_foo(1)`, a FUNCTION defined later, a FUNCTION called as a statement): left for
  later by the user (`DECISIONS.md` row of task 1.2).
- A built-in's name with a type suffix (`LEN$`, `abs$`, `base$`): with the item above.
- `$CHECKING:OFF`: `mem/mem_checking_off` keeps that mark after its `TYPE` block is compiled.
- `LEN` of a whole dynamic member array: marked, as `_MEM` of one is.
- A second `TYPE` block of a name already defined (two SUBs that each define `TYPE pt`) stays "not supported
  yet" unless `v48` shows a clear rule.

## Decisions

### D1. A plain scalar beside a typed array has the default type

In `variable` (`check\decl.rs`), the `typed_array` mark goes. A plain name that no declaration made a scalar is
then keyed by `var_key` as it already is: by `plain` (no entry), by `dim_shared_plain`, else by the default type.
So it is a new scalar of the default type, apart from the array, which is what `v30_x67`, `v18_d_index_none` and
`v30_x47` show. Nothing else changes for it. The scalar's own declaration, `DIM a AS LONG` beside `DIM a(3) AS
LONG`, is handled already (`t001`'s `valueSet`, `v24_x56`).

*Alternative*: give the scalar the array's type. Rejected: the three recordings show the default type (`PUT`
writes 4 bytes beside an INTEGER array).

### D2. Between two arrays of one name, the typed one keeps the plain name

`v48` showed that the first idea (an array without `AS` takes the plain name from a typed one) holds only in two
cases. The old compiler finds a plain array name by the array declared with `AS` in the scope, whichever came
first: `DIM c(2) AS DOUBLE` then `DIM c(3)` leaves `c(1)` the DOUBLE array and makes `c!` another array, in the
main module and a SUB alike (`v48_a` 7, `v48_x17`, `x18`, `x22`), and a `REDIM c(3)` without `AS` gives the
DOUBLE array new bounds (`x16`, built already). The two cases where the plain name moves:

- **A local array beside a shared one.** In a SUB, `DIM c(3)` beside the main module's `DIM SHARED c(2) AS
  DOUBLE` is the SUB's SINGLE array, and the plain `c` names it there (`x20`): the scope's own array comes first.
- **A `STATIC c() AS DOUBLE` list not reached yet.** The old compiler makes no array for the list until a `DIM` or
  `REDIM` of its type, or a use, reaches it (`qb64pe.bas` 9260; `x21`: a use makes it, error 9). A `DIM c(3)` or
  `REDIM c(3)` without `AS` before that does not match the list (its type differs), so it makes a SINGLE array,
  which is the only `c` there is (`v24_x126`, `v48_a` 11).

In `declare_array` (`check\places.rs`), an array declared without `AS` beside a typed scalar or array is made with
the default type, as now for a name with no typed neighbour; the mark goes. The plain name then follows the rule
above: `array_plain` keeps the typed array, except that the new array takes it (a) in a procedure, when the typed
entry is the main module's (`dim_shared_array_plain`; the scope gets an entry of the default type, which
`array_plain_ty` reads first), and (b) when the typed entry is an array listed by `STATIC … ()` that nothing has
reached yet: it is the procedure's own (`Storage::Static`) and has no number of dimensions (`dim_count` is
`None`). In case (b) a `REDIM` without `AS` does not take the listed array's type. A typed array beside a scalar
typed otherwise (`DIM d AS STRING: DIM d(3) AS LONG`) is two variables (`v48_a` 9): that mark goes too. An array
without `AS` beside one listed by `COMMON … AS` (in our model a listed array of the main module) is not measured
and keeps a narrowed mark, with a test and a `SOMEDAY.md` row.

*Alternative*: an array without `AS` always takes the plain name (the first form of this decision). Rejected by
`v48_a` 7 and `x17`.

### D3. `REDIM name [AS type]` in a procedure is the scalar `DIM name [AS type]` declares

The mark in `declare_each` (`check\decl.rs`) goes. The item continues to the scalar declaration that `DIM` uses
in a procedure, with `local_storage` (a `STATIC` procedure's storage above its first `STATIC` statement). `v48`
measures a second run of the `REDIM` (does it clear the scalar?), the item in a list with arrays (the old
compiler's own `REDIM provided(…) AS LONG, i AS LONG`), a `STATIC` procedure, and a name used before the `REDIM`.
`v48_b` shows that a second run keeps the value (`i`, `j` and a `STRING` alike), so nothing runs and no statement
is added. Below a use of the name it is `DIM`'s rule (`v48_x09`: `REDIM i AS LONG` makes a LONG `i`; `x10`: `REDIM
i` is "Name already in use", the error `DIM j` gives here already).

### D4. `TYPE` blocks are collected in pass 1 wherever they stand

`collect_block_settings` (`check\mod.rs`) appends each `TYPE` block it meets to a list in file order, the
top-level ones included. `declare_types` (`check\places.rs`) takes that list in place of the top-level
statements, with the same checks (`skips.past_cap`, `skips.usable`, `types_seen`, `types_marked`). So a block in
a SUB, a FUNCTION, an `IF`, a loop or an included file defines its type before any statement is checked, as the
old compiler's prepass does (`qb64pe.bas` 2483). `type_block` in pass 2 then always finds the block in
`types_seen`, and its marking arm goes. The `OPTION BASE` of a member array without `TO` is the one recorded at
the block (`type_bases`), already right for a nested block. `v48` measures whether an `OPTION BASE` inside the
SUB above the block applies. The parser already reads `TYPE` blocks inside blocks and procedures (the marks
are reported from `sema`, not as parse errors), so the parser does not change.

A second block of a name is the old compiler's "Name already in use" wherever the two stand (`v48_x05`), the
check `declare_type` makes for top-level blocks. `v48_x12` and `x25` show that the old compiler rejects a parameter
typed by a `TYPE` whose block stands below the SUB's header ("Illegal SUB/FUNCTION parameter"), a top-level block
as well as a nested one. This compiler accepts both, which it did for top-level blocks before this change: a
program the old compiler rejects, with no wrong output. It stays so; the two programs go on `known.list` with that
reason and the gap gets a `SOMEDAY.md` row.

*Alternative*: types scoped to their procedure. Rejected: the old compiler has one list of types (`lasttype`),
and `v48` checks a type of a SUB used in the main module and in another SUB.

### D5. `LEN` of a whole array is the byte count `_MEM` uses

In `len` (`check\builtins.rs`), a call node with an empty argument list that names a visible array (as
`reference_or_value` finds one) becomes a new `ExprKind::ArrayBytes(VarId)` of type LONG (the old compiler's `LEN`
is LONG: `typ& = LONGTYPE`, `qb64pe.bas` 20962; its value is `evaluatetotyp(-5)`, the helper `bytes_count` writes).
A whole member array becomes the number, as `LEN` of an element does: its size is the member's element size times
its count, known when compiling, and the index on the way to it is not evaluated. Both are checked by
`bytes_reference` with the word `LEN`, which gives the string-array and `_BIT` errors and keeps the dynamic member
array marked. The `TYPE` with a `STRING` member is the old compiler's error for `LEN` only ("UDT must have fixed
size", `v48_x03`, as for an element, `x13`): `_MEM` of the same array is no error (`x11`, 48 bytes), so `LEN` makes
that check itself before `bytes_reference`, whose mark for `_MEM` stays. `LEN(x())` of a name that is no array is
marked as `_MEM(x())` is: the old compiler makes `x` an array used without `DIM` there (`x04`, `x14`: 44 bytes,
11 SINGLEs). `ir` validates `ArrayBytes` as a value of a whole array variable. `codegen-cpp` writes it with
`bytes_count` (`all_bytes`: 0 for a dynamic array that is not allocated). The whole array's name is recorded for
the language server, as `_MEM` records it.

*Alternative*: an `ExprKind::MemOf` with only its size read. Rejected: `MemOf` locks the array's block and makes a
`_MEM` value, which `LEN` must not do. `v48_d` shows that `LEN(a())` on an erased array is 0 and raises no error,
as the old compiler's plain size expression raises none.

### D6. The follow-on tests take `TYPE cos`

`tests\frontend\unsupported.bas`, `explicit_after_unsupported.bas` and `mem_follow_on.bas` replace their `TYPE`
block inside an `IF` with a top-level `TYPE cos … END TYPE` ("a `TYPE` named like a keyword or built-in", which
stays marked: its rule is not known, `check\places.rs` `declare_type`). The block is declared in pass 1, so the
follow-on rule starts where pass 2 reaches it (`types_marked`), at the same place in file order. The real error
before it and the silence after it stay. Their comments' list of former examples gets this one. The snapshots
change only in the line and column of the mark and its text.

### D7. Measurements: the family `v48`

Recorded with `qb64pe.exe` before the code (task 1.1), each with its line in `verification\README.md`:

- `v48_a_scalar_array`: every pairing of a scalar and an array of one name, one typed by `AS`. A plain scalar
  beside a typed array: main, SUB, a `DIM SHARED` array seen in a SUB, under `DEFLNG`, beside a STRING array, and
  `a!` against the plain `a`. An array without `AS` beside a typed scalar and beside a typed array (main and SUB),
  and the uses above and below it. A typed array beside a scalar typed otherwise. `GET` and `PUT` of the plain
  name.
- `v48_b_redim_scalar`: `REDIM i AS LONG` and `REDIM i` in a SUB and a FUNCTION, in a list after an array, run
  twice in one call (cleared or not), in a `SUB … STATIC` called twice, after a use of `i`.
- `v48_c_type_anywhere`: a `TYPE` in a SUB used above it in the main module, in another SUB as a parameter's
  type and in a FUNCTION; a `TYPE` in an `IF` block, a `FOR` loop and a FUNCTION; an `OPTION BASE 1` in the SUB
  above the block and a member array without `TO`.
- `v48_d_len_whole`: the spec's scenarios (numbers, two dimensions, fixed-length strings, a user type, a dynamic
  array before and after `ERASE`, an array parameter, a shared array in a SUB, an implicit array, a whole member
  array of a scalar and of an element, `LEN(parent())`).
- `v48_x*`: one rejected or accepted form each. `x01`–`x04`, `x13`, `x14`: `LEN` of a STRING array, of a `_BIT`
  array, of an array and an element of a `TYPE` with a `STRING` member, `LEN(x())` of a scalar and of a new name;
  `x05`: two SUBs that each define `TYPE pt`; `x06`: `REDIM SHARED i AS LONG` in a SUB; `x07`: `t001`'s pairing
  under `DEFLNG`; `x08`: `OPTION BASE 1` in the SUB above the block; `x09`, `x10`, `x15`: `REDIM` of a scalar below
  a use, with and without `AS`, and two scalars in one list; `x11`: `_MEM` of an array of a `TYPE` with a
  `STRING` member; `x12`, `x25`: a parameter typed by a block below the SUB, nested and top-level; `x16`–`x24`:
  which array the plain name names between a typed and an untyped array (main module, SUB, `STATIC` with bounds
  and listed, shared); `x26`: a `DIM` of a type above its block in the same SUB.

Where a recording differs from a delta spec or this design, the spec or design is changed to match in task 1.1,
before the code. What changed so (2026-10-21): D2 (the typed array keeps the plain name, `x16`–`x24`), D3 (a run
changes nothing), D4 (a second block is an error; the parameter order left as it is), D5 (`_MEM` keeps its mark; a
whole member array is a number; a name that is no array stays marked).

### D8. Tests and lists

- **Front-end tests** (`tests\frontend\`): `names_arrays_typed.bas` (`typed`: the scalar and array pairings of
  D1 and D2, `REDIM i AS LONG` in a SUB, a `TYPE` block in a SUB used in the main module), `names_arrays_errors.bas`
  (`check-fail`: the pairings that stay marked, the `LEN` errors), and `len_whole_cpp.bas` (`cpp`: the size
  expression for a static, a dynamic, a fixed-length-string, a user-type array and a whole member array).
- **Upstream**: `arrays/t001_whole_array_core` leaves `deferred.list`, which is left empty, and joins `pass.list`
  with `arrays/size`, `arrays/test_mem_vs_len`, `arrays/test_mem_vs_len_classic` and `types/udt_array`.
- **Verification**: `v16_m4_type_in_sub`, `v18_h_type_in_sub`, `v18_d_index_none`, `v24_d_erase`, `v24_x126`,
  `v30_x47`, `v30_x67` and the `v48` programs onto `tests\verification\pass.list`, off `known.list`; the `v48`
  programs this change leaves (`x04`, `x14`, `x11`: marked; `x12`, `x25`: accepted here, rejected there) onto
  `known.list` with their reasons.
- No slice programs: the `v48` family covers the forms (tenth review, `study\35` §3 item 2).
- No call-site programs: `LEN` of a whole array calls no libqb function.

## Risks / Trade-offs

- [D2 changes what a plain array name means below a declaration, in programs that pass today] → only a program
  with both a typed and an untyped declaration of one name reaches it, and such a program is marked today. The
  full corpus and upstream runs at the close show any other effect.
- [Collecting nested `TYPE` blocks in pass 1 also declares a block that pass 2 would skip, after the 100-error
  cap or in a block with a parse error] → `declare_types` keeps `skips.past_cap` and `skips.usable` for each block,
  as for top-level blocks.
- [The follow-on tests lose their history if `TYPE cos` is compiled one day] → their comments list each former
  example, and the mark's rule is in `declare_type`. A change that compiles it moves the tests again, as this one
  does.
- [`LEN(a())` of an erased dynamic array] → `bytes_count` computes from the descriptor without touching the
  elements; `v48_d` measures the old compiler's 0 and that nothing raises.

## Open Questions

None. What `v48` measures can change a spec's wording or narrow a part. Task 1.1 settles it before the code, and
then the specs and this design say so.
