# Proposal: const-rulings

## Why

On 2026-10-18 the user ruled on the five behaviours `m3-constant-evaluation` kept QB64pe's way (`DECISIONS.md`
2026-10-18; `DIVERGENCES.md` D-027 to D-031): four are wrong values given silently, one a misleading message. The
rows are recorded and not built (`STATUS.md` "Next" item 2). `libqb-copy` made the first edit of libqb possible,
which D-031 needs at run time.

## What Changes

- **D-027:** an unsuffixed integer literal beyond `_INTEGER64` in a `CONST` is read in the unsigned lane up to
  18446744073709551615, and beyond that it is a compile error. The old evaluator clamps it to
  9223372036854775807. A float result that prints as plain digits beyond `_INTEGER64` (`9.3E+18 / 1`) stays the
  `_FLOAT` it is. The four auto-included limits built from such literals hold their types' limits:
  `_INTEGER64_MIN` and `_OFFSET_MIN` are -9223372036854775808, `_UINTEGER64_MAX` and `_UOFFSET_MAX`
  18446744073709551615.
- **D-028:** `^` in a `CONST` gives a float in every lane, read back by its text as every float is: `2 ^ -1` is
  0.5, `2 ^ 3` still the `_INTEGER64` 8. 0 to a negative power is a division by zero.
- **D-029:** a `CONST` whose value is beyond `_FLOAT`'s range is a compile error, as the whole value too. The old
  evaluator gives 0 there.
- **D-030:** already built by `m3-constant-evaluation`. A `CONST` above `$COLOR` that names a colour gets "`Black`
  is a `$COLOR` constant, which a `CONST` above the `$COLOR` line cannot name", with the old wording as a note.
  Here the row is only marked built, with its test.
- **D-031:** `_COTH` is the hyperbolic cotangent: in a `CONST`, the correctly rounded `double`, with an error at 0;
  at run time, through an edit of the copy of libqb (`1 / std::tanh(x)`, error 5 at 0), noted in `UPSTREAM.md`.
  `_COTH` becomes a supported built-in at run time; until now only the evaluator knew it.
- The copy of libqb gets its own edits: `tools\libqb\edits\*.edit`, applied by `copy_libqb.py` after copying, and
  held by `--check`. `libqb\SOURCE.md` lists them.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `language/constants`: "CONST values" (D-027, D-028, D-029), "Constants of the auto-included file" (the four
  limits), "Functions in a CONST" (`_COTH`).
- `language/builtin-functions`: `_COTH` at run time (added requirement).
- `compiler/pipeline`: "The copy of libqb equals the reference commit" with its listed edits.

## Impact

- Changed: `crates\sema\src\consteval.rs`, `certify.rs`, `builtins.rs`; `tools\libqb\copy_libqb.py` and its tests;
  `libqb\internal\c\libqb\include\extended_math.h` and `libqb\SOURCE.md` (written by the tool).
- New: `tools\libqb\edits\d031-coth.edit`; `tests\frontend\const_rulings_typed.bas`, `const_rulings_errors.bas`;
  the CLI test `const_rulings_run`; `tests\callsite\func_coth_*.bas`.
- Tests whose recorded cases the rulings change: `s142_const_float`, `s143_const_funcs`, `s146_const_edges` lose
  them and are re-recorded with `qb64pe.exe`; `s44_more_functions` gains `_COTH`; `consts_v45.rs` lists the two
  decided values; upstream `const/math_funcs` leaves `pass.list` for `tests\known_clean_not_passing.list`, since it
  tests `_COTH(.2)` against the old value.
- Programs the old compiler accepts and this one rejects: a literal beyond 64 bits, a value beyond `_FLOAT`,
  `_COTH(0)` and `0 ^ -1` in a `CONST`.
