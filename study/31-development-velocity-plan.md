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

- [ ] **P1** Inspect the test's transcript and runtime error path, and measure the individual error-260 test with
  `QB64PE_NOPROMPT=y` explicitly set. Preserve its expected error text and lack of program output.
- [ ] **P1** Introduce a small shared subprocess helper for the CLI tests. It must accept an explicit deadline,
  capture stdout and stderr without blocking on full pipes, and report elapsed time and the failing phase.
- [ ] **P1** Set `QB64PE_NOPROMPT=y` and close stdin when running generated programs unattended. Preserve explicit
  overrides for tests that intentionally exercise `Continue` or other prompt modes.
- [ ] **P1** Give compiler/build commands a separate, longer deadline from generated-program execution. Choose
  initial limits from existing timings with allowance for a cold runtime build; document those limits.
- [ ] **P1** On timeout, stop the owned process tree on Windows, reap it, and retain partial output. Avoid killing
  unrelated compiler or runtime processes. Use an existing suitable process utility if one is already present.
- [ ] **P1** Migrate direct subprocess waits in `cli.rs`, including library tests, to the helper. Check whether direct
  build tests can collide with another run using the reference clone; reuse the existing clone-lock protocol
  where necessary, with bounded lock waiting and a visible reason for the wait.

### Validation and completion

- [ ] **P1** A deliberately hanging child fails within its deadline and leaves no child process running.
- [ ] **P1** A child producing more output than a pipe buffer completes with its output captured.
- [ ] **P1** The error-260 test completes unattended and still asserts the expected runtime error.
- [ ] **P1** Run the ignored CLI suite once after migration, logging the result and per-test timing. Investigate any
  outlier rather than raising the overall timeout to make the suite pass.

**Done when:** every subprocess wait in the CLI suite is bounded or explicitly justified, and no generated
program needs a person to dismiss a dialog. Record cold-build time separately from normal suite time.

## 2. Reduce repeated documentation updates

**Overall: P1 for removing duplication; P2 for generation.** Simplify ownership and future task instructions
first, so the next feature benefits without waiting for a documentation tool to be built.

### Problem and scope

Each feature currently updates several overlapping descriptions of support and progress. Preserve the detailed
decision history and the evidence for compatibility; reduce duplication in current summaries and routine counts.

### Implementation

- [ ] **P1** Inventory the documents updated by the last three completed changes. Record each document's distinct
  purpose and identify repeated feature descriptions, counts and standing instructions.
- [ ] **P1** Use this ownership map when simplifying the active documentation:

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

- [ ] **P1** Replace the detailed feature catalogue in `crates/README.md` with a short capability overview and links
  to the specs, retaining useful build, crate-map and development instructions.
- [ ] **P2** Add a narrow generator for marked count/result blocks in the test READMEs and, where appropriate,
  `STATUS.md`. Read membership from lists and outcomes from runner JSON; do not infer a pass from compilation
  alone. Include run identity and configuration, and distinguish measured results from current membership.
- [ ] **P2** Make the generator deterministic, idempotent and usable in a check-only mode. Preserve prose outside
  its marked blocks. Do not publish local paths or raw transcripts.
- [ ] **P1** Remove repeated standing-rule prose from future task templates where a link to `CLAUDE.md` suffices.
  Update workflow instructions to perform routine summary updates once at close, while recording decisions
  and new measured findings when they occur.
- [ ] **P1** Update affected ownership references together. Preserve archived changes and detailed decision rows;
  the user explicitly chose to retain those rows.

### Validation and completion

- [ ] **P1** Check that every removed description has a working link to its retained authoritative content.
- [ ] **P2** Verify generation is unchanged on a second invocation, preserves manual prose, and rejects missing or
  incompatible result data rather than presenting stale numbers as a fresh pass.
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

- [ ] **P2** Build a coverage matrix for the existing closeout commands: selected programs, compiler/build identity,
  folding mode, reference root, assets, screen availability, environment, expected outputs, divergence checks,
  and pass/failure policy. Identify actual overlap before replacing a command.
