# Proposal

## Why

At the review fixes of `m3-network-embed` the work stopped for about ten minutes while the slice and upstream
lists ran, and both caches missed every program although no accepted program's C++ had changed. `study\34`
found two causes and made four recommendations; the user took the first three (2026-10-17: "lets do 1-3";
`DECISIONS.md`):

1. The build cache's key holds the size and time of every file under the clone's `internal\c`, `qbx.o` among
   them, which every `qb64pe.exe` compile rewrites. So a recording with the old compiler, which every change makes
   first, throws the whole cache away (`study\34` §2.2).
2. A run calls `target\release\qb64rust.exe`, which the next `cargo build --release` replaces, so nothing that
   needs a build can go on while a run does, and `close.py --check` judges a run by its log's time, not by the
   build it used (§2.1).
3. The full set at a merge into `main` (about an hour) could run in a second checkout, but a git worktree had its
   own lock files and so did not see the main checkout's runs (§3 item 3).

## What changes

- **The build key leaves out what builds write in the clone** (`crates\driver\src\build.rs`, `build_key`): the
  libqb objects and archives, `.tmp` files, and `qbx<N>.cpp` and `qbx*.o` in `internal\c`, the same files as the
  runner's `written_by_builds` (design D1).
- **A run tests a copy of the compiler** (`tools\legacy_tests\compiler_copy.py`): the runner and `callsite.py`
  copy `qb64rust.exe` at their start to `target\test-bin\<hash>\` and compile with the copy; their first line
  names the hash and the start; `close.py --check` judges those steps by that hash and start (design D2).
- **A worktree takes the main checkout's locks** (`clone_lock.py`, the CLI tests' support): found from the
  worktree's `.git` file; the merge runs may run in a worktree beside this repo (design D3).
- Documents: `tools\legacy_tests\README.md`, `tools\close\README.md`, `crates\README.md`, `CLAUDE.md` working rule
  9, `DECISIONS.md`, `study\34`, `STATUS.md`.

## What does not change

- The compiler's output: the C++ and every executable are the same; only which builds the cache reuses changes.
- The old compiler's runs: they use the clone as before and are not copied.
- The results files: they still name the compiler as given, with its hash.
- CI: it uses no build cache, and a copy changes nothing it judges.

## Capabilities

- `testing/golden-corpus`: one modified requirement (the clone's lock, from a worktree) and two added (the build
  cache's key, the copy a run tests).
