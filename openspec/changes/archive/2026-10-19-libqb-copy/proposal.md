# Proposal: libqb-copy

## Why

The new compiler builds every program inside the QB64pe reference clone: `make` runs there, reads libqb's
sources there and writes libqb's objects there. That ties every build to a folder this repo does not own, makes
runs of the new compiler share a lock with recordings of `qb64pe.exe`, and leaves nowhere to change libqb when a
fix needs it. The plain copy was decided on 2026-10-02 at the reference clone's commit (2026-10-09) and, by the
tenth review (`study\35` §3 item 1, `DECISIONS.md` 2026-10-18), is the first of the three changes that make the
compiler usable.

## What Changes

- A plain copy of what `make` reads from the clone goes into this repo, in `libqb\`, laid out as in the clone so
  QB64pe's `Makefile` runs there unchanged: `Makefile`, `internal\c\` (every tracked file at commit
  `16f629784e`, no toolchain), `licenses\` (read by `-f:GenerateLicenseFile`) and `COPYING.txt`. Byte for byte
  from the commit's files; `libqb\SOURCE.md` records the commit.
- `tools\libqb\copy_libqb.py` writes the copy from the clone at the commit and, with `--check`, says whether the
  copy still equals it.
- The driver builds against the copy: `make -C libqb`, `qbx.cpp` and `-I` from the copy, libqb's objects written
  in the copy's git-ignored places. The toolchain (`mingw32-make`, `gcc`, `ar`, `windres`, `objcopy`, `nm`) is
  still the clone's, passed to `make` by name. `DECLARE LIBRARY` still looks for files relative to the clone, as
  the old compiler looks relative to its own folder.
- The runner's runs of the new compiler, and the CLI tests that build, no longer take the clone's lock: they only
  read its toolchain, so a recording with `qb64pe.exe` can run beside them. The objects' lock stays, now for the
  copy's objects.
- The build cache's key is taken from the copy (its `qbx.cpp`, `Makefile` and `internal\c`) and the clone's
  toolchain; the clone's `settings\config.ini` leaves it, since `qb64pe.exe` never builds the copy's objects.
- `tools\repo_check` leaves the copy out (third-party bytes, held to the commit by `copy_libqb.py --check`).

No program's behaviour changes: the copy is the same libqb.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `compiler/pipeline`: "C++ output and build" builds with the copy's `Makefile` and libqb and the clone's
  toolchain; neither the clone nor the copy has a tracked file changed by a build.
- `compiler/cli`: "Locating the reference runtime" finds the copy as well as the clone.
- `testing/golden-corpus`: "Runs of the new compiler share the reference clone" (runs of the new compiler take no
  clone lock; the objects' lock is the copy's) and "The build cache keys on what make reads" (the copy's files,
  not the clone's).

## Impact

- New: `libqb\` (about 1,230 files, 27 MB of C and C++ sources, a lasting cost in the repo's size),
  `tools\libqb\copy_libqb.py` with its tests, `.gitignore` and `.gitattributes` lines for the copy.
- Changed: `crates\driver\src\build.rs` (`find_root`, `build_key`, `build_with_make`), `crates\driver\src\main.rs`
  (an option for the copy), the CLI tests' support (`crates\driver\tests\support\mod.rs`),
  `tools\legacy_tests\run_legacy_tests.py` and `clone_lock.py` (no clone lock for the new compiler),
  `tools\repo_check\check_repo.py`.
- Unchanged: the clone (still the reference for every recording, the toolchain and `qb64pe.exe`), the call-site
  check (both sides `-z`, nothing built), CI's `tier2` job (it still has a clone with the toolchain).
