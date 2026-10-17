# 34. Developing while tests run (2026-10-17)

Written at the user's request after the review fixes of `m3-network-embed`, where the work stopped for about ten
minutes while the slice and upstream lists ran: "how would a professional dev team do this? I feel like we need a
way that we can keep developing while tests run". Section 1 is what such teams do, section 2 what stands in the way
here (measured that day), section 3 the recommendations. Effort is counted in Claude's time (`CLAUDE.md` working
rule 8). **Decided** (2026-10-17, user: "lets do 1-3"; `DECISIONS.md`): items 1 to 3 of §3 are built as
`test-while-building` (`openspec\changes\`, its `design.md` for what differs from the sketch here); item 4 is not
taken.

## 1. What professional teams do

The common thread: **a test run is a job on a fixed input, not a look at the working tree**, so whoever started it
can keep changing the tree.

| Practice | What it means | Where it is common |
|---|---|---|
| **Immutable build artifacts** | A run tests one build, copied or named by its hash when the run starts. Rebuilding the working tree does not touch it, and the result says which build it was | Every CI system; release pipelines |
| **Content-addressed caching** | A build or test result is keyed by a hash of exactly what it reads. An unrelated change, such as a new timestamp, misses no entry; a relevant one always does | Bazel, Buck, Nix, `ccache`, Gradle's build cache |
| **Test impact analysis** | Only the tests whose inputs changed are run; the rest are reported as passed before | Large monorepos (Google's TAP, Meta), via the same content hashes |
| **Tiers by cost** | A fast set on every edit, a slower set before merging, the full set at night or at a merge | Most teams; "pre-submit" and "post-submit" |
| **Background runners** | Long suites run on other machines (CI, a self-hosted runner, a farm); the developer is notified when they finish | Every team with a suite over a few minutes |
| **Separate checkouts** | A second working copy (a `git worktree`) at a fixed commit runs the long suite while work goes on in the first | Individuals and small teams without a farm |
| **Merge queues and batching** | Several changes are tested together before they reach `main`; a failure is bisected among them | GitHub merge queue, Bors, Zuul |

This project already has the tiers (working rule 9: tier 1 while working, tier 2 once per change, the full set at
each merge into `main`), a pass cache and a build cache (a form of impact analysis), batching at the merge
(`study\32` §4 item 2), and runs that may run side by side (`test-parallel-runs`). What it lacks is the first two
rows done fully.

## 2. What stops work here

### 2.1 A run reads the compiler the next build replaces

The closing runs call `target\release\qb64rust.exe` (`close.py --plan`; the runner passes `args.qb64` to every
compile and copies nothing). A `cargo build --release` during a run replaces that file, so the run's later programs
are compiled by another compiler than its first, and the log does not say so. So Claude does not build while a long
run is going: the runs of the review fixes took about 8 minutes (upstream, 347 programs) and 10 minutes (slice
list, 453 programs) side by side, and nothing that needed a build could go on in that time.

`close.py --check` has the same assumption the other way: a run counts for "the current build" when its log is
newer than the executable, not because it names the build it used.

### 2.2 The caches missed everything, for an unrelated reason

The review fixes changed one duplicate check and the text of two errors; no accepted program's C++ changed. With
the caches working as `tools\legacy_tests\README.md` describes, the slice list takes about 24 s when nothing
changed. Instead:

- `--pass-cache: 0 of 453 programs not run` (slice) and `0 of 347` (upstream);
- 763 new entries appeared in `target\build-cache` during the two runs, about one for every program built: the
  build cache missed for nearly all of them, so every program was built again by `make`, and the new executables
  were not byte for byte the old ones, so the pass cache missed too.

**Cause** (found while writing this): the build key (`crates\driver\src\build.rs`, `build_key`) includes the size and
time of every file under the clone's `internal\c` except the `c_compiler` and `temp` folders. That includes
`internal\c\qbx.o`, which `qb64pe.exe` rewrites each time it compiles a program in the clone. The three
`verification\v44_x69`–`x71` recordings, made with `qb64pe.exe` just before the runs, rewrote it (16:26:45), so
every key changed. The new compiler never reads that file: it builds its own `qbx.o` in the program's build folder
and removes any left there (`build.rs`, `build_with_make`). The runner's stamp for its copies of the clone
(`run_legacy_tests.py`, `written_by_builds`) already leaves out `qbx*.o`, the libqb objects and `temp`; the driver's
key does not.

So **any recording with the old compiler throws the whole build cache away**, and every change records with the
old compiler first. This is likely why closes have felt slow even with both caches on. It is a cost that stays
(machine time at every close), not a correctness risk: a missed entry only builds again.

A related, smaller effect: when `qb64pe.exe` builds a new libqb object for a combination of parts it had not
built, that object's stamp also changes every key, although only programs linking that object read it.

### 2.3 What does not stop work

Tier 1 does not use the release binary, so `cargo test` and `cargo build` (debug) could run beside a tier 2 run
if 2.1 is fixed. Documents, specs and tests can be edited during a run already; that is what this session did. Runs
of `qb64pe.exe` in the clone need the clone alone (`tools\legacy_tests\README.md`, "Locks"), so recording a new
measurement still waits for a run of the old compiler, though not for one of the new compiler.

## 3. Recommendations

In order of value for the cost. Items 1 and 2 are one small change; 3 and 4 are optional.

1. **Key the build cache on what `make` reads** (fixes 2.2). Leave out of `build_key`'s stamps what a build of
   either compiler writes in the clone: `qbx*.o`, the `temp` folders (already), and the libqb objects (`*.o`,
   `*.a` outside `c_compiler`). Instead, hash the libqb objects a build links by contents after `make` has built
   them, or, simpler, keep them out and add the clone's libqb sources and headers, from which the objects are
   made. Mirror `written_by_builds` so the two stamps agree, with a unit test that rewriting `qbx.o` keeps the key
   and changing a libqb source changes it.
   *Cost:* Claude's time is small (about 15 minutes with the test). One risk to check: a libqb object rebuilt from
   unchanged sources with other flags. The make flags are already in the key, and the objects' names carry the
   variant, so it does not arise; the test should show it. *Gain:* a close after an old-compiler recording takes
   seconds for the programs it does not touch, instead of ten minutes.
2. **Test a copy of the compiler** (fixes 2.1). At its start a run of the new compiler copies `qb64rust.exe` to
   `target\test-bin\<hash>\qb64rust.exe` and compiles with that copy; its log's first line names the hash.
   `close.py --check` then compares that hash with the current build's, instead of comparing times. Copies older
   than a day are removed at the next run's start. *Cost:* about 20 minutes for both tools with their tests; disk
   about 10 MB a copy. *Gain:* Claude can build, run tier 1 and run front-end tests while tier 2 runs; the only
   thing that still waits is a recording with `qb64pe.exe`, which needs the clone alone.
3. **A worktree for the merge runs** (optional). The full set at a merge into `main` (no pass cache, both suites,
   about an hour) could run in a second checkout at the merge commit (`git worktree add`), so the next change can
   start meanwhile. *Cost:* a second `target` folder (several GB) and its first build; the runs still share the
   clone's lock. Worth it only if merge runs come to block work; with items 1 and 2 they mostly would not.
4. **Not recommended now: CI as the background runner.** The screen programs need a window that GitHub's runners
   do not give (`STATUS.md`, the `tier2` job), and the user has said CI is not a gate (working rule 9). A
   self-hosted runner on the user's machine would solve the window, at the cost of a service to keep running and
   secure. Revisit if work moves to more than one machine.

How Claude works during a run once 1 and 2 are done: start the long run in the background, go on with the next
task (code, tier 1, documents), and read the run's log when it ends (working rule 9, "read a run's output from a
log"). Do not record with `qb64pe.exe` while a run of it holds the clone; batch the recordings before the closing
runs, as this session did.
