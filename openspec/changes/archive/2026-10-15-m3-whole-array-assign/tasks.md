# Tasks

Order: group 1, then 2, 3, 4, 5. Each of groups 2 to 4 ends with `cargo fmt --check`, `cargo clippy --workspace
--all-targets -- -D warnings` and `cargo test` (tier 1) green and with its tests, lists and `crates\README.md`
lines in place, so the change can stop after any of them.

Standing rules for every task: work inline, no subagents; nothing staged or committed by Claude: each group ends
by naming the files to stage and a commit message; text with backslashes goes through the Edit and Write tools or
a script file; `python tools\repo_check\check_repo.py --untracked` is clean before a group is reported done.
**Test cadence** (`CLAUDE.md` working rule 9): while working, tier 1 and only the programs the work touches
(`--glob`, `--category`, or a list with `--pass-cache target\pass-cache`), each run's output sent to a log under
`target\`; tier 2, the run without folding and the full suites run once, in group 5. A behaviour of the old
compiler that looks questionable is implemented its way and listed in `SOMEDAY.md` ("QB64pe behaviours to
review"), apart from the two divergences of the proposal.

## 1. Measurements

- [x] 1.1 Measure with `qb64pe.exe` before the proposal (`verification\v36_*`, 58 programs, recorded 2026-10-15):
  `v36_a` numbers, `v36_b` the other kinds of element, `v36_c` the places of an array, `v36_d` member arrays,
  `v36_e` owner types, `v36_f` the run-time errors, `v36_x01`–`x52` one accepted or rejected form each.
  Verify: every program has its `.compile.txt` and `.out.txt`.
- [x] 1.2 Record the facts: a `v36` paragraph in `study\00` §5 (the form, the checks, the run-time checks, the
  copy by kind, member arrays and the index outside, owner types and `x52`, suffixed names), `v36*` in
  `CLAUDE.md`'s layout table, the `DECISIONS.md` row (the user's choice of the change and the two answers of
  2026-10-15). Verify: `check_repo.py --untracked` is clean.

## 2. The form, the checks and the operation

- [x] 2.1 `StmtKind::CopyArray`, `Op::CopyArray` with its documentation, lowering, dump and validation (design
  D3). Verify: `tests\frontend\whole_array_ir.bas` (`ir`) shows `CopyArray` for a variable pair, a member to a
  variable and a dynamic member; an `ir` validation unit test for types and dimensions that differ.
  As built: the dump reads `CopyArray <from> -> <to>:<type>`; the test also has the source in parentheses after
  `LET`, members of elements, an owner type and an array parameter; the unit test
  (`copy_between_arrays_of_another_type_or_shape`) also names a scalar. `copy_array` in `codegen-cpp` was
  written with this task, as the build needs every operation emitted (its check is task 3.1's).
- [x] 2.2 The form in `assign` (design D1): both sides whole, the source in parentheses, `LET`, suffixed names
  marked or rejected, each side resolved as an `ArrayRef`, target first. The mark "whole-array assignment of
  member arrays" goes. Verify: `v36_a`–`v36_e` and the accepted `x` programs pass `qb64rust` with no
  diagnostic (`--report unsupported` over `verification\v36_*` names only the suffixed forms).
  As built: a module of its own, `sema\check\whole.rs`. The report over `v36_*` names the suffixed forms (6
  programs) and two marks that are not this change's: `PRINT a()` (`x37`, element 0; design D1 keeps it marked)
  and `v36_a`'s literal `18446744073709551610` ("unsigned literals", met with `m3-console-files`). `x25` (a scalar
  and an array of one name, both typed by `DIM … AS`) compiles and is accepted.
- [x] 2.3 The checks and the error table (design D2), the dimension facts of parameters. Verify:
  `tests\frontend\whole_array_errors.bas` (`check-fail`) with each rejected form of `v36_x*`; the eight upstream
  `.err` programs get a real error and leave `tests\known_unsupported_rejections.list`; `x17`, `x39` accepted,
  `x45` rejected.
  As built: the table is applied where the whole side is an array variable `name()`; a whole member array beside
  a side of another form keeps its earlier errors (`t028`–`t044`). The other side is resolved to tell an element
  or a bare member array (both sides need parentheses) from anything else (the whole side's error).
  `DimFact::Same` has `copied`, for its message. `arrays_errors.bas`'s `x() = 1` now gets the real error (its
  snapshot and comment changed). Beyond the task: the 12 accepted upstream programs that now compile cleanly were
  run by themselves (`--list`, all 20 with the `.err` ones pass) and moved from `deferred.list` to `pass.list`
  here, as tier 1 asks for a list for every clean program (task 5.1's list part).

## 3. The copy of array variables

- [x] 3.1 `copy_array` (design D4) for array variables of every kind: the checks, numbers and bytes, `STRING`,
  user types with strings, owner types. Verify: `tests\frontend\whole_array_cpp.bas` (`cpp`); the C++ of `v36_a`
  (`-z`) compared with the old compiler's, the whole-array blocks equal apart from the names of temporaries;
  built and run, `v36_a`, `v36_b`, `v36_c`, `v36_f` and the accepted `x` programs print their recorded output.
  As built: `v36_a` does not compile here (its suffixed parts 7–9 are marked, its literal `18446744073709551610`
  too), so the check ran on a copy cut before part 7 with that literal made `1844674407370955161`, in a scratch
  folder: its 18 whole-array blocks (`-z`, `#line` lines dropped, array names replaced) are equal to the old
  compiler's, and both builds print the same. `v36_b`, `v36_c`, `v36_f`, `x14`, `x17`, `x20`, `x25`, `x27`, `x28`,
  `x30`, `x31`, `x36`, `x39` print their recorded output. Beyond the design: a dynamic member array's descriptor
  must also have the copy's number of dimensions (slot 3), which a `REDIM` can change (the old compiler reads the
  slots of the declared number); not measured.
