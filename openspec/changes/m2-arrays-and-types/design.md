# Design

## Context

See `proposal.md` for motivation and the spec deltas for requirements. Background: how the old compiler encodes
references, elements and members (`study\02` §2.1–§2.3), its array descriptors and allocation (§3.4), argument
passing (§4.1); the IR review's place question (`study\25` §3) and its value-tree criterion (§2.2); the order of
work (`study\24` §2, §4). The designs of the earlier changes still hold, in particular D8 of
`m2-control-flow-slice` (flat bodies, explicit jumps, the pending-error rule; final since `study\25`), and D3
("`a.b` stays one `Ident`") and D7 (`FieldExpr` after an index) of `m2-parser-breadth`.

Where the code stands (2026-10-07):
- The parser does not parse bounds in `DIM`: `DIM a(5) AS LONG` gives a `DimItem` holding `a` and an `Error` node
  for the rest. `TYPE … END TYPE` is a `TypeBlock` with `TypeField`s (`FieldName`, `AsClause`, element bounds for
  member arrays). `a(1).b` is a `FieldExpr`; `p.x` is one `Ident` (QB64 names may contain dots).
- `sema` marks a `TYPE` block, `DIM` of an array, a `DIM … AS <type name>`, `a(…)` that is no function, member
  access, `LBOUND`/`UBOUND` and array parameters "not supported yet".
- The IR's places are `VarId`s: `Op::Assign { place }`, `Op::AssignAll`, `Arg::Ref`, `ValueKind::Var`.
- Tier 1 lowers and emits only the snapshot tests; the 127 accepted corpus programs and the 23 upstream ones are
  first lowered in tier 2.

## Goals / Non-Goals

**Goals:**
- The IR names every place the slice needs (variable, element, member, member of an element), with the store rule
  per place measured, not read off the old compiler's source.
- `Ty` has user types, and every exhaustive `match` on `Ty` handles them (the no-wildcard lint, `study\21`).
- The corpus and upstream programs blocked only by the slice's constructs pass end to end.
- No program the old compiler rejects is accepted; nothing unmeasured is guessed.
- The value-tree question of `study\25` §2.2 is answered.

**Non-Goals:**
- `REDIM`, dynamic arrays, `OPTION BASE`, implicit arrays, array parameters, `ERASE`, member arrays, whole-array
  and whole-`TYPE` assignment, `TYPE` parameters, fixed-length strings, unsigned and `_BIT` types (later steps,
  `SOMEDAY.md`).
- The type table of step 7 (`Ty` as an index into a table): this change adds `Ty::User(TypeId)` and a list of
  user types, which step 7 then folds in.
- Matching the old compiler's C++ text beyond the ABI.

## Decisions

### D1. Measure first
Before code, `verification\v18_*` programs (run with `verification\run.sh`), findings in `study\00` §5. Each
program prints what it measures; error cases trap with a handler that prints `ERR` and does `RESUME NEXT`. The
questions:

- **Store rule per place** (`study\25` §3 item 2): with `x(9)` holding 5, `x(9) = ASC("")`; `x(99) = 5` for
  `DIM x(10)`; `u.m = ASC("")`; `a(99).m = 5` for an array of a user type (and which element, if any, is written);
  a scalar store `y = x(99)` (the placeholder of an element read: 0 or the value of some element?).
- **Two raising parts**: `x(99) = ASC("")` and `a(99).m = ASC("")`: which error number `ERR` reports, and whether
  the handler runs once or twice.
- **Element passed by reference** (`study\25` §3 item 3): a SUB that increments its parameter, called with `x(3)`,
  `(x(3))`, `x(99)`, `u.m`, `a(2).m`; a SUB with two parameters called with two raising arguments in both orders
  (which error `ERR` reports).
- **Indexes**: out of range above and below (error 9), a float index (`x(1.5)`, `x(2.5)`, `x(-0.5)`: which
  element), a string index (compile error), the wrong number of indexes, an index in a `PRINT` item.
- **Bounds**: `DIM a(l TO u)` with negative bounds, several dimensions, `DIM a(0)`, `DIM a(-1)`, reversed constant
  bounds, a `CONST` as a bound, a non-constant bound in the main module (dynamic: what is marked), `DIM` of an
  array inside a SUB (dynamic, per call: marked), `DIM` of the same array twice, a `DIM` line run twice (in a
  loop).
- **`LBOUND`/`UBOUND`**: with and without the dimension, a dimension out of range (0, too high), a float
  dimension, the type of the result (formatting of `UBOUND(x) / 3` and of a large value), on an array of a user
  type, on a name that is no array (compile error).
