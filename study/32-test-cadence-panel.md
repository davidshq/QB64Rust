# 32. Test cadence and parallel runs: panel notes (2026-10-17)

Written during the close of `m3-events`, after the user asked why the closing runs take so long, what each of
them is, why runs cannot share the machine, and what the best practice is (user, 2026-10-17: "it feels like we
are doing too small batches of changes to be making all these tests run every time. we probably need a better
way? what is best practices?"; "Is there something we should do about this inability to compile in parallel? it
seems kind of silly"). The panel is role-played inline (`CLAUDE.md` working rule 6) and counts effort by working
rule 8. Nothing here is decided: the open questions are in §5, and their answers go into `DECISIONS.md`.

## 1. The closing runs, and what they cost

Plain-language definitions of the terms (slice list, folding, without folding, differential pass list,
call-site check, tier 1 and tier 2): `GLOSSARY.md`, section "Testing".

What working rule 9 asks at a change's close, and what it took at the close of `m3-events` (release build,
`--jobs 8 --build-cache target\build-cache`, one run after another):

| Run | Programs | Time | What it can find that the others do not |
|---|---|---|---|
| tier 1 (`cargo test`), fmt, clippy | 338 tests | about 2 min | parser, IR and C++ snapshots, the coverage checks, the shrink-only lists |
| CLI tests that build (`--ignored`) | 37 | 25 s | the divergence traces, builds with settings |
| full corpus suite (slice list judged from it) | 471 | about 12 min, plus about 5 min of a run stopped and started again | every recorded program end to end, the screen programs |
| full upstream suite (upstream pass list judged from it) | 404 | about 5 min | QB64pe's own tests |
| slice list without folding | 446 | about 8 min | a constant worked out by `sema` that differs from the same value at run time |
| differential pass list | 59 | about 1 min | numeric operators on every pair of types |
| call-site check | about 1,860 | about 1 min with its cache | the libqb call of every built-in form, nothing built |

About 35 minutes of machine time. Measured over all sessions (`tools\testtime\REPORT.md`, 2026-10-17): waiting for
test runs is 18% of active time (17 h 58 min of 97 h 27 min), and the closes are its largest block.

Why so slow: each program of the three end-to-end runs is compiled to C++, built by the C++ compiler and run (5 to
50 s each, 8 at a time); the event and sound programs wait in real time; the run without folding changes the C++
of almost every program, so the build cache misses; and the runs cannot overlap (§2). The wasted run was a mistake
of order: the wording table (`tools\diagnostics\old_wording.py`) was regenerated after the full corpus had started,
which changed the compiler, so that run no longer counted for `--check-list` (§4, item 6).

## 2. Why runs of the new compiler cannot overlap, and what to do

