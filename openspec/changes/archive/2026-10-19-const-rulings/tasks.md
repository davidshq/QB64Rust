# Tasks

Standing rules: `CLAUDE.md`.

## 1. The evaluator (design D1–D4)

- [x] 1.1 D-027: `consteval::literal` reads plain digits in the signed, then the unsigned lane, and beyond 64 bits
  gives an error; `read_text` reads plain digits beyond `_INTEGER64` as a `_FLOAT`. Unit test
  `values_beyond_integer64_are_not_clamped`; the two tests that pinned the clamp changed.
- [x] 1.2 D-028: `power` gives the float lane's result; 0 to a negative power is a division by zero. Unit test
  `integer_powers_are_floats`.
- [x] 1.3 D-029: `INF` read back is an error, as the whole value too. Unit tests `errors`,
  `a_value_beyond_float_is_an_error`.
- [x] 1.4 D-031 in a `CONST`: `certify::function("_COTH")` is the correctly rounded `double` of `1 / tanh(x)`, an
  error at 0 (`f64_nearest` split out of `ucrt_within`). Unit test `coth_is_the_hyperbolic_cotangent` (`mpmath`'s
  values); `_COTH` out of the recorded-values test and the sample check.
- [x] 1.5 Front-end tests `const_rulings_typed.bas` and `const_rulings_errors.bas`; the changed snapshot of
  `const_chr_errors` (line 31: now the literal's error).
- [x] 1.6 `consts_v45.rs`: the two decided values (`_COTH(.2)`, `2 ^ -1`) listed with their rows.

## 2. `_COTH` at run time (design D4, D5)

- [x] 2.1 `copy_libqb.py` applies `tools\libqb\edits\*.edit`, lists them in `SOURCE.md`, and holds the copy to
  them with `--check`. Tests: an edit applied with the target's CR LF, checked clean, and the copy differing
  without it; an edit that does not fit, a missing target and an unended edit each stop the tool.
- [x] 2.2 `tools\libqb\edits\d031-coth.edit`; the copy written with it (`copy_libqb.py`, then `--check`: 0
  problems; `git status` shows only `extended_math.h` and `SOURCE.md`).
- [x] 2.3 `_COTH` a plain built-in held `double`; `tests\callsite\func_coth_{double,literal,long,single}.bas`, 4 of
  4 equal; `more_functions_typed.bas`; `s44_more_functions` calls it ("above 1", re-recorded with `qb64pe.exe`).
- [x] 2.4 The CLI test `const_rulings_run`: the D-027 literal and limits, `2 ^ -1`, `_COTH` at run time and error 5
  at 0.

## 3. Tests whose recorded cases change (design D6)

- [x] 3.1 `s142_const_float`, `s143_const_funcs`, `s146_const_edges` without the decided cases, re-recorded with
  `qb64pe.exe`, each passing with the new compiler.
- [x] 3.2 Upstream `const/math_funcs` from `pass.list` to `tests\known_clean_not_passing.list`.

## 4. Records

- [x] 4.1 `DIVERGENCES.md` D-027 to D-031 (built, with their tests; D-028 and D-029 name `0 ^ -1`); `UPSTREAM.md`
  (D-031 built); `SOMEDAY.md` (the rows of the four limits and of `$COLOR`); the layout rows of `libqb\` and
  `tools\libqb\` in `CLAUDE.md`; `tests\upstream\README.md` and `tests\corpus\README.md` where they name the
  programs.

## 5. Close

- [x] 5.1 The closing runs (`python tools\close\close.py --plan`, then `--check`), shared with `libqb-object-flags`;
  the run without folding, since the change touches `sema`'s constant evaluation.
- [x] 5.2 `python tools\testtime\test_time.py`; `STATUS.md` "Next" and its numbers.
