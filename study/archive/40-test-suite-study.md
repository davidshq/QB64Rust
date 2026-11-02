# 40. The test suite: as it is, as it has been, as it should be (2026-10-30)

Written at the user's request after the close of `desktop-sandbox` and the `_SCREENCLICK` fix (user, 2026-10-30:
"I'm still not confident that you have done a good job with the test suite. Step back and do a full study of the
test suite as it is, as it has been, as it should be … we want thorough testing but also you are wasting huge
amounts of time on every single little change"; "consult a panel of specialists, don't make unnecessary
recommendations, if it's already right that's fine, we just need to double-check"). The panel is role-played
inline (`CLAUDE.md` working rule 4) and counts effort in Claude's time.

**Method.** Every number here is measured, not remembered: the test-time record was refreshed into a scratch copy
under `target\` (the committed `tools\testtime\` files are untouched, since the record is written when the user
asks), the last close's logs under `target\close\` were read, and the tests were counted from the tree. Practice
elsewhere comes from the sources in §4. Nothing here is decided; §7 lists what the user is asked.

**The short answer.** The suite's shape is right and nothing in it should be cut. The cost the user feels does not
come from the closing set, which takes about ten minutes of wall-clock time with warm caches and runs in the
background; it comes from *how many closes happen*: eight closes' worth of list runs on 2026-10-30 alone, for work
that went into one branch, plus three cold-cache incidents this week that each cost 20 to 35 minutes. The rules
were restated five times in four weeks and the tool now decides which steps a change reaches (2026-10-30), which is
the fix practice elsewhere uses. One thing remained: a close per *batch of commits* rather than per OpenSpec
change (§6, R1; adopted, §7). A guard for the tool's reach table (R2) was dropped as redundant. §6 also says what
stays as it is, so that this review does not become a sixth restatement.

## 1. As it is

### 1.1 The layers

| Layer | What | Size | Time (last close, warm caches) | What it alone can find |
|---|---|---|---|---|
| **Tier 1** `cargo test --workspace` | unit tests of eleven crates; 459 insta snapshots of parse trees, IR and C++ (`crates\driver\tests\snapshots\`); `inputs.rs` (49 tests) runs every program of `tests\` through the front end in-process, checks the known lists are exact, every supported built-in has a call-site program; `mutate.rs` seeds mutants through parser, `sema` and C++ generation; the language server, the DAP proxy, the formatter against `tests\format\` (1,349 files) | 816 tests (driver 339, syntax 119, sema 91, dap 77, lsp 62, format 47, ir 35, base 26, codegen-cpp 15, builtins 12, difftest 6) | about 1 min incremental, 3 to 4 min after a `sema` change; runs on every push in CI (Windows and Linux) | a parser, checker, IR or C++ change in any program, before anything is built |
| **CLI tests that build** `--ignored` | 100 of `cli.rs`'s 154 tests: builds, the divergence traces, the debugger over `lldb-dap`, `qb64rust test` | 96 ran | 1 min | a build with settings, the debugger |
| **The lists** (`--pass-cache`, `--build-cache`, side by side) | the slice list of the corpus (546), the upstream pass list (404), the verification pass list (4,118) judged against QB64pe's recordings | 5,068 programs, every one compiled by the new compiler each run; only those whose executable or expected files changed are *run* | slice 2.5 min, upstream 2 min, verification 4.5 min, in parallel; cold: 20, 20 and 34 min | end to end: libqb, the C++ compiler, run-time behavior |
| **Compile-only suites** | QB64pe's converter test (`addprefix`, 1), QB64pe's qbasic programs (143) | 144 | 1.5 min | a program that stops building |
| **Call-site check** | the C++ call of every built-in form against the old compiler's, nothing built | 2,650 programs | 2 min with its cache | a wrong libqb call for a built-in no program runs |
| **Coverage check** | Civil War Strategy's scripts held to line coverage | 10 scripts | 1 min | a game path that stopped being reached |
| **Format runs** (when reached) | QB64pe's format tests (24), the format oracle (1,344) | 1,368 | under 1 min | the formatter |
| **Run without folding** (when reached) | the slice list with `QB64RUST_NO_FOLD=1` | 497 | about 8 min cold | a constant `sema` folds differently from run time |
| **Linux family** (when the build changed) | sync to WSL, tier 1, the slice list judged by the Linux recordings | 691 tests + 472 programs | about 3 min warm | a Linux-only fault |
| **Sandbox family** (when reached) | the seven desktop built-ins in Windows Sandbox, judged against QB64pe's builds | 7 programs | about 1 min, takes the screen | the desktop built-ins nothing else runs |
| **At a merge into `main`** only | the three full suites with no pass cache, the differential list, the pass lists judged from full runs, the built-in counts, the wording table, the demand reports | about 1,400 programs run | 35 to 60 min, in a worktree | a result that depends on time or load; the reports |
| **Tools' own tests** | `unittest` of the Python tools (runner, close, test-time record, coverage, wording, OpenGL, QB 4.5, libqb copy, sandbox, repo check) | a few hundred | seconds each | the tools' rules |
| **The extension** | the VS Code extension's tests (`npm`) | | 12 s median | the editor |

The closing set as `close.py --plan` printed it on 2026-10-30 came to 19 steps; with warm caches they took from
19:04 to 19:13 wall-clock for everything but the sandbox (which took the screen for a minute at 20:10). All of it
except tier 1, fmt and clippy runs in the background while work goes on (`study\archive\34`).

### 1.2 What the suite rests on

- **An oracle, not opinions.** Every end-to-end expectation is a recording of QB64pe (`verification\`,
  `tests\corpus\`, `tests\upstream\`), or of QB 4.5 under DOSBox where QB64pe's own result is in doubt. A difference
  is a bug in one of the two until measured otherwise (`QB64PE-DIVERGENCES.md`, `UPSTREAM.md`).
- **Three kinds of check at once** (the compiler-testing literature's three: §4): golden snapshots of each stage,
  end-to-end runs against the reference, and differential runs of operators across type pairs
  (`tests\differential\`).
- **Caches keyed on bytes.** The build cache keys an executable on the C++ emitted; the pass cache keys a run on
  the executable, source and expected files. So an unchanged program costs a compile of its source by the new
  compiler and a hash, and the compiler is still exercised on all 5,068 programs every close.
- **The tool decides.** `close.py --plan` prints the steps as commands; `--check` judges each log: missing, stale
  (older than a changed input that reaches it), failing, a cache that missed, a program on no list. Since
  2026-10-30 a step no changed file reaches is a comment and its last log stands (`REACH`, `builtins:` lines of
  libqb edits; `tools\close\README.md`, "What a changed file reaches").
- **Timing-sensitive programs run again alone** on failure (`run_legacy_tests.py`, since `test-parallel-runs`,
  2026-10-17), and the three known races are written down with their causes (`STATUS.md`, "Known bugs": the game
  script's `SHELL` under Windows Terminal, the clipboard watcher, `lldb-server`).

### 1.3 Where the time goes, measured

From the refreshed record (544 sessions, 2026-10-02 to 2026-10-30, 9,955 runs):

| | Whole project | 2026-10-28 to 10-30 |
|---|---|---|
| Active time | 201 h 35 min | 20 h 18 min |
| Waiting for test runs | 41 h 13 min (20%) | 4 h 28 min (22%) |
| Test runs in the background, not waited for | 59 h 48 min | 4 h 50 min |
| Runs repeated with nothing edited | 535 runs, 3 h 44 min (133 after a pass, 1 h 29 min) | 21 runs, 17 min (2 after a pass) |

The share waited has been between 17% and 27% every day since 2026-10-11, through every change of the rules
(§2). What changed is what the waiting is made of: the repeats after a pass, which `study\38` named, have all but
stopped (2 in three days); the long runs moved to the background; and the closes got more frequent.

**2026-10-30 in detail** (eight sessions, 13 h of active time counted, one branch, `misc-updates`):

| Kind | Runs | Time |
|---|---|---|
| slice list | 8 | 31 min |
| verification pass list | 5 | 50 min (one of them the 4,121-program cold run, 34 min) |
| upstream pass list | 4 | 27 min |
| tier 1, whole workspace | 10 | 21 min |
| tier 1, one crate | 21 | 16 min |
| call-site check | 4 | 12 min |
| tests of the Python tools | 35 | 5 min |
| repo check | 16 | 7 min |

The three lists ran together four times (11:50, 13:28, 16:03, 18:55): four closes in a day, for `v65` with the
member-array fix, `basic-unit-tests`' leftovers, `desktop-sandbox`, and the `_SCREENCLICK` edit with the
`close.py` reach work. Each was a close as the rule defines it (once per change, at its close). Each cost about
ten minutes of machine time with warm caches and a few minutes of the user's attention. Together they are the "all
the time" the user sees: the rule is per change, and the changes are now a few hours long.

**The outliers that feel like 25 minutes** are cold caches, not the set: on 2026-10-30 at 11:50 the verification
list ran all 4,121 programs (34 min) because `verification\qb45\` was taken as support of every program (fixed the
same day, `DECISIONS.md` 2026-10-30); on 2026-10-27 a verification run sat in the background for 15 h 36 min (a
hung program, never waited for); the close of 2026-10-30 at 18:55 was started for a libqb edit that reached only
the sandbox, then stopped (the 25 minutes of the user's message; it is why `REACH` exists).

## 2. As it has been

| Date | Rule or tool | What it fixed | What it did not |
|---|---|---|---|
| 2026-10-03 | `study\archive\19`: four tiers; tier 1 on every edit, a slice before a commit, the full corpus in CI, everything nightly | set the shape: front end in-process, programs end to end | assumed CI would carry tier 3; it could not (no window on the runner), so the full runs came back to the machine |
| 2026-10-12 | tier 2 once per change at its close; CI not a gate; weekly tier 2 | the slice list had run five times in one change, two tasks waited on CI | "once" was kept by memory; the closing set grew to about 35 minutes of serial runs |
| 2026-10-17 | `study\archive\32`, `33`, `34`: a lighter close with the pass cache, the full set at a merge, the readers-writer lock so runs overlap, rerun alone on a timing failure, `close.py` as the plan and the check, work goes on during a run | the serial 35 minutes became parallel background runs; the set was no longer worked from memory | the set was still "everything, cached", so a cold cache cost the full time; the rule text kept growing |
| 2026-10-28 | `study\38` §5.3: the close thinned (no reports, no counts, no wording table, no record), weekly CI stopped, the cadence moved from `CLAUDE.md` to `tools\close\README.md` | about 5 minutes and three steps a close; the reports to the merge | the user's message of 2026-10-30 came two days later: the per-close cost was not the whole of it |
| 2026-10-30 | the set stays one fixed set; `--check` guards the pass cache (`CACHE`); runs before the close go to the plan's logs | the 4,121-program run is now named, not silent; runs are not made twice to two logs | |
| 2026-10-30 | `REACH`: a changed file makes stale only the steps it reaches; libqb edits name their built-ins; a stopped run accepted with a reason | a `tests\sandbox\` or `_SCREENCLICK` change no longer runs every list | the table is hand-kept (§5, F6) |

Three things stand out from the history. First, every restatement responded to a real waste, measured, and each
removed the waste it named; the test-time record shows the repeats after a pass going from 133 to 2. Second, the
share of time waited did not move, because each time the saving went into more closes, more lists and more
programs (the verification list grew from 0 to 4,118 programs between 2026-10-19 and 2026-10-30). Third, the user
had to notice each time: the rule said "once", "only what the change touches", and Claude's sessions did not hold
to it until the tool printed it (`DECISIONS.md` 2026-10-30, "the three repeats of 2026-10-12, 2026-10-14 and
today").

## 3. Practice elsewhere

Read for this study (links at the end):

- **Compiler testing** uses three oracles: golden files of each stage's output (rustc's `ui` and `codegen` suites
  under `compiletest`, blessed with `--bless`; insta's `.snap` files here), end-to-end runs whose expected
  output comes from a reference implementation (differential testing against a "golden reference compiler", the
  pattern of the 1998 patent and of Csmith's users), and comparing the same program under different settings
  (optimization levels; here folding on and off). The guides agree that end-to-end runs find miscompilations the
  stage tests miss, and that stage tests make a failure cheap to locate. rustc's own full suite takes hours; its
  guide tells contributors to run one suite or one file.
- **Test sizes and flakiness.** Google's record: 0.5% of small tests (one process), 1.6% of medium (one
  machine), 14% of large tests (several machines, real time) are flaky, and binary size and RAM predict it. The
  project's three races are all in its largest tests (a game script under `SHELL`, the clipboard, a debugger over a
  server process). The accepted practice is to rerun, quarantine and record the cause, not to delete the test.
- **Selection before merge, everything after.** Google's presubmit runs the tests a change can reach by the
  build graph's reverse dependencies and defers the rest to postsubmit (Elbaum, Rothermel and Penix, FSE 2014:
  selection "greatly improves the cost-effectiveness of pre-submit testing" while deferring few faults); Bazel's
  CI runs presubmit on pending changes and the full set postsubmit and nightly; Teamscale's guide on impacted-test
  selection says to run the full suite at regular intervals because selection cannot guarantee every failing test
  is picked; a 2025 industrial paper on a merge queue (BMW) keeps selection out of the final gate so the gate stays
  a fail-safe. Module-level selection is what industry adopted; finer selection costs more analysis than it saves.
- **The pyramid.** Fowler: broad-stack tests are brittle, "an enhancement to the system can easily end up breaking
  lots of such tests, which then have to be re-recorded", so keep more of the narrow ones; the inverted shape
  gives slow CI and poor failure localization.

Against this, the project's design is the standard one: snapshots at each stage (narrow, 816 in tier 1), the
programs against a reference (broad, cached), selection by reach before a commit, the full set at a merge, the
flaky kinds rerun alone and written down. The one place it differs is that "presubmit" here is a close that can
happen several times a day on one branch, where the practice runs it once per change list sent for review.

## 4. The panel

- *Pragmatic engineer:* the set is not the problem; ten minutes in the background is fine. Four closes in a day
  on one branch is the problem, and that is a cadence question for the user, not a tool question. Do not add
  steps, do not remove steps, do not write the rule a sixth time. Say what a "change" is for the purpose of a
  close.
- *Test engineer:* the pyramid is the right way up (816 narrow tests, 5,068 cached broad ones, 7 that take the
  screen). Every broad run has an oracle. The pass cache's design, where the compiler runs on every program and
  only the execution is skipped, is the right trade: the thing under test is the compiler. The merge set with no
  cache is the fail-safe the merge-queue paper keeps. One gap: `REACH` is a hand-kept table and the review of
  2026-10-30 found two entries too narrow (`tests\format\` and the snapshots); a tier-1 test can hold it to the
  files the cargo tests actually read.
- *Compiler/languages engineer:* folding on and off, the differential list and the call-site check are the three
  checks a compiler needs beyond golden output, and all three are there and selected by reach. Nothing to add.
- *QB64 engineer:* the oracle is QB64pe's recordings, with QB 4.5 for the doubtful ones, and the divergences are
  logged. The timing failures are the ones a BASIC runtime always has (sound, timers, the mouse, the clipboard);
  they are rerun alone and their causes are written down. Right.
- *Rust engineer:* tier 1 is 1 to 4 minutes and that is the part run most (31 runs on 2026-10-30, 37 minutes).
  The rule already says `cargo test -p <crate>` while working and the whole workspace once at the end of a group.
  The ten whole-workspace runs in a day say the "once" is not held to, though two of them were the plan's and
  several followed a failed one. Nothing to change in the tests; `cargo nextest` would cut a few seconds, not
  minutes, and is not worth a toolchain change.
- *Build and test-infrastructure engineer* (fits: the question is about machine time): the caches and the lock
  are built and measured; a close is about ten minutes warm and runs in the background. The cold-cache cases are
  named by `--check` now. The one measured waste that remains and is not in a rule is the close count.
- *Measurement engineer* (fits: the user's doubt is about whether the problem is fixed, and the record is the
  evidence): the share of time waited has been flat at about 20% through every rule change, so no rule change
  moved it. Repeats after a pass went from 133 to 2, so that waste is gone. The remaining waiting is tier 1 and
  the closes. If the next period's record shows closes at one per batch and tier 1 at the rule's cadence, the
  share should fall; if it does not, the next thing to look at is the tier-1 count, not the lists.

**The panel's view:** the suite is right as it stands. Two things to decide, one small guard to build. (On a
second look the pragmatic engineer dropped the guard, R2: §6.)

## 5. Findings

- **F1. The shape is right.** Narrow tests at each stage, broad tests against a reference, selection by reach
  before a commit, the full set at a merge, flaky kinds rerun alone. This matches the practice in §3. Nothing
  should be cut from the set, and nothing is missing from it.
- **F2. The cost is the count of closes, not the set.** A close with warm caches is about ten minutes of
  background machine time. On 2026-10-30 there were four on one branch; on 2026-10-29 the lists ran six times.
  The rule says "once per change", and a change is now a few hours of work on a branch that will be merged as
  one pull request.
- **F3. The repeats are gone.** 133 runs repeated after a pass over the project, 2 in the last three days. That
  finding of `study\38` is closed.
- **F4. Cold caches are the outliers.** The 34-minute verification run (support files, fixed), the 15-hour
  background hang (a program that never ended; never waited for), the 25-minute close for a sandbox-only edit
  (fixed by `REACH`). `--check` names a cold run now. No further change.
- **F5. Tier 1 is run more than the rule says.** 31 runs on 2026-10-30 (10 whole, 21 one crate), 37 minutes
  waited. The rule's cadence (per crate while working, whole once per group) is right; the count says it is not
  always followed. Not a tool question: `cargo test` is the thing Claude runs by hand.
- **F6. `REACH` is right but unguarded.** It was corrected twice on the day it was written. A test that reads
  which `tests\` folders the cargo tests open and checks each is in a `REACH` entry that includes tier 1 would
  have found both.
- **F7. Verified, not assumed, in this study:** the pass cache skips the run only, never the compile (so the
  compiler is exercised on every listed program); the CLI tests that build are all `#[ignore]`, so tier 1 links no
  libqb; the three known races are the suite's only flaky kinds and each has a written cause.

