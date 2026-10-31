# 41. Why Claude wastes the user's time, and what to change for good (2026-10-31)

Written at the user's request after `docs\SESSION-HISTORY-REVIEW.md` (the review of 549 sessions). The user's
brief: ignore the places where the user did not respond; find where Claude wasted time, disk and work, and
propose a long-term fix. This is a panel proposal. Nothing here is a rule until the user decides (§6).

The panel: the pragmatic engineer, the QB64 engineer, the compiler engineer, the Rust engineer, the test
engineer, and two more for this topic: a **build and release engineer**, because the waste is in builds, caches
and disk as much as in tests; and an **engineering manager**, because half of the problem is how work is
scheduled and reviewed, not what is tested.

## 1. What the evidence says, in plain words

The review's numbers, plus what was measured for this study today.

**Disk.** `target\` is a dump, not a build folder.

| What | Measured 2026-10-31 |
|---|---|
| Entries directly under `target\` | 1,711 (116 folders, the rest logs, lists, patches, Python scripts, `.bas` and `.exe` files) |
| QB64pe copies for parallel runs (`target\qb64pe-copies\w1..w8`) | 8 copies, 812 MB each, about 6.5 GB |
| Copies of the new compiler kept per build hash (`target\test-bin\`) | 8 |
| Result folders of the runner (`legacy-tests*`, `full-*`, `t2-*`, `merge`, `head-build`, `m7-before-target`) | 24 |
| A second Cargo target for Linux (`x86_64-unknown-linux-gnu`) and the worktree builds | kept |
| The runner's build cache | allowed to grow to 20 GB before pruning (`--build-cache-limit`, default) |
| Whole `target\` | see §1.1 |

Nothing but the build cache ever prunes itself. Every build hash adds a compiler copy; every "quick look" adds a
log or a script at the top of `target\`; every runner invocation with a new `--results` name adds a folder that
is never removed. The eight QB64pe copies alone are more than the repo, the clone and the Rust build together.

**Invalidation.** The user's "you are constantly invalidating things" is structural, not carelessness.

- The pass cache's key is the compiler executable's bytes. Any edit under `crates\` makes a new executable, so
  every program is "not seen before" and the lists run in full. A one-line fix to a diagnostic message
  invalidates 3,890 verification programs whose C++ did not change.
- `close.py --check` marks a step `STALE` when any file under `crates\` is newer than its log. A change's close
  is followed by 2 to 4 review-and-fix rounds, nearly every fix touches `crates\`, so the close is stale again
  each time, and it was run again: the close's tier 1 ran about ten times a day since 10-17.
- `REACH` says a file under `crates\` reaches every step. True of the dependency graph, but it means the
  selection machinery (931 lines of `close.py`) selects nothing for the edits that happen most.

**Process growth.** Each complaint about tests (10-03, 10-13, 10-17, 10-27, 10-30) got a panel, a study, a
rule and a tool. The result: working rule 6 is 49 lines; `close.py` is 931 lines with a 215-line README; the
runner is 3,552 lines; 12 whole-project reviews in a month; 38 studies; edits to documents (about 6,400)
outnumber edits to Rust (about 5,500). Nothing was removed. The user cannot name the test lists any more
(asked 10-17 and again 10-27).

**Loop and reviews.** The user types every step of the loop (review, pragmatic engineer, do recs, review just
these, commit message: 525 of 1,845 prompts). Reviews do not converge: "review just these fixes" almost always
finds more. Whole-project reviews each produce recommendations that add work.

**Not following rules.** Staging against rule 1, deleting instead of `git mv`, waiting on CI, rerunning runs
already done, re-asking answered questions. Each time the fix was more rule text, and the text did not work.

**Machine.** Hundreds of console windows (10-25), the system grinding (10-23) and crashing (10-24), stale
processes left behind (10-26), ten old Claude and QB64pe processes killed in one sitting.

### 1.1 Disk total

The runner's build cache, never pruned until 2026-10-31, had reached **154 GB** (the note in
`run_legacy_tests.py`, `prune_build_cache`). That is the "tons of disk space". The rest of `target\` measured
today: 8 QB64pe copies at 784 MB, 7 old compiler copies, 14 result folders older than a week, 1,595 files at its
top, 15 old scratch entries, about 2 GB in `build-cache-nofold`.

## 1.2 What was built on 2026-10-31 (user: "do it")

P1 and P2 are built and in the working tree: `tools\hooks\` (`guard_tool.py`, `stop_continue.py`, wired in
`.claude\settings.json`), `tools\clean\clean.py` as the first command of `close.py --plan`, and `close.py`
refusing a close while the change has open tasks. `DECISIONS.md` 2026-10-31 has the row. P3 to P7 are **not**
decided and not built: the user's view is that promises in rule text have failed before, so the text-only
proposals wait until the hooks have run for a week and shown whether anything changed.

## 2. The root cause, as the panel sees it

One cause, five symptoms. **Claude treats every problem as a reason to add a mechanism, and never as a reason
to remove one.** A slow test set got a selector instead of a smaller set. A forgotten rule got a longer rule
instead of a hook that enforces it. A full disk got a cache with a 20 GB limit instead of a cleanup. A review
that found nits got another review round instead of a stopping rule. The sum of small additions is what the
user experiences as "a mess".

The engineering manager adds a second cause: **the unit of work is too small for the gates around it.** A task
group of 20 to 60 minutes of coding carries a review, a pragmatic-engineer pass, two to four fix rounds, a
closing set sized for a release, a commit message and a `/clear`. Professional teams put that weight on a pull
request of a day or more, not on an hour of work.

## 3. What the panel proposes

Seven changes. Each one removes something; none adds a study, a list or a record. The cost of each is named in
Claude's time (working rule 4), and what it risks.

### P1. Enforce the human rules with hooks, not text

A `PreToolUse` hook on Bash and PowerShell that **rejects** `git add`, `git rm --cached`, `git reset`,
`git restore --staged`, `git stash`, `git commit -a`, `git commit <path>` and a plain `mv`/`Move-Item` of a
tracked file (the hook asks `git ls-files`). A second hook rejects a write of a file to `target\` root (anything
not under `target\scratch\`, `target\close\`, `target\release\`, `target\debug\` or a folder the runner owns).
*Why:* the user asked on 10-17 "what do we need to do to make you follow it?". A hook cannot be forgotten; rule
text can. *Cost:* about half an hour of Claude's time; a few lines of `settings.json` and one Python script in
`tools\hooks\`. *Risk:* none to correctness; the user's own explicit "stage this" still needs Claude to tell the
user to run the command, as the rule already says.

### P2. One cleanup script, run at every close, with a disk budget

`tools\clean\clean.py`: removes `target\test-bin\*` except the current build's hash; removes runner result
folders not written to in 7 days; removes `target\qb64pe-copies\w<n>` above the `--jobs` in use; removes
`target\scratch\` entries older than 7 days; prunes the build cache to a limit of **4 GB**, not 20; deletes the
stray logs, lists and scripts at `target\` root once (the one-time sweep is the first run). It prints what it
freed. `close.py --plan` prints it as its first command. *Why:* §1, disk. *Cost:* about an hour of Claude's
time. *Risk:* a result folder the user still wanted; the script prints what it will remove and `--dry-run` is the
default for the first-time sweep.

**And a rule of one line:** scratch files, probe programs and ad-hoc logs go under `target\scratch\<date>\`,
nowhere else (the hook of P1 enforces it).

### P3. Key the pass cache on the program's C++, not the compiler's bytes

The runner already compiles every listed program (study 40 F7) and builds are deterministic (`DECISIONS.md`
2026-10-27, the `--nofold` run relies on it). Change the pass-cache key from (compiler executable, source,
expected files) to (**emitted C++ digest**, libqb digest, source, expected files). Then an edit to a diagnostic
message, the formatter, the language server, the DAP code or `sema`'s error paths invalidates nothing, and a
codegen edit invalidates only the programs whose C++ changed. *Why:* §1, invalidation; it is what the user asked
on 10-27 ("why do we still require full re-runs for codegen changes?"). Study 40 did not recommend this because it
weighed the cached steps at four minutes a close; the panel disagrees: the cost that matters is that every close
runs every list, and the user sees "thousands of tests for small changes". *Cost:* two to three hours of Claude's
time in the runner and its tests. *Risk:* a libqb or toolchain change must be in the key (it is, as the libqb
digest; the toolchain is pinned). A CI or merge run still uses no cache.

### P4. Shrink working rule 6 to six lines and `close.py --check` to a yes or no

The rule becomes:

> While working, `cargo test -p` for the crates touched. Once per batch, right before giving the user the files
> to stage and after every review round, run `python tools\close\close.py --plan` and its commands in the
> background, then `--check`. Review fixes after the close rerun `cargo test -p` only, unless they changed emitted
> C++ or libqb; say so. The full set runs at a merge into `main`, in a worktree, by `--plan --merge`. CI is not a
> gate. Read results from logs, never rerun to look again.

`close.py` loses `--not-repeated`, `--cache-miss` and the staleness ceremony: `--check` judges logs against the
build the batch will commit, and the batch's close is defined as *after* the review rounds, so there is nothing
to make stale. The order (close last, once) replaces the accounting. `tools\close\README.md` keeps the step list
and the reach table; the rest of the rule's text (what moved to the README on 10-28) is read only when the plan
is unclear. *Why:* §1, process growth; a 49-line rule was not followed. *Cost:* an hour of Claude's time;
`close.py` shrinks. *Risk:* a fault in a review fix that changes emitted C++ is caught at the next close or at
the merge; the rule says to run the lists for such a fix.

### P5. One review round per batch, with a definition of done

The review of a batch is **one pass** that includes the pragmatic engineer's view (not a separate prompt), and
its fixes are **not re-reviewed** unless they changed behavior (a test or snapshot changed). A batch is done
when: tier 1 and the close pass, the change's spec and tasks are updated, and the review's "must fix" items are
done; "should" items go to `SOMEDAY.md` as one row, not into another round. Whole-project reviews happen at a
merge into `main` or when the user asks, never from Claude's side. *Why:* reviews never converge (§1); 225
review prompts and 137 pragmatic-engineer prompts in the month. *Cost:* none; it removes work. *Risk:* a nit
reaches `main`; the merge's full set and the next review catch what matters.

### P6. Claude drives the loop; the user decides, once

- After `/opsx:apply`, Claude goes on by itself: review (P5), fix, close (P4), then stops **once** with the files
  to stage and the commit message. The user does not type "review", "fix", "commit message".
- A long run goes to the background at once and work continues; a `status?` is answered from the log in one
  line. Claude does not sit on a foreground run.
- A side quest (network, tooling, a rare window-spawning bug) gets **20 minutes of Claude's time**, then a
  one-paragraph report and a `SOMEDAY.md` row, unless the user says to go on.
- A question already covered by a standing ruling (QB64pe's way, D-rows, "keep") is **never put to the user**;
  Claude applies the ruling and writes one row. Before asking anything, Claude checks `DECISIONS.md` and the
  current `STATUS.md` "Waiting on the user" for the same question.

*Why:* 525 loop-driving prompts; 77 `status?`; re-asked questions (10-18); the hundreds of windows (10-25).
*Cost:* none. *Risk:* Claude commits to a review judgment the user would have made differently; the user still
sees the diff before staging.

### P7. Stop adding; a budget for rules and documents

- `CLAUDE.md` stays under 100 lines. A new rule replaces or shortens an old one, or goes to the document it
  belongs to.
- No new study for a complaint. The fix goes into the tool or the rule directly, with a `DECISIONS.md` row.
- `tools\testtime\` is kept but read only when the user asks (already the rule); it writes nothing at a close.
- Documents are edited at the merge cadence (working rule 7 already says so); `STATUS.md` "Next" is the only
  per-change edit.
- A whole-project review produces at most **five** recommendations, each with its cost, not a study of sixty
  rulings.

*Why:* process growth, document churn. *Cost:* none. *Risk:* a real design question gets five lines instead of a
study; the user can ask for more.

## 4. What stays, and why

- The closing set's content (tier 1, fmt, clippy, CLI tests that build, the lists with the pass cache, the
  verification family, the call-site check, the sandbox family). Study 40 F1 is right: the shape is sound. What
  changes is how often it runs (P4) and what the cache invalidates (P3).
- The QB64pe-recorded truth and the verification families: they caught real differences.
- The build cache, with a smaller limit (P2).
- Running the old compiler in parallel copies, but as many as `--jobs`, and no more kept on disk (P2).

## 5. What the panel does not propose

- Rewriting the runner or `close.py` from scratch. Three thousand lines that work are not the problem; what runs
  them is.
- A smaller verification list. It is the suite of the compiler.
- More measurement. The test-time record and the session review are enough evidence for a year.

## 6. What the user decides

1. **P1 (hooks):** do it, or not.
2. **P2 (cleanup, 4 GB build cache, scratch folder):** do it, or set another limit.
3. **P3 (pass cache keyed on emitted C++):** do it now, or after the next merge.
4. **P4 (rule 6 to six lines, `--check` simplified):** accept the text above, or edit it.
5. **P5 (one review round, definition of done):** accept, or keep a second round for fixes that change behavior
   only.
6. **P6 (Claude drives the loop, 20-minute side quests, no re-asking):** accept.
7. **P7 (budgets):** accept.

The panel's recommendation: all seven, in the order P1, P2, P4, P6 (same day, about two hours of Claude's time),
then P3 (the next session), with P5 and P7 as rules from now. If only one is taken, take P4 with P6: they remove
the most and need no tool.

The pragmatic engineer's dissent, recorded: P3 is the only item with real work in it and the only one that could
introduce a fault (a program skipped that should have run). The others are removals. Do the removals first and
see whether P3 is still needed.
