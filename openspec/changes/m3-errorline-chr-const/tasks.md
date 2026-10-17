# Tasks

Standing rules: CLAUDE.md. Differently here: the measurements of group 1 were taken before the proposal.

## 1. Measurements and records

- [x] 1.1 Record what was measured (`verification\v43_*`, 54 programs, included files in `v43_inc\`): the facts of
  the proposal into `study\00` §5 (a `v43` paragraph), the family into `verification\README.md`, a row in
  `DECISIONS.md` for the step. Verify: `python tools\repo_check\check_repo.py --untracked` is clean.

## 2. CHR$ in a CONST

- [x] 2.1 `consteval::chr` and its unit test; `check\constants.rs` (a call, its errors, `CHR$` without
  parentheses, a built-in function's name an error, design D3, D4); `check\auto.rs` (a term, design D3). Verify:
  front-end test `const_chr_errors` (`check-fail`); the three tests whose example was `CHR$` in a `CONST` changed to
  `EXP`; `auto_const_errors` without the `CHR$` constants.
- [x] 2.2 Slice program `s135_chr_const`, recorded with `qb64pe.exe` (`--record`) and run with this compiler;
  upstream `font/test` added to `tests\upstream\pass.list`. Verify: each passes (`--glob`).

## 3. The error functions

- [x] 3.1 `sema`: the line of a joined line (design D1), `include_line` of statements and labels (D2); the four rows
  of `builtins.rs`. `ir`: the field, lowered and dumped; `codegen-cpp` `evnt_args`. Verify: front-end tests
  `errorline_errors`, `errorline_typed`, `errorline_cpp`; the `ir` snapshots of included files show the main line;
  the seven call-site programs equal (`tools\callsite\callsite.py --glob "*error*"`).
- [x] 3.2 Slice programs `s136_errorline` and `s137_errorline_include` (its included files in its `.files` folder;
  tier 1 reads includes from there, `inputs.rs` `beside_files`), recorded and run; upstream `environ/set_environ`
  and `image/curico_test` added to the pass list. Verify: each passes (`--glob`).

## 4. Close

- [x] 4.1 The closing runs (working rule 9; the run without folding too, `consteval` changed), the shrink-only
  lists, the demand reports and `tools\diagnostics\old_wording.py` regenerated, the counts by
  `tools\legacy_tests\counts.py`. Verify: every run passes or fails only by known failures. Checked at the end with
  the new `tools\close\close.py --check` (`--not-repeated` for the runs older than the last edit of
  `consteval.rs`, which is test code only); tier 1 339 of 339 once the `verification\v40_rc` files the
  `.gitattributes` edit made CRLF were checked out again; `v43_e` run by hand as the runner would run it, equal.
- [x] 4.2 Documents: `STATUS.md`, `ROADMAP.md`, `SOMEDAY.md`. Verify: `openspec validate m3-errorline-chr-const
  --strict` passes.
- [x] 4.3 `python tools\testtime\test_time.py`. Verify: its five files are named among those to stage.

## Workflow follow-up

- Archive the change when the user has looked at it.
