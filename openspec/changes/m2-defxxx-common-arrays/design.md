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
advanced **in file order in both passes**. Measured (1.1, `study\00` §5): the table follows the file and nothing
else. A statement applies to every line below it, run or not; one inside a block or inside a SUB or FUNCTION goes
on applying after the block or procedure ends, to later procedures and later main-module lines; a procedure above
the statement keeps the earlier default for its parameters, result and variables. So the first pass applies
**every** default-type statement of the file in order, those nested in blocks and procedure bodies included, and
types each procedure header with the table as it stands at the header. The second pass starts again from SINGLE
and does not save or restore the table at a procedure's start or end.

*Alternative considered:* a table per source position computed up front. Rejected: two cursors over one walk is
less state, and the statements are met in order anyway. (Open again in 2.1 if the first pass has no cheap way to
reach the nested statements: a list of the statements by position gives the same answers.)

A name without a suffix is the name plus the default type at that line (measured): after `a = 1.5: DEFINT A-Z`,
`a` is `a%`, a new variable, and the same holds for a variable declared by `DIM c` without a type, for a
parameter and for an array. Only a variable declared with `AS` is found by its bare name whatever the default.
This is the lookup `sema` has today with SINGLE written out, so the function replaces the constant and no rule
is added. A FUNCTION is found by its bare name whatever the default at the call, and its bare name inside its
body stays the result. A `CONST` is not typed by the default. The `_` entry is real: `__a` (two underscores, a
valid name) is typed by it, and `_` may be named alone and in a range (`y-_`).

`_DEFINE` takes every type a variable may have by suffix, `STRING * n`, `_BIT` and `_BIT * n` included (the
proposal expected these rejected); a user type, `_MEM`, `_UNSIGNED` with `STRING` or a float, and `STRING * 0`
are compile errors.

### D3. What an array is
`Var::dims` becomes a shape:

```text
shape = Scalar
      | Static  { bounds: Vec<(i64, i64)> }      exists for the whole program, bounds known
      | Dynamic { dims: Option<usize> }          allocated by a statement; None: not known yet (DIM a(), COMMON a(),
                                                 an array parameter never indexed)
```

`sema` decides (`study\10` §1.4, corrected by 1.2, `study\00` §5): **what an array is, is settled by the first
declaration of it in the file.** Static when that declaration has only constant bounds **and** is `DIM`, `STATIC
a(n)` or `COMMON` **and** `$DYNAMIC` is not in effect at that line **and** it stands in the main module or in a
`STATIC` procedure (or is `STATIC a(n)`); an array used without `DIM` is static unless `$DYNAMIC` is in effect
where it is first named. Everything else is dynamic.

**A constant bound is a spelling, not a value** (measured in 3.2, `verification\v24_b_constant_bounds*`): one
number (any literal form, a minus before it included) or one `CONST` name, in any number of parentheses. A bound
with an operator (`2 + 3`, `k + 1`, `-k`, `10 \ 3`) makes the array dynamic in the old compiler although `sema`
can fold it. `sema` therefore asks the syntax of each bound for static against dynamic, and the folded value only
for the bounds of a static array. `m2-arrays-and-types` took every foldable bound for constant, so today `DIM
f(-(2 ^ 2) TO 10 \ 3)` is compiled as a static array; the programs that could tell (the `DIM` skipped or run
twice, `REDIM` or `ERASE` of it) are few and none is in the test sets. **4.1 moves these to the dynamic side**, in
the same task that makes dynamic arrays work, so no program that passes today stops compiling in between; until
then it is listed under "Known bugs" in `STATUS.md`. The key of an array stays name plus type; `REDIM` without a
type finds an array of that name declared with `AS` first (measured, `v09` test 3, `v24_x48`).

A declaration after the first is only a statement that runs:

| First declaration | Later statement | Result |
|---|---|---|
| static | `DIM` with constant bounds | compile error "Cannot redefine a static array!" (as today) |
| static | `DIM` with a variable bound, `REDIM`, `REDIM _PRESERVE` | compiles; raises 10 when it runs, bounds not evaluated |
| dynamic | `DIM` (constant or variable bounds) | `Allocate` in mode `Dim`: raises 10 if allocated |
| dynamic | `REDIM`, `REDIM _PRESERVE` | `Allocate` in mode `Redim` or `Preserve` |

