# Tasks

Standing rules: CLAUDE.md.

## 1. Reading the two lines

- [x] 1.1 `run_legacy_tests.py`: `recorded_line(compile_txt)` and `earliest_error_line(output, main)` (design D1–D4).
      Verified by `test_run_legacy_tests.py`: the first of two `LINE n:`; none recorded; "in line 2 of … included"
      and "… auto-included" give `None`; errors printed on lines 7 then 3 give 3; a marked error on an earlier line
      skipped; an error in `v16_inc/x.bi` left out; CRLF output; no unmarked error of the main file.

## 2. The rule in the compare

- [x] 2.1 `load_lines_list` (`tests\verification\lines.list`, D5) and the pin in `verification_compare`'s rejection
      branch: the stage `error line` with both lines in the detail, the exception's line standing for QB64pe's, the
      detail `lines.list row not needed`, the fixes-on run untouched (D1). The run's end lists unneeded rows beside
      the known-list programs that passed. `read_by_runs` (`--list-only`) names `lines.list` (D7). Verified by the runner tests
      with a stand-in compiler: rejected on the right line, on another line, with an earlier error printed later,
      only in an included file; no line recorded;
      QB64pe's error in an included file; an exception kept, moved and unneeded; `lines.list` an input of the step (`test_check.py`'s
      `test_every_file_a_suite_reads_is_an_input_of_its_step`, once the file exists, task 2.2).
- [x] 2.2 `tests\verification\lines.list` created empty (a header comment giving the format) and
      `tests\verification\README.md` and `tools\legacy_tests\README.md` ("Verification programs") say what it is.
      Verified by reading both against the spec delta.

## 3. The tier-1 check

- [x] 3.1 `crates\driver\tests\it\inputs.rs`: `lines.list` checked as design D8. Verified by `cargo test -p
      qb64rust-driver --test it inputs::` passing on the empty list, and failing (then reverted) on a row naming a
      program that compiles.

## 4. The full run and its triage

- [x] 4.1 `just check verification` with the release build; read the failures at the stage `error line` from the
      results file and sort them into the reasons of D5 and D6. Stop and give the user a table of reasons with
      counts, before writing any row, at the first of: more than 40 exceptions, more than 10 bug rows, or a reason
      other than the block-error kinds covering more than 3 programs (design, "Risks").
- [x] 4.2 Each failure triaged: a place both compilers can defend goes to `lines.list` with its reason (D5); a wrong
      earliest error goes to `known.list` as `bug: <what>`, off `pass.list`, with a line in `STATUS.md`'s known bugs
      (D6). Verified by the verification step run again passing, with no `lines.list` row reported unneeded, and
      `inputs::` passing.

## 5. Close

- [x] 5.1 `STATUS.md` "Next" names this change and the number of exceptions (`TODO.md` row 1 goes at the archive,
      as `TODO.md` says); the `DECISIONS.md` row of the user's rulings. `just quick` in the background,
      and the files handed over to stage.
