# Design

## Context

See `proposal.md` for motivation and the spec deltas for requirements. Background: how the old compiler declares
and allocates (`study\10` §1: statement modes, when `DIM` code runs, `STATIC` list arrays, `COMMON`, `ERASE`;
`study\02` §3.4: the descriptor, static against dynamic, `REDIM`, `_PRESERVE`; §4.1: array arguments), what is
measured already (`study\00` §5: `verification\v09_dim.bas`, `v18_*`), and the design of `m2-arrays-and-types`
(D4 arrays, D5 places, D6 the store rule per place, D7 the emitter), which still holds.

Where the code stands (2026-10-10):

- **The parser reads every statement of this change**: `RedimStmt` (with `_PRESERVE`, `SHARED`, a member array as
  target), `CommonStmt` (with a block name), `EraseStmt`, `DefTypeStmt` (letter ranges, `_DEFINE … AS`),
  `OPTION BASE`, `DIM AS type` lists, array parameters, bounds on a `TYPE` member (`crates\syntax\src\parser\decl.rs`).
  No program the old compiler accepts has a parse gap.
- **`sema` marks them** "not supported yet" (`check\mod.rs` 515–524, `check\decl.rs` 516 and 553, `check\proc.rs`
  110, `check\places.rs` 105, 160, 208, 229, 298, 344–360, 479, 557).
- **An array is `Var { dims: Vec<(i64, i64)> }`**: constant bounds, main module only, keyed by name plus type in
  `scope().arrays`. `find_array` sees a main array from a procedure only through `DIM SHARED`.
- **The default type SINGLE is written out** in seven places (`check\decl.rs` 311, 407, 563; `check\places.rs`
  314; `check\proc.rs` 32, 75, 138).
- **Procedures are collected in a first pass** (their parameter and result types are known before any statement
  is checked), as are `TYPE` blocks.
- **The IR shares `sema`'s value tree** (`Expr`, `Place`); its own are the statement side, the variable list and
  the error rule's facts. The emitter builds one descriptor per array at program start (`codegen-cpp\src\decl.rs`)
  and nothing allocates or releases at run time except strings.
- A user type is members in order, each a numeric type, a fixed-length string or an earlier user type; a variable
  of it is one block of bytes, zeroed.

## Goals / Non-Goals

**Goals:**
- Each rule is measured with `qb64pe.exe` before it is designed in detail; nothing is read off the old compiler's
  source alone.
- Static against dynamic is decided in `sema`, from four facts: constant bounds, the statement, `$DYNAMIC` at that
  line, and the procedure (`study\10` §1.9). The IR says what an array is and what happens to it; the emitter owns
  descriptors, allocation and release.
- The default type goes through one function; no place assumes SINGLE.
- No program the old compiler rejects is accepted, and no name is silently turned into an array that the old
  compiler would treat as something else (D5).
- Every group leaves the tree green, with its own tests and lists, so the change can stop after any group.

**Non-Goals:**
- `CHAIN` and what `COMMON` passes through it; the `_STATIC`/`_DYNAMIC` member markers; `_ARRAYCOPY`; whole-array
  assignment; `REDIM _RETAIN`; `_MEM` over arrays; `VARPTR`/`VARSEG` of arrays and conventional memory.
- Matching the old compiler's C++ text beyond what libqb's ABI needs.
- `$CHECKING:OFF` (index checks stay on).

## Decisions

### D1. Measure first
`verification\v24_*`, run with `verification\run.sh`; findings in `study\00` §5; each program prints what it
measures, error cases trap with a handler that prints `ERR` and does `RESUME NEXT`, rejections are one `v24_x*`
program each. Where a finding disagrees with a decision below or with a spec delta, the decision and the delta
are corrected in the measuring task and the correction is listed there. Wrong code in the old compiler (a store
outside an array, freed memory read) is put to the user before it is designed. The questions:

- **Default types (`v24_a`)**: does a `DEFxxx` of the main module reach a procedure defined after it, before it,
  and a procedure's parameters and result; does one inside a procedure end with the procedure; a FUNCTION typed by
  default called with the matching and with another suffix; a variable met before and after the statement;
  `DEFINT a-z` then `x!`; the `_` entry (names starting with `_`); letters out of order (`DEFINT z-a`), a repeated
  letter, lower case; every type after `_DEFINE … AS`, and which the old compiler rejects (`STRING * n`, `_BIT`, a
  user type); `DEFSTR` and an array, a parameter, `FOR`, `CONST`; a `DEFxxx` after `DIM x` without a type.
