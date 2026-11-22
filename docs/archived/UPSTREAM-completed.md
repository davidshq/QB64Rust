# Fixes to offer QB64pe: completed

The fixes of `docs\UPSTREAM.md` that QB64pe has taken, under the same headings. The fixes not yet sent or not
yet merged are in `docs\UPSTREAM.md`.

## `_STATUSCODE` of a handle never opened crashes

- **Wrong:** `_STATUSCODE(h)` of a negative handle that was never opened, or was closed, reads the type of the
  entry `list_get` did not find: an access violation (exit 139). Measured in this repo: `verification\v44_x59`,
  `v63_d1`.
- **Where:** `internal/c/libqb.cpp`, `func__statusCode`.
- **Fix:** `if (!sh || sh->type != special_handle_type::Http)` raises error 52, as `func__connected` tests.
- **Here:** `DIVERGENCES-QB64PE.md` D-051; the edit `tools/libqb/edits/d051-statuscode-missing-handle.edit`.
- **Sent:** pull request QB64-Phoenix-Edition/QB64pe#817 (opened 2026-11-13), from branch
  `fix-statuscode-unopened-handle` of `davidshq-contribute/QB64pe`. It adds a never-opened negative handle to the
  test `tests/compile_tests/http/statuscode_invalid_handle`. Merged on 2026-11-15 by Samuel Gomes (`a740g`): commit
  `e8e357fa3`, merge `ae934c3f1`.

## `_MEMFREE` of a `_MEMNEW(0)` block frees memory twice

- **Wrong:** the 0-byte branch of `_MEMNEW` sets no type on its lock, so a lock taken again from the freed list
  keeps type 1 and the pointer of the block freed before, and `_MEMFREE` of the 0-byte block frees that pointer a
  second time (exit 127). Measured in this repo: `verification\v32_d33_free_zero_size`, `v63_c2`.
- **Where:** `internal/c/libqb/src/memblock.cpp`, `func__memnew`, the `if (!bytes)` branch.
- **Fix:** `mem_lock_tmp->type = 0;` there, the type of a block that owns no memory.
- **Here:** `DIVERGENCES-QB64PE.md` D-050; the edit `tools/libqb/edits/d050-memnew-zero-lock.edit`.
- **Sent:** pull request QB64-Phoenix-Edition/QB64pe#822 (opened 2026-11-17), from branch
  `fix-memnew-zero-double-free` of `davidshq-contribute/QB64pe`. It adds the test
  `tests/compile_tests/mem/memnew_zero_free`. Merged on 2026-11-18 by Samuel Gomes (`a740g`): commit
  `57b8baa6e`, merge `660925bba`.

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
- **Here:** `DIVERGENCES-QB64PE.md` D-031, built by `const-rulings`: the edit `tools/libqb/edits/d031-coth.edit` of this
  repo's copy of libqb (`1 / std::tanh(num)`, error 5 at 0) is the patch to offer, and the `CONST` evaluator here
  computes the same function.
- **Sent:** pull request QB64-Phoenix-Edition/QB64pe#823 (opened 2026-11-18), from branch
  `fix-coth-hyperbolic-cotangent` of `davidshq-contribute/QB64pe`. It adds the test `tests/compile_tests/math/coth`
  and corrects `const/math_funcs`, which expected the old `_COTH(.2)` of 1.4428. Merged on 2026-11-19 by Samuel
  Gomes: commit `2a2f3710f`, merge `35f943174`. The merged `func_coth` is the same as this repo's edit.
