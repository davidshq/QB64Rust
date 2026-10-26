# Fixes to offer QB64pe

Bugs of QB64pe that this project fixes in its own code or its copy of libqb, kept here so they can be reported or
sent to QB64pe later (user, 2026-10-18). Each entry says what is wrong, where in QB64pe's sources at the
reference commit (`16f629784e`), the fix, and this project's divergence row. Nothing here has been sent yet;
when one is, its entry says where (issue or pull request) and what came of it.

## `_COTH(x)` is not the hyperbolic cotangent

- **Wrong:** `_COTH(x)` returns `2 * EXP(x) - 1`: `_COTH(0)` is 1, `_COTH(.2)` 1.442805516320339709 and
  `_COTH(1)` 4.4366 (the hyperbolic cotangent: undefined, 5.0665 and 1.3130). It also raises error 5 for any
  argument above 44.014845, where the hyperbolic cotangent is 1 to double precision. Measured in this repo:
  `verification\v45_x32`, `v45_b_funcs`.
- **Where:** `internal/c/libqb/include/extended_math.h`, `func_coth`. The `CONST` evaluator
  (`source/utilities/const_eval.bas`, `CASE "_COTH"`) calls `_COTH` of the compiler itself, so the one fix
  corrects both a statement and a `CONST`.
- **Fix:** the hyperbolic cotangent, `(EXP(2x) + 1) / (EXP(2x) - 1)`, written so it does not overflow for large
  arguments (for example `1 / std::tanh(num)`), with error 5 at 0 as the neighbouring functions raise for a
  division by zero (`func_csch`), and no upper limit.
- **Here:** `DIVERGENCES.md` D-031, built by `const-rulings`: the edit `tools/libqb/edits/d031-coth.edit` of this
  repo's copy of libqb (`1 / std::tanh(num)`, error 5 at 0) is the patch to offer, and the `CONST` evaluator here
  computes the same function.
- **Sent:** not yet.

## A `TYPE` with a `STRING` in conventional memory has no strings

- **Wrong:** a `TYPE` variable of the main module that holds a `STRING` member, and that `VARPTR`, `VARSEG`,
  `SADD` or `VARPTR$` names anywhere in the program, is allocated in conventional memory with its bytes zeroed
  and its strings never made: the first use of the string reads a null pointer and the program crashes (exit
  139). Measured in this repo: `verification\v56_a5`, `v60_d6`.
- **Where:** `source/qb64pe.bas`, `dim2`, the scalar user-type branch (lines 17790–17852 at the reference
  commit): `initialise_udt_varstrings` and `free_udt_varstrings` are written only in the `ELSE` branch (ordinary
  memory), not after the `memset` of the `cmemlist` branch.
- **Fix:** the same two calls in the conventional-memory branch, after its `memset` (the string descriptors are
  ordinary `qbs` values, so no offset in `dblock` moves).
- **Here:** `DIVERGENCES.md` D-037, built by `m4-left-marked` (`crates\codegen-cpp\src\cmem.rs`, `alloc_dblock`).
- **Sent:** not yet.

## `-y` writes a number in `_ARRAYCOPY` subscripts as its internal element

- **Wrong:** formatting `_ARRAYCOPY g(1.5 TO 3.5) TO gd(0.5)` with `-y` (or the IDE's auto-format) writes
  `_ArrayCopy g(1.5E+0,1.5 To 3.5E+0,3.5) To gd(0.5E+0,0.5)`: each number with a point or an exponent comes out
  as the element `lineformat$` makes of it, the value and the spelling joined by a comma. The commas make another
  program (four subscripts where there were two). Found by this repo's format oracle:
  `tests\format\corpus\slice\s131_arraycopy.bas`, line 167.
- **Where:** `source/utilities/arrcpy.bm`, `ArrayCopyRefCaseLayout$`, which lays out the raw elements of the
  reference through `CompactMemberRefLayout$` (`source/qb64pe.bas`); a number element is never turned back into
  the text after its comma, as `fixoperationorder$` does for an expression's layout.
- **Fix:** lay out each number element by the part after its comma (the spelling as written), as the expression
  layout does; or build the subscripts' layout from `fixoperationorder$`'s.
- **Here:** `DIVERGENCES.md` D-038, built by `m5b-formatter`: the formatter writes the number as written.
- **Sent:** not yet.

## Other candidates

Bugs this project fixes in the `CONST` evaluator (built by `const-rulings`), which QB64pe might also want (not
ruled to be sent; listed so they are not forgotten):

- Values beyond `_INTEGER64` in a `CONST` clamped, and the four limits `_INTEGER64_MIN`, `_OFFSET_MIN`,
  `_UINTEGER64_MAX`, `_UOFFSET_MAX` wrong by it (`DIVERGENCES.md` D-027).
- `^` of two integers in a `CONST` rounded to an integer, `2 ^ -1` being 0 (D-028).
- A `CONST` beyond `_FLOAT` range being 0 or "Unexpected element 'INF'" (D-029).
