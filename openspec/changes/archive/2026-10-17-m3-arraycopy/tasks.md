# Tasks

Standing rules: CLAUDE.md. Differently here: the measurements of group 1 were taken before the proposal
(`verification\v41_*`, 87 programs, 2026-10-17).

## 1. Measurements and records

- [x] 1.1 Record what was measured: the facts of the proposal into `study\00` §5 (a `v41` paragraph: the six
  forms, the kinds, the places, the order of evaluation, the run-time errors, the compile errors and `x75`, the
  two faults), the family into `verification\README.md`, a row in `DECISIONS.md` (the step; the two divergences
  chosen by Claude for the user to confirm), D-004 widened and D-025 in `DIVERGENCES.md` (`v41_j`, `v41_i`).
  Verify: `python tools\repo_check\check_repo.py --untracked` is clean.
  - Done: the `_FLOAT` row is D-025, not the D-023 the proposal named: `DIVERGENCES.md` says D-023 and D-024
    (`m3-declare-library`, not built) are not reused; the artifacts were changed to match.

## 2. Front end

- [x] 2.1 `syntax`: an unexpected `TO` in an `_ARRAYCOPY` item is a plain syntax error (design D2). Verify: `x04`
  and `x35` get errors that are not "not supported yet"; a parser test; `cargo test -p qb64rust-syntax`.
  - Done: also `x02` (a second `TO` after the destination), which was "not supported yet: word `TO`" the same
    way; it is "one `TO` stands between the source and the destination of `_ARRAYCOPY`".
- [x] 2.2 `sema`: `check\arraycopy.rs` with the form (D1), the helper `member_path_array` shared with
  `redim_member`, the checks in the old compiler's order (D2), the checks left for `solve_dims` (D3), and
  `StmtKind::ArrayCopy`; the "not supported yet" goes. Verify: `tests\frontend\arraycopy_errors.bas`
  (`check-fail`, every rejected form of `v41_x*`) and a `check-ok` test of the accepted ones pass; `x75`'s
  `never` and `x76` are rejected, its `cp` accepted.
- [x] 2.3 `ir`: `Op::ArrayCopy`, its lowering, validation (with unit tests of each problem) and dump (D4). Verify:
  `tests\frontend\arraycopy_ir.bas` (`ir`) passes.
  - Done: D4's "`to` not a constant array" has nothing to check: the IR has no constant arrays.

## 3. Code generation

- [x] 3.1 `codegen-cpp`: `arrays.rs` `array_copy`, the entry points, the element sizes with `_FLOAT` by
  `sizeof(long double)`, the guard for member sides (D5). Verify: `tests\frontend\arraycopy_cpp.bas` (`cpp`)
  passes; `cargo test`, fmt and clippy green.
- [x] 3.2 Call-site programs `tests\callsite\arraycopy_*.bas` (D6). Verify: `python tools\callsite\callsite.py
  --glob "arraycopy*"` reports every one equal.
- [x] 3.3 Slice programs `s131_arraycopy` and `s132_arraycopy_members` recorded with `qb64pe.exe` (`--record`) and
  added to `tests\corpus\slice.list`; CLI tests `arraycopy_through_an_index_outside` and `arraycopy_of_float_arrays`
  with this compiler's output. Verify: both slice programs pass (`--glob`); the CLI tests pass (`--ignored`).
