# Tasks

Standing rules: `CLAUDE.md`. No delta spec (`skip_specs`): no output changes. Built before
`m4-handlers-logging-checking`, which was planned but not started (2026-10-21), so the two diffs stay apart. Scripts that find or rewrite sites live in the scratchpad,
not in the repo.

## 1. The base for the comparison (design D8)

- [x] 1.1 On the base build (before any edit of this change), the three lists of the closing set run with
  `--build-cache target\build-cache --pass-cache target\pass-cache` (the commands of `close.py --plan` for the
  slice list, the upstream pass list and `tests\verification\pass.list`), in the background while group 2
  starts. Check: each log's last line; the names of any program that failed are written in this task, as the only
  ones allowed to run again at the close.
  **Result (2026-10-21, base `4b9078d`):** slice 475 of 475 pass, upstream 374 of 374, verification 2756 of 2756;
  none failed, so at the close every program of the three lists must be "not run".

## 2. The numeric-type macros (design D1)

- [x] 2.1 `numeric_types!()` and `numeric_value_types!()` in `crates\sema\src\lib.rs` beside `place_only_types!`,
  each with a doc comment that names its variants and, for the second, why `_BIT` is missing; re-exported by
  `crates\ir\src\lib.rs`. Check: `cargo build -p qb64rust-ir`.
- [x] 2.2 Each arm in `sema`, `ir` and `codegen-cpp` that names every numeric type (or every one but `_BIT`), in
  any order, uses the macro; `numbers!` (`codegen-cpp\src\decl.rs`) and `numeric!` (`codegen-cpp\src\mem.rs`)
  are deleted. Check: the scratchpad script finds no list of the 21 of 2026-10-21 left (number of sites changed
  written here); `cargo test` passes with no `.snap` file in `git status`; `cargo clippy --workspace
  --all-targets -- -D warnings` is clean.
  **Result:** 19 arms rewritten (the script found 21 lists; the other two were the bodies of `numbers!` and
  `numeric!`), and the 14 uses of the two private macros moved to the shared ones (`numeric!() | … | Ty::Bit {
  .. }` became `numeric_types!() | …`). In `ir` the macros are used as `crate::…!()`, as `sema` does. The script
  finds no list left; tier 1 passes with no snapshot changed; clippy is clean.

## 3. `clear_temp` once (design D2)

- [x] 3.1 `clear_temp` in `tools\legacy_tests\clone_lock.py` (with `run_legacy_tests.py`'s comment on
  `temp.bin`), deleted from `run_legacy_tests.py` and `tools\callsite\callsite.py`, imported by both. Check:
  `grep -rn "def clear_temp" tools` finds one; `python -m unittest` in `tools\legacy_tests` passes (`test_run_legacy_tests.py`,
  `test_clone_lock.py`); `python tools\callsite\callsite.py --help` starts.

## 4. Smaller repeats in the compiler (design D3–D6)

- [x] 4.1 `flat_index_in` in `crates\codegen-cpp\src\place.rs`, used by `flat_index` and `dyn_element_desc`.
  Check: `cargo test -p qb64rust-driver` with no `.snap` file changed (the `cpp` snapshots).
- [x] 4.2 `Shape::is_array`, `dim_count`, `bounds` in `crates\sema\src\lib.rs`; `Var`'s three methods in
  `sema` and `ir` delegate. Check: `cargo test`.
- [x] 4.3 `ir` re-exports `sema`'s `ProcKind` and `RuntimePart` (its own deleted, the comment giving the
  2026-10-07 reason), and `crates\ir\src\lower.rs` copies them without converting; a `DECISIONS.md` row
  (2026-10-21 or the day it is built: the IR shares `ProcKind` and `RuntimePart`, extending the 2026-10-07
  decision; `ir::Resume` stays its own, its `To` being an IR label). Check: `cargo test` with no `.snap` file
  changed; `crates\driver\src\build.rs` builds unchanged.
- [x] 4.4 `as_words` and `split_unsigned` in `crates\sema\src\check\mod.rs`, used at the six and four sites of
  D6; `new_proc` in `crates\sema\src\check\proc.rs`, used by `declare_proc` and `declare_broken`. Check: `grep -rn
  "type_words().map" crates\sema` finds only `as_words`; `cargo test` with no `.snap` file changed.

## 5. The screen-state tests (design D7)

- [x] 5.1 One helper in `crates\driver\tests\cli.rs` for the build with `screen_state.cpp` and the run with a keys
  file; the five screen-state tests use it, each keeping its program, keys, prompt time, comments and assertions.
  Check: `cargo test -p qb64rust-driver --test cli -- --ignored` (the CLI run of the closing set, task 6.3,
  counts for this check if nothing in `cli.rs` changes after it).
  **Result:** two functions, `build_with_screen_state` and `run_screen_state` (design D7 says why two); all five
  tests pass in the closing CLI run (46 passed).

## 6. Close

- [x] 6.1 `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings`.
- [x] 6.2 `STATUS.md`'s "Next": this change done.
- [x] 6.3 The closing runs (`python tools\close\close.py --plan`, then `--check`). Check, beyond `--check`: the
  slice, upstream and verification logs report every program "not run: passed before" but those named in 1.1
  (design D8); a program that ran again is explained here before the change closes.
  **Result (2026-10-21):** `--check` passes on every step (tier 1 404 passed; slice 475, upstream 374,
  verification 2756 pass; coverage PASS; call-site 2023 of 2023 equal; CLI 46 passed). Every program that
  builds an executable was "not run", so each executable, and its C++, is unchanged. The ones that ran are the
  kinds the pass cache never skips, having no executable: expected compile errors and compile-only programs
  (slice 1 compile-only; upstream 43 errors and 16 compile-only; verification 1710 errors), the same counts as
  the base runs of 1.1, all passing. No run without folding: no conversion or constant evaluation changed, only
  how the patterns of `match` arms are written.
  **Rerun (2026-10-21)** after the review's fixes (two more `split_unsigned` sites, doc comments): `--check` passes
  again with the same counts, and the same programs ran (every executable unchanged).
- [x] 6.4 `python tools\testtime\test_time.py`.
- [x] 6.5 `tools\testtime\test_time.py`, found by the review of the record: calls sent together count the time
  they cover once (fmt sent beside `cargo test` had counted both); a verification run with the new compiler is a
  test, not a measurement (since `verification-compare`); the coverage check and the wording check are on record.
  Check: `python -m unittest discover -s tools/testtime -p "test_*.py"` with a case for each (47 pass, one
  for a run that waited for earlier calls being left out of what its kind takes alone); the record written
  again.
