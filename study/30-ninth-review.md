# 30. Ninth review: panel on the whole repository during M3 (2026-10-16)

Asked by the user: an expert panel over the codebase, which has been built steadily with lesser models, looking
for what could derail the project, what slows its trajectory, poor architecture, bugs and errors. No nits.

The panel (`CLAUDE.md` rule 6, inline): a pragmatic engineer, a QB64 engineer, a compiler/languages engineer, a
Rust engineer, a test engineer, and two extra roles: a **documentation and hand-off engineer** (the largest
findings are about the documents every session and the user must read, which no other role owns) and a
**release and CI engineer** (the gap between `main` and the working branch). Effort is counted by rule 8.

Evidence read today: `STATUS.md`, `ROADMAP.md`, `DECISIONS.md` (the rows since 2026-10-13 in full),
`DIVERGENCES.md`, `SOMEDAY.md`, `CLAUDE.md`, `crates\README.md`, the IR (`crates\ir\src\lib.rs`, `validate.rs`),
the checker's state (`crates\sema\src\check\mod.rs`, `Checker`), the emitter's entry and arrays
(`codegen-cpp\src\lib.rs`, `arrays.rs`), the build (`driver\src\build.rs`, `main.rs`), tier 1 (`driver\tests\inputs.rs`),
`rust.yml`, the shrink-only lists, the open change `m3-declare-library` (`tasks.md`, `design.md`), the test-time
report, the GitHub Actions run list, and a `cpp` snapshot. Runs made today on this machine, each to a log under
`target\`: `cargo test --workspace` (every suite passes, 27 build tests ignored as designed), `cargo clippy
--workspace --all-targets -- -D warnings` (clean), `cargo fmt --check` (clean), `check_repo.py --untracked`
(12,512 files, 0 findings).

Counts: 45,718 lines of Rust in 9 crates (`sema` 16,768, `syntax` 9,568, `codegen-cpp` 5,895, `ir` 4,430,
`driver` 4,152); 328 front-end tests; 367 snapshot files (1.9 MB under `driver\tests\snapshots`); 899 verification
programs tracked (7,064 files; 2.1 GB on disk with their git-ignored executables); 7,122 lines of merged specs in
38 files under `openspec\specs`; 98 decision rows averaging 674 characters; `STATUS.md` 403 lines, 39 KB;
`CLAUDE.md` 29 KB, of which one table cell is 11.5 KB. Fourteen days of work, 119 commits, 86 hours of active
session time (`tools\testtime\REPORT.md`).

## 1. Verdict

**The compiler is in better shape than its documents.** No bug was found by reading, every check that can be run
locally is green today, and the evidence behind the numbers (431 of 431 slice programs, 1,677 call-site programs
equal, 59 differential programs, 310 of 399 upstream programs, a real game played from a keys file and compared
screen by screen) is the strongest part of the project. The direction, the pipeline split, the IR and the test
tiers should stay as they are.

What could derail the project is not in `crates\`. It is that the hand-off documents have outgrown both the
humans and the models that read them, that decisions Claude made alone are piling up unconfirmed, and that the
working branch has not met `main` or the weekly tier 2 job in eleven changes.

| § | Finding | Kind | Severity |
|---|---|---|---|
| 2 | `STATUS.md` and `CLAUDE.md` had become history files that every session must read | trajectory | done 2026-10-16 (one point left) |
| 3 | Decisions "for the user to confirm" are scattered and unanswered | trajectory, correctness | high |
| 4 | `main` is eleven changes behind; tier 2 has never run on GitHub as its own job | release | medium |
| 5 | Each change updates eight or nine documents; three of them say the same thing | trajectory | medium |
| 6 | The checker is a 72-field object with ambient mode flags | architecture | medium, not yet |
| 7 | The emitter is a transcription of the old compiler's C++; say where that ends | architecture | low, note |
| 8 | Snapshot weight and one load-sensitive test | tests | low |
| 9 | No bug found; where correctness exposure is | correctness | none found |
| 10 | What the lesser models need most | process | medium |

## 2. The hand-off documents had become history files (done 2026-10-16, one point left)

**Was:** `STATUS.md` was 403 lines and 39 KB against its own rule ("the current state only, no history"): a block
of "for the user to look at" bullets for each of ten M3 changes and a 40-line paragraph narrating every M3 step.
`CLAUDE.md` was 29 KB, 11.5 KB of it one layout-table cell cataloguing every verification family (`v13`–`v37`),
appended on every change, plus counts that go stale. The two files every session starts from came to about
68 KB, and the questions for the user were buried among them (finding 3).

**Now** (user: "fix 2"; `DECISIONS.md` 2026-10-16): `STATUS.md` is 146 lines and 14 KB: where we are in one
paragraph, a "Waiting on the user" list (oldest first, the only place that asks anything; the first point of
finding 3), the next step, a one-line table of M3's steps, the numbers, known bugs. The catalogue is
`verification\README.md`, moved word for word; `CLAUDE.md` is 18 KB, holds no counts, and says that counts and
catalogues belong elsewhere. The open change's closing task points to the new homes.

**Left:** `DECISIONS.md` rows cut to one or two sentences with the document that holds the details (the
2026-10-13 to 2026-10-15 rows are 700 to 1,500 characters and repeat their change's `design.md`). On the
"Waiting on the user" list (item 12), since it moves details between files.

## 3. Decisions made by Claude are piling up unconfirmed (QB64 engineer, pragmatic engineer)

The project's compatibility rule is strict and good: do what QB64pe does, even where it is wrong; only crashes,
hangs, memory misuse and failed builds are fixed, each as a `DIVERGENCES.md` row. Under that rule Claude has, since
2026-10-13, made at least these choices alone and marked them "for the user to confirm": D-021 (three behaviours),
D-022, D-023, D-024, the widening of D-004 twice, the three forms of `m3-images-fonts-mouse` built instead of
marked, the four choices of `m3-mem`, the three of group 5 of `m3-typefields`, and the "beyond the proposal" items
of four changes. The phrases "for the user to confirm", "Claude's choice" and "put to the user" occur 20 times
across `STATUS.md`, `DECISIONS.md`, `DIVERGENCES.md` and the open change. There is no single list, no order, and
no record of which the user has seen.

This matters for correctness, not only tidiness: every one of these is a place where the new compiler does not do
what the old one does. Each is small and well argued, and each is probably right. But the rule that keeps the
project honest is that the user decides divergences, and that rule is being met by default through silence. The
2026-10-08 rule ("a failed compile may be fixed") is being read widely: D-023 and D-024 turn programs the old
compiler fails to build into working ones, which is a new behaviour, not a fix of one.

**What to do** (Claude's time: minutes; the user's time: one sitting):

- One section "Waiting on the user" in `STATUS.md`: one line per open question, oldest first, each with its
  register row or task number. Nothing else in the file asks anything.
- The user answers the current list in one message ("all as chosen", or exceptions). A question not answered
  within two changes is recorded in `DECISIONS.md` as accepted by default, and the row says so.
- A divergence that makes an old failed build succeed stays a register row, but its register row says "new
  behaviour" and not "fix", so the user can see the class growing.

## 4. `main` is eleven changes behind, and tier 2 has not run on GitHub as its own job (release engineer)

`main` is at `25e4ff6`, the merge of `m2-console-screenhide` on 2026-10-12. Branch `m3` is 30 commits and eleven
closed changes ahead of it. The GitHub run list for `rust.yml` shows 47 runs, every one on a push or a pull
request; no run of the `tier2` job by hand or by schedule has happened since the trigger was changed to those two
events on 2026-10-12. `STATUS.md`'s line that tier 2 "was seen 2026-10-13, 20 min 58 s" describes the last
pull-request run before that change. The weekly schedule runs on the default branch, so its first run on
2026-10-19 will test M2's code.

This is within the user's rule that CI is not a gate, and nothing here says tier 2 fails on GitHub. But two
unknowns named three days ago are still unknown (whether the runner gives the 66 screen programs a window; whether
two sound programs run without a device), and the longer `m3` runs alone the larger the surprise when it merges:
one 20-minute job may then fail for dozens of programs at once for a reason that has nothing to do with any change.

**What to do** (Claude's time: minutes; machine time: one 20-minute job; the user's time: one click and one merge):

- Dispatch `tier2` by hand on `m3` now ("Run workflow", branch `m3`), read it once, and record what the runner
  does with a shown window. If it fails for the environment, the design's fallback (a list run locally and not by
  the job) is ready and should be applied before the merge, not after.
- Merge `m3` into `main` at the close of `m3-declare-library`, and after that at the close of every two or three
  changes, as M2 was merged (four pull requests). A branch that carries a whole milestone is a merge of 40 commits
  and 500 files at once; the pull requests so far were reviewed by nobody but the author, so the merge is the only
  point where the user looks at the whole.

## 5. Each change updates eight or nine documents, three of which say the same thing (pragmatic engineer)

A change's close (task 6.1 of the open change is typical) touches `DIVERGENCES.md`, `SOMEDAY.md`,
`crates\README.md`, two `tests\*\README.md`, `CLAUDE.md`'s table, `ROADMAP.md`, `STATUS.md`, and before that
`study\00` §5 and the merged specs under `openspec\specs`. Three of these describe what the compiler supports:
`crates\README.md`'s "What the compiler supports so far" (a list that has grown to the length of a chapter and
is appended per change), `ROADMAP.md`'s checked items, and the merged specs (7,122 lines). `study\00` §5 is the
one home for measured facts, and that rule works; there is no such rule for "what is supported", so it is written
three times.

The test-time report puts 75% of active time outside commands. Reading the session histories in `STATUS.md`
suggests that a sizeable share of that is documents, not code. Rule 8 says a document's cost is the user's reading
time and the cost that stays; both grow with every copy.

**What to do** (Claude's time: an hour once, then less per change):

- One home for "what is supported": the merged specs, because the OpenSpec workflow writes them anyway. The list
  in `crates\README.md` becomes a pointer to `openspec\specs\language` and the one-line crate map it started as.
  `ROADMAP.md` keeps its checked items, which are a different grain (steps, not features).
- Counts leave every rule file (finding 2) and stay in the files that are regenerated (`tests\*\README.md` can be
  written by the runner's `--summary`, if it has one; else keep them there only).
- The standing rules that each `tasks.md` repeats in its header (no subagents, no staging, backslashes through the
  tools, the test cadence, `check_repo.py`) live once in a short file the proposal skill links, not in every change.

## 6. The checker is a 72-field object with ambient mode flags (compiler engineer, Rust engineer)

`Checker` in `sema\src\check\mod.rs` has 72 fields, and `sema\check` is 16,768 lines across 14 modules that all
`impl Checker`. Among the fields are mode flags that a caller sets around a call so that a callee behaves
differently: `len_place` (a whole user-type place is a value only when `LEN` asks), `mem_store` (a block-valued
call is allowed only when stored into a `_MEM` place), `no_join` (an implicit allocation may not run first in this
statement), `follow_on` (real errors are dropped after a marked declaration), beside the file-position state
(`defaults`, `option_base`, `dynamic_mode`, `explicit`), the program settings (`console`, `window_hidden`,
`asserts`, `exe_icon`, `echoes`, `runtime_parts`) and the dimension solver (`dim_facts`, `dims_open`).

Every field is documented and tied to a measurement, which is why it works. The cost is that M4's targets
(`qb64pe.bas`, 30,000 lines using everything at once, and the upstream programs that remain: event handlers,
`$VERSIONINFO`, `_SNDOPEN`) will each add a flag or two, and each flag multiplies the states a reader of a callee
has to hold. The `wildcard_enum_match_arm` lint, `forbid(unsafe_code)`, bytes everywhere and the overflow checks
in release are the right choices and should stay.

**What to do**: not a rewrite, and not now. At the start of M4, group the fields into four structs by concern
(the position in the file, the program's settings, the typing context of the current expression, the dimension
solver) and pass the typing context as a parameter where it is set around a call, so that `len_place` and
`mem_store` stop being state. Claude's time: a few hours; no behaviour change, tier 1 and the `typed` snapshots
prove it. The reason to wait is that the open change is mid-way and the regrouping touches every module.

## 7. The emitter transcribes the old compiler's C++; say where that ends (compiler engineer)

`codegen-cpp` writes the old compiler's text line for line: `#line` before every line, `allocate_with` is 260
lines of `out.push(format!(...))` reproducing the descriptor dance, and the call-site check compares the result
with `qb64pe.exe -z` as text. For M3 and M4 this is exactly right: parity is measured, not argued, and the C++
of Civil War Strategy was compared line by line (no call differs on 4,332 lines).

