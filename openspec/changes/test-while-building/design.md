# Design

## D1. The build key: libqb's sources, not its objects

`build_key` stamped every file under the clone's `internal\c` except the `c_compiler` and `temp` folders, so that
"a changed file of the clone or a rebuilt libqb object gives a new key". Among them was `internal\c\qbx.o`, which
`qb64pe.exe` writes at each compile and the new compiler never reads (it builds its own `qbx.o` in the program's
build folder). After `verification\v44_x69`–`x71` were recorded, every key had changed: 763 cache misses in the
two closing runs (`study\34` §2.2).

The key now leaves out every file a build of either compiler writes there: `.o`, `.a` and `.tmp` files anywhere,
and `qbx<N>.cpp` and `qbx*.o` in `internal\c` itself (`written_by_builds`, the rule of the runner's
`written_by_builds`, which its stamp for the copies of `qb64pe` already used). What a libqb object is made from
is still in the key: its sources and headers by size and time, and the make flags, the clone's `Makefile` and the
toolchain's executables as before. The object built per set of the program's flags carries the set in its name
(`libqb_make_00100100.o`, 17 of them in the clone on 2026-10-17), so two objects that `make` would build
differently do not share a name; the other 535 objects (`libqb\src`, the parts) are built from their sources
alone, as `make` itself assumes when it reuses them.

A libqb object rebuilt from unchanged sources with other compiler options is half covered (review of
2026-10-17). `qb64pe.exe` takes its options from the clone's `settings\config.ini` (`ExtraCppFlags`,
`OptimizeCppProgram`), and the key holds that file's `[COMPILER SETTINGS]` section (`compiler_settings`; only
that section, as `qb64pe.exe` rewrites the file at each run; unit test
`build_key_follows_the_compiler_settings_only`). Not covered, and accepted: the same options given for one
compile on the command line (`-f:`, which the runner's `--cpp-opt` passes) never reach the file. `make` rebuilds
an object only when it is missing or older than its sources, so this needs the objects removed (a purge of the
clone) and then rebuilt by such a compile; a cache entry from before would link the old objects' code. The
build cache's folder is safe to delete (`tools\legacy_tests\README.md`) after a purge of the clone.

Checked (2026-10-17): five slice programs built with `make` (3.0 s each, the keys being new), then one compiled
with `qb64pe.exe` (its `qbx.o` rewritten), then the five again: each from the cache, 0.4–0.6 s. Unit test
`build_key_leaves_out_what_builds_write`.

## D2. The copy a run tests

At its start, after its locks, a run of `qb64rust` copies the executable it is given to
`target\test-bin\<first 16 hex digits of its SHA-256>\qb64rust.exe` (written under a `.part` name and renamed, so a
run beside it never runs half a file; reused when it is there with the same hash) and compiles with that copy.
The driver finds the reference clone from the folder it was built in, a path fixed at build time
(`build::find_root`), so the copy finds the same clone; the compiler root it takes from its own folder when no
`--include-root` is given holds no included file in `target\release` either.

The first line of the output is `[compiler] <path as given> sha256 <hash> started <seconds since 1970>`. The
results file names the compiler as given, so `counts.py`, which requires the release build at its path, reads
it as before. `callsite.py` does the same.

`close.py --check` judges the steps that run a copy (slice, upstream, the run without folding, the call-site
check) by that line: stale when the hash is not the current build's, or when the run started before a changed
input that does not reach it through the build (one under `tests\` or the test tools; one under `crates\` or the
`Cargo` files reaches it only through the build, and the `build` step is judged against those). Before, a run
that started on one build and ended after the next counted for the new build, and one rebuilt with the same bytes
was stale. A log without the line (from before this change) is judged by its time.

Copies not used for a day are removed at the next run's start; one still running cannot be removed on Windows
and stays for a later run. About 10 MB each.

## D3. A worktree takes the main checkout's locks

The clone's locks were under each checkout's `target\`, so a worktree's run of `qb64pe` would not wait for the
main checkout's runs. `clone_lock.py` (`main_checkout`) and the CLI tests' support (`main_checkout`) now read the
checkout's `.git`: a directory means the main checkout; a file `gitdir: <main>/.git/worktrees/<name>` names a
folder whose `commondir` file leads to the main `.git`, and the locks are under its parent's `target\`. Only the
lock files move; a worktree keeps its own build, results, pass cache and copies.

The worktree must sit beside this repo (`..\QB64Rust-merge`), since the driver, the runner and `callsite.py`
look for the clone as `..\QB64pe` of their checkout. The commands are in `tools\legacy_tests\README.md`, "The
runs at a merge into `main` in a worktree". Nothing is automated: the merge runs are every two or three changes,
and a worktree costs several GB of `target\` while it exists.