- **Dynamic arrays (`v24_b`)**: `DIM` with a variable bound in the main module, twice (error 10), in a loop; bounds
  that give no element (error 5) and a very large request (257); an element used before allocation (`DIM a()`,
  after `ERASE`): which error; `'$DYNAMIC` then `DIM a(5)`, `'$STATIC` after it, from which line each applies (the
  comment rules are measured: `v16_m1_*`); a dynamic `DIM SHARED`; `LBOUND`/`UBOUND` of an array not allocated.
- **`REDIM` (`v24_c`)**: first `REDIM` as declaration; new bounds; the type kept without `AS`; a type changed
  (`REDIM x(5) AS LONG` then `AS INTEGER`); another number of dimensions; `REDIM SHARED`; `REDIM` of a static array
  (error 10, bounds kept); `_PRESERVE` growing, shrinking, of strings, of fixed-length strings, of a user type, of
  two dimensions, changing the lower bound, of an array not allocated; evaluation order when a bound raises.
- **`ERASE` (`v24_d`)**: static and dynamic, of each element kind, several names, a name that is no array, a
  scalar of the same name, `ERASE` then `DIM` (allowed again?) and then `REDIM`, in a procedure, of a parameter.
- **`OPTION BASE` (`v24_e`)**: 0 and 1, any other number, twice, after an array exists, inside a procedure, effect
  on implicit arrays, on `REDIM`, on member arrays, the literal-0 rule (`DIM a(0)`, `DIM a(0, 3)`, `DIM a(n)` with
  `n = 0`, a `CONST` that is 0).
- **Implicit arrays (`v24_f`)**: bounds per number of indexes; with a suffix and without; of a `DEFxxx` type; in a
  procedure (`study\00` §5 says every access raises 9 there: measured again and understood); after `LBOUND(x)`;
  `OPTION _EXPLICITARRAY`; a name that is a FUNCTION defined later in the file; a name that is a built-in of
  another suffix; `ERASE` and `REDIM` of one.
- **Arrays in procedures (`v24_g`)**: `DIM` per call (zeroed, released), twice in one call, constant against
  variable bounds; `STATIC a(n)`; `STATIC a()` then `DIM`/`REDIM`; `SUB … STATIC` with constant and variable
  bounds; `SHARED a()`, with and without `AS`, for a static and a dynamic main array, before the main `DIM` in the
  file; visibility of `DIM SHARED`, `REDIM SHARED` and `COMMON SHARED` arrays by file order; a recursive procedure
  with a local array; a local array live when an error leaves the procedure. **Array parameters**: static and
  dynamic arguments, element types with other signedness or length, `a()` passed on, `REDIM` and `ERASE` of a
  parameter, an element instead of an array, the number of dimensions (used with two indexes, passed one),
  `LBOUND`/`UBOUND`, a FUNCTION with an array parameter, an array of a user type. **`COMMON`**: scalars and arrays,
  with and without `SHARED`, a block name, `COMMON x` then `DIM x(3)`, twice, in a procedure, after executable
  statements.
- **`TYPE` (`v24_h`)**: member arrays: both declaration forms, bounds forms, `OPTION BASE`, of each member type, of
  a user type with a member array, `LEN` of such a type, initial values per variable and per array element, index
  errors and the store rule for `t.v(i)`, `a(i).v(j)` (which of value, outer index, inner index comes first; which
  is skipped: the D-004 question again), an element passed by reference, `LBOUND`/`UBOUND` with a dimension;
  **`REDIM`, `REDIM _PRESERVE` and `ERASE` of an unmarked member array: with the same bounds, with other bounds,
  and what `LEN`, a copy `q = p` and a second variable show afterwards** (D8 rests on this). `STRING` members:
  initial value, in locals, in arrays, in a nested type, `LEN` of the type, passed by reference; `q = p` with
  string members and member arrays, between elements, with itself; `SWAP` of two; a `TYPE` parameter with a
  variable, an element, a member, in parentheses, of another type.

