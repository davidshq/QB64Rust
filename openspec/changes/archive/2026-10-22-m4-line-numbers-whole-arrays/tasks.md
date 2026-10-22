# Tasks

Standing rules: CLAUDE.md. What this change does differently: every recording with `qb64pe.exe` (the family `v54`)
is batched in task 1.1, before the code, and may change the delta specs and design D1, D4 before anything is
built.

## 1. Measurements and records

- [x] 1.1 The family `verification\v54_*` of design D5, recorded with `qb64pe.exe` (the verification suite), with
  one line in `verification\README.md`. Where a recording differs from a delta spec or the design (the
  spelling that names a line number, `ERL` after an unnumbered line and in a SUB, the `~&&` stores, the element a
  whole array stands for, the places the old compiler takes `a()` in, an erased array), the spec or design is
  changed here. Verify: every program has its recorded output or error; `tools\repo_check\check_repo.py
  --untracked` finds nothing in them; `openspec validate m4-line-numbers-whole-arrays --strict` passes after any
  edit.
- [x] 1.2 `DECISIONS.md`: a row for the change. It records the plan of the rest of M4 (design D6, user 2026-10-22,
  `SUB _GL` a change of its own) and the ruling that a whole array as a value is built as QB64pe has it (user,
  2026-10-22, against the rows of 2026-10-15 and 2026-10-20). Verify: the row names
  `openspec\changes\m4-line-numbers-whole-arrays\design.md`.

## 2. Unsigned literals

- [x] 2.1 `sema\src\literal.rs`: an unsuffixed literal above `i64::MAX` up to `u64::MAX` takes the `~&&` path
  (design D3); the unit test `integer_typing` gives the new type, and a literal of 2^64 stays `Overflow`. Verify:
  `cargo test -p qb64rust-sema`; a `typed` front-end test with the literal printed, stored and in an operation;
  `v22_b_print`, `v22_d_swap`, `v25_signs`, `v30_a_using_numbers`, `v30_g_iif_min_max`, `v30_h_shifts`,
  `v33_c10_system_unsigned`, `v37_b_types` and `v54_b*` pass (`--suite verification`, a short `--list`), or show
  their next mark.

## 3. Line numbers

- [x] 3.1 `sema`: line numbers entered as labels (design D1): `declare_labels` takes the `LineNumber` node,
  `line_label_name` gives its name, `Label` gains `line_number`; the dispatch passes `LineNumber` and lowers
  `ImplicitGoto` as `GOTO`; `jump_target`, `restore`, `on_error`, `resume`, `events.rs` and the `ON n GOTO/GOSUB`
  list take a number through `label_ref`; `zero` keeps `0`. Verify: `tests\frontend\line_numbers_typed.bas`
  (`typed`: each target) and `line_numbers_errors.bas` (`check-fail`: duplicate, not defined, another body,
  `RETURN n` in a SUB, the `v54_x` errors), with the old compiler's wording through the wording table where the
  messages are new; `cargo test -p qb64rust-sema`.
- [x] 3.2 `ir` and `codegen-cpp`: the label op carries the line number and `last_line=<n>;` is written after the
  label (design D2). Verify: `tests\frontend\line_numbers_ir.bas` (`ir`) and `line_numbers_cpp.bas` (`cpp`: the
  label, `last_line`, the event check); `cargo test -p qb64rust-ir -p qb64rust-codegen-cpp`.
- [x] 3.3 Run the line-number programs. Verify: `v16_m3_*`, `v16_m4_line_if_jump_colon`, `v16_m4_single_if_else`,
  `v17_b_if_then_line`, `v22_c_restore`, `v22_x86_restore_missing_line_number`, `v29_x64_on_play_gosub`,
  `v33_d01_debug`, `v33_d02_no_debug`, `v42_x46_line_number`, `v43_a_errorline`,
  `v50_x13_newhandler_line_number` and `v54_a*` pass, or show a next mark that is not a line number; the
  `lsp` crate's go-to-definition test of a line number still passes.

