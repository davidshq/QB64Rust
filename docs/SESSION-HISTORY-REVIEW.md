# Session history review: why progress slowed

Written 2026-10-31 at the user's request, from every Claude Code transcript of this project (2026-10-01 to
2026-10-31): 549 sessions, 1,845 user prompts, about 51,700 tool calls. Its purpose is to let another model
(Fable) work out what Claude has been doing wrong. It is a log and an analysis, not a decision; nothing here
changes a rule until the user says so.

How it was made: a script read the transcripts (outside the repo) and counted per session the tool calls, the
shell commands by kind (cargo test, tier 2 lists, `close.py`, clippy, …), the wall time each command took, and
the files edited by area; the user's prompts were read in order. Git history gave the lines added per area per
day. Wait times are approximate: a command's time includes any permission prompt, and a background command
counts only until it returned.

## 1. The numbers

### 1.1 Per day: sessions, prompts, test runs, edits

`n_*` = number of commands of that kind; `min_*` = minutes waited on them; `e_*` = Edit/Write calls on files in
that area. "tier2" counts any run of the program lists (`run_legacy_tests.py`, `--list`, `--glob`, `run.sh`,
pass cache).

| day | sessions | prompts | tool calls | n cargo test | n tier2 | n close.py | n clippy | min cargo test | min tier2 | e rust src | e rust tests | e tools | e docs+study | e openspec |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 10-02 | 5 | 55 | 568 | 2 | 22 | 0 | 0 | 0 | 24 | 0 | 0 | 17 | 85 | 0 |
| 10-03 | 17 | 109 | 1371 | 27 | 72 | 0 | 7 | 3 | 23 | 27 | 6 | 41 | 177 | 68 |
| 10-04 | 12 | 63 | 1415 | 99 | 62 | 0 | 45 | 26 | 52 | 230 | 30 | 53 | 91 | 76 |
| 10-05 | 15 | 49 | 1021 | 64 | 46 | 0 | 16 | 31 | 70 | 160 | 37 | 10 | 57 | 85 |
| 10-06 | 7 | 26 | 573 | 39 | 34 | 0 | 14 | 15 | 24 | 69 | 6 | 3 | 42 | 50 |
| 10-07 | 21 | 56 | 2083 | 158 | 68 | 0 | 37 | 31 | 41 | 417 | 38 | 37 | 122 | 81 |
| 10-08 | 18 | 53 | 1908 | 81 | 66 | 0 | 40 | 16 | 41 | 217 | 25 | 26 | 183 | 173 |
| 10-09 | 18 | 47 | 1447 | 92 | 78 | 0 | 30 | 24 | 43 | 240 | 8 | 77 | 66 | 39 |
| 10-10 | 22 | 74 | 1943 | 65 | 131 | 0 | 20 | 23 | 85 | 146 | 7 | 177 | 223 | 119 |
| 10-11 | 14 | 40 | 1749 | 96 | 107 | 0 | 30 | 44 | 70 | 233 | 2 | 62 | 150 | 131 |
| 10-12 | 24 | 63 | 1811 | 70 | 107 | 0 | 24 | 57 | 285 | 248 | 5 | 21 | 190 | 120 |
| 10-13 | 23 | 52 | 2413 | 103 | 138 | 0 | 31 | 45 | 61 | 222 | 16 | 125 | 204 | 181 |
| 10-14 | 29 | 52 | 2592 | 91 | 148 | 0 | 27 | 38 | 87 | 215 | 18 | 100 | 174 | 125 |
| 10-15 | 32 | 69 | 2402 | 84 | 114 | 0 | 29 | 23 | 54 | 248 | 19 | 40 | 172 | 116 |
| 10-16 | 19 | 58 | 1459 | 60 | 83 | 0 | 21 | 49 | 41 | 132 | 27 | 60 | 147 | 79 |
| 10-17 | 39 | 114 | 3681 | 149 | 240 | 27 | 44 | 64 | 54 | 264 | 69 | 228 | 291 | 145 |
| 10-18 | 8 | 30 | 579 | 15 | 23 | 13 | 5 | 12 | 3 | 35 | 5 | 23 | 100 | 19 |
| 10-19 | 18 | 52 | 1470 | 65 | 65 | 21 | 16 | 36 | 11 | 80 | 19 | 89 | 79 | 71 |
| 10-20 | 17 | 44 | 1714 | 71 | 106 | 35 | 16 | 41 | 28 | 122 | 16 | 72 | 60 | 95 |
| 10-21 | 17 | 66 | 2038 | 89 | 125 | 27 | 23 | 63 | 58 | 173 | 8 | 72 | 57 | 139 |
| 10-22 | 17 | 45 | 2456 | 135 | 190 | 33 | 25 | 80 | 50 | 208 | 8 | 57 | 51 | 168 |
| 10-23 | 11 | 33 | 1488 | 48 | 87 | 9 | 18 | 35 | 20 | 151 | 7 | 41 | 20 | 144 |
| 10-24 | 15 | 42 | 1944 | 81 | 115 | 17 | 26 | 70 | 35 | 177 | 16 | 75 | 63 | 123 |
| 10-25 | 26 | 77 | 2720 | 159 | 105 | 42 | 35 | 121 | 20 | 274 | 123 | 84 | 147 | 181 |
| 10-26 | 27 | 93 | 2224 | 97 | 137 | 41 | 26 | 78 | 49 | 227 | 52 | 85 | 111 | 92 |
| 10-27 | 27 | 146 | 2129 | 86 | 113 | 25 | 17 | 61 | 51 | 97 | 46 | 70 | 179 | 76 |
| 10-28 | 13 | 70 | 1803 | 68 | 82 | 16 | 15 | 85 | 57 | 119 | 61 | 90 | 109 | 51 |
| 10-29 | 21 | 86 | 1273 | 34 | 60 | 22 | 13 | 23 | 54 | 66 | 40 | 32 | 83 | 42 |
| 10-30 | 13 | 68 | 1334 | 37 | 87 | 91 | 8 | 34 | 35 | 22 | 2 | 154 | 138 | 19 |
| **total** | 549 | 1845 | 51701 | 2265 | 2817 | 419 | 658 | 1228 | 1526 | 4819 | 716 | 2022 | 3592 | 2808 |