It also means `codegen-cpp` cannot be the M6 backend: its spelling is pinned to libqb's ABI and to the call-site
check. The roadmap says M6 "owns the ABI"; it does not say that this starts with a second backend crate beside this
one, which it must, since the call-site check would reject every improvement. Writing that one sentence into
`ROADMAP.md` now stops a future session from "modernising" the emitter in place. No work beyond the sentence.

## 8. Snapshot weight, and one load-sensitive test (test engineer)

367 snapshot files, 1.9 MB of them under `driver\tests\snapshots`, most of them `cpp` dumps of whole programs. A
change to a line every program shares (a prologue, a `#line` rule, the licence lines) rewrites dozens of them,
and "five changed `cpp` snapshots" has already been reported as a beyond-scope item of one change. The pressure
to accept all is real, and a snapshot accepted is a test that no longer tests. Tier 2's equality with the old
compiler is the check that matters; `cpp` snapshots should be kept for the smallest program of each feature, and a
new feature should add one, not three.

`screen_state_prompt_that_never_comes` (500 ms) failed once under load in a closing run and passed alone. A test
that depends on wall-clock time in tier 1 is a test that fails on the GitHub runner on a slow day; give it a
longer limit (five seconds costs nothing when it passes) or mark it `#[ignore]` with the build tests.

