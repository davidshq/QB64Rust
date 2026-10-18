# 31. Development velocity improvement plan

Proposed 2026-10-16, following the audit of the Claude transcripts and `tools/testtime/` reports.
This document plans four improvements; it does not change the current testing or documentation rules.

## Outcome

Reduce the elapsed time to finish a compiler feature while preserving compatibility evidence and regression
coverage. Address long stalls first, repeated documentation second, overlapping execution third, and gaps in
measurement fourth. Implement these as four small changes with focused validation, rather than one overhaul.

The historical report measured 92 h 14 min of active session time: 18% waiting for tests, 4% measuring the old
compiler, and 3% building. The remaining 75% combines coding, test authoring, reading, reviews, documents and
other work. It cannot be treated as coding time. The report predates the sound change's final runs.

The transcript audit counted 3,136 documentation/planning Edit/Write calls and 2,930 product-code calls across
275 available transcripts. These are approximate activity counts, not time shares; shell-generated work is
not included. Use them to identify an opportunity, not to set a target for the ratio of code to documents.

## Pragmatic engineer's priority ranking

Reviewed inline on 2026-10-16. Each checkbox carries a priority:

- **P1 — first:** remove observed stalls or recurring duplication, including the validation needed to make
  those changes safe.
- **P2 — next:** implement useful automation and consolidation after their prerequisites are established.
- **P3 — follow-up:** add richer analysis after the simpler measurements show which questions remain.

Priorities weigh agent implementation/checking time, the user's waiting and reading time, correctness risk,
and continuing maintenance cost. They do not use estimates of how long a human engineer would take. A lower
priority does not make a correctness check optional: it runs with the implementation it validates. Keep the
numbered changes as implementation boundaries, but pull forward the lightweight measurement markers in item 4
before the next feature; the extended reporting work need not block the practical fixes.

## 1. Make CLI integration tests unattended and bounded

**Overall: P1.** The observed multi-hour stall is the clearest immediate loss. Fix the prompt behavior first;
finish bounded subprocess execution before treating this change as complete.

### Problem and scope

The sound change's `target/cli_ignored.log` reported 8,150.60 seconds for 33 passing tests. Its last outstanding
test was `a_function_the_dll_does_not_export_is_error_260`. In `crates/driver/tests/cli.rs`, that test invokes
the executable without explicitly setting `QB64PE_NOPROMPT` and waits through `Command::output()` without a
deadline. An interactive error dialog is a likely explanation, not a confirmed reproduction of the stall.

### Implementation

- [x] **P1** Inspect the test's transcript and runtime error path, and measure the individual error-260 test with
  `QB64PE_NOPROMPT=y` explicitly set. Preserve its expected error text and lack of program output.
  *Confirmed in libqb, not guessed:* error 260 is critical (`error_handle.cpp`, `fatal_error_alert`); with
  `QB64PE_NOPROMPT` unset it calls `gui_alert` (`parts\gui\gui.cpp`), which writes `Runtime error: …` to stderr
  and then opens a modal `tinyfd_messageBox`. So the test passed its stderr assertion once a person had closed
  the box, 8,150 s later. With the variable set: 2.6 s for the whole test, the same assertions.
- [x] **P1** Introduce a small shared subprocess helper for the CLI tests. It must accept an explicit deadline,
  capture stdout and stderr without blocking on full pipes, and report elapsed time and the failing phase.
  `crates\driver\tests\support\mod.rs`: `try_run`/`run`, one reader thread per pipe, a `cli-time <test> <phase>
  <seconds>` line per child (captured; shown with `--nocapture`), a panic naming the phase on a timeout.
- [x] **P1** Set `QB64PE_NOPROMPT=y` and close stdin when running generated programs unattended. Preserve explicit
  overrides for tests that intentionally exercise `Continue` or other prompt modes. (`support::unattended`;
  `an_error_that_ends_the_program_traces_nothing` sets `y` and `continue` itself, which still takes effect.)