- [x] 3.2 Slice program `s119_whole_array`, recorded with `qb64pe.exe`, on `slice.list`. Verify: it passes with
  the new compiler (`--glob`). It passed on its first run.

## 4. Member arrays and owner types

- [x] 4.1 Member arrays on either side (a `passN` descriptor, a dynamic member's own), target first, nothing
  copied after an index on the way raised (`DIVERGENCES.md` D-004 widened). Verify: `v36_d`, `x33`, `x46`–`x49`
  print their recorded output; `x48` prints this compiler's (no copy), shown in a CLI test.
  As built: written with task 2.1 (`whole_desc`); `x50` prints its recorded output too. `x48` prints ` 0 []`
  where the old program prints ` 0 [z]`: CLI test `whole_array_to_a_member_through_an_index_outside`;
  `DIVERGENCES.md` D-004 widened.
- [x] 4.2 Arrays of owner types: each element by `copy_owner`, in place; other bounds copy nothing
  (`DIVERGENCES.md` D-022, with its CLI test from `x52`). Verify: `v36_e`, `x51` print their recorded output.
  As built: written with task 2.1. `x52` prints ` error 9  9  0` and ` error 9  8  0  0` where the old program
  copies (`4  5`, `4  5  0`): CLI test `whole_array_of_an_owner_type_with_other_bounds`; `DIVERGENCES.md` D-022.
- [x] 4.3 Slice program `s120_whole_array_members` (member arrays, a dynamic member, owner types below the flag;
  the lines of the two divergences left out), recorded, on `slice.list`. Verify: it passes. It passed on its
  first run; it also holds `x46`, `x47` (inline and `_DYNAMIC` member arrays of an owner type), `x49`–`x51`.

## 5. Close

- [x] 5.1 Upstream: the 16 programs leave `deferred.list` (`t001` stays, named for "a plain scalar beside an array
  typed by `DIM … AS`"); those that pass and the eight `.err` programs join `pass.list`; the demand report over
  `tests\upstream` names no whole array but `t001`'s other feature. Verify: they run by themselves; tier 1's list
  tests pass. The lists moved with task 2.3 (all 20 passed run by themselves); 310 of 399. The report names no
  whole-array assignment; the whole arrays it still names are `LEN(a())` (`arrays/size`,
  `test_mem_vs_len_classic`, `types/udt_array`; `LEN` of a whole member array in `test_mem_vs_len`), another use.
- [x] 5.2 Documents: `SOMEDAY.md` (the whole-array row goes, the suffixed form and the marks left by the change
  are added), `DIVERGENCES.md` (D-004 widened, D-022), `crates\README.md`, `tests\upstream\README.md`,
  `ROADMAP.md`, `STATUS.md`. Verify: `check_repo.py --untracked` is clean. Also `tests\corpus\README.md` (431 on
  `slice.list`, 127 slice programs) and `CLAUDE.md`'s layout table (456 corpus programs, 431, "x of 399", the 5
  of `deferred.list`).
- [x] 5.3 Closing runs, once (working rule 9): tier 1 with the command-line tests that build, the runner's tests,
  the slice list with and without folding, the upstream and differential pass lists, the two full suites, the
  demand reports, the shrink-only lists regenerated (the call-site check is not affected: no built-in changes; say
  so); `python tools\testtime\test_time.py`. Verify: numbers in `STATUS.md`; nothing on a pass list fails; the
  files to stage and a commit message named. Tier 1 (311) was not run again: it ran at the close of group 4,
  after the last change to the compiler, and only documents changed after it. The command-line tests that build
  27, the runner's 73, the slice list 431 of 431 with and without folding, upstream `pass.list` 310 of 310,
  differential 59 of 59, the full corpus 450 pass (+ `218_fre`, 5 known failures), the full upstream suite 310
  pass, 93 rejected, 1 known failure, none wrong; the lists unchanged after group 2's.
  After the closing runs, the review of the change found two errors of the guard and fixed them: a value in
  parentheses (`n = (a())`, `a() = (b(1))`) now gets the error it gets without them, and a SUB as the source is
  named a SUB (measured, `verification\v36_x53`–`x55`; three lines of `whole_array_errors.bas`). Rerun: tier 1
  (311), fmt and clippy. Not rerun, as the fix changes only which error a rejected statement gets: the slice and
  pass lists, the full suites and the CLI tests.