Totals over the month: about 656 session-hours; 2,265 `cargo test` runs, 2,817 runs of program lists, 419
`close.py` runs, 658 clippy and 1,033 `cargo fmt`-tagged commands. Waiting on commands, by kind: `cargo test`
about 20 h, program lists about 25 h, `cargo build` 17 h, clippy 12 h, fmt 7 h, git 11 h. Waiting on the user
to answer `AskUserQuestion` (108 questions): about 18 h. Edits: 13,649 Edit + 3,272 Write calls; of those, Rust
source 4,819, docs + study 3,592, OpenSpec 2,808, tools 2,022, test fixtures 2,179.

**Edits to documents (docs + study + OpenSpec, 6,400) outnumber edits to Rust source and Rust tests (5,535).**

### 1.2 What landed in git, lines added per day

From `git log --numstat` (author date; renames left out). "rust src" = `crates/**.rs` outside tests and
snapshots.

| day | commits | rust src | rust tests | tests/verification | tools | docs+study | openspec | rust src share |
|---|---|---|---|---|---|---|---|---|
| 10-04 | 21 | 8001 | 5167 | 45237 | 462 | 896 | 2673 | 12.7% |
| 10-05 | 7 | 2984 | 2064 | 2904 | 41 | 331 | 1066 | 31.8% |
| 10-06 | 3 | 2292 | 620 | 3763 | 0 | 176 | 289 | 32.1% |
| 10-07 | 11 | 8332 | 3755 | 5602 | 3389 | 1275 | 2567 | 33.4% |
| 10-08 | 4 | 5553 | 3597 | 216518 | 81 | 867 | 2633 | 2.4% |
| 10-09 | 7 | 2873 | 3718 | 8850 | 645 | 1121 | 884 | 15.9% |
| 10-10 | 5 | 3088 | 3289 | 10957 | 1965 | 1111 | 2956 | 13.1% |
| 10-11 | 7 | 3393 | 6916 | 11581 | 0 | 761 | 2705 | 13.4% |
| 10-12 | 12 | 2932 | 3270 | 10499 | 2122 | 998 | 4303 | 12.1% |
| 10-13 | 6 | 3028 | 3571 | 17081 | 5357 | 1279 | 4351 | 8.7% |
| 10-14 | 14 | 3220 | 5590 | 31988 | 1088 | 1178 | 5109 | 6.7% |
| 10-15 | 7 | 2820 | 3951 | 6231 | 262 | 590 | 2659 | 17.0% |
| 10-16 | 12 | 2063 | 5144 | 4064 | 4634 | 2335 | 2139 | 10.0% |
| 10-17 | 14 | 3119 | 4102 | 24171 | 2541 | 1558 | 3581 | 7.9% |
| 10-18 | 5 | 3691 | 746 | 12541 | 399 | 3187 | 824 | 17.2% |
| 10-19 | 7 | 946 | 867 | 2990 | 2268 | 810 | 2763 | 0.1% (libqb import) |
| 10-20 | 5 | 1625 | 1457 | 13168 | 1662 | 447 | 2321 | 7.8% |
| 10-21 | 5 | 1410 | 3488 | 4782 | 3829 | 602 | 3389 | 8.0% |
| 10-22 | 4 | 713 | 1832 | 11547 | 639 | 412 | 2958 | 3.9% |
| 10-23 | 5 | 2054 | 5026 | 4494 | 390 | 206 | 2512 | 14.0% |
| 10-24 | 10 | 6262 | 4151 | 14343 | 2300 | 847 | 5596 | 18.7% |
| 10-25 | 11 | 8721 | 4985 | 67464 | 1737 | 1068 | 4506 | 9.7% |
| 10-26 | 9 | 3237 | 2307 | 4717 | 1145 | 1496 | 2297 | 19.7% |
| 10-27 | 5 | 1311 | 2287 | 1047 | 784 | 890 | 1745 | 15.1% |
| 10-28 | 3 | 35 | 71 | 433 | 479 | 461 | 759 | 1.5% |
| 10-29 | 8 | 1658 | 1329 | 1203 | 681 | 260 | 1479 | 22.5% |
| 10-30 | 5 | 1840 | 875 | 1009 | 1830 | 942 | 1520 | 21.3% |

