# Design

## Context

For the motivation and the nine programs, see `proposal.md`; for the behaviour, the delta specs. How the parts
stand in this compiler (2026-10-24, on `m4-sub-gl`):

- **`STATIC v(bounds)`** (`crates\sema\src\check\places.rs`, the declaration of an array, from "`STATIC
  a(bounds)` is no static array"): a bound whose value is known when compiling gives `Shape::Dynamic { at_start:
  Some(bounds) }` (`crates\sema\src\lib.rs`), numbers only. A bound that names a variable of the procedure is an
  error; any other bound is "not supported yet" (`v55_x74`, `x75`). `codegen-cpp` writes the allocation into
  `maindata.txt` (`lib.rs`, after `per_program_var`; `arrays.rs` `allocate_at_start`, mode `Dim`, the bounds as
  integer constants). `maindata.txt` stands at the top of `QBMAIN`, which libqb enters again for each error
  handler and event (the old compiler's `qbx.cpp`, line 1588, and ours the same), so the allocation runs again
  there and, with an error pending, allocates only an array that was released.
- **`REDIM` with a bound that first names an array used without `DIM`** (`places.rs`, the `redim` arm of the
  bounds, `NoJoin::RedimBound`): in a procedure, when the array being given bounds is the shared main-module one
  and the bound made a new array, the statement is "not supported yet" (`v24_x268` case 5, `v55_x84`). The
  recording shows a local `sd` given its bounds and the shared `sd` kept; the old compiler's reason was not found
  (`SOMEDAY.md`, row "In a procedure, `REDIM sd(lc(1) + 4)`").
- **A `TYPE` that holds a `STRING`, in conventional memory** (`places.rs` `var_address`, after "a variable of a
  SUB or FUNCTION"): `holds_strings`, `owns_desc` or `_MEM` in the type is "not supported yet". The emitter's
  `alloc_dblock` (`codegen-cpp\src\decl.rs`) has the old compiler's text for a `TYPE` value: `cmem_sp-=size; …
  memset(V,0,size);`, no strings made. In the old compiler that is all (`qb64pe.bas` 17790–17852:
  `initialise_udt_varstrings` stands only in the other branch), and its program crashes when a member string is
  first used (`v56_a5`, exit 139).
- **`FOR` over an element of a member array** (`crates\sema\src\check\blocks.rs` `for_var`): marked. The loop
  (`crates\ir\src\lower.rs` `for_loop`) names its place three times: `v = t` at the entry, `t = st + v` and `v =
  t` at each `NEXT`; the test reads the temporary `t`. For a place whose index is evaluated where it is named, that
  is 1 + 2 × 3 = 7 evaluations for four entries, the count `v56_f3` records.

## Goals / Non-Goals

**Goals:**

- The demand report over `tests\verification\known.list` names no program; the eight of the nine that the old
  compiler runs to the end pass, and `v56_a5` fails only where D-037 says.
- Each form is measured before it is built where `v55` and `v56` left a question (`v60`, D5).

**Non-Goals:**

- Graphics `GET` and `PUT` of a `TYPE` that holds a `STRING` (the old compiler writes over the string pointers),
  `_MEMGET` and the rest of such a value, `VARPTR` of a `TYPE` that owns a dynamic member array or a `_MEM`, and
  `VARPTR` of a procedure's variables: their marks and rows stay.
- A `STATIC` bound that names a variable of the procedure stays the error it is. One that names an array of the
  procedure (a parameter array, a local or `STATIC` one) is "not supported yet": not measured (review of this
  change).

## Decisions

### D1. A `STATIC` bound read by the start code

`at_start` holds the bounds as expressions: `Shape::Dynamic { at_start: Option<StartBounds> }`, where `StartBounds`
is `Values(Vec<(i64, i64)>)` (as now) or `Read(Vec<(Expr, Expr)>)`, a bound that names a shared main-module
variable, a main-module `CONST`, or calls a FUNCTION. `places.rs` takes `Read` where it gives "not supported yet"
now; the check for a variable of the procedure stays in front of it. The expressions are checked as in the main
module, since that is where the old compiler writes them: a name is looked up among the shared variables
visible to the procedure, and a name that is not one is a variable of the procedure (the existing error).
`allocate_at_start` writes `Read` bounds with the emitter's `value()` outside any procedure, so a FUNCTION call is
the C++ call the main module would make; temporaries go in `maindata.txt`'s `temps(None)`, written after the
allocations as now. `Values` keeps its integer constants, so no program that compiles now changes (the pass cache
shows it).

What `v55` shows: a shared `n` is 0 at the start (`x74`: `v(0 TO 0)`, then error 9 at `v(1)`), and what a FUNCTION
prints there is lost (`x75`). What `v60_a` measured (task 1.1) and the old compiler's C++ of `v60_a2` and `a5`
shows (task 1.2): the bounds are evaluated inside the allocation's branch for an array that is not allocated
(`if (v[2]&1){ if (!error_occurred) error(10); } else { alloc_new_desc[5]=(qbr(FUNC_F()))…`), so a FUNCTION in a
bound runs when the program starts (also for a SUB that is never called, `a4`) and again only when a handler is
entered while the array is released (`a2`: the count stays 1 over two handled errors with the array allocated,
and goes to 2 after `ERASE`; what it prints then is seen, `[f]`); a shared variable or element is read then with
its value of that moment (`a1`, `a3`, `a6`: `v(7)` after `ERASE` and `n = 7`); several bounds run in the order of
the arrays in the file (`a5`: `312`). `allocate_with` already writes its bounds inside that branch, so the
emitter follows with no guard of its own. A FUNCTION's argument passed by reference gets the old compiler's
`passN` temporary (`FUNC_F(&(pass2= 3 ))`), declared before the allocation that uses it: `maindata.txt` writes
the allocations with read bounds after the temporaries and `pass` declarations of the main module's code, and the
temporaries they need are declared there with them. Those allocations so come after every variable's
allocation, where the old compiler writes each in its place among them; this differs only for a FUNCTION that
reads a variable allocated after the array, which the old program reads before it exists (not measured).

Each of these is a QB64pe behaviour kept: a row of `SOMEDAY.md`'s "QB64pe behaviours to review" (the row of
`v55_x74`, `x75` moves there from "Left marked by …").

*Alternative*: evaluate the bound when the procedure is first called. Rejected: it is not what the old compiler's
program does, and `x74` would print other bounds.

### D2. `REDIM` in a procedure of a shared array, with a new array in its bound

`v60_b` (D5) recorded which `sd` each statement of the procedure names (task 1.1). The rule they fit, with
`v24_x268` case 5 and `v55_x84`: **in such a procedure, `sd` (that name and type) is a dynamic array of the
procedure in the whole procedure, above the `REDIM` too, made on each call and released at its end; the shared
array is untouched.**

- Above the `REDIM` the name is the procedure's array, not yet allocated: `UBOUND(sd)`, `sd(1)` and `sd(1) = 33`
  raise error 9 (`b1`), also below a `SHARED sd()` statement (`b8`), whose rule is that it changes nothing below
  an array of the name the procedure has already.
- It is new on each call: the second call sees it unallocated above the `REDIM` (`b1`), and `REDIM _PRESERVE`
  keeps nothing from the call before (`b3`), also in a `SUB … STATIC` (`b5`: `p: 3 0` on both calls). So its
  storage is the call's even where the procedure's other variables live for the program.
- Another procedure, and one it calls, names the shared array (`b2`: `r: 1 11`); the main module keeps its
  bounds and elements (every `b` program).
- `AS LONG`, a suffix, a FUNCTION, the new array in a lower bound or another dimension take the same rule (`b4`,
  `b6`, `b9`); in a list, only the item whose bound names the new array (`b10`).
- When the array used without `DIM` was named above the `REDIM` in the procedure, the `REDIM` gives the shared
  array its bounds (`b7`: `main: 4 0`): the rule needs the bound to be the first place that names it.

The old compiler compiles each program twice and keeps the arrays its first pass made, which explains a local
array seen above the statement that made it. The checker here is one pass, so it does the same: where the
condition holds (the `redim` arm of `places.rs` that marks it now), it notes the procedure, the name, the type,
its number of dimensions and whether the plain name names it, and returns its error as now; `check_with` then
checks the program again with those notes, and on entering such a procedure (`proc_def`) makes the procedure's
array (storage `Local`, `Shape::Dynamic`) before its first statement. In that run the `REDIM` finds the
procedure's array, so it allocates it and nothing is marked. A check stops making arrays used without `DIM` after
its first marked declaration (design D5 of `m2-defxxx-common-arrays`), so it can miss a second such `REDIM`;
`check_with` checks again until a check finds none it did not have. Only a program with the form is checked more
than once.

In a `SUB … STATIC` the array used without `DIM` that the bound first names was marked ("not measured"); `b5`
measures it: error 9 at each call, a dynamic array that nothing allocates, as `$DYNAMIC` makes it there. It is
built so (storage `Static`, as below `$DYNAMIC`).

A row of "QB64pe behaviours to review" replaces the row "In a procedure, `REDIM sd(lc(1) + 4)`" of "Left marked
by …".

### D3. A `TYPE` that holds a `STRING`, in conventional memory: the strings made (D-037)

`var_address` stops marking `holds_strings` (`owns_desc` and `_MEM` keep their mark). The variable becomes
conventional as any other. Its layout is the one the program has everywhere (8 bytes for each `STRING` member,
design D10 of `m4-memory-records`), so the offsets follow D1 of that change with no new rule: `v56_x08`, `x26`,
`x27` and `v60_d1`–`d4` check them (`d1`: a nested member's string at `65500`, the variable at `65498`). `VARPTR`
and `VARSEG` of a `STRING` member of a variable are taken (`x08`: the offset of its pointer). Two forms of a
`STRING` member fail the old compiler's C++ build (`v60_d7`: `VARPTR` of the member of an array element;
`d8`: `VARPTR$` of the member of a variable), so they become compile errors, as `v24_x121` did; they were "not
supported yet".

`alloc_dblock` writes, after the `memset`, the strings of a new value as `alloc` writes them in ordinary memory
(`decl.rs` `new_strings`, our counterpart of the old compiler's `initialise_udt_varstrings`); an array
of such a `TYPE` in conventional memory makes its elements' strings as the ordinary array does (`arrays.rs`,
`holds_strings`). The strings are ordinary `qbs` values: their descriptors are not in `dblock`, so no offset of
another variable moves. The old compiler leaves the strings out and its program crashes when one is first used
(`v56_a5`: exit 139, after its first two lines). A crash is no behaviour to build, so this is **D-037** in
`DIVERGENCES.md`, a fix in `UPSTREAM.md` (`qb64pe.bas` 17790–17852: the same call in the conventional branch),
and `v56_a5` stays on `known.list` with `D-037` as its reason. The new compiler's run of it is held in tier 1: a
CLI test that builds it checks the first two lines equal the record and that `abc  3` follows.

*Alternative*: build the old compiler's allocation, so that `v56_a5` crashes as recorded. Rejected: the crash
reads a null string pointer, which is undefined, not a value.

### D4. `FOR` over an element of a member array

`for_var` returns the place (`Place::Member` with the member array's indexes, through members of members too) in
place of the mark, when the element is numeric. `for_loop` already names the place at each use and keeps no
address, so the loop finds the element again at each use, with the evaluations the old compiler makes (Context).
The lowering is checked by an `ir` front-end test of the loop over `r.m(idx(3))`, which shows three uses of the
place. An element of a member array of an array element (`ar(i).m(j)`) is a member of an array element, the
compile error it is now (`v56_f2`).

The old compiler's C++ of `v56_f3` (task 1.2) names the place twice per pass and once at the start, as `for_loop`
does: `fornext_value7=fornext_step7+(<the element>)` at `NEXT`, then at the entry label `<the element>=
fornext_value7`, each with `FUNC_IDX(&(pass6= 3 ))` in its index. `v60_c` records the rest: a step, a negative
step, `EXIT FOR` after the index changed (`c1`), a nested member (`c2`, `idx` 7 times for four entries with step
2), an index out of range at a pass under a handler that resumes (`c3`), every numeric member type at the end of
its range (`c4`, as for a plain variable), `_OFFSET` (`c5`) and two dimensions (`c6`).

`c3` shows what `for_loop` did not have: an error raised by the place itself. The old compiler's `NEXT` has no
event check between the step, the store and the test, which takes its value; the pending error is serviced at
`fornext_error2:;`, the event check with the `FOR` line before the body, so `RESUME NEXT` goes on into the body
(its C++ of `v60_c3`). For a place with an index, `for_loop` so writes the step, the store and the test as one
statement with the `FOR` line and `OnError::UseValue`, followed by a jump to the body as a statement of its own,
and the entry from the header as a store and test of its own followed by the same jump. A plain variable and a
member without an index keep their lowering, so no other program's C++ changes. `c3` itself stays on `known.list`
as D-004: its store out of range is skipped here and writes element 0 in the old program, so the passes count on
differently (the program bounds its loops so that it ends under both).

### D5. The measurement family `v60`

Recorded with `qb64pe.exe` before the code (task 1.1), the catalogue line in `verification\README.md`:

- `v60_a*`: `STATIC v(n)` with a shared `n` and with a FUNCTION that prints and counts its calls, under `ON ERROR`
  handlers entered several times, with `ERASE v` before an error and `n` changed between; a bound that names a
  shared array's element; `STATIC v(n)` in a SUB never called;
- `v60_b*`: the forms of D2;
- `v60_c*`: `FOR` over `r.m(i)` with a step, `EXIT FOR`, a nested member (`ro.n.m(i)`), an index out of range at
  a pass under a handler, every numeric member type;
- `v60_d*`: a `TYPE` that holds a `STRING` in conventional memory with its offsets: a nested `TYPE`, two string
  members, an array of one, `VARPTR$` of a member, `VARSEG` (the strings are not used, so the old compiler's
  program does not crash); `d5` the string of an array element used (no crash: an array is not in `dblock`),
  `d6` a nested member's string used (exit 139, D-037); `d7`, `d8` the two forms whose C++ build fails (D3).

Each that the new compiler passes goes to `pass.list`; one that crashes in the old compiler is recorded as it is
and listed with D-037.

## Risks / Trade-offs

- **A FUNCTION in `maindata.txt`** runs before libqb's console or screen exists; what it does then is what the
  old compiler's program does only if our start code is in the same order. `x75` and `v60_a` show it; a
  difference is found at the first run.
- **D2 rests on recordings, with a likely reason** (the old compiler's two passes) that was not read in its
  source. The rule is checked against every `v60_b` program, and a form outside them keeps no guarantee; the
  `SOMEDAY.md` row says so. The second check of such a program costs its check time once more.
- **D-037 changes a program that the old compiler crashes on** and nothing else: a `TYPE` with a `STRING` in
  conventional memory was a mark until now, so no passing program is touched.