- **Names**: an array and a scalar of the same name, an array with a suffix (`DIM a%(3)`, `a%(1)`; `DIM b(3) AS
  LONG` then `b&(1)`, `b!(1)`, `b(1)`), an array and a FUNCTION of the same name, an array used before its `DIM`
  line, `DIM SHARED` arrays read and written from a SUB, an element as `FOR` variable, initial values (0, `""`).
- **`TYPE`**: members of each slice type, a nested user type, a member with a suffix in use (`p.x&`, `p.x%`),
  a member declared with a suffix, an unknown member (measured: "Element not defined", `m2-parser-breadth` M8), a
  user-type variable local to a SUB (zero on each call?), `STATIC` and `SHARED` ones, a `TYPE` used before its
  block, a `TYPE` block inside a SUB, a whole user-type value in an expression or `PRINT` (compile error), `p = q`
  (accepted: marked here), a member as `FOR` variable (marked since `m2-control-flow-slice`), a `STRING` member
  and a `STRING * n` member (to decide D3).
- **Error numbers and order**: the error raised by each of the above, so the slice programs can cover every
  measured rule.

### D2. Parser: array bounds in `DIM`
`DimItem` gains the `ArrayBounds` child `TypeField`'s `FieldName` already has (`(u)`, `(l TO u)`, several
dimensions, `()`), with an accessor `DimItem::bounds`. Only `DIM`, `DIM SHARED`, `STATIC` and `SHARED` items; the
`REDIM` statement and array parameters (`a()` in a parameter list) stay with `m2-parser-breadth` task 7.2. The
parse-gap list shrinks by every program whose only gap was a `DIM` with bounds.

