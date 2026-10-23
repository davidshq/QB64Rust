# Tasks

Standing rules: CLAUDE.md. What this change does differently: the recordings of `v52` with `qb64pe.exe` come in
group 1, before the code. The long name script (1.2) runs in the background while groups 2–5 are built, and group
6 waits for it.

## 1. Measurements and records

- [x] 1.1 The programs `verification\v52_x*` with the forms of design D1 (`_INSTRREV`, the five functions, the
  whole-file functions and `_FULLPATH$`, the `ASC` statement, `STOP`, `key$`/`exp$`/`base$` in each place,
  `name$(i)` of a `STRING * n` array and `s$` of a `STRING * n` scalar). Record them with `--suite verification`,
  and add one line for `v52` in `verification\README.md`. Where a recording differs from a delta spec or the
  design, change the spec or the design to match in this task. Verify: every program has its recorded output or
  error, and `openspec validate m4-qb64pe-sources --strict` passes after any edit of the specs.
- [x] 1.2 `verification\v52_builtin_name_suffixes.py` (from `v15_builtin_names.py`, with `--jobs`) and its
  `.txt`, started in the background once 1.1's recordings are done. Verify: the `.txt` has one line per keyword
  or built-in name and suffix of design D1, each `variable`, `statement` or `rejected` with its detail. The
  script's docstring gives the run's time.
- [x] 1.3 The hidden marks: a temporary `QB64RUST_NO_FOLLOW_ON=1` (debug builds only) and `-z` of the 10 programs
  in the clone's `tests\compile_tests\qb64pe`. List every mark that is not in the proposal's table. Each gets a
  `v52` program and a task here, or a line under the proposal's out-of-scope with the user's word. Verify: the
  list is written into design D1 ("As measured"), and the switch is removed from the code before group 7.
- [x] 1.4 `DECISIONS.md`: a row for the change, with its scope (the 10 `qb64pe/*` upstream programs, user
  2026-10-21) and the rule of D1/D6: a reserved name with a suffix follows the record. Verify: the row names
  `openspec\changes\m4-qb64pe-sources\design.md`.

## 2. Built-in functions and `_WRITEFILE`

- [x] 2.1 `sema` `builtins.rs`: the rows of design D2 (`_INSTRREV`, `_READFILE$`, `_FULLPATH$`, `_ARCSEC`,
  `_ARCCSC`, `_ARCCOT`, `_SECH`, `_CSCH`) and of D3 (`_WRITEFILE`), with a `Rule` override wherever `v52` shows
  one. Verify: `tests\frontend\qb64pe_builtins_typed.bas` (`typed`: each call and its type) and
  `qb64pe_builtins_errors.bas` (`check-fail`: wrong argument counts and types, as `v52` records them);
  `cargo test -p qb64rust-sema`.
- [x] 2.2 `tests\callsite`: one program per new form and argument type (design D2, D3). Verify: `python
  tools\callsite\callsite.py` on the new programs (`--glob`) reports each one equal.

## 3. The `ASC` statement

- [x] 3.1 `sema` `check\io.rs` `asc_assign`, and the arm in `check\mod.rs`. Remove `implicit_array`'s mark for
  `ASC` (design D4). Verify: `tests\frontend\asc_stmt_ok.bas` (`check-ok` and `ir`: a variable, an element, a
  member, a fixed-length string, with and without the position) and `asc_stmt_errors.bas` (`check-fail`: a number
  as the place, a literal, a string position or value, as `v52` records them); `cargo test -p qb64rust-sema`.
- [x] 3.2 `ir` `validate.rs` and `codegen-cpp` `builtins.rs`: `StmtRule::AscAssign` written as the old compiler's
  inline code (design D4). Verify: `tests\frontend\asc_stmt_cpp.bas` (`cpp`); the `v52` `ASC` programs pass
  (`--glob "v52_x*asc*"`).

## 4. `STOP`

