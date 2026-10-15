# Tasks

Order: group 1, then 2 to 5 in order, then 6. Each of groups 2 to 5 ends with `cargo fmt --check`, `cargo clippy
--workspace --all-targets -- -D warnings` and `cargo test` (tier 1) green and with its tests, lists and
`crates\README.md` lines in place, so the change can stop after any of them.

Standing rules for every task: work inline, no subagents; nothing staged or committed by Claude: each group ends
by naming the files to stage and a commit message; text with backslashes goes through the Edit and Write tools or
a script file; `python tools\repo_check\check_repo.py --untracked` is clean before a group is reported done.
**Test cadence** (`CLAUDE.md` working rule 9): while working, tier 1 and only the programs the work touches
(`--glob`, `--category`, or a list with `--pass-cache target\pass-cache`), each run's output sent to a log under
`target\`; tier 2, the run without folding and the full suites run once, in group 6. A behaviour of the old
compiler that looks questionable is implemented its way and listed in `SOMEDAY.md` ("QB64pe behaviours to
review"), unless the user decided otherwise (D-019, D-020, D-004 widened).

## 1. Measurements and decisions

- [x] 1.1 Measure with `qb64pe.exe` before the proposal (`verification\v35_*`, 69 programs, recorded 2026-10-14):
  `v35_a` a member of a variable, `v35_b` members of array elements and the parent's `REDIM`/`ERASE`/`SWAP`/copy,
  `v35_c` every kind of element, `v35_d` procedures, `v35_e` statements and whole-array files, `v35_f` `_STATIC`
  and unmarked members beside `_DYNAMIC`, `v35_g`–`g3` runtime errors, `v35_h` the metacommand, `v35_x01`–`x59`
  one accepted or rejected form each. Verify: every program has its `.compile.txt` and `.out.txt`.
- [x] 1.2 Record the facts and decisions: a `v35` paragraph in `study\00` §5 (the flag's position, the markers,
  the descriptor and its life, the layout, files, the parent's place, the three misuses); `v35*` in `CLAUDE.md`'s
  layout table; `DECISIONS.md` rows (the user's three answers of 2026-10-14: one change for the family, measured
  before the design, the three misuses fixed); `DIVERGENCES.md` D-019 (`GET` into a member with other bounds),
  D-020 (a member too large) and D-004 widened (a store through a parent index outside). Verify:
  `check_repo.py --untracked` is clean.

## 2. The metacommand and the markers

- [x] 2.1 `$UNSTABLE` (design D1): the parser takes the metacommand; the block walk records `type_flags`; the
  errors of the other forms; the warning of `MIDI` and `HTTP` with `-w`. Verify: `tests\frontend\unstable_forms.bas`
  (`check-fail`, with the forms of `v35_x10`–`x12`, `x14`, `x45`) and a `check-ok` program with `v35_h`'s and
  `x13`, `x15`, `x47`'s forms; a CLI test for the warning.
- [x] 2.2 The markers (design D2): every position the old compiler reads, `Member::dynamic`, the errors; `_STATIC`
  compiles as an unmarked member; a `_DYNAMIC` member stays "not supported yet" until group 3. Verify:
  `tests\frontend\typefields_markers.bas` (`check-fail`: `x05`–`x09`, a marker without the flag; `x60`, a marker
  before an `AS type` list, measured and added by the review of the group);
  `member_arrays_errors.bas` without its marker lines and its snapshot redone; `v35_f` without its `_DYNAMIC`
  member and `x34`, `x40`, `x44`, `x53` compile and give their recorded output.
- [x] 2.3 Upstream: the programs of `deferred.list` that need only the flag and `_STATIC` (the `.err` programs
  `t028`–`t044`, `t233`, `t234`, `t_qb64pe_refactor_inline_reset_*`; each checked by itself) leave it, and those
  that pass join `pass.list`. Verify: they run by themselves; tier 1's list tests pass.

## 3. `_DYNAMIC` members of variables and elements

- [ ] 3.1 Owner types and the layout (design D3): `owns_desc`; `ty_size`, `member_offset` with the aligned layout
  for owner types; `file_layout`. Verify: `tests\frontend\typefields_cpp.bas` (`cpp`) shows `x52`'s sizes and
  offsets; every existing `cpp` snapshot is unchanged.
- [ ] 3.2 The IR (design D5): `Allocate` and `Release` take an `ArrayRef`, validation of a `Member` there, the
  dumps. Verify: `tests\frontend\typefields_ir.bas` (`ir`) shows `Allocate Retain` and `Release` of a member;
  existing `ir` snapshots unchanged.
- [ ] 3.3 The descriptor and the life functions for numbers and `STRING * n` (design D4, D6 first three rows,
  D9, D10): `udt_new_N`, `udt_free_N`, `udt_clone_N`; variables, `STATIC` locals, locals at each return; `REDIM`,
  `_PRESERVE`, `_RETAIN`, `ERASE` of a member through `allocate_with` and `release`; elements, `LBOUND`, `UBOUND`
  through the guard; the member as an array argument; the parent's place evaluated once. Verify: `v35_a`, `v35_d`,
  `v35_g`, `x01`–`x04`, `x17`, `x21`, `x26`, `x29`–`x33`, `x36`, `x38`, `x58`, `x59` print their recorded output
  (`x58` 3. by D-004); the `cpp` test of 3.1 gains the functions of one owner type.
- [ ] 3.4 Slice programs `s114_dyn_members` (`v35_a`, `v35_g`, `x58`) and `s115_dyn_member_procs` (`v35_d`),
  recorded with `qb64pe.exe` (the line of D-004 recorded from a program without it), on `slice.list`. Verify: both
  pass (`--glob`).

## 4. Owner values in arrays, copies and every kind of element

- [ ] 4.1 Arrays of an owner type (design D6, the array rows): new elements, plain `REDIM`, `_PRESERVE` and
  `_RETAIN` moving kept elements, `ERASE` of a dynamic and of a static parent, `SWAP`. Verify: `v35_b`, `x27`,
  `x28`, `x49`, `x51` print their recorded output.
- [ ] 4.2 Strings and nested owners in the life functions: `_DYNAMIC` members of `STRING`, of user types with
  strings and of owner types; `STRING` members of owner types. Verify: `v35_c`, `v35_e` 1.–4., `x42` print their
  recorded output.
- [ ] 4.3 `CopyWhole` of an owner type (design D7): read `AppendDynUDTDescCopy` (`type.bas` 10), measure an owner
  type nested in another (one more `v35_x` program), write the same evaluations. Verify: `x56` and the new
  program print their recorded output; `x57` too; the evaluation order fixed by a `cpp` or `typed` test.
- [ ] 4.4 Slice programs `s116_dyn_member_parents` (`v35_b`, `x51`) and `s117_dyn_member_kinds` (`v35_c`, `x56`,
  `x57`), recorded with `qb64pe.exe`, on `slice.list`; `SOMEDAY.md` gains the behaviours to review (`ERASE` of a
  static parent, the evaluations of D7). Verify: both pass (`--glob`).

## 5. Files and the rejected forms

- [ ] 5.1 `PUT` and `GET` of a whole array of an owner type (design D8): pack, unpack, the bounds check, D-019.
  Verify: `v35_e` 5.–6. print their recorded output apart from D-019's line (`q(0).v` keeps its element);
  `x55` its own; a CLI or slice test of D-019.
- [ ] 5.2 The forms the old compiler rejects (design D4 last sentence): `LEN`, `_OFFSET`, `COMMON`, `PUT`/`GET` of
  one value, a whole array of a type with a `STRING`, `REDIM` without `TO` or with another type. Verify:
  `tests\frontend\typefields_rejected.bas` (`check-fail`) with `x18`–`x20`, `x39`, `x41`, `x43`, `x46`, `x48`,
  `x50`, `x54`; `_MEM` of an owner value and of a member (`x22`–`x24`, `x52`) compile and print their output.
- [ ] 5.3 A slice program `s118_dyn_member_files` (`v35_e`, `x55`; D-019's line recorded from a program without
  it), on `slice.list`. Verify: it passes (`--glob`).
- [ ] 5.4 Upstream: the rest of the 93 programs leave `deferred.list`; those that pass join `pass.list`; one that
  fails is fixed or, if it needs more than this design, put to the user; the demand report over `tests\upstream`
  names no `$UNSTABLE` or marker. Verify: the 93 run by themselves; tier 1's list tests pass.

## 6. Close

- [ ] 6.1 Closing runs, once (working rule 9): tier 1 with the command-line tests that build, the runner's tests,
  the slice list with and without folding, the upstream and differential pass lists, the two full suites, the
  demand reports, the shrink-only lists regenerated (the call-site check is not affected: no row of the built-in
  table changes; say so). Verify: numbers in `STATUS.md`; nothing on a pass list fails.
- [ ] 6.2 Documents: `STATUS.md` (also the stale line on `m3-asserts-debug-zlib`), `ROADMAP.md`,
  `crates\README.md`, `tests\upstream\README.md`, `SOMEDAY.md` (the `TYPEFIELDS` row goes), `CLAUDE.md`'s layout
  table; `python tools\testtime\test_time.py`. Verify: `check_repo.py --untracked` is clean; the files to stage
  and a commit message are given to the user.

## Workflow follow-up

- Archive the change (its specs merged; `language/unstable-features` becomes a main spec) after the user has
  committed it. Then what `DECISIONS.md` 2026-10-14 leaves: whole-array assignment and `_ARRAYCOPY` stay deferred
  unless the user takes them; the plain copy of `internal\c` when libqb must change.