### D3. `sema`: user types
Measured (2.1): `study\00` §5 "`TYPE`", `verification\v18_h_*`.
- `TYPE` blocks of the main module are collected **before** the statements are checked, like procedures (pass 1):
  measured, a type may be used before its block. A member's type must be one of the slice's numeric types or a user
  type; a user type used as a member must be defined in an earlier block (a later or the same one is "not
  supported yet": not measured). `sema::Program` gains `types: Vec<UserType>` (name, members in order with their
  types) and nothing about layout (sizes and offsets are the emitter's, D7). Member arrays, `STRING * n`, `STRING`
  (decided 2.1: no corpus or upstream program blocked only by this slice needs one, and they need initialisation
  and release per variable), unsigned and `_BIT` members: "not supported yet". A member declared with a suffix
  (`x&`, `x& AS LONG`) and a type name already used: real errors. A `TYPE` or member named like a keyword or
  built-in (measured after 7.1, `v18_h_type_named_*`, `v18_h_member_names*`): `TYPE Point`, `TYPE cls` and
  members `left`, `len`, `color`, `name` are accepted; `TYPE len`, `TYPE print`, `TYPE long` and a member `print`
  are "Name already in use"; any other such name is "not supported yet" (no rule fits the data: as variable names
  all of these are taken). A `TYPE` block inside a SUB (accepted by the old
  compiler): "not supported yet".
- `Ty` gains `User(TypeId)`. `Ty` stays `Copy`; `is_numeric` is false for it; `qb_name` needs the type's name, so
  the dumps print it through the program's type list.
- `DIM p AS t` declares a variable of the user type in every storage class a scalar may have (main, `DIM
  SHARED`, local, `STATIC`, `SHARED p AS t`); `DIM p& AS t` is a real error.
- **Dotted names** (measured, M8 and `v18_h_dotted_before_dim`, file order): `a.b` is member access when a scalar
  variable `a` of a user type is visible at that point, else a plain variable named `a.b` (also before a later
  `DIM a AS t`). When an **array** `a` of a user type is visible and no such scalar, `a.b` is a real error ("Invalid
  expression"). An unknown member is a real error; a member used with its own type's suffix (`p.x&`) is the member,
  with another suffix a real error. `x(1).b` for a numeric array is a real error.
- A whole user-type value is allowed only as the base of a member access. In an expression, a `PRINT` item, or
  stored into a scalar it is a real error ("User defined types in expressions are invalid"); a number stored into
  it (`p = 5`) is a real error; `q = p` and a whole value passed to a `TYPE` parameter (accepted by the old
  compiler) are "not supported yet"; a `TYPE` parameter is "not supported yet".

### D4. `sema`: arrays
Measured (2.1): `study\00` §5 "Bounds", "Indexes", "Names", "`LBOUND`/`UBOUND`", `verification\v18_d_*` to
`v18_g_*`.
- An array is a `Var` with `dims: Vec<(i64, i64)>` (lower and upper bound of each dimension), element type the
  variable's `ty` (the slice's numeric types, `STRING`, a user type). Bounds are constant expressions evaluated by
  the `CONST` evaluator (`consteval`, so `CONST` names and `-(2 ^ 2) TO 10 \ 3` work) and converted to
  `_INTEGER64` rounding half to even; an upper bound below the lower one (also `DIM a(-1)`) is a real error. A
  bound the evaluator cannot take (a variable: a dynamic array; or one of its "not supported yet" corners) is
  "not supported yet".
- An array is a name plus a type, in its own name space beside scalars (measured): `DIM c(3) AS LONG` is `c&()`;
  `c&(2)` names it, `c!(1)` would be an implicit array (below).
- Supported: `DIM` and `DIM SHARED` in the main module with constant bounds; their `DIM` does nothing at run time
  (static). A second `DIM` of the same array is a real error. Everything else that makes an array is "not
  supported yet": `DIM` in a procedure, `STATIC a(n)`, `REDIM`, `$DYNAMIC`, `OPTION BASE`, `SHARED a()`, array
  parameters, and **implicit arrays**: `a(…)` where no array `a` of that type is visible and no FUNCTION `a`
  exists (also in a SUB for a main array that is not `DIM SHARED`, and `LBOUND(x)` of a non-array, which makes
  one).
- An array and a FUNCTION of the same name: real error. An element as `FOR` variable: real error.
- `a(i, j)` names an element when `a` is an array visible at that point: one index per dimension (else a real
  error), each converted to `_INTEGER64` as for an assignment (half to even, measured); a string index is a real
  error. `a` alone is the scalar `a`.
- `LBOUND(a[, d])`, `UBOUND(a[, d])`: a node of their own, `ExprKind::Bound { upper, array, dim }` (the first
  argument is an array, not a value), typed `_INTEGER64` (measured; the table's LONG is wrong), `dim` converted to
  LONG half to even, absent for the first dimension. Not folded (decided 2.1): a dimension out of range raises 9 at
  run time, also when it is a constant, and the emitter calls libqb's functions. `LBOUND(a())`: real error.

### D5. Places in the typed tree and the IR
`sema` and the IR both gain:

```text
Place = Var(VarId)
      | Element { array: VarId, index: Vec<Expr> }   indexes converted to _INTEGER64, one per dimension
      | Member { base: Box<Place>, member: MemberId } base is a place of a user type
```

`ExprKind::Var` becomes `Load(Place)`, `StmtKind::Assign { place: Place }`, `Arg::Ref(Place)`; the same in the IR
(`ValueKind::Load`, `Op::Assign { place }`, `Arg::Ref`). `Op::AssignAll` keeps `VarId` (only the lowering's
temporaries). A `Place::may_raise`: an element's index may raise error 9; a member never raises by itself.
`sema` resolves `a(i).m` to `Member { base: Element { … } }` and `p.a.b` to two nested `Member`s.

### D6. The store rule per place
Measured (2.1): `study\00` §5 "Store rule per place", `verification\v18_a_*`, `v18_b_*`, `v18_h_member_index_order`.
The first error of a statement wins throughout (libqb's `error()` keeps a pending error).
- `Var`: the value is evaluated and stored, also as a placeholder (`v17_a_pending`).
- `Element`: the indexes are evaluated first, left to right, each checked against its own dimension (error 9);
  **while an error is pending after them the value is not evaluated and nothing is stored**; else the value is
  evaluated and stored, also as a placeholder.
- `Member` of a variable (`p.m`, `p.a.b`): as `Var`.
- `Member` with an element in its base (`a(i).m`): **the value is evaluated first, then the indexes** (measured);
  the store is skipped when an index is out of range or when the indexes raised an error while none was pending
  before them. **Decided (user, 2026-10-07, `DIVERGENCES.md` D-004):** the old compiler stores into element 0 in
  both cases; the order and the error reported stay as measured. When the value's error is pending, an index that
  raises in range cannot be told apart and the store goes to its placeholder element, as in the old compiler.
- A read with a bad index raises 9 and gives **element 0's value** (measured; "keep", `study\00` §6): the emitter
  reads through libqb's `array_check`, which returns position 0 after raising.
- An argument that is an element or a member of exactly the parameter's type is passed by reference; arguments
  are evaluated left to right; a procedure entered with an error pending returns at once, so a bad index never
  reaches the procedure (measured, `v18_c_byref`).

The IR states this as part of the pending-error rule, per place; the emitter encodes it (D7).

### D7. The emitter
- Arrays: a descriptor per array (`ptrszint` slots as libqb expects, `study\02` §3.4: data pointer, flags 1+2,
  per dimension in reverse order lower bound, count, multiplier), allocated once at program start and zero-filled;
  string elements each `qbs_new(0,0)`. The flat position with `array_check` per dimension, first dimension
  fastest. The element store guard and the member-of-element store of D6.
- User types: a byte block per variable, members at offsets the emitter computes: **members in order, no
  padding**, sizes INTEGER 2, LONG 4, `_INTEGER64` 8, SINGLE 4, DOUBLE 8, `_FLOAT` 32, a user type its size
  (measured with `LEN`: 64 for one member of each plus a 6-byte type). Arrays of a user type: one block of
  `count * size` bytes. A local is zeroed on each call, a `STATIC` one once.
- Elements and members passed by reference: their address (`&element`, a member's `(T*)(base + offset)`), in
  argument order.
- `LBOUND`/`UBOUND`: libqb's `func_lbound`/`func_ubound` with the descriptor, the dimension and the number of
  dimensions.

### D8. Tier-1 safety net (first task)
`crates\driver\tests\inputs.rs` lowers and emits every accepted input (the front end gave no error) under
`catch_unwind`, without compiling C++, and runs `ir::validate` on the IR. `ir::validate(&Program) -> Result<(),
Vec<String>>` checks: every label is placed (`at <= stmts.len()`); every `Jump`, `Branch` and `Gosub` names a label
of its own body and every `Return(Some)`, `Resume::To` and `SetHandler` a label of the main module; every `VarId`
and `ProcId` referenced exists; a procedure's parameters and result have its storage class; a `Temp` variable is
used only in its own body. The lowering calls it under `debug_assert!` too, replacing the "label never placed"
assertion. A failure names the input and the first problem.

### D9. Tests and corpus
- `tests\frontend\`: `parse-ok` for the `DIM` bounds forms, `typed` for places and `LBOUND`/`UBOUND`, `check-fail`
  for every real error and marked form of D3–D4, `ir` and `cpp` for each place and each store rule.
- `tests\corpus\slice\s20_arrays`, `s21_types`, `s22_store_rules` (names may change), recorded with `qb64pe.exe`,
  one end-to-end program per measured rule of D6 and per scenario of the spec deltas.
- The three shrink-only lists are regenerated after each group.

### D10. `ir::Value` or `sema::Expr` (last task)
By the criterion of `study\25` §2.2: if after this change `value_of` in `lower.rs` is still a field-by-field
mapping (count its lines that are not `Box::new(value_of(..))` or a one-to-one variant), `ir::Value` is replaced
by `sema::Expr` (re-exported by the IR as `Ty` and `BinOp` are; the emitter ignores `span` and `qb`), and
`ir::Place` by `sema::Place`; the IR keeps its own variable list (the lowering's temporaries). If the lowering
does work on values (index arithmetic, member offsets as IR operations), the copy stays. The decision and the
count go into this design ("As decided") and `DECISIONS.md`.

**As decided (task 8.1, 2026-10-07): shared.** After the slice, `value_of`, `args_of` and `place_of` in
`lower.rs` were 66 lines, every arm a variant copied into its twin (`Int` into `Const(Int)`, `Call` into
`CallBuiltin`, `Load(place)` into `Load(place)`); none rewrote a value. Index arithmetic, `array_check`, the
descriptor slots and member offsets all ended up in the emitter, as §3 item 5 of `study\25` recommended. So the IR
now re-exports `sema`'s `Expr`, `ExprKind`, `Place`, `Arg`, `VarId` and `ProcId`; the three functions became
clones, and the `FOR` lowering builds its values with its statement's span (`Make`). What stays the IR's own:
the statement side (`Op`, `Stmt`, `Body`, labels), its variable list (`Var` with `Storage::Temp`; `sema`'s ids
with the temporaries appended) and the facts the error rule asks of a value (`may_raise`, `uses_strings`, trait
`Facts`). No snapshot changed (the IR dump prints the same text), tier 1 and tier 2 unchanged. The compiler
engineer's note of `study\25` §4 (a `Place` in `sema` is fine, since `sema` resolves `a(i)` anyway) is what
happened.

## Risks / Trade-offs

- **The old compiler's member store writes element 0 on a bad index**: measured (`v18_b_member_store`), decided
  (D6, D-004). The divergence shows only after an error 9 the program ignores.
- **Static allocation of large arrays**: measured, `DIM big(10000000) AS LONG` works in the old compiler. The new
  one allocates at program start as well; a size beyond what the descriptor's `ptrszint` count can hold is a real
  error in `sema` (none in the inputs).
- **`Ty` stops being a closed set of seven**: every place that assumed a scalar (`is_numeric`, printing,
  conversions) needs a decision for `User`. The no-wildcard lint finds them; the risk is a wrong arm, covered by
  `check-fail` tests for user types in each context.

## Open Questions

None left after 2.1: string arrays are in, `STRING` members later (D3); `LBOUND`/`UBOUND` are not folded (D4).