- [x] 3.4 `verification\v41_*` built and run with this compiler; upstream `arrays/ArrayCopyTest`, `_1`, `_2`, `_3`
  run, added to `tests\upstream\pass.list` and taken out of `deferred.list`. Verify: every `v41` program prints
  what the old compiler's does but for the lines of the two divergences, every rejected one is rejected; the four
  upstream programs pass (`--glob`).
  - Done: the 26 accepted programs recorded with `qb64pe.exe` in a scratch corpus root and run with this compiler:
    22 pass; `v41_b` (its `_FLOAT` line), `v41_i` (`_FLOAT`) and `v41_j` (D-004) differ on those lines only,
    `v41_h` differs by the D-004 trace alone (its 9. and 10.). `v41_x62` is not run: its `$CHECKING` is marked by
    itself. The 60 rejected programs are each rejected, with one error. The four upstream programs pass.
  - Found: a FUNCTION in a bound, after an error on the way to a member array, is entered with the error pending
    and runs no line, in both compilers (their functions' first line); design D5 said "still called".

## 4. Close

- [x] 4.1 The closing runs (working rule 9), the shrink-only lists, the demand reports and
  `tools\diagnostics\old_wording.py` regenerated, the counts by `tools\legacy_tests\counts.py`. Verify: every run
  passes or fails only by known failures.
  - Found: `old_wording.py` kept 3 of the new messages only. Its test for "text of one program" took `_A` out of
    the old wording's `_ArrayCopy` as a name in capitals; it now takes whole words in capitals only, as its
    docstring says. 34 more patterns are kept: 28 of `_ARRAYCOPY` and 6 of earlier changes (`_MEM cannot
    reference a TYPE value that contains explicit _Dynamic member arrays` for `GET`, `PUT`, `LEN`, `_OFFSET` and
    `COMMON` of an owner value; the `_DEPTHBUFFER` and `_PRINTMODE` syntax references), each the old compiler's
    recorded wording, agreed by every pair; `--check`: 795 notes, all the recorded message.
- [x] 4.2 Documents: `STATUS.md`, `ROADMAP.md`, `SOMEDAY.md` (the `_ARRAYCOPY` row goes; the questionable behaviours
  of the proposal). Verify: `openspec validate m3-arraycopy --strict` passes.
- [x] 4.3 `python tools\testtime\test_time.py`. Verify: its five files are named among those to stage.

## 5. After the review

- [x] 5.1 A `_DYNAMIC` member that a `REDIM` gave another number of dimensions (the review's finding 1): measured
  (`verification\v41_l`: error 9 and nothing copied, every form, both sides), the check in `arrays.rs`
  `array_copy` (design D5), slice program `s133_arraycopy_dynamic_dims` recorded with `qb64pe.exe`; the spec's
  run-time requirement and the IR's documentation say it. Verify: `s131`–`s133` and upstream `ArrayCopyTest*`
  pass (`--glob`); `cargo test`, fmt and clippy green.
  - Not run again: the call-site check (its programs have no member side, whose C++ alone changed), the CLI tests
    that build (no `_DYNAMIC` member), the full suites.
- [x] 5.2 The review's smaller findings: `STATUS.md`'s line on `m3-versioninfo` (committed, `3c93e14`), design D4
  and D5 brought to the code, `old_wording.py` takes whole words in capitals only (no part of a word by
  backtracking). Verify: `old_wording.py` writes the same table, and `--check` passes.
- [x] 5.3 The review of 5.1 and 5.2: a reshaped `_DYNAMIC` side beside a member side whose place raised took the
  D-004 trace, though the old compiler copies nothing there either (its stand-in from `UDTDynArrayDescGuard` is
  not allocated); the trace now waits on `array_copy_dims`. The old guard named in design D5 and `study\00` §5.
  Verify: CLI test `arraycopy_reshaped_member_beside_an_index_outside` passes (`--ignored`), with the two other
  `arraycopy_*` ones; `s131`–`s133` and upstream `ArrayCopyTest*` pass; `cargo test`, fmt and clippy green.
  - Checked with `qb64pe -z` (a scratch copy, the clone clean before and after) and `--dump cpp`: a FUNCTION in
    the index of a plain member side runs before error 9 of the reshaped side in both compilers.
  - Found by the review of 5.3: the trace writes a row once per run, so the test's first version, which also held
    a statement that takes D-004, passed without the fix. It now holds the two statements with a reshaped side
    alone and expects no row; with the fix reverted it fails (`D-004` taken), with the fix it passes.

## Workflow follow-up

- Put the two divergences (D-004 widened, D-025) to the user with the change; archive it when the user has looked
  at it.
