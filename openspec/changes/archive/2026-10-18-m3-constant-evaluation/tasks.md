# Tasks

Standing rules: CLAUDE.md. Differently here: a few scratch measurements were taken before the proposal; task 1.1
records them in the family.

## 1. Measurements and records

- [x] 1.1 Record the proposal's measurements as `verification\v45_a_*` (x87 precision, `_E`, typing by text, the
  unsigned lane, the clamp, the auto-included limits, `_PI`, `ROOT`, a suffixed constant in a `CONST`), with
  their recorded output; a row in `DECISIONS.md` for the step and one for `astro-float-num` (D3); the facts into
  `study\00` §5 (a `v45` paragraph); the family into `verification\README.md`. Verify: `python
  tools\repo_check\check_repo.py --untracked` is clean.
- [x] 1.2 Measure what the design leaves to measurement (`v45_b_*` accepted, `v45_x*` rejected): the conversion of
  a float to each integer lane (halves, negatives, beyond range); an integer-valued float between 2^63 and 10^19
  (plain digits, clamped?); each function's argument count and string or number arguments; `SQR(-1)`,
  `_ARCSEC(0.5)`, `LOG(0)`, `_SECH(100)`, `_ACOS(2)`, `ASC("")`, `ASC("ab", 5)` (what `qb64pe.exe` does: an
  error, a run-time error inside it, a value); `c$` of a numeric constant and `s%` of a string one inside a
  `CONST`; each colour function at each mode; `$COLOR` inside a SUB, after a use of its names, in an included
  file, twice, both sets, `$COLOR:16`, a program variable or SUB named `Red`. Facts into `study\00` §5; a kept
  behaviour into `SOMEDAY.md`'s "QB64pe behaviours to review". Verify: two recordings agree.
- [x] 1.3 The sample of design D4: `verification\v45_sample.bas` prints the bits of each function's result at 500
  arguments, recorded with `qb64pe.exe`; the bound of each model is set from it in the table of `design.md` (with
  task 3.1, which gives the models). Verify: the sample is recorded twice, the same.

## 2. The software `_FLOAT` and the lanes

- [x] 2.1 `crates\sema\src\f80.rs` (design D1): the format, the operations, `to_g19`, `from_decimal`; `astro-float-num`
  in `sema`'s manifest and `deny.toml`. Verify: its unit tests (halfway cases, subnormals, the `v45` texts, the
  round trip); `cargo deny check` passes.
- [x] 2.2 `consteval.rs` over `F80` with the three lanes, `ValTy`, `Range` and the re-read by text (design D2);
  `check\constants.rs` (the walk, suffixed constants inside a `CONST`, design D6); the unsigned and `_FLOAT`
  constant types through `settle`, `convert` and `const_use`. Verify: the unit tests of `consteval.rs` (the old
  ones kept where their value is the old compiler's, the "not supported yet" ones changed to values);
  `tier 1` with the front-end snapshots whose float constants change type reviewed one by one.
- [x] 2.3 `literal_expr`: a `_FLOAT` constant as 19 digits and `L`, a DOUBLE as 19 digits (design D2). Verify:
  front-end test `const_float_cpp` (`cpp`), equal to the old compiler's text for the `v45_a` constants; the
  call-site check unchanged (`tools\callsite\callsite.py`).
- [x] 2.4 Slice program `s142_const_float`, recorded with `qb64pe.exe` and run; upstream `const/const_float`,
  `const/const_replace`, `const/rho_test` added to the pass list. Verify: each passes (`--glob`); the run without
  folding for these (`QB64RUST_NO_FOLD=1`).

## 3. The functions

- [x] 3.1 `certify.rs` (design D3, D4): the reference at two precisions, the fuzzy steps, each function as its
  library's expression, the bounds of task 1.3; the `mpmath` values for its tests (script beside
  `extract_builtins.py`). Verify: its unit tests, the sample of task 1.3 within every bound.
- [x] 3.2 The exact functions and their errors in `consteval.rs` and `check\constants.rs` (`FUNCTIONS` becomes the
  table of the evaluator with argument counts); the palettes into `crates\builtins\src\palettes.rs` by
  `extract_builtins.py` and `matchcol` (design D5). Verify: front-end test `const_funcs_errors` (`check-fail`), one
  form of each `v45_x*`.
- [x] 3.3 `check\auto.rs` through the evaluator's paths (design D7). Verify: the program naming every auto-included
  constant compiles with no "not supported yet" (none is left: with D-026 every constant has a value);
  `auto_const_errors` updated.
- [x] 3.4 Every recorded constant of `v45_b_funcs` and `v45_b_funcs_plain`: equal to the old compiler's, or with
  the D-026 warning on its line (the user's decision of 2026-10-18 replaced "not supported yet"); never different
  without it (`crates\driver\tests\consts_v45.rs`). Verify: the test passes; 72 of 75 equal, `COS(1000)`,
  `SIN(100)` and `3 ^ 41` (also D-007) with the warning.
- [x] 3.5 Slice program `s143_const_funcs`, recorded and run; upstream `const/math_funcs`, `func_tostr/mmConst`,
  `auto_include/general` added to the pass list where they pass (else in `tests\known_clean_not_passing.list` with
  the constant that does not certify). Verify: each passes (`--glob`).

## 4. $COLOR

- [x] 4.1 `extract_builtins.py` copies `color0.bi` and `color32.bi` (design D8); `syntax` reads the metacommands and
  their errors; `driver` parses the colour file before the program. Verify: front-end test `color_errors`
  (`check-fail`), the forms of task 1.2.
- [x] 4.2 Slice programs `s144_color0`, `s145_color32`, recorded and run; upstream `auto_include/color0`, `color32`
  added to the pass list. Verify: each passes (`--glob`).

## 5. Close

- [x] 5.1 Pipeline spec: the front-end tests behind "No follow-on errors after an unsupported declaration" use a
  `TYPE` block inside a SUB in place of `CONST c = EXP(1)`. Verify: those tests pass.
- [x] 5.2 The closing set (`python tools\close\close.py --plan`, the run without folding included: `consteval`
  changed), the shrink-only lists, the demand reports, `tools\diagnostics\old_wording.py` regenerated, the counts
  by `tools\legacy_tests\counts.py`. Verify: `python tools\close\close.py --check` passes.
- [x] 5.3 Documents: `STATUS.md`, `ROADMAP.md` (the "Constant evaluation" item), `SOMEDAY.md`, `DIVERGENCES.md`
  D-007's text (design, Risks), `crates\README.md` only if the dependency list there names crates. Verify:
  `openspec validate m3-constant-evaluation --strict` passes.
- [x] 5.4 `python tools\testtime\test_time.py`. Verify: its five files are named among those to stage.
- [x] 5.5 From the review at the close: the warning of D-026 at every use of a constant that took the correctly
  rounded value (`_E` warned only when first named inside a `CONST`; a `PRINT _E` never did): the checker keeps
  the constants that fell back, a `CONST` that names one warns, and so does a use of an auto-included one outside a
  `CONST`. Verify: `consts_v45.rs` `each_use_of_an_uncertified_constant_warns`; the closing runs again
  (`close.py --check`).

## Workflow follow-up

- Archive the change when the user has looked at it, with the kept behaviours (the clamped limits and what task
  1.2 finds) put to the user.