### D2. Default types
A table of 27 types (A to Z and `_`), all SINGLE at the start, in `sema`'s checker. `DefTypeStmt` updates it when
the checker reaches the statement. Every one of the seven places that write SINGLE today asks one function, "the
default type of this name here".

The first pass needs the table at each procedure's header, and the second at each statement, so the table is
advanced **in file order in both passes**: the first pass walks the top-level statements in order, applies each
default-type statement, and types each procedure header with the table as it stands there (how a procedure before
or after the statement is typed is D1's question; the walk is the same either way, only where the table is read
changes). The second pass starts again from SINGLE.

*Alternative considered:* a table per source position computed up front. Rejected: two cursors over one walk is
less state, and the statements are met in order anyway.

A variable that exists keeps its type (it is found by name before the default is asked). Names starting with `_`
are reserved today; the `_` entry matters only once such names may be declared, and is measured so the table is
right when they are.

### D3. What an array is
`Var::dims` becomes a shape:

```text
shape = Scalar
      | Static  { bounds: Vec<(i64, i64)> }      exists for the whole program, bounds known
      | Dynamic { dims: Option<usize> }          allocated by a statement; None: not known yet (DIM a(), COMMON a(),
                                                 an array parameter never indexed)
```

`sema` decides (`study\10` §1.4, corrected by D1): static when every bound is a constant expression **and** the
statement is `DIM` or `COMMON` **and** `$DYNAMIC` is not in effect **and** it stands in the main module or in a
`STATIC` procedure; `STATIC a(n)` with constant bounds as measured; an implicit array is static. Everything else
is dynamic. The key of an array stays name plus type; `REDIM` without a type finds an array of that name declared
with `AS` first (measured, `v09` test 3).

`$DYNAMIC`/`$STATIC` become checker state, switched where the metacommand comment stands by the rule
`syntax::meta` already implements. `OPTION BASE` is checker state too and is **folded into the lower bound in
`sema`**: a dimension without `TO` gets a constant lower bound of 0 or 1, the literal `0` keeps 0. The IR never
sees `OPTION BASE`.

This task changes the type and every `match` on it with no change of behaviour (all snapshots equal), before any
new array kind comes in.

### D4. Array statements in the typed tree and the IR
```text
Allocate { array: ArrayRef, bounds: Vec<(Expr, Expr)>, mode: Dim | Redim | Preserve }
Clear    { array: ArrayRef }        ERASE of a static array
Release  { array: ArrayRef }        ERASE of a dynamic array
```

Bounds are converted to `_INTEGER64` (half to even) in `sema`. `Allocate` may raise (5, 10, 257); the order of
evaluating bounds and of the "already allocated" check is as measured. A static `DIM` stays no statement at all. A
whole-statement `REDIM a(3), b(4)` is one `Allocate` per item. `ir::validate` gains: `Allocate`/`Release` name a
dynamic array, `Clear` a static one, the number of bounds equals a known number of dimensions.

*Alternative considered:* lowering `REDIM _PRESERVE` to a loop of element copies in the IR. Rejected: the copy is
by flat position over raw storage (`study\02` §3.4), which is the emitter's knowledge, like index arithmetic was
in `m2-arrays-and-types` D10.