## 4. A whole array as a value

- [x] 4.1 `sema`: `Place::First(ArrayRef)` (built as an element with no indexes, design D4) made in
  `check\places.rs` `element` for the places task 1.1 settled;
  `check\whole.rs` takes suffixes on both sides as an assignment of two first elements (design D4); the places
  the old compiler rejects keep their errors. Verify: `tests\frontend\whole_values_typed.bas` (`typed`) and
  `whole_values_errors.bas` (`check-fail`); the `check-fail` snapshot of the whole-array assignment checks
  loses the "not supported yet" of the suffix scenario; `cargo test -p qb64rust-sema`.
- [x] 4.2 `ir` and `codegen-cpp`: the first element written at offset 0 of the array's data, with no index check
  (`codegen-cpp\src\place.rs`). Verify: `tests\frontend\whole_values_cpp.bas` (`cpp`); `v22_x101_swap_arrays`,
  `v22_x49_input_file_array`, `v22_x70_read_whole_array`, `v30_x18_using_whole_array`, `v30_x99_iif_array_parts`,
  `v30_x109_field_whole_array`, `v30_x153_lset_whole_array`, `v36_x24`, `v36_x37`, `v36_x38`, `v36_x40`,
  `v36_x43`, `v36_x44`, `v36_a_numbers` and `v54_c*` pass, or show their next mark.
- [x] 4.3 `SOMEDAY.md`: the rows of a whole array as a value (97, 144, 233's first part, 365) leave "deferred" and
  each place becomes a row of "QB64pe behaviours to review"; the rows of line numbers (81, 185, 247) and of the
  unsigned literal (99) are removed. Verify: no row of `SOMEDAY.md` says line numbers or unsigned literals are
  "not compiled yet".

## 5. The review of the change

- [x] 5.1 The findings of the code review, settled by `verification\v54_x39`–`x45` (recorded with `qb64pe.exe`):
  `VARPTR(a())` is taken (the IR's validation took no offset without an index, a panic in a debug build); `08&`
  and a number above 2^64 - 1 are an error at the line number (`check\flow.rs` `cpp_line_problem`, design D1),
  where both compilers' C++ builds failed; a duplicate whole line number is shown by its name (`010` as `10`), as
  the old compiler shows it; `t.m&() = u.m&()` keeps its error (the old compiler's "Expected array index");
  `copy_array` decides the case of two suffixes itself (`both_suffixed` gone); the wording table takes a
  backtick span that is a number alone as a placeholder (`old_wording.py`, `base\src\old_wording.rs`, with
  tests), so a line number's errors get the old wording. The erased `STRING` arrays needed nothing (`x42`–`x44`
  pass). Verify: `cargo test`, fmt, clippy; `old_wording.py --check`; `v54_x30`–`x45` pass, or stay known
  (`x38`, D-027) (`--suite verification`, `--glob`).

## 6. Lists and closing

- [x] 6.1 `tests\verification\known.list` and `pass.list`: every program that now passes moves to `pass.list`; one
  whose first mark changed keeps its line with the new reason. Verify: the tier-1 test that names each program
  once across the two lists; the demand report over `known.list`'s "not supported yet" programs no longer names
  line numbers, unsigned literals or the whole array of a declared array.
- [x] 6.2 The closing runs: `python tools\close\close.py --plan`, each command run by its own, then
  `tools\close\close.py --check` passes; the counts of the test READMEs by `tools\legacy_tests\counts.py`;
  `STATUS.md`'s "Next" names step 2 of design D6; `python tools\testtime\test_time.py`. Verify: `--check`
  passes. Done (2026-10-22): every step ok but `nofold`, 484 of 485, the one failure `screen/g12_game.files`,
  the timeout of `STATUS.md`'s "Still to watch" (twice at `--jobs 8`; passes run alone without folding).

## Workflow follow-up

- Archive the change once the user has reviewed it (`/opsx:archive`).
- Tell the user which files to stage and suggest a commit message (nothing is staged by Claude).