- [x] **P1** Give compiler/build commands a separate, longer deadline from generated-program execution. Choose
  initial limits from existing timings with allowance for a cold runtime build; document those limits.
  The runner's limits: 900 s for a compiler or toolchain run (`COMPILE_TIMEOUT`, which allows for the libqb
  objects' first build, about 3 min, `tools\legacy_tests\README.md`), 60 s for a program
  (`CORPUS_RUN_TIMEOUT`); documented in `crates\README.md`, "Tests".
- [x] **P1** On timeout, stop the owned process tree on Windows, reap it, and retain partial output. Avoid killing
  unrelated compiler or runtime processes. Use an existing suitable process utility if one is already present.
  `taskkill /T /F /PID <child>`, as the runner's `kill_tree` (a Job Object would need `unsafe`, which the
  workspace forbids); only the child's own tree is named. A child that ends while a process it started keeps its
  pipes open is reported after 10 s, not killed: its tree can no longer be found by parent.
- [x] **P1** Migrate direct subprocess waits in `cli.rs`, including library tests, to the helper. Check whether direct
  build tests can collide with another run using the reference clone; reuse the existing clone-lock protocol
  where necessary, with bounded lock waiting and a visible reason for the wait.
  Every `output()`/`spawn()` in `cli.rs` now goes through the helper, the language server's too (its input is
  written by the helper). They could collide: `make` runs in the clone. Tests that build take
  `target\qb64pe-clone\in-use.lock` with `File::try_lock` once per test process (waiting at most 30 min, with
  one console line saying why); checked against `clone_lock.py` holding it: the test waited 20 s, said why, then
  passed.

### Validation and completion

- [x] **P1** A deliberately hanging child fails within its deadline and leaves no child process running.
  `a_child_past_its_deadline_is_killed_with_its_tree`: `cmd` starts a `cmd` that writes a tick a second, and the
  tick file must stop growing. Run once with `/T` removed: it failed ("the grandchild still writes").
- [x] **P1** A child producing more output than a pipe buffer completes with its output captured.
  `a_child_with_much_output_is_read_whole`: 1.3 MB on each of stdout and stderr.
- [x] **P1** The error-260 test completes unattended and still asserts the expected runtime error.
- [x] **P1** Run the ignored CLI suite once after migration, logging the result and per-test timing. Investigate any
  outlier rather than raising the overall timeout to make the suite pass.
  2026-10-16: 33 of 33 in 29.6 s (before: 8,150.6 s); 52 compiler runs (204 s in all, longest 9.5 s, Civil
  War Strategy), 33 program runs (16 s; longest 5.3 s, `screen_state_prompt_that_never_comes`, whose 5 s wait
  is the test), 18 toolchain runs (4 s). No outlier. Not measured: a cold build, which would mean deleting the
  libqb objects of the read-only reference clone; the 900 s limit rests on the runner's recorded 3 minutes.

**Done when:** every subprocess wait in the CLI suite is bounded or explicitly justified, and no generated
program needs a person to dismiss a dialog. Record cold-build time separately from normal suite time.

## 2. Reduce repeated documentation updates

**Overall: P1 for removing duplication; P2 for generation.** Simplify ownership and future task instructions
first, so the next feature benefits without waiting for a documentation tool to be built.

### Problem and scope

Each feature currently updates several overlapping descriptions of support and progress. Preserve the detailed
decision history and the evidence for compatibility; reduce duplication in current summaries and routine counts.

### Implementation

