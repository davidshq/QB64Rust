# Tasks

Order: group 1, then 2, then 3, then 4. Each of groups 2 and 3 ends with `cargo fmt --check`, `cargo clippy
--workspace --all-targets -- -D warnings` and `cargo test` (tier 1) green and with its tests, lists and
`crates\README.md` lines in place, so the change can stop after either group.

Standing rules for every task: work inline, no subagents; nothing staged or committed by Claude: each group ends
by naming the files to stage and a commit message; text with backslashes goes through the Edit and Write tools or
a script file; `python tools\repo_check\check_repo.py --untracked` is clean before a group is reported done.
**Test cadence** (`CLAUDE.md` working rule 9): while working, tier 1 and only the programs the work touches
(`--glob`, `--category`, or a list with `--pass-cache target\pass-cache`), each run's output sent to a log under
`target\`; tier 2, the run without folding and the full suites run once, in group 4. A behaviour of the old
compiler that looks questionable is implemented its way and listed in `SOMEDAY.md` ("QB64pe behaviours to
review").

## 1. Measurements

- [x] 1.1 Measure with `qb64pe.exe` before the proposal (`verification\v34_*`, 28 programs, recorded 2026-10-14):
  `v34_a` numbers, `v34_b` the other kinds of element, `v34_c` the places of an array, `v34_d` a FUNCTION in a
  bound that gives the same array new bounds (exit 127), `v34_x01`–`x24` one accepted or rejected form each.
  Verify: every program has its `.compile.txt` and `.out.txt`.
- [x] 1.2 Record the facts: a `v34` paragraph in `study\00` §5 (the word order, the coordinate copy, the member
  array, `v34_d`), `v34*` in `CLAUDE.md`'s layout table. Verify: `check_repo.py --untracked` is clean.

## 2. The mode and the words

- [x] 2.1 `AllocMode::Retain`, `Declarer::Redim { mode }`, `redim_member` on "keeps the elements", the IR's
  documentation of `Op::Allocate` (design D1). Verify: `tests\frontend\redim_retain_ir.bas` (`ir`) shows
  `Allocate Retain`; a member array with `REDIM _RETAIN` gives no operation, as with `_PRESERVE`.
  Done 2026-10-14: "keeps the elements" is `AllocMode::keeps_elements`.
- [x] 2.2 The words after `REDIM` (design D2): `_RETAIN` and `_RETAIN SHARED` accepted, every other sequence an
  error at its second word. Verify: `tests\frontend\redim_retain_errors.bas` (`check-fail`) with each form of
  `v34_x03`–`x05`, `x13`, `x22` and `_PRESERVE _PRESERVE`; `redim_unsupported.bas` without its two `_RETAIN`
  lines and its snapshot redone. Done 2026-10-14; `SHARED SHARED` added ("stands twice").

## 3. The coordinate copy

- [x] 3.1 `allocate_with` under `Retain` (design D3): the old descriptor kept, the overlap, the walk, the payload
  of each kind. Verify: `tests\frontend\redim_retain_cpp.bas` (`cpp`) with an array of each kind and one of
  three dimensions; the C++ of `v34_a` and `v34_b` (`-z`) compared with the old compiler's, the `REDIM _RETAIN`
  blocks equal apart from `static`.
  Done 2026-10-14. The `_RETAIN` lines of `v34_a`, `v34_b` and `v34_c` (declarations, the descriptor copy, the
  overlap, the walk, the offsets of two and three dimensions, every payload) are equal apart from `static` and
  three differences that `_PRESERVE` here has already: `preserve_any=0` stands at its declaration in place of
  the old compiler's `else` branch; the walk is written once, where the old compiler writes it in both its
  `cmem` and its plain branch (this compiler makes the new block in either and copies after them); a user
  type's parts are spelt with fewer parentheses (same addresses, sizes and calls). Built and run, `v34_a`–`c`
  and the 12 accepted `x` programs print their recorded output; the rejected `x` programs are rejected.
- [x] 3.2 Slice programs `s112_redim_retain` (numbers) and `s113_redim_retain_kinds` (strings, fixed-length
  strings, bits, user types, a procedure, a static array, conventional memory, a `_MEM` block), recorded with
  `qb64pe.exe`, on `slice.list`. Verify: both pass with the new compiler (`--glob`).
  Done 2026-10-14: both passed on their first run; `slice.list` 424.
- [x] 3.3 Upstream: the seven `_RETAIN`-only programs leave `deferred.list`; those that pass join `pass.list`;
  `array_elements_change` leaves `tests\known_unsupported_rejections.list`; the demand report names no
  `REDIM _RETAIN` outside the `TYPEFIELDS` programs. Verify: the seven run by themselves; tier 1's list tests
  pass.
  Done 2026-10-14: all seven pass (`array_elements_change` by its error, as the `.err` programs already on the
  list), so all seven joined `pass.list` (193). **Found**: with `_RETAIN` no longer marked, `ArrayCopyTest_3`
  (a `TYPEFIELDS` program) got a real error at `REDIM _RETAIN holder4D.values(…)`: a `_DYNAMIC` written before
  a member's name (`_Dynamic values(…) As Long`) stays a token of the `TypeField`, outside the `FieldName`,
  and `sema` read only the `FieldName`'s, so the member was taken as a fixed member array. `TypeField::modifier`
  (`syntax\src\ast.rs`) now gives it, and the member is "not supported yet" as one with the word after its name.
  The demand report over `tests\upstream` names no `REDIM _RETAIN`.

## 4. Close

- [ ] 4.1 Closing runs, once (working rule 9): tier 1 with the command-line tests that build, the runner's tests,
  the slice list with and without folding, the upstream and differential pass lists, the two full suites, the
  demand reports, the shrink-only lists regenerated (the call-site check is not affected: no row changes; say so).
  Verify: numbers in `STATUS.md`; nothing on a pass list fails.
- [ ] 4.2 Documents: `STATUS.md`, `ROADMAP.md`, `crates\README.md`, `tests\upstream\README.md`, `SOMEDAY.md`
  (the `_RETAIN` row goes; `v34_d` among the behaviours to review), `CLAUDE.md`'s layout table;
  `python tools\testtime\test_time.py`. Verify: `check_repo.py --untracked` is clean; the files to stage and a
  commit message are given to the user.

## Workflow follow-up

- Archive the change (its specs merged) after the user has committed it; then the second change,
  `$UNSTABLE:TYPEFIELDS` (`DECISIONS.md` 2026-10-14).