## 9. No bug found; where the correctness exposure is (all roles)

Reading found no bug, and today's local runs agree with `STATUS.md`. The places where a mistake would not be
caught by the current checks, for the record:

- **A behaviour that no corpus program and no upstream program exercises** is covered only by the verification
  programs, which are recorded and not run by any test. The 166 rows of `SOMEDAY.md`'s "QB64pe behaviours to
  review" are such behaviours, implemented QB64pe's way; a regression in one would be found only when a program
  that uses it joins a list. The differential tester covers the numeric operators across every type pair; nothing
  similar covers strings, files or arrays. A string and array differential group (concatenation, comparison,
  `MID$`, `LEFT$`, element and bound errors across every element type) would be generated by the same crate in an
  hour of Claude's time and would close the largest such gap.
- **The build cache's key** leaves out a header's own includes (stated in the design); a test that changes such a
  header would get a stale executable. Fine while no test has one; the first `DECLARE LIBRARY` slice program with
  a nested include should not use the cache.
- **Eight parallel `make` runs in the clone's root** share its libqb object folders; the runner builds the first
  program alone so that missing objects are built once (`run_legacy_tests.py` 1195), which is correct, but a clone
  whose objects are cleaned between two runner processes started at once is not protected. Not seen; noted.
- The 400 `unwrap`s and 114 `unreachable!`s are internal invariants; the CLI's panic hook, the LSP's
  `catch_unwind` around the parse and tier 1's `catch_unwind` around every input make a panic a reported internal
  error, not a silent failure. Right as it is.