Rough reading: new compiler source ran at about 3,000 lines a day from 10-04 to 10-18 and about 1,500 a day
from 10-19 to 10-30 (10-24/25 the exception), while OpenSpec text stayed at 1,500 to 5,500 lines a day
throughout. On most days the OpenSpec change files alone are as large as the compiler code they describe.

## 2. Timeline

(Filled in from the prompts, in order. User quotes are verbatim, typos included.)

### 2026-10-01 to 10-03: study, panels, and the switch to a Rust rewrite

- 10-01: the goal starts as "modularize the QB64pe BAS codebase so that we can unit test it". The first panel
  (pragmatic, QB64, languages, test engineers) is asked to review the plan, then whether to refactor first.
- 10-02: panel reviews of QB64pe vs QB64Fresh; decision to rewrite in Rust (new `QB64Rust` repo); the IDE
  dropped in favor of a VS Code extension. The user asks for a no-subagents rule and "never save things to your
  memory ONLY". Eight `status?` prompts in 40 minutes while a long study runs.
- 10-02/03: panels on the old VS Code extension, then a full review of QB64Fresh, its docs branches, FreeBASIC.
  Repeated "review your documents and tools", "is 00-synthesis.md updated", "archive the crap we've
  accumulated", "soften language" passes. **Two days of study and reviews before the first compiler code.**
- 10-03: M1 (VS Code extension scaffold) via OpenSpec. The user: "why haven't we run the tests localy? why are
  you waiting on github actions?" First sign of waiting on CI.
- 10-03 evening, golden corpus: seven `status?` in an hour, then "this is taking a long time? are we goign to
  need to do this regularly?" and "if we are running it all the time, its really going to slow dev velocity
  dsown". **The concern about test time is raised on day 3.**
- 10-03 23:30: the backslash-collapsing shell bug corrupts files four times; becomes CLAUDE.md working rule 5.

### 2026-10-04 to 10-07: M2, the working loop forms

- The per-change loop is set: `what's next?` → `do it` → `review changes` → `pragmatic engineer on above?` →
  `do prag recs` → `review just these changes` → `provide concise commit message for staged`. Each step is a
  prompt by the user; a change takes 5 to 8 of them, plus a session `/clear` between changes.
- Big-picture reviews ("are we on the right path, not nits") on 10-04 15:29, 10-04 21:44, 10-05 20:25, 10-05
  22:41. Each produces a list of changes that is then done.