- [ ] **P2** Extend runner result records only where needed to identify those properties and whether a case was
  executed, reused from a build cache, compile-only, skipped, rejected or a known failure.
- [ ] **P2** Add a summary/check operation that evaluates pass-list membership against the full suite's results.
  Missing, skipped or unexpectedly failing required cases must fail that check. A known failure allowed by the
  full suite must not silently satisfy a required pass-list entry.
- [ ] **P2** Replace separate normal-folding slice/pass-list runs with those derived checks only when the full run
  covers identical inputs, roots, assets and assertions. Preserve a separate run where they differ.
- [ ] **P2** Keep the slice run without folding as a distinct configuration. Keep differential execution, call-site
  ABI comparisons, screen comparisons and CLI integration assertions that the full suites do not cover.
- [ ] **P1** Keep focused checks during implementation. Continue using build caches; retain the current prohibition
  on pass-cache skipping for closing runs, since unchanged executables may behave differently under load.
- [ ] **P2** Produce one closeout summary listing executed configurations, required memberships, known failures and
  exclusions. Log each underlying test command directly so the current recorder still sees it; introduce a
  wrapper only after item 4 can record its child runs reliably.
- [ ] **P2** Update `CLAUDE.md`, the active cadence guidance and future task templates together. Preserve the user's
  rule that CI is not a gate and need not be waited on.

### Validation and completion

- [ ] **P2** Add focused runner tests for missing required results, unexpected failures, configuration mismatch,
  compile-only cases, and full-suite known failures that are forbidden by a pass list.
- [ ] **P2** For one representative compiler state, compare old and consolidated closeout outcomes. This is a
  one-time migration check; reuse matching retained results where their identity can be established.
- [ ] **P2** Demonstrate equal required coverage, no weaker pass policy, and fewer program executions. Measure the
  elapsed saving, including the time to build, execute and summarize.

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

- [ ] **P2** Preserve the existing CSV history and definitions. Add versioned, additive records or separate files;
  do not relabel old estimates as newly measured facts.
- [ ] **P2** Associate work with an OpenSpec change where the transcript or an explicit workflow marker establishes
  it. Use stable session/tool identifiers to deduplicate records. Mark ambiguous attribution as unknown.
- [ ] **P2** Record event categories: product implementation, test/fixture authoring, reference measurement
  preparation, documentation/planning, review/investigation, test execution, builds, and unknown/mixed work.
- [ ] **P2** Classify edits by file and tool activity, while distinguishing product source from test source and
  generated outputs. Treat action counts and written bytes as activity proxies, not elapsed-time percentages.
- [ ] **P1** Add lightweight start/completion markers at feature boundaries through the existing workflow. Record
  wall-clock elapsed duration, estimated active duration, foreground waiting, and background execution
  separately. Expose gaps and interruptions; do not count the entire wall-clock interval as agent work.
- [ ] **P3** Capture structured command start/end, exit status, configuration, result/log location and timeout where
  available. Separate lock waiting and approval waiting from execution when evidence permits; otherwise label
  the duration as combined or unknown. Recognize wrapper child runs before adopting a closeout wrapper.
- [ ] **P3** Compare equivalent work with a matching change/build/configuration identity, not command text alone.
  Preserve the current repeat heuristic for history, but label its limitations and distinguish intentional
  repeat recordings and folding variants from redundant reruns.
- [ ] **P2 for basic totals; P3 for failure-cause attribution.** Extend the report with per-change elapsed and active time, activity counts by category, closeout time,
  longest stalls, repeat execution, and evidence about failure causes. Separate product defects from fixture,
  environment and infrastructure failures where the logs establish the distinction.
- [ ] **P1** Retain the privacy rule: persist aggregate categories and identifiers, not prompts, raw commands with
  local paths, or secrets. Handle incomplete sessions without marking them complete.

### Validation and completion

- [ ] **P2 for baseline reporting; P3 for advanced attribution.** Use small synthetic transcripts to check background overlap, long idle gaps, missing results, timeout
  handling, duplicate events, mixed commands, feature boundaries and ambiguous attribution.
- [ ] **P2** Reconcile the old foreground totals on a fixed transcript sample, explaining intentional differences.
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