## 6. What should be: the recommendations, and what stays

Few, and each with its cost.

- **R1. A close per batch, not per OpenSpec change** (the user's call, §7 question 1). Keep the set as it is;
  run it once when the branch's work is about to be committed or when the next piece of work touches other
  crates, not at the end of each change when several land in one day on one branch. *Why:* F2; practice runs
  the selected set once per change list sent for review. *Cost:* a fault in the first change of a batch is found
  at the batch's close, hours later instead of minutes; the build cache finds which change. *Risk to correctness:*
  none beyond that delay. *Where:* one sentence in `CLAUDE.md` rule 6 and `tools\close\README.md`, "The test
  cadence"; no tool change (`--check` already judges by what changed since `HEAD`, which is the batch).
- **R2. A tier-1 test that holds `REACH` to the files the cargo tests read.** *Dropped* after the pragmatic
  engineer's second look (user, 2026-10-30): CI runs tier 1 and the command-line tests that build on every push,
  so an entry too narrow for them is caught there; the only entries without tier 1 are the sandbox's, and no
  cargo test reads `tests\sandbox\` (`inputs.rs` walks corpus, upstream, snippets, differential and programs);
  and a grep for folder names would miss paths built in pieces (`tests.join("corpus")`). F6 stands as a known
  limit, with the push and the merge as its guards.
- **What stays, deliberately:** the closing set (one fixed set, 2026-10-30); the pass cache skipping runs only;
  the full set with no cache at a merge; CI not a gate; `cargo test` by crate while working, the workspace once a
  group; the rerun-alone rule; the sandbox family last and alone; the record written when the user asks. None of
  the sources in §3 argue against any of these, and each was adopted for a measured reason that still holds.
- **Not recommended, and why:** caching the compile-only and call-site steps (about 4 minutes a close; the
  user chose not to, 2026-10-30, and R1 makes it matter less); `cargo nextest` (seconds); cutting the verification
  list (it is the suite of the new compiler; 4.5 minutes warm, in the background); a finer `REACH` by crate (the
  module-level selection industry settled on is what it does now; a crate reaches everything, which is true of
  this workspace's dependency graph).

## 7. What the user decides

1. **R1, the cadence of a close:** per batch of commits (recommended), per OpenSpec change as now, or something
   between (for example per change, but a second close on the same branch within the day only for what the new
   work reaches, which is what `REACH` already gives when the earlier close's logs are current).
2. **R2, the `REACH` guard:** build now, or not.

Nothing else is asked. If the answer to 1 is "as now", the suite stays as it is and this study is archived with
that ruling.

**Answered** (user, 2026-10-30: "do r1", after the pragmatic engineer's view; `DECISIONS.md` 2026-10-30): (1) a
close runs once per batch, when Claude is about to give the user the files to stage (`CLAUDE.md` rule 6,
`tools\close\README.md`, "When a close runs"); (2) R2 is dropped (§6). All its recommendations are decided and
built, so the study can be archived (working rule 7).

## Sources

- Compiler testing: [Compiler Testing: A Systematic Literature Analysis](https://arxiv.org/pdf/1810.02718);
  [differential testing of compilers against a golden reference (US patent 5754860)](https://patents.google.com/patent/US5754860);
  [Testing, Debugging, and Fuzzing Your Compiler](https://kindatechnical.com/compiler-design/testing-debugging-and-fuzzing-your-compiler.html);
  [rustc-dev-guide, the compiler testing framework](https://rust.googlesource.com/rustc-guide/+/refs/heads/master/src/tests/intro.md).
- Test sizes and flakiness: [Google Testing Blog, Jeff Listfield, 2017](https://testing.googleblog.com/2017/04/);
  [Test Sizes (summary)](https://documentdrivendx.github.io/helix/research/google-test-sizes/).
- Selection and the full set: [Elbaum, Rothermel, Penix, "Techniques for improving regression testing in
  continuous integration development environments", FSE 2014](https://research.google/pubs/techniques-for-improving-regression-testing-in-continuous-integration-development-environments/);
  [Practical Pipeline-Aware Regression Test Optimization for Continuous Integration (2025)](https://arxiv.org/html/2501.11550);
  [Bazel CI: presubmit and postsubmit](https://bazel.googlesource.com/continuous-integration/+show/1d48b13f537e50783608f2c848f69102b81d11a7/docs/user.md);
  [Teamscale, improving test efficiency](https://docs.teamscale.com/tutorial/improving-test-efficiency);
  [Shi, Zhao, Marinov on module-level regression test selection](https://par.nsf.gov/servlets/purl/10175678);
  [Leong et al., test selection at Google, ICSE SEIP 2019](https://mpapad.github.io/publications/pdfs/ICSE-SEIP2019.pdf).
- The pyramid: [Martin Fowler, Test Pyramid](https://www.martinfowler.com/bliki/TestPyramid.html).
- Snapshots: [insta](https://docs.rs/insta).