- 10-04: Rust setup review (lints, custom rules) is done in full ("do as suggested").
- 10-05: the user finds 1,400+ generated `.bas`/`.txt` files being committed each time ("I've been committing
  all these … assuming they are needed").
- 10-06: "a concise list of what we plan to do from now till completion" (becomes `ROADMAP.md`). Panel on
  80-bit floats.
- 10-07: glossary written on request.
- 10-07: Fable architecture review ("We use Fable to help us set our path then use less expensive models for
  implementation"). A panel review again at 23:15 the same day, and on 10-08 15:59 ("Consider the panel an
  architectural review"), 10-10 03:09, 10-10 20:24. **About one big-picture review a day**, each followed by
  "do recs", each recommendation adding work, documents or process.
- 10-07 22:24: "there are so many files - 2815 that are uncommitted, it's impossibe to know what should be
  committed". Generated output again in the working tree.

### 2026-10-08 to 10-12: M2 slices; the review-fix-review spiral; test speed-ups

- 10-08 20:21, key user ruling: "I want you to do what QB64pe does generally … add them to a doc for review, but
  implement them QB64pe's way." (Later becomes the `SOMEDAY.md` "behaviours to review" rows.)
- 10-08: side quests the user asked for: Rust ecosystem crates review, crate versions, toolchain version.
- 10-09: "Give me the short bversion - tier , one end-to-end programs )total), full tier 2" / "is there anyting
  we can do to shorten this without it resulting in poorer code?" Then Python lint side quests (ruff vs flake8,
  docstrings, "Is there a reason we are using Python instead of Rust here?").
- 10-09 23:06: "how long does tier 2 usually take to run?"
- 10-10 06:05: the user tries to break the loop: `/opsx:apply` "do all eight streps without stopping, if you hit
  a question, record it in an md and then move on". (The loop came back next session.)
- 10-10 14:07 to 17:36: a whole afternoon making QB64pe run in parallel for the corpus (12 min → 3 min 44 s),
  then folding `run.sh` into the runner, then four rounds of "review just the latest changes" → "fix" on the
  runner. 10-10 15:33 user rule: count effort in Claude's time, not a person's (now CLAUDE.md working rule 4).
- 10-11 17:30, user: "revioew your changes, youd on't need to rerun testrs you've just run unless you make
  changes that affect those tests". **First explicit complaint about reruns.**
- 10-12: `m2-console-screenhide` staged review goes review → fix → review just these fixes → fix → review just
  these fixes → fix (three rounds, 15:22 to 17:53). **Each review of a fix finds something new to fix.**
- Pattern visible from 10-09 on: one commit per task group, each with its own session(s): `what's next?` →
  `do group N` → `review staged` → `fix` → `review just these` → `commit message` → `/clear`. M2 ran to about
  10 groups per change, so 40 to 60 prompts per change, most of them one to four words.

### 2026-10-13: M3 starts; first test-cadence intervention

- 10-13 01:41, user: "it feels like we run the tests over and over and over again and that maybe this is a waste
  sometimews? … like we are -s-pendign a lot of time just running tests? … not sure why you are so insistent on
  making ci be green. its cool to check occasionally, but it also takes a lot of time". Followed at 01:50 by "are
  we running only the testrs we need to? or are we running whole suites …". This produced the first version of
  working rule 6.
- The response was more process: test timing records (`tools\testtime\`), a study of session runtime. Same day
  14:51: "Do the test findings reveal any patterns, aberrations, unnecessary runs".
- Side quest same night: replace volta with mise, Node 24 machine-wide.
- 10-13 14:53: the user pastes Claude's own analysis of the test record: 823 `cargo test` runs costing about 3
  h, **453 of them started within three minutes of the previous one ending**; the median whole run grew from 6 s
  (10-03) to 24 s (10-12) because call-site and corpus programs were put into tier 1; 52 foreground runs over
  five times their median (about 3.6 h), several cut off at the 10-minute foreground limit; slice list run 10–11
  times on single days (40 min), the differential list 12 times in a day (49 min), the call-site check 15–18
  times in a day.

### 2026-10-13 to 10-17: M3 features at a steady pace, under a growing review load

- M3 changes in these days: sound, console files, Civil War Strategy screens, image built-ins/fonts/mouse,
  type fields, whole-array assignment, DECLARE LIBRARY, `_SNDOPEN` family, `_ARRAYCOPY`, version info, event
  handlers, CHR$ in CONST/`_ERRORLINE`. **This is the most productive stretch**: about one OpenSpec change a
  day or more, each a real feature.
- Yet nearly every task group still goes `review all changes` → `fix` → `review just these fixes` → `fix` →
  (sometimes a third round) → commit message → `/clear` → `what's next?`. Counted in the prompts of 10-13 to
  10-17: about 45 "review" prompts and 40 "fix" prompts.
- 10-14 18:27, user: "look at the tests we've been running, are we wasting time anyuwheree? it seems like its
  still taking a long tiome?" (second time in two days).
- 10-15 00:00, user: "I stage things you idiotr" (Claude staged files against Human Rule 1).
- 10-16 13:43, user asks a Fable panel for "things that could derail us, are slowing down our trajectory".
  The result (`study\30`, the ninth review) is put to the user **item by item: about 60 rulings** answered
  "Accept"/"Keep" over 40 minutes (14:35 to 15:18). Several rulings ask for more measuring ("Do measure and
  apply accordingly"), one adds a test to watch a QB64pe behavior, one adds old error wording beside new
  wording ("Do it for all D-rows that have runtime paths"), each becoming work.
- 10-16 15:22 to 15:59: a tier 2 run on CI fails (screen programs time out); user: "has it ruun successfully
  locally? i'm not as concerned about the ci" and "lets not hold up on ci tests … put this into tyour rules".
- 10-17 01:51: `study\31` "development velocity plan" implemented, then reviewed and fixed twice.
- 10-17 14:11 to 14:39, after two `status?` prompts during a close: "why does this take so long? it feel likes
  we are spending a lot of time oin this? was is a cslice-listg? what is fo,ding? what is without folding? what
  iks the differential list? what is the call-site check?" / "I thought we had made multiple copies of QB64pe
  so we wreren't stuck waiting" / "**it feels like we are doing too small batches of changes to be making all
  these tests run every trime**. we probably need a bettere way? what is best practices?". Produced `study\32`
  (test cadence panel) and, the same day, `tools\close\close.py` (first `close.py` runs are on 10-17).
  **The user does not know what the test lists are; the test system has grown past what its owner can follow.**
- 10-17 14:50 and 15:15: the user now has to write "fix issues, don't run tests" / "fix, don't run test" into
  the prompt.
- 10-17 16:52 to 16:55: "are we using the new testing practices we outlined in the study?" / "do we need to
  run it? didn'tm you just run it? if you did it as long as it was a good rujn we don't need to do ikt againb.
  also, **why did you not follow the rule? what do we need to do to make you follow iut?**" Answer: another
  expert panel, more rules.
- 10-17 19:05, during `_OPENCLIENT`/`$EMBED`, after two `status?`: "**wtf? come onm? how trhe fuyck is this
  happening? you are still running so many fucking tests?**"
- 10-17 20:44: "how would a professional dev team do this? I feel like we need a way that we can keep
  developing while tests run?" → a study on professional practice → worktrees for merge runs.
- 10-17 21:32: test runs open windows and make the machine "somewhat less than usable" → a VM/sandbox
  investigation (later the Windows Sandbox work of 10-29/30).
- 10-17 23:12 to 23:41: kept QB64pe behaviors of `m3-events` walked through one by one ("Kep QB64pe way",
  "keep", "keep for now"). The user rules "keep QB64pe" every time; the asking still happens.

### 2026-10-18 to 10-20: tenth review, document archiving, merge runs

- 10-18 20:43, user to a Fable architect: "Ways we should increase velocity? **It feels like this is taking a
  long time...**" → `study\35` (tenth review). Then 30 minutes archiving and de-duplicating studies, with three
  "did you lose anything" checks.
- 10-18 22:00: "So do we still need to do 1? I feel like they were probably run recently". 22:02: "what is up
  with 4? i feel like I answered those earlier didn't I? when you first asked about them?" → "give me one by one
  the items i need to render a decision on". **Claude re-asks questions already answered.**
- 10-19 21:16 to 21:27, a merge into `main`: "you push the tag, didn't the full set get done with ci?" / "run the
  full merge" / "we are running tests in another cvonversation in the worktree I think? so what comes after
  that" / "didn't we just finish 1 or was that a different set of tesets?" / "run verification families".
  **The user can no longer tell which test set is which or which already ran.**
- 10-19 21:54: "why didnt you finish the tasks?"
- 10-20 00:50, user sets a concrete goal: "focus on wayteevr is needed to get civilw ar strategy to compile with
  QB64Rust" / 02:33 "the whole game, not just two turns". (One of very few goals stated as an end result.)

### 2026-10-20 to 10-25: M4, the review loop gets a pragmatic-engineer step each round

- From 10-19 the loop is `review changes` → `pragmatic engineer on above?` → `do prag recs` → `review just
  these fixes` → `pragmatic engineer on above?` → `do prag recs` → `review just these fixes` → … The user types
  these. Examples: 10-21 12:23 (two rounds), 10-21 14:33 (three), **10-21 15:50, a DRY refactor: four rounds**
  (15:50 to 16:39), 10-21 18:04 (three), 10-22 19:18, 10-23 14:24, 10-24 18:50, 10-24 22:16, 10-25 00:30 (two
  each). A "review just these fixes" almost never comes back clean.
- 10-21: merge runs moved to a worktree; then "integrate the things from your worktree into the main worktree,
  do the docs" and remove the worktree.
- 10-21 19:18: "what remains beforer we are at parity with QB64pe?" (a good goal question; answered by an
  OpenSpec proposal).
- 10-22: a VS Code extension test fails on CI because of a feature change (`_DEVICES` now supported).
- 10-23 20:22 to 20:56: the machine "seems to grind almost to a halt"; an investigation of processes, ten old
  Claude/QB64pe processes killed, Pylance file watching. 10-24 13:43: "system crashed, resume".
- 10-24 14:38: "the message realy isn't very clear, neither have been a lto of your previous oen" (commit
  messages written in the project's compressed house style were not clear to the user).
- 10-24 19:32: "archive, run full set for merge, create a pr, update roadmap" in one prompt (user batching to
  save round trips).
- 10-25 03:24: M4 done. M5 (debugging, DAP) starts after another Fable panel (03:32).

### 2026-10-25 to 10-27: M5 and M6; machine overload; the 165 rows; tests questioned again

- 10-25 16:36 to 16:58, a "test environment" item: "too many windows" / "**I think you started hundreds of the
  windows, fix it**" / "canb't you test this witghout doing crazyuines?" / then "I'm not sure this is actually
  worth doing anything about? Maybe just note it? itrs pretty rare right?" **Claude chased a rare issue the user
  judged not worth fixing, at the cost of making the machine unusable.**
- 10-26 11:44: "things are running slowly, sometimes you don't close your old sessions" (stale sessions and
  processes left running).
- 10-26 14:07: another architect/panel review ("Don't consider anything unchangeable") → `study\38`, the
  direction review. The 11th or so whole-project review in 25 days.
- 10-26 19:47 to 20:07: "how much are we passding in context wioth each message?" Memory held 10k ("which seems
  like a lot, especially bc you are supposed to keep thingfs in-repo"); `LAYOUT.md` split out of CLAUDE.md.
- 10-26 20:50: "youa re taking a really long time inthe toher session. Why? what have yhou spent yiourrt time
  on?"
- 10-26 21:29 to 22:54: why QB64pe must be installed beside it → the bundled C++ toolchain (`m6-toolchain`),
  user-prompted install, Windows 7 question → SOMEDAY.
- 10-27 01:31: "**The 165 rows.** 'QB64pe behaviours to review' waits on the user and grows with every change.
  By the 2026-10-08 rule most of them are already decided (kept)". The user goes through them: about 20
  prompts answered "A", many with "panel?" first; then "**always give the panel thoughts**" (now in CLAUDE.md
  rule 4). Out of it comes the fixes switch (`qb64rust.toml`, a master switch plus per-fix names), another
  design and change.
- 10-27 01:30: the user challenges a review's claim of "one real program" ("I thought we had like 100+ real
  programs?").
- 10-27 02:05 to 02:23: a direction review edited to remove completed items, then restored from git under two
  names because the edit lost content.
- 10-27 02:57 to 03:00 and 15:23 to 15:38: the user has to say "I'm explicitrly telling you to satged" and
  "you stage batch 1, i'm giving hyou eplciit permission": the no-staging rule now blocks the user's own
  instructions.
- 10-27 14:22: "it looks like there isn't much left to be done?" (on the roadmap). 15:46: "Do we have mac and
  linux support built in?" → roadmap item.
- 10-27 16:04: DECISIONS.md split into DECISIONS and DECISIONS-SMALL ("there is a lot of minutaie").
- 10-27 18:27: "**why do we still require full re-runs for codegen chnages?** have a panel of experts weigh in on
  our tests and how they comapre to otrher languages and best practices" (the fourth test panel: 10-03,
  10-13, 10-17, 10-27). Then **11 `status` prompts in 33 minutes** (18:56 to 19:28) while a run finishes, and
  "what is the call-site check" asked again (first asked 10-17).
- 10-27 20:54: "is there something we should do with libqb so this isn't the case in the futue? **it seems stupid
  that every change to libqb causes this much work?**" / "we can expect a LOT MORE libqb edits in the future".
- 10-28 13:11 to 13:14: Claude deletes files instead of `git mv`; the user, angrily: "fuckijng git mv makes
  sense. fix yourself." (Now an exception in Human Rule 1.)

### 2026-10-28 to 10-31: M7 Linux, WSL detours, and the test complaint at its peak

- 10-28 13:47: "I think we ran tests on github, why run them aagin locally?" / 14:16: "do merge now, **no more
  tests**".
- 10-28 15:16 to 10-29 13:41, M7 Linux in WSL: lldb install failures, corrupted downloads, WSL mirrored
  networking. User at 02:24: "you seem to get stuck with the network troubleshooting, why?" / 02:26: "you are so
  annoying, I want you to figure out the network issue but I akso want you to stop just sitting there … it isn't
  apparent you are doinganything". 13:40: "what where w doing before we got sidetracked by network issues?"
- 10-28 16:44: American English rule added (CLAUDE.md working rule 9).
- 10-29 17:34: on a study: "it feels like this is one huge way of text" / 17:46 "did you actually change
  anything?" 17:54, to a feature-parity panel: "**You are so boring**".
- 10-29 17:47 to 18:37: a sweep of studies, archive, gaps docs, SOMEDAY additions (eight small doc sessions in an
  hour).
- 10-29 18:25 and 10-27 20:57: the user raises rewriting libqb (in Rust) twice; no decision recorded yet.
- 10-29: CI failures on Linux (DAP stepping timeout, a VS Code test) chased across three sessions, "is there a
  way we can test thsi locally? e.g. in docker desktop?", then three `status?` in 11 minutes.
- 10-30 14:39 to 16:30: two unmeasured member-array forms are measured (QB 4.5 set up again: "umm, you setup qb45
  previously, look at whaty you did before"). Then: "**it fels like we wran a ton of tests again?**" / "Also, we
  need to understand why yiou keep doing this, **you are always running thousands of tests for even fairly small
  changes!**" → panel → close-per-batch rule (`study\40` R1).
- 10-30 14:46 to 22:33: a Windows Sandbox change so that desktop built-ins can run (`desktop-sandbox`).
- 10-30 17:48: "Which someday.md items are low-hangign fruit" → 10-31 15:30: "**are you running all the dam tests
  againa nd if so, why?**"
- 10-30 23:46, on Fable: "**wtf are we running 25m?** have the panel weigh in, I feel like youa re still running
  a fuck load of testrs all the fucking time! we can't have these many trests every time something changes!!!!"
  23:50: "You have been fucking this up repeatedly and repeatedly offering solutions and then repeatedlys till
  fucking things up. look at your history". (Result: the `REACH` table in `close.py`, a libqb edit naming its
  built-ins, "a step the plan prints as a comment is not run".) 00:32: "I'm still not confident … do a full study
  of the test suite as it is, as it has been, as it should be" → `study\40`.
- 10-31 15:46: this review. 15:50, in another session: "I think the code may be causing issues with the
  computer. Can we put this in a Windows VM".

## 3. Further measurements

### 3.1 What the user's prompts are

1,790 prompts sorted by their first words (approximate):

| kind | count |
|---|---|
| `review changes` / `review just these fixes` | 225 |
| `provide concise commit message for staged` | 163 |
| `what's next?` | 144 |
| `pragmatic engineer on above?` | 137 |
| `/opsx:apply`, `/opsx:archive`, `/opsx:propose` | 121 |
| `fix` | 91 |
| `status?` | 77 |
| whole-project panel/architect reviews | 42 |
| everything else (real direction, questions, rulings) | about 780 |

By week, the share of prompts that only drive the loop (status, what's next, review, pragmatic engineer, do
recs, fix, commit message, apply/archive): 45% (10-01..07), 62% (10-08..14), 66% (10-15..21), 47% (10-22..31).
**The user is the scheduler of the loop; Claude does not move from one step to the next by itself.**

### 3.2 What the test runs are

- Identical `cargo test` commands rerun with no edit between them: only 13 in the month. **The waste is not
  literal reruns; it is the size of what each run covers, and how often the whole set is started.**
- The closing set (whole workspace tier 1 to `target\close\tier1.log`, the ignored CLI tests, the list updates,
  the program lists, the verification families, the call-site check, later Linux and the sandbox) began on 10-17
  and ran its tier 1 step about 149 times in 14 days, **about 10 a day**. Full verification-suite runs (2,800+
  programs): 0 before 10-10, then 240. Full call-site checks: 102. Full corpus runs: 123.
- `tools\close\README.md` line 57: a file under `crates\` "reaches every step". Nearly every review fix edits
  `crates\`, so **each fix after a close makes the whole closing set stale**, and `--check` then demands it
  again. With 2 to 4 review rounds per change after the close, the close repeats per round. Since 10-17, review
  sessions (55) ran the close's tier 1 about twice each.
- Tier 1 itself grew: the median whole `cargo test` went from 6 s (10-03) to 24 s (10-12), because corpus,
  call-site and verification programs were put inside `cargo test` (driver snapshot and CLI tests that build
  and run BASIC programs through g++).
- Since 10-13, every complaint about tests got the same answer: a panel, then a study or a rule or a tool.
  `study\31` (velocity plan), `study\32` (test cadence), `tools\testtime\`, the parallel QB64pe copies, worktree
  merge runs, `close.py` (915 lines) with its README (214 lines), the pass cache, the build cache, Windows
  Sandbox, `study\40`. CLAUDE.md working rule 6 alone is now about 40 lines. **The machinery to decide what to
  run grew instead of the amount run shrinking**, and the rules became too long and too fine to follow (10-17:
  "why did you not follow the rule?").
- Test runs disturb the machine: windows opened by the hundreds (10-25), the system grinding (10-23) and
  crashing (10-24), stale sessions and processes left running (10-23, 10-26), and on 10-31 the user suspects the
  code "may be causing issues with the computer".

### 3.3 Output per day

OpenSpec changes archived per day: 2–3 (10-03..13), 2–7 (10-14..25), then 3, 2, 1, 1, 2 (10-26..30). The changes
from 10-26 on are mostly not compiler features: `m6-toolchain`, `qbasic-testcases`, `fixes-switch`,
`m6-compiler-download`, `libqb-fixes`, `m7-linux`, `basic-unit-tests`, `desktop-sandbox`. Rust source added per
day halved from the first half of the month to the second (section 1.2). 79 OpenSpec changes and 226 commits in
29 days; about 38 studies (7 active, about 42 archived files).

## 4. Patterns

What follows is the reading of the evidence above, for Fable to test, not settled fact.

1. **Every problem is answered with more process.** A complaint about tests becomes a panel, a study, a rule
   and a tool; a review becomes recommendations that each add a document, a check or a list. Little is ever
   removed. The cost of each addition is small; the sum is what slows everything.
2. **Small batches, heavy gates.** The unit of work is an OpenSpec task group, often 20 to 60 minutes of
   coding, and each one goes through review → pragmatic engineer → fix → review just these fixes (2 to 4
   rounds), then a commit message and a `/clear`. The closing set is sized for a release and was run per change
   (until 10-30) and again after fixes. User, 10-17: "we are doing too small batches of changes to be making all
   these tests run every trime".
3. **Reviews never converge.** "Review just these fixes" nearly always finds something more. Each round's
   pragmatic-engineer step finds recommendations to "do". Nothing defines "good enough to commit".
4. **Whole-project reviews every one to two days** (about 12 in the month, most on Fable): each produces a study
   to keep, recommendations to act on, then reviews of the study, then archiving and de-duplicating studies.
5. **Rulings are pushed back to the user even when already decided.** The 10-08 ruling "implement it QB64pe's
   way and list it for review" still led to item-by-item walkthroughs (10-17 twice, 10-27 with 165 rows) and
   about 60 rulings from the ninth review (10-16). Claude also re-asked answered questions (10-18).
6. **Measure-first for every small form.** Each unclear behavior gets a recorded QB64pe (or QB 4.5) program
   before it is built; the verification programs grew from 77 to over 2,800 and all of them sit in lists that
   the close runs.
7. **Document churn.** Edits to docs, studies and OpenSpec (about 6,400) outnumber edits to Rust (about 5,500).
   Recurring reorganizations: archive studies, de-duplicate, `docs\` move, `DECISIONS-SMALL.md`,
   `ROADMAP-COMPLETED.md`, `LAYOUT.md`, SOMEDAY clean-ups, glossary. Writing is dense and compressed; the user
   finds it hard to read (10-24 commit messages, 10-29 "one huge way of text", "You are so boring").
8. **Claude does not follow its own rules reliably**, and the fix each time is more rule text: staging against
   rule 1 (10-15) and refusing to stage when told to (10-27, 10-29); deleting instead of `git mv` (10-28);
   rerunning runs already done (10-11, 10-17); waiting on CI (10-03, 10-16).
9. **Waiting in the foreground and side quests.** 77 `status?` prompts, bursts of 7 to 11 in under an hour
   (10-02, 10-03, 10-27), while Claude sits on a long run. Side quests followed past their worth: hundreds of
   console windows for a rare issue (10-25), WSL networking (10-28/29), volta→mise (10-13).
10. **The user cannot see the test system any more.** "what is a slice list? what is folding? what is the
    differential list? what is the call-site check?" (10-17), the same question again (10-27), "didn't we just
    finish 1 or was that a different set of tests?" (10-19).
11. **Few goals are stated as end results.** The work picks "what's next" from `STATUS.md` one step at a time;
    goals like "Civil War Strategy whole game" (10-20) or "what remains before parity" (10-21) are rare, and
    "it looks like there isn't much left to be done?" (10-27) shows the roadmap no longer tells the user where
    the end is.

## 5. What went well (so it is kept)

- The compiler went from nothing to M7 (Linux) in 29 days: parser, sema, IR, C++ codegen, libqb, sound, graphics,
  events, the whole Civil War Strategy game, a VS Code extension with formatter, language features and a DAP
  debugger, a bundled toolchain.
- The QB64pe-recorded truth (corpus, verification) caught real differences; few regressions reached `main`.
- The user's quick rulings, when asked in batches with a recommendation, were fast (10-27: about 20 answers in an
  hour, mostly "A").

## 6. Questions for Fable

1. What should the default check after a code change be, and what should run only at a merge? Should
   `crates\` really reach every step, or should tier 1 (`cargo test`) shrink back to fast unit and snapshot tests,
   with programs run only on request or at merge?
2. Should reviews have a stopping rule (one review round, fixes not re-reviewed unless they change behavior)?
   Should the pragmatic-engineer pass be part of the first review instead of a separate prompt?
3. Should the task-group-sized commit be replaced by larger batches (one commit per OpenSpec change, or per day)?
4. Which documents, rules, studies and tools can be deleted rather than added to? Is CLAUDE.md rule 6 better
   replaced by a few lines?
5. How should Claude stop pushing decisions already covered by a standing ruling back to the user?
6. What is the definition of done for the project, so that "what's next" is chosen against an end goal?
