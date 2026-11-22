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