**What happens now.** Every run of the test runner, `tools\callsite\callsite.py` and the CLI tests that build take
one lock, `target\qb64pe-clone\in-use.lock` (`tools\legacy_tests\clone_lock.py`), so a second run waits or stops.
The eight copies under `target\qb64pe-copies\` (about 800 MB each) are used only by runs of the old compiler with
`--jobs`, which compile there.

**What a build of the new compiler shares with the clone** (`crates\driver\src\build.rs` `build_with_make`): its
fragments, its copy of `qbx.cpp`, `qbx.o` and the executable are in its own build folder, and the clone's
`Makefile` is given that folder's `temp` as `PATH_INTERNAL_TEMP` (the `Makefile` sets it with `:=`, lines 52 and
89, which a command-line value overrides). What it shares is **libqb's object files** under the clone's
`internal\c`, which `make` builds the first time a set of build flags needs them and reuses after (553 objects
today). Within one run, eight builds already share those objects at the same time.

**So the lock is stricter than it needs to be** for runs of the new compiler. It matters for the old compiler
(`qb64pe.exe` empties and fills the clone's own `internal\temp`), and for the first build of a set of flags, where
two `make`s could write the same object at once.

**Panel.**

- *Build and test-infrastructure engineer:* make the lock a readers-writer lock. A run that only builds with the new
  compiler takes it shared; a run that uses `qb64pe.exe` in the clone (the verification suite, the call-site
  check's old side, a corpus run of the old compiler without `--jobs`) takes it exclusive. Before taking it shared,
  a run builds the libqb objects of the flag sets it will need (one small program per set, which `make` turns into
  the missing objects) under the exclusive lock, so the shared part never writes into the clone. Then the full
  corpus, the full upstream suite and the run without folding run side by side: the close waits about as long as
  the slowest of them (about 12 min), not their sum (about 25 min).
- *Pragmatic engineer:* simpler first step with the same effect: give each run of the new compiler its own copy of
  the clone (`--qb64pe-root`, the copies exist), so nothing is shared at all. The cost that stays: the first run in
  a copy builds its libqb objects (a few minutes, once per copy and flag set) and the copies' disk. A run that
  wants a copy takes that copy's lock (`lock_folder` exists for the copies).
- *Rust engineer:* either way the driver needs no change; the runner decides the root and the lock. The CLI tests
  (`crates\driver\tests\support`) take the same lock with `File::try_lock`; they would take it shared too.
- *Test engineer:* keep one rule: a run may never read a clone or copy while another writes libqb objects there.
  Both designs keep it. The runner's own tests need cases for the shared and exclusive paths.
- *QB64 engineer:* the copies must stay at the reference clone's commit (they are remade when `qb64pe.exe`, the
  `Makefile` or `internal\c` change, `tools\legacy_tests\README.md`), so a copy can never test against another
  libqb than the clone's.

**Recommendation.** The readers-writer lock with an exclusive warm-up of the objects: no extra disk, no first-run
cost per copy, and the old compiler's runs keep their guarantee. A small change of its own (runner, `clone_lock.py`,
the CLI tests' lock, their tests, `tools\legacy_tests\README.md`, `CLAUDE.md` working rule 9).

## 3. Which tests to run, and when: the panel

- *Pragmatic engineer:* a change here is one feature, a few hours of work, 4 to 30 new test programs; its close
  runs about 1,400 programs, nearly all untouched by it. The full suites find something new only where the change
  touched shared code, and tier 1 already guards that code with snapshots. The project pays for full certainty at
  every close where certainty at each merge to `main` would do.
- *Test engineer:* do not drop coverage, move it into tiers: a fast set on every edit (tier 1, as now), a set chosen
  by what the change touched at its close, everything at each merge or on a schedule. The selected set mostly
  exists: the build cache gives an unchanged program a byte-identical executable, and `--pass-cache` skips a
  program whose executable did not change. Closing runs forbid the pass cache because a result that depends on time
  or load can hide (`m10_screen_10`, `CLAUDE.md` working rule 9); that is a reason to run without it at the merge,
  not at every close.
- *Compiler/languages engineer:* the run without folding is the clearest case for selection: it can differ from the
  normal run only where `sema`'s constant evaluation or a numeric conversion differs from run time. `m3-events`
  touched neither and the run cost 8 minutes. Run it when a change touches `sema`'s constants or conversions, and
  at each merge. The differential list is about a minute; not worth selecting.
- *QB64 engineer:* what catches regressions here is the screen programs (timing, the runtime), the corpus (real
  programs) and the call-site check (cheap, broad). Keep the call-site check at every close, plus the change's own
  programs and those that use its part of libqb. Programs that depend on timing (sound, timers, the mouse) fail
  now and then under load (`u02_mouse_text` in the run without folding at this close, 445 of 446): they need a
  rule to run again alone, in the runner, not more runs.
- *Rust engineer:* tier 1 already holds every step's parser, IR and C++ in snapshots. When a change alters only one
  statement's path, a full end-to-end close mostly proves again the C++ compiler and libqb, which did not change.
  The trigger for the corpus should be the generated C++: if a change alters the C++ of programs outside its own
  tests, those programs run; the build cache already knows which.
- *Build and test-infrastructure engineer:* two mechanical gains whatever the policy: runs side by side (§2), and one
  closing command that reads the changed files and picks the runs, so the choice is not made by hand each time; it
  must start each run by its own command so that `tools\testtime` still records it (working rule 9; `study\31`
  item 3 left this open until item 4 records child runs).
- *Pragmatic engineer, summing up:* the changes are small (19 steps of M3 in six days). Either batch the full runs
  or select per change; doing both full and per change is the cost the user feels.

## 4. Best practice, and the proposal

**Practice elsewhere.** Run the tests a change can affect at each change (test impact analysis: Bazel and similar
build systems choose tests by their dependencies; large companies' presubmit systems choose tests per change), run
everything less often (at each merge, or nightly), keep tests that depend on time or load apart with a rule to
retry them alone, and run independent suites in parallel, not only the programs within one suite.

**The panel's proposal.**

1. **At a change's close:** tier 1 with fmt and clippy; the CLI tests that build; the change's own new programs
   and its `verification` family; the call-site check; the slice and upstream pass lists **with `--pass-cache`**
   (only programs whose executable changed run); the run without folding **only when** `sema`'s constant evaluation
   or conversions changed; the demand reports, the shrink-only lists and the wording table as now.
2. **At each merge of `m3` (or `changes`) into `main`, every two or three changes:** today's full closing set, no
   caches: both full suites with the pass lists judged from them, the run without folding, the differential list,
   the call-site check. A failure there is bisected over the two or three changes with the build cache.
3. **Parallel runs** (§2): the readers-writer lock with a warm-up of the libqb objects.
4. **Timing failures:** the runner runs a failed program of a timing-sensitive kind (screen, sound, timers) again
   alone and reports both results, instead of a person doing it by hand.
5. **The close's order:** everything that changes the compiler (the wording table, generated tables) before the
   first full run, so a run is never discarded for a new build.

**The risk it takes:** a regression in a program the change did not seem to touch is found at the merge, up to two
or three changes later, and still before `main`.

## 5. Open questions for the user

1. **Cadence:** adopt §4 items 1 and 2 (a lighter close, the full set at each merge), keep working rule 9 as it is,
   or something between (for example the run without folding only when relevant, the rest unchanged).
2. **Parallel runs:** build §2's readers-writer lock now, as a small change before the next M3 step, or later.
3. **Timing failures:** add §4 item 4 to the same change, or not.

When answered: a row in `DECISIONS.md`, `CLAUDE.md` working rule 9 updated, and this section marked with the
answers.
