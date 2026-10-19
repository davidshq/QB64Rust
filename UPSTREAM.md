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

## Other candidates

Bugs this project fixes in the `CONST` evaluator (built by `const-rulings`), which QB64pe might also want (not
ruled to be sent; listed so they are not forgotten):

- Values beyond `_INTEGER64` in a `CONST` clamped, and the four limits `_INTEGER64_MIN`, `_OFFSET_MIN`,
  `_UINTEGER64_MAX`, `_UOFFSET_MAX` wrong by it (`DIVERGENCES.md` D-027).
- `^` of two integers in a `CONST` rounded to an integer, `2 ^ -1` being 0 (D-028).
- A `CONST` beyond `_FLOAT` range being 0 or "Unexpected element 'INF'" (D-029).
