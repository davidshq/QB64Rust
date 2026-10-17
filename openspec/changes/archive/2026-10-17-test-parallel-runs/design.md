# Design

## D1. Two locks: the clone's, shared by runs of the new compiler, and the objects', taken by the driver

`study\32` §2 sketched one readers-writer lock with a warm-up: before taking it shared, a run builds the libqb
objects of the flag sets it will need, alone. The runner cannot know those flag sets before it compiles each
program (they follow from the program's metacommands and statements, `make_flags` in
`crates\driver\src\build.rs`), so the warm-up would need a compile of every program first. Instead:

- **The clone's lock** (`target\qb64pe-clone\in-use.lock`) keeps a run of `qb64pe.exe` apart from every other
  run, as before, and is shared by runs that build with `qb64rust` only.
- **The objects' lock** (`target\qb64pe-clone\objects.lock`, named by `QB64RUST_OBJECTS_LOCK`) is taken by the
  driver around each `make`. Under a shared lock it runs `make -q` on a goal it adds with a second makefile in the
  build folder (`qb64rust-objects: $(QBLIB) $(EXE_LIBS)`: everything `exe` takes from the clone, nothing of the
  program's own) with the build's own variables. Exit 0: the objects are up to date and the build only reads them,
  so it keeps the shared lock. Otherwise (1: to be built; 2: `make` could not tell) it gives the lock back, takes it
  alone and builds; `make` builds the missing objects as before.

So nothing writes an object while another build reads it, between runs and within one `--jobs` run, where eight
builds that needed one missing object could race before. A build that the build cache answers runs no `make` and
takes no lock.

Checked by hand (2026-10-17): `make -q` gives 0 for the objects of a built flag set, 1 for a flag set never built,
and 1 with `-W internal\c/libqb.cpp` (the source pretended newer, nothing touched); a build waits while another
holder has the objects' lock alone and goes ahead beside a shared holder. Cost: about 0.12 s per build that runs
`make` (1.80 s against 1.68 s for a small program).

The clone's lock alone still guards `qb64pe.exe`, whose own `make` writes objects without the objects' lock.

## D2. The lock primitive: `LockFileEx` over the whole file

Python's `msvcrt.locking` has no shared mode. `clone_lock.py` calls `LockFileEx` through `ctypes` with offset 0
and length `0xFFFFFFFF_FFFFFFFF`, the range Rust's `File::lock` and `File::lock_shared` take on Windows, so the CLI
tests (`crates\driver\tests\support`, now `try_lock_shared`) and the tools see each other's locks; elsewhere
`fcntl.flock`, as Rust's. Checked: a run of `qb64pe` through the runner stopped with exit code 3 while the CLI
tests held the lock shared, and a corpus run of `qb64rust` went ahead beside them. The folder locks (results,
copies) use the same primitive, exclusive.

## D3. A failure beside other builds runs again alone

With `qb64rust` and `--jobs` above 1 (corpus and compile suites), each program that failed runs once more after
the others, alone in this run, and both outcomes are printed and kept in its detail, by the code that already did
this for `qb64pe` with its copies (`run_again_alone`). A pass alone counts as a pass. It fails the run (exit 1,
with a `note:`) unless the program is **timing-sensitive** (`timing_sensitive`): a screen program, or one whose
source names a word that waits in real time, reads the clock, plays sound or reads the keyboard or the mouse while
it runs (`TIMING_WORDS`). A failure of such a program under load is the known kind (`m10_screen_10`,
`u02_mouse_text`); any other program that passes only alone points at something shared between builds, which is
worth a look. With `qb64pe` every pass alone fails the run, as before.

With `qb64rust` a failure that cannot pass alone is not run again (`SETTLED_STAGES`): the stage `expected` (the
program's expected files contradict each other) and `error result`. With `qb64rust` the latter compares no text:
a program expected to be rejected was not rejected with a real error (`new_compiler_rejection`: an internal error,
only "not supported yet" errors, or an exit without an error summary). The last can be a failed C++ build when the
compiler wrongly accepted the program; alone it then fails with "compiled successfully, expected an error". The
outcome is the same, and only the kept detail may name the build instead of the accepted program. A `compile`
failure is run again: its exit code is also the C++ build's, which is where builds meet, and alone it may pass. With `qb64pe` every failure is run again, as before, since a copy can differ from the clone at
any stage.

The list is wide: the source is searched as text, so a word in a comment counts, and `RANDOMIZE TIMER` makes a
program timing-sensitive. 83 of the 471 corpus programs and 33 of the 404 upstream programs match (2026-10-17). A
fault between builds in one of them does not fail the run; it shows only as its `note:`. Accepted: a narrower list
would fail runs on load alone, and the note names the program either way.

"Alone" is alone in this run: a run beside it that shares the clone's lock may still be building. That is what the
user asked for (the runs side by side) and is said in the README.

## D4. `--pass-cache` with the compile suite

The close runs the upstream pass list with `--pass-cache` (working rule 9, `study\32` §4 item 1). A compile-suite
program that runs is remembered by the same `pass_key`: its executable, source, `.output`, divergence sidecar,
`QB64PE_NOPROMPT` and every file of its category folder, where it runs (a program that writes a file there and
leaves it only makes it run again). A compile-only or error test is never run, so there is nothing to skip.
`--check-list` already refuses a run with `--pass-cache` as a stand-in for a run of a list.