- [x] **P1** Inventory the documents updated by the last three completed changes. Record each document's distinct
  purpose and identify repeated feature descriptions, counts and standing instructions.
  Done over the four commits `fe98a44`, `b9b3525`, `447bb05` (three changes) and `b1a818d` (`m3-sound-handles`).
  Besides the change's own `proposal.md`, `design.md`, `tasks.md` and delta specs, a change edits 10 to 13
  documents. Per document, what is distinct and what repeats:

  | Document | Distinct | Repeated |
  |---|---|---|
  | `openspec\specs\` (merged) | what a program may use | — (the home) |
  | `crates\README.md` | crate map, build, test commands | **the feature again** (28 lines for the sound handles: the rows, the array check, the tests by name), the change's design in short |
  | `ROADMAP.md` | the milestone's step | the feature in one line, the upstream count |
  | `STATUS.md` | where we are, what waits | the step's row (feature again), **every count** of the close |
  | `tests\upstream\README.md` | membership rules | **a progress paragraph appended per change** (counts, the programs that joined, by name) |
  | `tests\corpus\README.md` | what a kind of program is | the slice count, a "joined with …" sentence per change |
  | `study\00` §5 | measured facts | — (the home) |
  | `DECISIONS.md`, `DIVERGENCES.md`, `SOMEDAY.md` | decisions, divergences, deferred work | — (each the home; rows kept whole, user 2026-10-16) |
  | `verification\README.md` | the families | — |
  | `tools\testtime\REPORT.md` | generated | — |
  | `tasks.md` header | what this change does differently | **the standing rules of `CLAUDE.md`** (subagents, staging, backslashes, `check_repo.py`, test cadence), a paragraph per change |

  So one feature is written four times (spec, `crates\README.md`, `ROADMAP.md`, `STATUS.md`), each count of a
  close three times (`STATUS.md`, a test README, `tasks.md`), and the standing rules once per change.
- [x] **P1** Use this ownership map when simplifying the active documentation:

  | Information | Authoritative home | Other documents contain |
  |---|---|---|
  | Supported language behavior | Merged OpenSpec specs | A short summary or link |
  | Measured QB64pe behavior | `study/00-synthesis.md` section 5 | Evidence links |
  | Decisions and compatibility exceptions | `DECISIONS.md`, `DIVERGENCES.md` | Links to the applicable entries |
  | Current state, open questions, next work | `STATUS.md` | Links to current status |
  | Milestone completion | `ROADMAP.md` | Links to the roadmap |
  | Per-change implementation history | The change's `tasks.md` and `design.md` | A concise result |
  | Test-set membership and results | Lists and runner result files | Generated count/result blocks |
  | Build and contributor instructions | `crates/README.md`, tool READMEs | Links to those instructions |

- [x] **P1** Replace the detailed feature catalogue in `crates/README.md` with a short capability overview and links
  to the specs, retaining useful build, crate-map and development instructions.
  The catalogue (692 lines) moved unchanged to `study\support-record.md`, a closed record as
  `study\progress-record.md` is, because it is the only place that names, feature by feature, the code that
  implements it; deleting it would lose that. In its place, "What the compiler supports": where each kind of
  information now lives and a table from area to spec (20 rows). `crates\README.md`: 900 lines to 236. The
  standing paragraphs after it ("not supported yet" means an error, the follow-on rule, the panic report) stay.
- [x] **P2** Add a narrow generator for marked count/result blocks in the test READMEs and, where appropriate,
  `STATUS.md`. Read membership from lists and outcomes from runner JSON; do not infer a pass from compilation
  alone. Include run identity and configuration, and distinguish measured results from current membership.
  `tools\legacy_tests\counts.py`: five blocks, membership (corpus by folder, upstream "x of y", differential)
  and results (corpus, upstream: the last full run's date, compiler build, options, counts, and the pass list
  judged in it by `check_list`). The corpus README's hand-kept `.output` and `.err` rows went to the block; the
  upstream README's progress paragraphs stay as history, closed to new ones. **`STATUS.md` not generated:** its
  table of numbers is a sentence per row with names and reasons, and its rows come from a dozen tools.
- [x] **P2** Make the generator deterministic, idempotent and usable in a check-only mode. Preserve prose outside
  its marked blocks. Do not publish local paths or raw transcripts.
  `--check`; a second run rewrites nothing; each file keeps its own line ends (the checkouts are CR LF).
- [x] **P1** Remove repeated standing-rule prose from future task templates where a link to `CLAUDE.md` suffices.
  Update workflow instructions to perform routine summary updates once at close, while recording decisions
  and new measured findings when they occur.
  There is no template: the paragraph was written by habit. `openspec\config.yaml` now has `rules` for `tasks`
  (no standing rules restated, one line "Standing rules: CLAUDE.md." at most; decisions and measured facts where
  they happen, `STATUS.md`, `ROADMAP.md` and test-README counts once in the closing task; no feature text in
  `crates\README.md`) and for `design` (name where the code lives, since nothing else does now). `openspec
  instructions tasks` serves them; the propose skill applies them without copying them.
- [x] **P1** Update affected ownership references together. Preserve archived changes and detailed decision rows;
  the user explicitly chose to retain those rows.
  `STATUS.md`'s table of homes, `CLAUDE.md`'s layout table, `study\00`'s document map (this study and the
  record), `SOMEDAY.md`'s row of the window built-ins (it pointed into the moved text). Archived changes and
  `DECISIONS.md` rows untouched.

### Validation and completion

- [x] **P1** Check that every removed description has a working link to its retained authoritative content.
  Nothing was deleted: the text is in `study\support-record.md`, linked from the overview. Every reference to
  `crates\README.md` outside the archives was read: one (`SOMEDAY.md`) pointed into the moved text and now names
  the record; the others point to what stayed (crate map, tests table, build). No test or tool parses the file.
  The 20 areas of the overview were checked against the specs' contents (`COMMON`, `DEF SEG`, `$EXEICON`, the
  auto-included constants each found in the spec named).
- [x] **P2** Verify generation is unchanged on a second invocation, preserves manual prose, and rejects missing or
  incompatible result data rather than presenting stale numbers as a fresh pass.
  `test_counts.py` (5 tests); on the repo, the second run: "5 blocks, 0 differ".
- [ ] **P1** Pilot the simplified closeout on the next feature and record which documents actually needed edits.

**Done when:** routine counts are generated, each detailed feature description has one home, and a feature's
closeout no longer requires copying the same explanation into several summaries.

## 3. Consolidate overlapping closing runs

**Overall: P2.** The saving is plausible, but configuration equivalence must be demonstrated before deleting
any run. Keep the assertion-preservation work in the same change as the execution reduction.

### Problem and scope

Closing checks run the slice and upstream pass lists separately from full suites that include many of the same
programs. Retain every distinct assertion and configuration while executing equivalent cases once.

### Implementation

- [x] **P2** Build a coverage matrix for the existing closeout commands: selected programs, compiler/build identity,
  folding mode, reference root, assets, screen availability, environment, expected outputs, divergence checks,
  and pass/failure policy. Identify actual overlap before replacing a command.
  From the commands of `m3-sound-handles`' close (its transcript) and the runner's code. Every runner command
  used `--qb64 target\release\qb64rust.exe --jobs 8 --build-cache target\build-cache`, one build of the compiler,
  the same results folder and `known_failures.txt`:

  | Closing run | Programs | Differs from a full suite in | Covered by |
  |---|---|---|---|
  | slice list | 440 of the corpus | the selection only (`--list`); each program goes through the same `corpus_test`, with its screen state, keys, `.divergences` and known-failure rule | the full corpus: **derived** |
  | slice list, `QB64RUST_NO_FOLD=1` | the same 440 | folding off | nothing: **kept** |
  | upstream pass list | 325 of the clone's `compile_tests` | the selection only; same root (the clone) | the full upstream suite: **derived** |
  | differential pass list | 59 | another corpus root (`tests\differential`) | nothing: **kept** |
  | full corpus, full upstream suite | 465, 404 | — | **kept** (they carry the derived checks) |
  | call-site check, `--ignored` CLI tests, `cargo test`, the runner's tests, demand reports | — | other tools | nothing: **kept** |

  One difference in policy: a list run let a program of the list that is in `known_failures.txt` pass as KFAIL;
  the derived check fails it. No program of either list is a known failure today.
- [x] **P2** Extend runner result records only where needed to identify those properties and whether a case was
  executed, reused from a build cache, compile-only, skipped, rejected or a known failure.
  A `run` record per summary (selection, roots, folding, `--cpp-opt`, `--record`, `--jobs`, caches, SHA-256 of the
  compiler and the runner, start and end). Per program nothing new was needed: `kind` (output, compile only,
  error), `status` (PASS, FAIL, KFAIL) and the pass cache's "not run" detail were there; a build-cache hit is not
  recorded, since the program is still compiled, run and compared.
- [x] **P2** Add a summary/check operation that evaluates pass-list membership against the full suite's results.
  Missing, skipped or unexpectedly failing required cases must fail that check. A known failure allowed by the
  full suite must not silently satisfy a required pass-list entry.
  `run_legacy_tests.py --check-list <list>` (`check_list`); also fails a program whose files changed after the
  run started, and a run of another compiler build, runner, root or folding.
- [x] **P2** Replace separate normal-folding slice/pass-list runs with those derived checks only when the full run
  covers identical inputs, roots, assets and assertions. Preserve a separate run where they differ.
- [x] **P2** Keep the slice run without folding as a distinct configuration. Keep differential execution, call-site
  ABI comparisons, screen comparisons and CLI integration assertions that the full suites do not cover.
- [x] **P1** Keep focused checks during implementation. Continue using build caches; retain the current prohibition
  on pass-cache skipping for closing runs, since unchanged executables may behave differently under load.
  Unchanged; the check refuses a run made with `--pass-cache`.
- [ ] **P2** Produce one closeout summary listing executed configurations, required memberships, known failures and
  exclusions. Log each underlying test command directly so the current recorder still sees it; introduce a
  wrapper only after item 4 can record its child runs reliably.
  *Not done:* the `run` record and the check's output hold the parts; a single summary over all closing runs
  would be a new tool, and the plan says to log each command directly for now.
- [x] **P2** Update `CLAUDE.md`, the active cadence guidance and future task templates together. Preserve the user's
  rule that CI is not a gate and need not be waited on.
  `CLAUDE.md` working rule 9, `crates\README.md` ("When to run what"), `tools\legacy_tests\README.md`; CI and its
  rule unchanged (the `tier2` job runs no full suite, so it keeps its list runs). `test_time.py` records a check as
  its own kind, "pass list checked from a full run".

### Validation and completion

- [x] **P2** Add focused runner tests for missing required results, unexpected failures, configuration mismatch,
  compile-only cases, and full-suite known failures that are forbidden by a pass list.
  `CheckList` in `test_run_legacy_tests.py`, 7 tests (also: kept by the pass cache, an input changed after the
  run, a summary without a run record); 87 runner tests pass.
- [x] **P2** For one representative compiler state, compare old and consolidated closeout outcomes. This is a
  one-time migration check; reuse matching retained results where their identity can be established.
  The release build of `m3-sound-handles` (17:37, unchanged since; its close ran at 20:00–20:11 with it, logs
  kept). New full runs with this runner: corpus 459 pass, `218_fre` and 5 known failures, as at the close;
  upstream 325 pass, 78, 1 known failure, as at the close. `--check-list`: `slice.list` 440 of 440, `pass.list`
  325 of 325, program for program the same set the close's list runs passed.
- [x] **P2** Demonstrate equal required coverage, no weaker pass policy, and fewer program executions. Measure the
  elapsed saving, including the time to build, execute and summarize.
  Coverage equal (the matrix), policy stricter (KFAIL, changed inputs, run identity), 765 program runs fewer per
  close; a check takes about a second. **The saving in time is smaller than the list runs' rows suggest:** at
  that close the slice list took 7 min and the upstream and differential lists 4 min, but they filled the build
  cache, after which the full suites took 50 s each. Without the list runs the full suites build what changed, so
  the saving is the second compile, run and compare of 765 programs: about 1.5 min of machine time per close
  (the full suites' time with a warm cache, scaled to the lists' share), and two commands fewer to start and read.

**Done when:** each required program/configuration pair runs once at close, except for intentional repeated
recordings or load-sensitive checks with a documented purpose.

## 4. Measure work categories and feature completion time

**Overall: P2 for a useful baseline; P3 for richer attribution.** Start with feature boundaries and a small
report. Detailed failure classification and duplicate-work inference should not become a prerequisite for
improving velocity or a new project that consumes the time being saved.

### Problem and scope

`tools/testtime/` measures test-command waiting, not the full cost of testing or the elapsed time to finish a
feature. Its five-minute activity heuristic and background-run accounting cannot establish coding hours or
the time the user spent waiting. Improve attribution without inventing precision or adding manual timesheets.

### Implementation

- [x] **P2** Preserve the existing CSV history and definitions. Add versioned, additive records or separate files;
  do not relabel old estimates as newly measured facts.
  `runs.csv` and `sessions.csv` keep their columns and rules; two new files, `changes.csv` and `activity.csv`,
  each with a `version` column (1), kept for sessions whose transcript is gone as the old files are.
- [x] **P2** Associate work with an OpenSpec change where the transcript or an explicit workflow marker establishes
  it. Use stable session/tool identifiers to deduplicate records. Mark ambiguous attribution as unknown.
  Each moment goes to the change named last in its session (`tools\testtime\README.md`, "Changes"); before any
  naming, and after an archive, to "no change named". Actions are deduplicated by tool-use id.
- [x] **P2** Record event categories: product implementation, test/fixture authoring, reference measurement
  preparation, documentation/planning, review/investigation, test execution, builds, and unknown/mixed work.
  Twelve kinds (README, "Kinds of work"); review and investigation is "reading" for the tools that only read,
  while a shell command that is no run stays "other commands", since its purpose is not read. Added after the
  first run: "scratch files" (a temporary folder), which held 11% of the bytes written as "other".
- [x] **P2** Classify edits by file and tool activity, while distinguishing product source from test source and
  generated outputs. Treat action counts and written bytes as activity proxies, not elapsed-time percentages.
- [x] **P1** Add lightweight start/completion markers at feature boundaries through the existing workflow. Record
  wall-clock elapsed duration, estimated active duration, foreground waiting, and background execution
  separately. Expose gaps and interruptions; do not count the entire wall-clock interval as agent work.
  No new step: the workflow already marks the boundaries in every transcript (`openspec new change`, the first
  touch of the change's folder; `openspec archive`, or a move of the folder into the archive), so the record
  goes back to 2026-10-02. `REPORT.md`, "By change": elapsed (calendar), active, waits per bucket, background,
  idle stretches and the longest, the longest wait, the closing stretch. 11 of the 31 archived changes are seen
  archived; the other 20 are marked "archived, not seen" and end where they were last seen.
- [ ] **P3** Capture structured command start/end, exit status, configuration, result/log location and timeout where
  available. Separate lock waiting and approval waiting from execution when evidence permits; otherwise label
  the duration as combined or unknown. Recognize wrapper child runs before adopting a closeout wrapper.
- [ ] **P3** Compare equivalent work with a matching change/build/configuration identity, not command text alone.
  Preserve the current repeat heuristic for history, but label its limitations and distinguish intentional
  repeat recordings and folding variants from redundant reruns.
- [ ] **P2 for basic totals; P3 for failure-cause attribution.** Extend the report with per-change elapsed and active time, activity counts by category, closeout time,
  longest stalls, repeat execution, and evidence about failure causes. Separate product defects from fixture,
  environment and infrastructure failures where the logs establish the distinction.
  *P2 part done* (per change: elapsed, active, actions by kind, the closing stretch from the last edit of product
  code to the archive, the longest wait and idle stretch, repeats). *P3 part not done*: failure causes.
- [x] **P1** Retain the privacy rule: persist aggregate categories and identifiers, not prompts, raw commands with
  local paths, or secrets. Handle incomplete sessions without marking them complete.
  The new files hold a session's 8-character id, a change's name, a kind and numbers; a change is "open" until a
  transcript shows its archive.

### Validation and completion

- [ ] **P2 for baseline reporting; P3 for advanced attribution.** Use small synthetic transcripts to check background overlap, long idle gaps, missing results, timeout
  handling, duplicate events, mixed commands, feature boundaries and ambiguous attribution.
  *P2 part done* (`tools\testtime\test_record.py`, class `Changes`, 6 tests besides the 38 before): feature
  boundaries, work before a naming and after an archive, two changes in one session, an archived change read, a
  background run, a 20-minute idle stretch, an archive refused or failed, a line held twice, the kinds of
  paths, a change summed over two sessions; timeout handling and mixed commands were tested before.
- [x] **P2** Reconcile the old foreground totals on a fixed transcript sample, explaining intentional differences.
  All 277 transcripts of 2026-10-16: the per-change rows sum to the session totals within 8 s of 338,523 s of
  active time and 3 s of each bucket's waiting, the rounding of each row to whole seconds; the synthetic test
  asserts the equality exactly.
- [ ] **P2** Pilot the additional metrics over the next three comparable feature changes. Review elapsed completion
  time and defect outcomes together; do not optimize for fewer tests, fewer documents or more code lines alone.

**Done when:** the report distinguishes test execution from test authoring and documentation activity, and
shows elapsed feature completion time without presenting inferred work categories as measured coding hours.

## Delivery order and review

| Change | Priority | Deliverable | Evidence to retain |
|---|---|---|---|
| 1 | P1 | Bounded, unattended CLI subprocesses | Error-260 result, timeout cleanup checks, one CLI suite log |
| 2 | P1 simplification; P2 generation | Documentation ownership and generated summaries | Removed duplication, generator checks, first feature pilot |
| 3 | P2 | Consolidated closeout execution | Coverage matrix, outcome comparison, program counts and duration |
| 4 | P1 boundary markers; P2 baseline; P3 richer analysis | Extended activity and completion reporting | Reconciliation sample and three-feature pilot |

Capture a small baseline before each change using existing results where possible. Start the three-feature
measurement pilot after item 4; provisional feature start/end markers may be added earlier so intermediate
improvements can be compared. Do not stop compiler development while waiting for pilot data.

At each change's close, report what improved, the evidence, and remaining limitations. Apply the current
impact-based testing rule to these changes themselves: documentation-only edits do not require executable
compiler regression suites, and reporting changes require recorder tests rather than unrelated sound tests.

Success means shorter feature completion and closeout times, no indefinite child-process waits, fewer repeated
summaries, and unchanged required compatibility assertions. Set numerical improvement targets after the first
comparable measurements; the historical 18% does not justify promising a particular overall speedup.

## Sources

- Local timing data and limitations: [report](../tools/testtime/REPORT.md),
  [recorder README](../tools/testtime/README.md).
- Existing documentation findings: [ninth review, section 5](30-ninth-review.md).
- Current cadence and working rules: [CLAUDE.md](../CLAUDE.md).
- CLI subprocess behavior: [cli.rs](../crates/driver/tests/cli.rs).
- Runner execution and locking constraints: [runner README](../tools/legacy_tests/README.md).
- External guidance: [DORA test automation](https://dora.dev/capabilities/test-automation/) recommends fast,
  reliable feedback, finding failures with the cheapest suitable test layer, and tracking whether failures
  represent product defects. The concrete changes above are project-specific proposals.