### D5. Implicit arrays, and what must not become one
`name(…)` that is no visible array, no FUNCTION and no built-in makes a static array `OPTION BASE` to 10 per
written index, in the scope it stands in. This removes the largest class of "not supported yet" messages, and it
is the one decision here that can turn a rejection into wrong code, so the rule is narrow: the name becomes an
array **only if** it is not reserved (no leading `_`), is not a keyword or a built-in the table knows under any
suffix or spelling (`CHR` beside `CHR$` as measured), and no FUNCTION of that name exists anywhere in the program
(procedures are collected first, so a FUNCTION defined later is known). A name the compiler knows but does not
compile yet (`_CONNECTIONADDRESS$(…)`, a built-in missing from the table) stays "not supported yet". After a
declaration that was marked, uses of its name stay follow-on errors as today (`compiler/pipeline`, "No follow-on
errors after an unsupported declaration"); they do not become implicit arrays.

`LBOUND(x)`/`UBOUND(x)` of a non-array makes the same array with one dimension.

### D6. Arrays in procedures, parameters, `COMMON`
- **Storage** as for scalars: `Local`, `Static`, `Param`. A local array is dynamic and is released when the call
  ends, in the emitter (the IR does not say when: spec delta `compiler/pipeline`); a `STATIC` one and every array
  of a `STATIC` procedure lives for the program.
- **Pending declarations**: `STATIC a()` and `COMMON a()` name an array without bounds; one list per scope, keyed
  by name, type and whether the type was written with `AS` (`study\10` §1.9), consulted by the next `DIM` or
  `REDIM` of that key, which then takes the listed storage (`STATIC`) or sharing (`COMMON SHARED`).
- **`SHARED a()`** binds the main array of that name and type into the procedure's scope, as `SHARED x` does for
  a scalar, creating it as measured when the main module has none.
- **Array parameters**: `Dynamic { dims: None }` with storage `Param`; the first use in the body with indexes fixes
  the number of dimensions (the old compiler restarts the compile for this; here the body is checked before the
  calls are judged). Calls are checked against it **after every body is checked**: arguments to array parameters
  are recorded and compared at the end, so a call above the procedure sees the same rule as one below. The
  argument is `Arg::Array(ArrayRef)`; the element type must be the parameter's exactly (measured: which
  signedness or length differences pass).
- **`COMMON`** is `DIM` with a flag: the same declaration path, placement errors as measured, `SHARED` after it as
  `DIM SHARED`. The block name is read and dropped. Nothing is written for `CHAIN`.

### D7. The emitter
- **Dynamic descriptor**: the same slots as a static one (`study\02` §3.4), made at program start (or per call)
  in the "not allocated" state libqb expects (lower bound 2147483647, count 0, the flag for "defined" clear), so
  every access fails `array_check` with error 9 until an `Allocate` runs.
- **`Allocate`**: the old compiler's sequence: error 10 for a static array or (mode `Dim`) an allocated one; count
  and overflow checks (5, 257); a zeroed block; strings made per element; the descriptor committed last. `Redim`
  releases the old block after the new one exists; `Preserve` copies the smaller number of elements by flat
  position (strings assigned, raw bytes copied) first. `_MEM` locks are not kept (non-goal); the descriptor's lock
  slot stays as today.
- **`Clear`/`Release`** as `study\10` §1.8.
- **Per-call release**: a procedure's local arrays are released on every way out of the procedure, strings first,
  where its local strings are released today (`codegen-cpp\src\procs.rs`).