## 10. What the lesser models need most (pragmatic engineer, documentation engineer)

The rules the repository gives a model are good: measure before design, no subagents, no wildcard arms, read a
run from a log, one error per statement, a program that fails goes on a shrink-only list. The record shows they
work: 311 tests, zero false errors, 86 hours to M3's eleventh step. The two things that would help a lesser model
most are the two cheapest in this review: a `STATUS.md` it can read in a minute (finding 2) and one list of what
the user still has to decide (finding 3). Everything else here is a note for M4 or M6.

Two patterns of the last week to watch, because they are how findings 2 and 3 grew: "beyond the proposal" items
in every change (each was justified; the mechanism is that a measurement made during a build reveals one more
form, and building it is faster than marking it), and `tasks.md` files whose "As built" paragraphs are longer than
the tasks. Both are a model doing more than asked and saying so, which is the better failure; the fix is where the
saying goes (one line in the queue, the paragraph in `design.md`), not the doing.

## 11. What not to change

The pipeline split into crates with the dependency graph as the layering; the IR's flat bodies, explicit jumps and
pending-error rule; bytes and byte columns throughout; the three tiers and the shrink-only lists; the call-site
check; the screen-state check and the scripted game; the binding to the reference clone's `qbx.cpp`, libqb and
`Makefile` until M6; the measurement discipline. The 2.1 GB of built verification executables are git-ignored and
rebuilt by `run.sh`; they cost disk only.

## 12. Order of actions

1. Dispatch `tier2` on `m3` by hand and read it once (user: one click; Claude: the reading). Merge `m3` into
   `main` at the close of `m3-declare-library` (§4).
2. Done 2026-10-16: `STATUS.md` cut to its own rule with the "Waiting on the user" list; the verification
   catalogue and the counts out of `CLAUDE.md` (§2, §3). Shorter `DECISIONS.md` rows still open.
3. The user answers the list in one message (§3).
4. Continue `m3-declare-library` at group 4 as planned.
5. One sentence in `ROADMAP.md` under M6 (§7); the long limit on the load-sensitive test (§8).
6. At the start of M4: the checker's fields grouped (§6); a string and array differential group (§9); the
   "what is supported" list reduced to one home (§5).