- [x] 4.1 `sema`, `ir`, `codegen-cpp`: `StmtKind::Stop` (design D5), with the expression checked and dropped.
  Verify: `tests\frontend\stop_ok.bas` (`check-ok`, `ir`, `cpp`) and `stop_errors.bas` (`check-fail`: a string);
  the `v52` `STOP` programs pass (`--glob "v52_x*stop*"`).

## 5. `name$(i)` of a fixed-length string array

- [x] 5.1 `sema` `check\places.rs` `find_array` (and the scalar lookup, if `v52` measured it so), as design D7 and
  the recording decide. Verify: `tests\frontend\fixed_string_dollar_name.bas` (`check-ok` and `typed`: the element
  is the declared array's); the `v52` programs of this form pass; `cargo test -p qb64rust-sema`.

- [x] 5.2 Found by tier 1 once the other marks were gone (design D1, "As measured"): a dotted name in a procedure
  beside a main-module `TYPE` variable it does not share (`num.typ` in `const_eval.bas`) is a variable of the
  procedure's own, `check\places.rs` `user_head`. Verify: `verification\v52_x65`, `x66` recorded and passing; the
  delta spec `language/arrays-and-types` has the scenario.

## 6. Reserved names with a suffix

- [x] 6.1 `sema` `check\decl.rs` `reserved`: the rule of design D6 from `v52_builtin_name_suffixes.txt`, with its
  "keep marked" list. Verify: `crates\driver\tests\names.rs` holds the rule to every line of the `.txt`; the `v52`
  programs of `key$`, `exp$`, `base$` pass; tier 1's front-end runs add nothing to
  `tests\known_false_errors.list`.

## 7. The 10 programs and the lists

- [x] 7.1 The 10 `qb64pe/*` programs compiled by the release build: no diagnostic. Then run them with `--suite
  compile --glob "qb64pe/*"`. Each mark or difference that is left becomes a task in this group before it is
  closed. Verify: 10 of 10 pass.
- [x] 7.2 `tests\corpus\slice\s154_qb64pe_sources_builtins.bas` (design D8), recorded with `qb64pe.exe` and put
  on `tests\corpus\slice.list`. Verify: it passes with the new compiler, and tier 1's coverage check finds each
  new row called.
- [x] 7.3 Onto `tests\upstream\pass.list`: the 10 programs. Onto `tests\verification\pass.list` or `known.list`
  (with its reason): every `v52` program. Verify: each moved program passes (`--list` of those moved), and each
  verification program is named once across the two lists (tier 1).
- [x] 7.4 From the review of the change (2026-10-22): `v52_x67`–`x74` recorded. `CONST` of a reserved word with
  any suffix is "Name already in use" (`x32` moves to `pass.list`); `REDIM s$` and, in a SUB, `STATIC s$` beside a
  `STRING * n` variable `s` mean that variable, and `COMMON SHARED s$` there is marked (it had made a second,
  variable-length `s$` without a mark). Front-end tests `const_errors`, `fixed_string_dollar_name` and
  `fixed_string_dollar_errors`. Verify: the eight programs and `x32` pass.

## 8. Close

- [x] 8.1 The wording table regenerated (`tools\diagnostics\old_wording.py`), before the long runs. Then the
  closing runs: `tools\close\close.py --check` passes. Verify: the upstream counts block of
  `tests\upstream\README.md` (written by `tools\legacy_tests\counts.py`) shows 401 of 404, and
  `known_false_errors.list` and `known_parse_gaps.list` are still empty.
- [x] 8.2 Documents. `STATUS.md`'s "Next". `SOMEDAY.md`: a row for each form that `v52` showed but this change left
  marked, and each questionable behaviour kept under "QB64pe behaviours to review". Verify: `openspec validate
  m4-qb64pe-sources --strict` passes.
- [x] 8.3 `python tools\testtime\test_time.py`. Verify: its five files are named among those to stage.

## Workflow follow-up

- Archive the change once the user has reviewed it (`/opsx:archive`).
- Tell the user which files to stage and suggest a commit message (nothing is staged by Claude).
