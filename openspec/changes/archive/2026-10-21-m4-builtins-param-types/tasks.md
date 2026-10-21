# Tasks

Standing rules: CLAUDE.md. What this change does differently: the `v49` recording with `qb64pe.exe` is batched in
task 1.1, before the code; the clipboard is set by tests (design D3).

## 1. Measurements and records

- [x] 1.1 The family `verification\v49_*` (design D6), recorded with `--suite verification`, with one line in
  `verification\README.md`. Where a recording differs from a delta spec or the design, the spec or design is
  changed to match in this task. That covers `_CAST` of a float out of range, the bare `_FILES$` before a call
  with an argument, `CLEAR , , LEN("a")`, `_CAST` in a `CONST`, and a `TYPE` in the procedure's own body.
  Verify: every program has its recorded output or error; `openspec validate m4-builtins-param-types --strict`
  passes after any edit of the specs.
- [x] 1.2 `DECISIONS.md`: a row for the change. It records the scope (the six built-ins and the parameter's type,
  user 2026-10-21) and that the tests set the clipboard (design D3). Verify: the row names
  `openspec\changes\m4-builtins-param-types\design.md`.

## 2. The plain built-ins

- [x] 2.1 `sema`: the rows of `_OS$`, `_MD5$`, `_CLIPBOARD$` (function and statement) and `_FILES$` (design D1).
  Verify: `tests\frontend\builtins_more_typed.bas` (`typed`: each, `_FILES$` bare and with its argument) and
  `builtins_more_errors.bas` (`check-fail`: the `v49_x` forms of these four); `cargo test -p qb64rust-sema`.
- [x] 2.2 Call-site programs `tests\callsite\` for each form of 2.1, and a slice program
  `tests\corpus\slice\` (`_OS$`, `_MD5$`, `_FILES$` in a folder of its own, `_CLIPBOARD$`) with output recorded
  from `qb64pe.exe`, on `slice.list`. Verify: the coverage test of tier 1; the call-site check of the new
  programs; the slice program passes.

## 3. _CAST

- [x] 3.1 `sema`: `Rule::Cast` and its check, `ConvKind::Cast`, the evaluator leaving it unfolded (design D2).
  Verify: `builtins_more_typed.bas` gains a `_CAST` of every type named, with the types the dump shows;
  `builtins_more_errors.bas` the `_CAST` errors of `v49_x`; `cargo test -p qb64rust-sema`.
- [x] 3.2 `ir` and `codegen-cpp`: `Cast` validated and written as `((ctype)(value))`. Verify:
  `tests\frontend\cast_cpp.bas` (`cpp`) pins the expressions; the C++ builds (a CLI test with `--ignored`);
  `v49_a_cast` passes; `cargo test -p qb64rust-ir`.
- [x] 3.3 A call-site program for `_CAST` and its lines in the slice program of 2.2. Verify: as 2.2.

## 4. CLEAR

- [x] 4.1 `sema`: the `CLEAR` row and its two errors (design D4). Verify: `builtins_more_typed.bas` gains
  `CLEAR` and `CLEAR , , 100`; `builtins_more_errors.bas` the `CLEAR` errors of `v49_x`; `cargo test -p
  qb64rust-sema`.
- [x] 4.2 `codegen-cpp`: `CLEAR` written as the old compiler writes it; `clear.txt` changed only where `v49_e`
  shows a difference. Verify: `v49_e_clear` and `95_type_codegen_clear_inline_owner` pass; a call-site program
  for `CLEAR`.

## 5. A parameter's type below the procedure

- [x] 5.1 `sema`: `types_above` and the check in `declare_params` (design D5). Verify:
  `tests\frontend\param_type_order_errors.bas` (`check-fail`: the type below at the top level, in a SUB below,
  an array parameter, a FUNCTION); `v48_x12`, `v48_x25` and `v49_f_param_type` pass; the corpus, upstream and
  `tests\programs` front-end runs of tier 1 report no new error.

## 6. Programs and lists

- [x] 6.1 Onto `tests\upstream\pass.list`: `cast/cast_test`, `clipboard/clipboard_test`,
  `filesystem/filesys_test`, `hash/test`, `audio_sound_test/sound_test`, `arrays/95_type_codegen_clear_inline_owner`.
  Off `tests\known_unsupported_rejections.list`: `upstream/arrays/45_err_solved_bug_Incorrect_nr_of_args`. Onto
  `tests\verification\pass.list`, off `known.list`: `v48_x12_type_param_above`, `v48_x25_type_param_top_below`,
  and the `v49` programs (any that cannot pass goes on `known.list` with its reason). Verify: each passes with
  this compiler (`--list` of the programs moved); each verification program is named once across the two lists.

## 7. Close

- [x] 7.1 The closing runs: `tools\close\close.py --check` passes. Verify: the upstream demand report no longer
  names `_CAST`, `_CLIPBOARD$`, `_FILES$`, `_MD5$`, `_Os$` or `CLEAR`.
- [x] 7.2 Documents. `STATUS.md`'s "Next" and its "Known bugs" (the parameter bug gone). `SOMEDAY.md`: a row for
  each form task 1.1 left marked, and the questionable behaviours kept under "QB64pe behaviours to review" (a
  parenthesis inside a `CLEAR` argument rejected, if measured so). Verify: `openspec validate
  m4-builtins-param-types --strict` passes.
- [x] 7.3 `python tools\testtime\test_time.py`. Verify: its five files are named among those to stage.

## Workflow follow-up

- Archive the change once the user has reviewed it (`/opsx:archive`).
- Tell the user which files to stage and suggest a commit message (nothing is staged by Claude).