`DIM a()` and `REDIM a()` (nothing between the parentheses), and `REDIM x AS LONG`, declare the **scalar** `a`
and no array (measured; the proposal took `DIM a()` for an array without bounds). `Dynamic { dims: None }` is
left for `COMMON a()`, `STATIC a()` and array parameters (1.3).

An array used without `DIM` below `$DYNAMIC` is dynamic and is allocated (0 or 1 to 10 per index) by the first
statement in the file that names it: `sema` puts an `Allocate` in mode `Dim` before that statement, which is what
the old compiler writes (run twice it raises 10; skipped, every use raises 9).

`$DYNAMIC`/`$STATIC` become checker state, switched where the metacommand comment stands by the rule
`syntax::meta` already implements. `OPTION BASE` is checker state too. Both follow the file like the default
types (D2): a statement or comment inside a procedure goes on applying below it, and none is undone at `END SUB`.
`OPTION BASE` is **folded into the lower bound in `sema`**: a dimension without `TO` gets a constant lower bound
of 0 or 1; a bound written as the number 0 in plain digits (`0`, `00`) keeps 0 to 0, any other constant that is 0
does not (`0&`, `0.0`, `(0)`, `0 + 0`, a `CONST`: the old compiler compares the text). The IR never sees `OPTION
BASE`. Only the word after `BASE` is read by the old compiler (`OPTION BASE 0 + 1` is base 0); 3.2 checks what
the parser does with such a line and matches it.

A static array may now have a dimension with no element: under `OPTION BASE 1`, `DIM a(zero)` with a `CONST`
that is 0 (or `0&`, `(0)`, `0.0`) is 1 to 0, which the old compiler accepts and answers with error 5 when the
program starts. `sema` keeps such an array (bounds 1 to 0) and the emitter raises 5 where it sets the array up
at program start, which the set-up code did already (`if (count<=0) error(5)`). `DIM a(0 + 0)` is not this case:
it is a dynamic array (a bound with an operator) and raises 5 at the `DIM`.

This task changes the type and every `match` on it with no change of behaviour (all snapshots equal), before any
new array kind comes in.

### D4. Array statements in the typed tree and the IR
```text
Allocate   { array: ArrayRef, bounds: Vec<(Expr, Expr)>, mode: Dim | Redim | Preserve }
Clear      { array: ArrayRef }      ERASE of a static array
Release    { array: ArrayRef }      ERASE of a dynamic array
EraseParam { array: ArrayRef }      ERASE of an array parameter: one or the other, known when it runs (D6)
```

Bounds are converted to `_INTEGER64` (half to even) in `sema`. `Allocate` may raise 5 and 10, and may end the
program (257 is critical in libqb: no handler runs). Measured order (1.2): the static check, then (mode `Dim`)
the allocated check, then the bounds; **an error does not stop the statement**: a bound that raises, and a
dimension with no element (error 5), still allocate with the bounds computed. So `Allocate` is not skipped by a
pending error the way a store is, and its bounds are not guarded. A static `DIM` stays no statement at all. A
whole-statement `REDIM a(3), b(4)` is one `Allocate` per item. `Allocate` may name a static array (`REDIM` of
one, or `DIM a(n)` after `DIM a(5)`): it raises 10 and its bounds are not evaluated; the emitter writes the test
of the descriptor's flag in every case, since an array parameter may be either kind (1.3). `ir::validate` gains:
`Release` names a dynamic array, `Clear` a static one, the number of bounds equals a known number of dimensions.

*Alternative considered:* lowering `REDIM _PRESERVE` to a loop of element copies in the IR. Rejected: the copy is
by flat position over raw storage (`study\02` §3.4), which is the emitter's knowledge, like index arithmetic was
in `m2-arrays-and-types` D10.

