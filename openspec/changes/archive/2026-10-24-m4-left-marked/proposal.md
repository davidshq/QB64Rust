# Proposal

## Why

Nine programs of `tests\verification\known.list` still carry "not supported yet": the forms that steps 2 and 3
of M4's plan (`DECISIONS.md` 2026-10-22, `m4-names-implicit-arrays` and `m4-memory-records`) measured and left
marked (`DECISIONS.md` 2026-10-23, item 4a: "The 9 programs that steps 2 and 3 left marked stay marked"). After
step 5 (`m4-sub-gl`) they are the last marks the demand report finds in the verification families, and M4 ends
with step 6, the coverage review, after them (`STATUS.md`, "Next" item 2).

## What Changes

Four forms, each built as the old compiler's program behaves, measured first in a new family `verification\v60`
where `v55` and `v56` left a question:

- **`STATIC v(n)` with a bound read when the start code runs** (`v55_x74`, `x75`): a bound that names a shared
  main-module variable or calls a FUNCTION is written into the main module's start code (`maindata.txt`) as the
  old compiler writes it, and is evaluated each time that code runs: when the program starts (a shared `n` is 0
  then, so `v(0 TO 0)`) and again whenever an error handler enters it. What a FUNCTION prints there is lost, as
  in the old compiler. A bound that names a variable of the procedure stays the compile error it is.
- **`REDIM` in a procedure of a shared array whose bound first names an array used without `DIM`**
  (`v55_x84`, `v24_x268` case 5): the old compiler gives the procedure an array of its own of that name, which
  that `REDIM` allocates, and leaves the shared array as it was. Measured first (`v60_b*`): the procedure's array
  is the name's in the whole procedure, above the `REDIM` too, and new on every call.
- **A `TYPE` that holds a `STRING`, in conventional memory** (`v56_a5`, `x08`, `x26`, `x27`): `VARPTR`, `VARSEG`
  and `VARPTR$` of such a variable, member or element are taken, with the old compiler's layout and offsets (a
  `STRING` member is an 8-byte pointer). **The value's strings are made as for any other `TYPE` variable**: the
  old compiler leaves them out there and its program crashes at the first use of one (`v56_a5`, exit 139). That
  is a new row of `DIVERGENCES.md` (D-037) and an entry of `UPSTREAM.md`; `v56_a5` stays on `known.list` with the
  row as its reason. `VARPTR` of the `STRING` member of an array element and `VARPTR$` of a `STRING` member fail
  the old compiler's C++ build (`v60_d7`, `d8`) and become compile errors.
- **`FOR` over an element of a member array** (`FOR r.m(i) = 1 TO 3`, `v56_f3`): the element is found again
  wherever the old compiler's loop names it (once at the start, then at each `NEXT` for the step and for the
  test), so a change of the index in the body moves the loop to another element and an index's FUNCTION runs
  as often as in the old compiler (7 times for 4 passes).

After it, none of `tests\verification\known.list` is "not supported yet": eight programs move to `pass.list`,
`v56_a5` takes D-037. The `SOMEDAY.md` rows that named these forms as marked (lines of "Left marked by …") are
closed, and those that keep a QB64pe behaviour become rows of "QB64pe behaviours to review" (the start code's
second evaluation, the local array of `REDIM`, the element found at every pass).

Out of it: graphics `GET` and `PUT` of a `TYPE` with a `STRING` (its row stays), `VARPTR` of a procedure's
variables, a dynamic member array in conventional memory.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `language/arrays-and-types`: requirement "Arrays in procedures" (a `STATIC` bound read by the start code is
  taken, no longer "not supported yet"; the `REDIM` of a shared array with a bound that first names an array used
  without `DIM` in a procedure).
- `language/conventional-memory`: requirements "What VARPTR and VARSEG accept" and "Variables in conventional
  memory" (a `TYPE` with a `STRING` member is taken, its strings made).
- `language/control-flow`: requirement "FOR loops" (an element of a member array as the variable).

## Impact

- `sema` (`check\places.rs`: the `STATIC` start bounds and `REDIM`'s implicit bound; `check` of `VARPTR`,
  `VARSEG`, `VARPTR$`; `for_var`), the IR (`Shape::Dynamic`'s start bounds as expressions; the loop's place),
  `codegen-cpp` (`arrays.rs` `allocate_at_start`, `decl.rs` `alloc` for a conventional `TYPE` with strings, the
  `FOR` loop).
- `verification\v60_*` (new family, recorded with `qb64pe.exe` before the code), `tests\verification\pass.list`
  and `known.list`, `DIVERGENCES.md` (D-037), `UPSTREAM.md`, `SOMEDAY.md`, `DECISIONS.md` (one row).
- No new dependency.
