# Tasks

Standing rules: CLAUDE.md. What this change does differently: the `v51` recording with `qb64pe.exe` is batched in
task 1.1, before the code, and may run while the merge runs of `m4` use the worktree (they take the clone's lock
only for its own test folders).

## 1. Measurements and records

- [x] 1.1 The family `verification\v51_*` with the forms of design D1, recorded with `--suite verification`, with
  one line in `verification\README.md`. Where a recording differs from a delta spec or the design, change the
  spec or design to match in this task. Verify: every program has its recorded output or error;
  `openspec validate m4-upstream-rejections --strict` passes after any edit of the specs.
- [x] 1.2 `DECISIONS.md`: a row for the change. It records the scope (the 13 upstream programs, user
  2026-10-21) and the rule of design D1–D5: a mark becomes an error only for cases the old compiler is recorded
  to reject. Verify: the row names `openspec\changes\m4-upstream-rejections\design.md`.

## 2. Type names

- [x] 2.1 `sema`: `no_type` beside `type_of_in` (the planned `is_type_word` helper was not needed). In
  `type_of_in`, "Unknown type" for any other name and "Expected STRING * number/constant" for a word glued to
  `STRING` before `* n` (design D3). Verify: `tests\frontend\type_names_errors.bas` (`check-fail`: the four
  upstream `types/` forms and the `v51` type-name forms recorded as errors); `cargo test -p qb64rust-sema`; the
  front-end runs of tier 1 add nothing to `tests\known_false_errors.list`.
- [x] 2.2 `sema`: "Undefined type" in `member_type` (design D3). Verify: `type_names_errors.bas` gains
  `array (10) AS notdefined` and the `v51` member-type forms; a `TYPE` defined in a later block still compiles
  as `v51` records it; `cargo test -p qb64rust-sema`.
- [x] 2.3 `sema`: a `TYPE` whose name starts with `STRING` is a type in every `AS` clause (review of the staged
  change, 2026-10-21: the prefix test ran before the user-type lookup and rejected `DIM a AS StringList`). Verify:
  `tests\frontend\type_string_prefix_ok.bas` (`check-ok`); tier 1; the `types/` upstream programs and
  `v51_x60`–`x89` still pass; `old_wording.py --check`. The recording of the form with `qb64pe.exe`
  (`v51_x90_dim_string_prefix_type`) waits for the next batch with the clone ("Workflow follow-up").

## 3. Member array bounds

- [x] 3.1 `sema`: the program's `CONST` names collected before pass 1, the included files' among them, and
  `member_bounds` giving the error for a bound that names none of them (design D2). Verify:
  `tests\frontend\member_bound_errors.bas` (`check-fail`: `39_err_member_must_be_const`'s form, `v24_x195`'s, and
  the `v51` bound forms recorded as errors); a `CONST` bound, also one from an included file under
  `tests\frontend\inc\`, still gives the mark alone; `cargo test -p qb64rust-sema`.

## 4. LBOUND and UBOUND of an element

- [x] 4.1 `sema`: the indexed member array, and each other element form that `v51` records, as their errors in
  the bound functions' argument match (design D4). Verify: `tests\frontend\bound_element_errors.bas`
  (`check-fail`: `UBOUND(f(10).a(5), 1)`, `LBOUND` and the `v51` forms); forms `v51` does not record keep the
  mark; `cargo test -p qb64rust-sema`.

## 5. CONST expressions

- [x] 5.1 `syntax`: in a `CONST`, an operator word where an operand is expected is an error, not a mark
  (design D5). Verify: a parser test with `CONST c = OR`, `NOT OR` and `1 OR` (an error each) and `x = OR`
  outside a `CONST` (still the mark); `cargo test -p qb64rust-syntax`.
- [x] 5.2 `sema`: the last arm of a name in `check\constants.rs` gives an error, and the cases `v51` records as
  accepted keep their mark (design D5). Verify: `tests\frontend\const_names_errors.bas` (`check-fail`: the four
  upstream `const/` forms and the `v51` `CONST` forms); `cargo test -p qb64rust-sema`.

## 6. Wording and lists

- [x] 6.1 The wording table regenerated (`tools\diagnostics\old_wording.py`), before any long run. Verify:
  `old_wording.py --check` passes, and each of the 13 upstream programs shows the old compiler's message as the
  note after its error.
- [x] 6.2 Onto `tests\upstream\pass.list`: the 13 programs of the proposal's table. Off
  `tests\known_unsupported_rejections.list`: the 12 programs that now have real errors. One run of
  `tests\verification\known.list` with this compiler, and every program that passes moved to `pass.list`
  (`v24_x195` and `v24_x232` at least), with the `v51` programs placed on one of the two lists. Verify: each
  moved program passes (`--list` of those moved); each verification program is named once across the two lists
  (tier 1).

## 7. Close

- [x] 7.1 The closing runs: `tools\close\close.py --check` passes. Verify: the upstream counts block of
  `tests\upstream\README.md` (written by `tools\legacy_tests\counts.py`) shows 391 of 404, and
  `known_false_errors.list` and `known_parse_gaps.list` are still empty.
- [x] 7.2 Documents. `STATUS.md`'s "Next". `SOMEDAY.md`: a row for each form that `v51` showed but this change left
  marked, and any questionable behaviour kept under "QB64pe behaviours to review". Verify: `openspec validate
  m4-upstream-rejections --strict` passes.
- [x] 7.3 `python tools\testtime\test_time.py`. Verify: its five files are named among those to stage.

## Workflow follow-up

- Archive the change once the user has reviewed it (`/opsx:archive`).
- With the next batch of recordings with `qb64pe.exe`: `verification\v51_x90_dim_string_prefix_type` (a `TYPE`
  named `StringList` in `DIM`, `REDIM`, `STATIC`, `SHARED` and a parameter; expected accepted, as `dim2` and
  `typname2typ` read), onto `tests\verification\pass.list`.
- Tell the user which files to stage and suggest a commit message (nothing is staged by Claude).
