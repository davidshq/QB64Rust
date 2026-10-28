# Proposal

## Why

`study\39` ruled 19 crash-class rows of "QB64pe behaviours to review" (user, 2026-10-26). `fixes-switch` built the
switch and the rows whose fix lives in the compiler. The rest (rows 2, 5, 9, 11, 12, 17 and 18) are fixed in
libqb, and until they are built a program can still hang at the end of its input, write past libqb's
conventional memory, free memory twice, crash on a network handle it never opened, or end at a division by zero
that no handler receives. Row 17's switched fix, `bit-number-check`, waits on this change too.

## What Changes

- **Always-on fixes in libqb** (no switch), each an edit of libqb's copy through `tools\libqb\edits\`, with a
  `QB64PE-DIVERGENCES.md` row and an `UPSTREAM.md` entry:
  - row 2: a console `INPUT`, `LINE INPUT` or `INPUT$` at the end of a redirected input raises error 62
    (trappable) instead of waiting for ever; `END` skips its key wait when the input is not a console (it still
    prints "Press any key to continue", as QB64pe does on Linux);
  - row 5: `BLOAD` and `BSAVE` whose start plus length runs past libqb's block of conventional memory raise
    error 5 before touching anything; inside the block they work as today;
  - row 9: `_MEMFREE` of a `_MEMNEW(0)` block frees nothing, whatever lock it was given;
  - row 11: `_STATUSCODE` of a handle never opened raises error 52 instead of crashing;
  - row 12: error 11 (division by zero) is trappable, from `\`, `MOD` and `ERROR 11`; the CPU's exception stays
    fatal as a backstop;
  - rows 17, 18: a bit number or shift count outside 0 to 63 is taken modulo 64 in every case, so a constant
    gives what a variable gives.
- **A switched fix**, `bit-number-check` (row 17): with it on, a bit number outside 0 to 63 in `_READBIT`,
  `_SETBIT`, `_RESETBIT` and `_TOGGLEBIT` raises error 5. It joins the seven fixes of `fixes-switch` through the
  same switch (`qb64rust.toml`, `--fix`, `--list-fixes`).
- **Measurements first** (`study\39` §3): QB64pe's side of each row recorded with `qb64pe.exe` in a new family
  `v63`, before any edit.

Not in this change: the candidates of `study\39` §4 (float `/` by zero, the shifts under the switch, a trappable
error 7, a warning for OpenGL outside `SUB _GL`); a trappable `INPUT$` from a window program.

## Capabilities

### New Capabilities

None.

### Modified Capabilities
- `compiler/fixes`: the eighth name, `bit-number-check`.
- `language/console-and-window`: console input at the end of a redirected input; `END`'s key wait when the input
  is not a console.
- `language/conventional-memory`: `BLOAD` and `BSAVE` past the block.
- `language/mem-blocks`: `_MEMFREE` of a `_MEMNEW(0)` block.
- `language/network`: `_STATUSCODE` of a handle never opened.
- `language/numeric-semantics`: a divisor of 0 raises a trappable error 11.
- `language/error-handling`: error 11 leaves the errors no handler receives.
- `language/builtin-functions`: bit numbers modulo 64; the fix `bit-number-check`.

## Impact

- `tools\libqb\edits\`: new edit files for `libqb.cpp` (rows 2, 5, 11), `libqb\src\memblock.cpp` (row 9),
  `libqb\src\error_handle.cpp`, `libqb\include\error_handle.h` and `qbx.cpp` (row 12), `libqb\include\bitops.h`
  (rows 17, 18); `libqb\` rewritten by `copy_libqb.py`, `libqb\SOURCE.md` lists them. libqb's objects are built
  again once.
- `crates\base` (`fixes.rs`: the eighth fix) and `crates\codegen-cpp` (`builtins.rs`: the check of
  `bit-number-check`).
- `QB64PE-DIVERGENCES.md`, `UPSTREAM.md`, `DIVERGENCES-QB45.md` (Q-003 measured as matching now).
- Verification programs whose recording shows the old behaviour (`v22_e_eof`, `v02b_idiv_zero`,
  `v32_d33_free_zero_size`, `v44_x59`, the four that time out at `END`) move to `known.list` with their `D-0nn`.
