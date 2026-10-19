# Tasks

Standing rules: `CLAUDE.md`.

## 1. The copy (design D1, D6)

- [x] 1.1 `tools\libqb\copy_libqb.py` writes `libqb\` from the clone's commit blobs (`Makefile`, `internal\c`,
  `licenses`, `COPYING.txt`) and `libqb\SOURCE.md`; `--check` lists differing, missing and extra files, ignoring
  git-ignored ones. Tests in `tools\libqb\test_copy_libqb.py` on a small git repo made by the test: a fresh copy
  checks clean, an edited, a removed and an added file are each named, a built `.o` is not.
- [x] 1.2 The copy written at `16f629784e`; `.gitattributes` (`libqb/** -text`) and `.gitignore` (`*.o`, `*.a`,
  `*.tmp` under `libqb/`) lines; `copy_libqb.py --check` exits 0, and `git status --porcelain libqb` shows only
  new files, none of them git-ignored ones.
- [x] 1.3 `check_repo.py` leaves `libqb\` out except `SOURCE.md` (a test in its own tests, if it has them, else
  checked by a run): `check_repo.py --untracked` reports 0 findings with the copy in place.

## 2. Building against the copy (design D2, D3, D4)

- [x] 2.1 `find_libqb` and `--libqb-root` / `QB64RUST_LIBQB_ROOT` (`build.rs`, `main.rs` with its usage line); looked
  for only when an executable is built. Unit test of the three places and the error's wording; a CLI test that a
  build with `--libqb-root` naming an empty folder fails with status 1 naming the option and the variable, and
  that `-z` there succeeds.
- [x] 2.2 `build_with_make` runs `make -C <copy>` with the clone's toolchain in `AR`, `CC`, `CXX`, `OBJCOPY`,
  `WINDRES`, and `qbx.cpp` and `-I` from the copy. Checked by building `tests\corpus\slice\s01*` with the build
  cache off: it runs, its libqb objects appear under `libqb\internal\c` and none is written or touched in the
  clone (`git status --porcelain` in the clone unchanged, the clone's objects' times unchanged).
- [x] 2.3 `build_key` from the copy, the clone's `config.ini` out of it, version 4; the build-key unit tests on the
  copy's layout, `compiler_settings` and its test removed.
- [x] 2.4 The CLI test of "Reference clone unchanged" also checks `git status --porcelain libqb` in this repo
  (spec scenario "The copy unchanged").

## 3. Locks (design D5)

- [x] 3.1 The runner takes the clone's lock only for `qb64pe.exe`; a run of `qb64rust` takes none and still names
  the objects' lock. Tests in `test_run_legacy_tests.py` (or `test_clone_lock.py`) for both cases.
- [x] 3.2 The CLI tests that build stop taking the clone's lock (`crates\driver\tests\support\mod.rs`); they keep the
  objects' lock.
- [x] 3.3 Checked for real: a short `qb64pe.exe` verification run (one family, re-recording nothing that changes)
  goes ahead while a `qb64rust` slice run is going.
- [x] 3.4 Documents of the locks: `tools\legacy_tests\README.md` ("Locks"), `clone_lock.py`'s docstring,
  `CLAUDE.md` working rule 9's line that runs of the new compiler share the clone's lock.

## 4. Close

- [x] 4.1 `crates\README.md` (the build: the copy, the clone's toolchain, `--libqb-root`), `DECISIONS.md` 2026-10-09's
  pointer for moving the reference commit (also `copy_libqb.py`), `CLAUDE.md`'s layout table (a `libqb\` row and a
  `tools\libqb\` row).
- [x] 4.2 The closing runs: `tools\close\close.py --check` passes. Every executable changes, so the pass cache runs
  every program; the first build in the copy builds all of libqb's objects.
- [x] 4.3 `python tools\testtime\test_time.py`; `STATUS.md` "Next" (the first of the three "After M3" changes done).