- **Array arguments**: the descriptor pointer.
- Where the emitter writes a libqb call it did not write before, a `tests\callsite\` program covers it if the
  call-site check can compare it (`tools\callsite\README.md`); allocation code is inline C++ and is covered by the
  slice programs instead.
- **`_BIT` arrays** only if D1 shows them to be `getbits`/`setbits` over a packed block with no further rule;
  otherwise they stay marked and the upstream program that needs them stays out.

### D8. Member arrays (decision point after D1)
Places first, since they do not depend on the layout:

```text
ArrayRef = Var(VarId) | Member { base: Box<Place>, member: MemberId }
Place    = Var(VarId) | Element { array: ArrayRef, index: Vec<Expr> } | Member { base: Box<Place>, member: MemberId }
```

A member gains an optional shape (constant bounds, `OPTION BASE` folded in when the block is read). `x(0).n(1).q(2)`
resolves left to right as today, each `(…)` after a member that is an array making an `Element`. The store rule
for a chain with several indexes is measured (D1) and stated per place as D6 of `m2-arrays-and-types` does; the
D-004 decision (a member store with a bad index is skipped) is taken to extend to every index of the chain unless
the measurement shows a reason to ask again.

**The layout is not decided here.** Two outcomes of the `v24_h` measurement:

- *(a) An unmarked member array is inline*: its elements are part of the variable's bytes (`count × size`), `LEN`
  includes them, and `REDIM` with its declared bounds only clears (or is an error with other bounds). Then the
  emitter's layout adds a multiplied offset, variable-length strings in member arrays follow D9, and group 9 is
  built as written.
- *(b) An unmarked member array can change its bounds at run time*: each variable then holds a descriptor per
  member array (the old compiler's "descriptor-layout" flag, `study\02` §3.4), with set-up, release and deep copy
  per variable and per element of an array of the type. That is the layer `study\02` calls tangled.

In case (b), or if the measurement finds wrong code, **the findings and the choice go to the user before any
member-array code**: build it in this change, split it into a change of its own, or support only what (a) covers
and mark the rest. The measurement runs in group 1, so the decision does not wait for the other groups; the code
is the last group either way, and the groups before it do not depend on it.

### D9. `STRING` members, whole values, `TYPE` parameters
Added to step 10's list by Claude (see `proposal.md`); group 8, which no earlier group depends on.
- A `STRING` member is a string slot in the variable's block; a variable of such a type (and each element of an
  array of it) has its strings made when it is made and released when it goes, like local strings. `LEN` of the
  type and whatever else the old compiler refuses for such types are compile errors as measured.
- `q = p` is one `CopyWhole { from: Place, to: Place }` in the tree and the IR; the emitter copies the fixed bytes
  and assigns each string member (`study\02` §2.3, `copy_full_udt`). Its error rule (indexes on either side) is
  measured.
- A `TYPE` parameter takes `Arg::Ref(place)` of a user type: the address of the block.
- `ERASE`, `REDIM _PRESERVE` and `SWAP` over types with string members follow from the same per-variable set-up.

If group 9 comes out as case (b), per-variable set-up and release already exist from this group.

### D10. Tests, lists, order
- Per group: `tests\frontend\` (`typed` for what `sema` resolves, `check-fail` for each error and each form still
  marked, `ir` and `cpp` for each new statement and place), `tests\corpus\slice\` programs recorded with
  `qb64pe.exe` (one per requirement, covering its scenarios; names from `s50_` on, checked against the folder
  first), the shrink-only lists regenerated (`tests\upstream\README.md`), `slice.list`, `pass.list` and
  `known_clean_not_passing.list` updated for every program that becomes clean, `crates\README.md` updated.
- A spec scenario with a literal output is an expectation until its slice program is recorded; where the recording
  differs, the old compiler is right and the scenario is corrected in that task.
- Order: measurements (all, member arrays included); default types; the shape refactor with `OPTION BASE`,
  `$DYNAMIC` and `DIM AS` lists; dynamic arrays in the main module; implicit arrays; procedures; parameters and
  `COMMON`; the rest of `TYPE`; member arrays; integration. Groups 2 and 3 are independent of each other; 4 to 7
  build on 3; 8 on none; 9 on 3 and 8.
- No subagents (`CLAUDE.md` rule 1). Claude does not stage or commit (rule 1 of the human rules): each group ends
  by naming the files to stage and a commit message.

## Risks / Trade-offs

- **Member arrays are the old compiler's most tangled layer** → measured in group 1, built last, with a decision
  point (D8); every other group stands without them. If they leave the change, about 59 upstream programs wait.
- **Implicit arrays can hide a missing feature as wrong code** → the narrow rule of D5; a `check-fail` test per
  class of name that must stay marked (`_` names, built-ins of another suffix, names after a marked declaration);
  the shrink-only lists show any program the old compiler rejects that starts compiling.
- **Default types reach every implicit name** → one function instead of seven constants, done as its own group
  with no other change; the corpus and upstream programs with a `DEFxxx` run in tier 2 before the group closes.
- **A size the change has not had before** (ten groups) → groups are independent where D10 says so, each closes
  green; the change may be archived after any group with the rest moved to a follow-up.
- **Run-time allocation is new in the emitter** (release on every exit path, a pending error during `Allocate`, a
  recursive procedure) → slice programs for each, run with `QB64RUST_NO_FOLD=1` too; a leak is not observable in
  the tests, so release is checked by reading the emitted C++ in `cpp` snapshot tests.
- **The dimension count of an array parameter is known only after its body** → calls are judged at the end (D6);
  the risk is an order-dependent message, covered by `check-fail` tests with the call above and below.
- **Counts in the proposal are upper bounds** → each group's task states the programs it expected and the programs
  it got, as the earlier changes did.

## Open Questions

- Whether the three `TYPE` additions of D9 stay in this change. They are group 8; striking them changes no other
  group except that case (b) of D8 would then bring its own per-variable set-up.
- `_BIT` arrays (D7): decided by the measurement, one upstream program either way.