### D5. Implicit arrays, and what must not become one
`name(…)` that is no visible array, no FUNCTION and no built-in makes an array `OPTION BASE` to 10 per written
index, in the scope it stands in. This removes the largest class of "not supported yet" messages, and it is the
one decision here that can turn a rejection into wrong code, so the rule stays narrow. Measured (1.3): the old
compiler lets a name be an array exactly where it lets it be a variable: `chr(1) = 5`, `left(3)` and `abs$(1)`
are arrays (a built-in that needs a suffix leaves the other spellings free), `__foo` is, `_foo` is not, a
keyword, a built-in statement and the name of a SUB are "Name already in use", and a spelling that calls a
built-in function or a FUNCTION defined anywhere is that call. So the name becomes an array **only if** the
name with its suffix is one `sema` accepts for a scalar variable today (the reserved-name rule it has), no SUB
or FUNCTION of that name exists anywhere in the program (procedures are collected first, so one defined later is
known), and the built-in table has no function this spelling would call, compiled or not: a built-in the
compiler knows but does not compile yet stays "not supported yet". After a declaration that was marked, uses of
its name stay follow-on errors as today (`compiler/pipeline`, "No follow-on errors after an unsupported
declaration"); they do not become implicit arrays.

What the array is depends on where it stands (measured, `study\00` §5):

| Where | The array |
|---|---|
| main module | static, exists from the start |
| main module below `$DYNAMIC` | dynamic, allocated by the first statement in the file that names it (D3) |
| `SUB … STATIC` (not below `$DYNAMIC`) | static, lives for the program |
| any other procedure, and every procedure below `$DYNAMIC` | **a local dynamic array that nothing allocates: every use raises 9**, until a `DIM` or `REDIM` of it in the procedure runs |

The last row is what the old compiler does (its C++ holds the descriptor and no allocation); QuickBASIC 4.5
allocates such an array. **Decided (user, 2026-10-11): do as QB64pe**; the difference from QuickBASIC 4.5 is
`DIVERGENCES-QB45.md` Q-009 and the behaviour is on the review list of `SOMEDAY.md`.

`LBOUND(x)`/`UBOUND(x)` of a non-array makes the same array with one dimension. The number of indexes of the
first use is the number of dimensions; another number later is a compile error, a later `DIM` of a static one
too.

### D6. Arrays in procedures, parameters, `COMMON`
- **Storage** as for scalars: `Local`, `Static`, `Param`. A local array is dynamic and is released when the call
  ends, in the emitter (the IR does not say when: spec delta `compiler/pipeline`); a `STATIC` one and every array
  of a `STATIC` procedure lives for the program.
- **No pending list.** Measured (1.3): `STATIC a()` and `COMMON a()` are themselves the first declaration of a
  **dynamic array that is not allocated**, `Dynamic { dims: None }`, with storage `Static` (in a procedure) or as
  a main-module array, shared after `COMMON SHARED`. A `DIM` of that name and type below it is then the ordinary
  `Allocate` in mode `Dim` of D3's table (error 10 when it runs a second time), `REDIM` the ordinary `Redim`, and
  the first statement that gives bounds or indexes fixes the number of dimensions. This replaces the list keyed
  by name, type and `AS` that the old compiler keeps (`study\10` §1.5, §1.6): the key of an array in `sema` is
  name plus type already, so `STATIC a() AS DOUBLE` then `DIM a(3)` is another array without a rule for it.
  What stands between the parentheses of `COMMON a(3)` is ignored, as measured.
- **`SHARED a()`** binds the main array of that name and type into the procedure's scope, as `SHARED x` does for
  a scalar. The main array must be declared above the procedure in the file; none is made ("Array 'a' not
  defined").
- **Visibility follows the file**: a shared main array is seen by a procedure only if its declaration stands
  above the procedure. `find_array` therefore asks the position of the declaration; in a procedure above it the
  name is an array used without `DIM` (D5, last row).
- **Array parameters**: `Dynamic { dims: None }` with storage `Param`. The number of dimensions belongs to the
  parameter and is one unknown per parameter, settled by everything that names it: an index count or a `REDIM`
  in the body, each argument passed to it (an argument that is itself a parameter makes the two unknowns equal).
  The constraints are collected while the bodies are checked and **solved after every body is checked**, so a
  call above the procedure sees the same rule as one below and a chain of arrays passed on needs no second pass
  (the old compiler recompiles for this). Two constraints that disagree are the compile error, reported at the
  later one in the file. A parameter nothing constrains keeps `None`. The argument is `Arg::Array(ArrayRef)`; its
  element type must have the parameter's kind and width, signedness and `_OFFSET` not compared (measured). An
  array parameter may be static or dynamic at run time, so `Allocate` and `ERASE` of one test the descriptor's
  flag when they run: `ERASE` of a parameter is a third form, `EraseParam`, beside `Clear` and `Release` (D4).
- **`COMMON`** is `DIM` with a flag for scalars: the same declaration path, placement errors as measured,
  `SHARED` after it as `DIM SHARED`. **A block name is a compile error** (measured: "Invalid variable name"; the
  proposal expected it accepted). Nothing is written for `CHAIN`.
- **Accepted by the old compiler but failing its C++ build**, so compile errors here by the rule of
  `DECISIONS.md` 2026-10-08: `STATIC v(n)` with a bound that is not constant, and an array argument in
  parentheses (`fill (x())`).

### D7. The emitter
- **Dynamic descriptor**: the same slots as a static one (`study\02` §3.4), made at program start (or per call)
  in the "not allocated" state libqb expects (lower bound 2147483647, count 0, the flag for "defined" clear), so
  every access fails `array_check` with error 9 until an `Allocate` runs.
- **`Allocate`**: the old compiler's sequence, read from its C++ (1.2): error 10 for a static array (nothing
  else); mode `Dim` of an allocated array error 10 unless an error is pending; per dimension the lower bound and
  the count, `error(5)` when the count is 0 or less **without leaving**; then `error(257)` (critical, the program
  ends) for a negative count, a bound or count that does not fit a pointer, a product or a size in bytes that
  overflows, and a failed allocation; a zeroed block; strings made per element; the descriptor committed last.
  `Redim` releases the old block after the new one exists; `Preserve` copies the smaller number of elements by
  flat position (strings assigned, raw bytes copied) into the new block, then releases the old one. `_MEM` locks
  are not kept (non-goal); the descriptor's lock slot stays as today.
- **A static array with no element** (D3) raises 5 in its set-up at program start, as the old compiler's does.
- **`Clear`/`Release`** as `study\10` §1.8.
- **Per-call release**: a procedure's local arrays are released on every way out of the procedure, strings first,
  where its local strings are released today (`codegen-cpp\src\procs.rs`).
- **Array arguments**: the descriptor pointer.
- Where the emitter writes a libqb call it did not write before, a `tests\callsite\` program covers it if the
  call-site check can compare it (`tools\callsite\README.md`); allocation code is inline C++ and is covered by the
  slice programs instead.
- **`_BIT` arrays: decided in (1.2).** They are `getbits`/`getubits`/`setbits` over a packed block of `count *
  bits / 8 + 1` bytes at the checked flat position, static and dynamic alike, with no further rule; `Preserve`
  copies bytes, which keeps flat positions because the width does not change. `SWAP` of two elements and `LEN` of
  one are compile errors, an element is passed by value. They come in with 4.2.

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

**The layout: case (a), measured in 1.4** (`study\00` §5, `verification\v24_h_member_*`, `v24_x182`–`x226`). The
two outcomes the measurement could have had:

- *(a) An unmarked member array is inline*: its elements are part of the variable's bytes (`count × size`), `LEN`
  includes them, and `REDIM` with its declared bounds only clears (or is an error with other bounds). Then the
  emitter's layout adds a multiplied offset, variable-length strings in member arrays follow D9, and group 9 is
  built as written.
- *(b) An unmarked member array can change its bounds at run time*: each variable then holds a descriptor per
  member array (the old compiler's "descriptor-layout" flag, `study\02` §3.4), with set-up, release and deep copy
  per variable and per element of an array of the type. That is the layer `study\02` calls tangled.

**It is (a).** The evidence: `LEN` of `id AS LONG, v(0 TO 3) AS LONG, tail AS INTEGER` is 22; `REDIM t.v(0 TO 3)`
clears and `REDIM t.v(0 TO 9)`, `REDIM t.v(1 TO 4)` and `REDIM t.v(0 TO n)` are compile errors; `REDIM
_PRESERVE` with the declared bounds does nothing; `ERASE t.v` clears and the bounds stay; no statement changes
the bounds of an unmarked member array, so no variable needs a descriptor. Group 9 is built as written, with
these additions from the measurement:

- **Bounds of a member** are constants read where the `TYPE` block stands: `OPTION BASE` as it is at the block
  (types are collected before the statements, so the first pass must know `OPTION BASE` at each block: it is
  advanced in file order beside the default types, D2); the literal-0 rule of D3 does **not** apply (`z(0)`
  under base 1 is "Invalid array bounds").
- **`REDIM` of a member array** is accepted only as a clear: constant bounds equal to the declared ones, written
  `lower TO upper` or with `AS` and the member's type. It lowers to `Clear` of `ArrayRef::Member`; `REDIM
  _PRESERVE` of one lowers to nothing; `ERASE` to `Clear`. No `Allocate` ever names a member array
  (`ir::validate`).
- **A whole member array as an array argument** (`fill t.v()`) is accepted by the old compiler. The emitter
  would hand the procedure a descriptor made for the call over the variable's bytes (static, the member's
  bounds). It is in 9.2 if that descriptor proves to be all it takes; otherwise it stays marked and is named in
  `SOMEDAY.md`. `LBOUND(t.v())` with the empty parentheses is accepted for a member array.
- **The store rule** is the one of D-004 at every index of the chain, as decided above: the value first, then
  the indexes left to right, the first error wins, and with an error pending nothing is stored (the old compiler
  writes element 0 of the dimension whose index was outside: `t.v(9) = 5` sets `t.v(0)`). This is D-004's rule
  applied to more places, not a new decision, and `DIVERGENCES.md` D-004 gains the member-array forms in 9.1.

**Answered by the user (task 1.4, 2026-10-11, `DECISIONS.md`):** a whole-value store with an index outside stays
as QB64pe does it (D9); an array used without `DIM` in a procedure stays not-allocated as in QB64pe, noted as
`DIVERGENCES-QB45.md` Q-009 (D5); the additions of D9 stay in the change.

### D9. `STRING` members, whole values, `TYPE` parameters
Added to step 10's list by Claude (see `proposal.md`); group 8, which no earlier group depends on.
- A `STRING` member is a string slot in the variable's block; a variable of such a type (and each element of an
  array of it) has its strings made when it is made and released when it goes, like local strings. `LEN` of the
  type and whatever else the old compiler refuses for such types are compile errors as measured.
- `q = p` is one `CopyWhole { from: Place, to: Place }` in the tree and the IR; the emitter copies the fixed bytes
  and assigns each string member (`study\02` §2.3, `copy_full_udt`). Measured (1.4): the old compiler does not
  guard the copy: `a(9) = p` raises 9 and overwrites `a(0)`, `q = a(9)` raises 9 and copies `a(0)` into `q`.
  **Decided (user, 2026-10-11, `DECISIONS.md`): as QB64pe for now.** The copy is not guarded: with an index
  outside, error 9 is raised and element 0 stands in for the element named, on either side. Claude had proposed
  storing nothing, as D-004 does for a member; that is on the review list of `SOMEDAY.md`. The emitter gets this
  for free: `array_check` gives 0 after raising, as for a read.
- A `TYPE` parameter takes `Arg::Ref(place)` of a user type: the address of the block. The argument is a
  variable, an element or a member of exactly that type; an element with an index outside raises 9 and the
  procedure is not entered (the entry check that elements have today). A variable in parentheses (`move (q)`)
  is an internal error of the old compiler and a compile error here; a FUNCTION cannot have a user type as its
  result.
- `ERASE`, `REDIM _PRESERVE` and `SWAP` over types with string members follow from the same per-variable set-up.

Group 9 came out as case (a), so nothing but `STRING` members needs per-variable set-up and release; a member
array of `STRING` (`s(2) AS STRING`, measured: accepted, each element its own string) is that many string slots
in the block.

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

None. Closed 2026-10-11: the three `TYPE` additions of D9 stay in this change (user, `DECISIONS.md`); `_BIT`
arrays are in (D7, by the measurement).
