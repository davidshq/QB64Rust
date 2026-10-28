# 38. Where the project stands, and where it could go (2026-10-26)

## 2. Where we are (measured 2026-10-26)

### 2.1 What exists

| | Measure |
|---|---|
| Tooling | 14,600 lines of Python in `tools\`: the test runner, the closing-run planner, coverage, call-site, the libqb copy, the built-in extractor, the wording table, the test-time record, the repo check |
| Documents | 1,780 lines across the eight top-level files (`CLAUDE.md` 215 with 12 working rules); 15 active studies, 20 archived; `SOMEDAY.md` holds about 165 "QB64pe behaviours to review" and 18 "other ideas" |

### 2.2 What a user gets today

Someone who installs `vscode-v0.2.0` on Windows with QB64pe already installed gets: 
- syntax errors as they type,
- every error of a program at its column on save, 
- formatting as the QB64pe IDE does it, 
- outline, go to definition, hover, references, rename, completion
- build and run with the new compiler,
- debugging by BASIC line with variables, watches and data breakpoints. That is more than the QB64pe IDE gives in several respects (all errors
at once, columns, rename, a real debugger UI) and less in others (no help system, no `$NOPREFIX` conversion, no
export). 
Without QB64pe installed they get nothing from 0.2.0: no toolchain (the next release installs its own,
`m6-toolchain`). On Linux or macOS they get nothing.

### 2.4 What is fragile

- **Some recordings are the machine's.** Sound device rate, clipboard history, the console hand-over, the desktop's
  window size (`STATUS.md` "Known bugs", today's `v59_g_readback`). Each was handled when it appeared; the list
  grows by one every few days.
- **CI cannot run the screen programs** (no window on the runner), so tier 2 is local; the weekly job fails at its
  time limit. CI is declared not a gate (working rule 9), which is honest, but it means the only full check runs on
  one machine.
- **One real program is run.** QB64pe's `tests\qbasic_testcases` holds 143 programs by community authors (Pete,
  TheBob, qb45.com; up to 6,000 lines), and all of them build (`qbasic-testcases`), but the suite never runs them.
  Civil War Strategy is the only real program that is run and compared. The other suites are small programs
  written as tests, measured against QB64pe's own behaviour, so they say "the same as QB64pe", not "programs people
  wrote work". The missing data files are not what stops the 143 (scan of 2026-10-26, `SOMEDAY.md` "Other
  ideas"): 97 read no file and only two lack one. What they lack is a way to compare them: nearly all open a
  window, read keys and use `RND` or `TIMER`, so each needs a keys file and screen states as Civil War Strategy
  has, apart from about ten that run in a console with typed input.
- **The 165 rows.** "QB64pe behaviours to review" waits on the user and grows with every change. By the 2026-10-08
  rule most of them are already decided (kept); only the 18 or so that end the program or touch memory need a
  ruling. The list is doing the work of a decision that was already made.
- **The process is sized for building.** Twelve working rules, a closing set of a dozen runs per change, a full
  set of an hour per merge, a test-time record read from the transcripts, count blocks in four READMEs, numbers
  blocks in `STATUS.md`. Each was added for a reason recorded in `CLAUDE.md`; together they are what a session
  spends its first and last hour on.

## 3. What the panel reads from this

**RUST:**  The risk
in the code is not design but coverage of inputs we never saw: the seeded mutation test
(`crates\driver\tests\mutate.rs`) stops at `sema`, there is no coverage-guided fuzzing, and only one real program is
run.

**TEST:** The suites are the strongest part of the project and also its heaviest. The three full suites with
coverage are right at a merge. The rest of the closing set (demand reports, count blocks, wording table, test-time
record) was built to keep a fast-moving build honest, and can be thinned now that the build is done.

**MAINT:** For a year of maintenance the costs that stay are: the pinned QB64pe clone (3.3 GB with its
executables, needed for every recording), the libqb copy with its edits, the toolchain dependency, the documents.

## 4. The options

Each is independent; several can run together. Effort is Claude's time; the columns that matter are the last
three.

| # | Option | Claude's time | Cost that stays | Risk | Who gains |
|---|---|---|---|---|---|
| 1 | **Usable without QB64pe**: the compiler as its own release asset with a short manual; Marketplace listing (`study\37` §5.2; its toolchain is built, `m6-toolchain`) | small | one toolchain pin to keep | low | anyone who would try it |
| 2 | **Linux and macOS**: libqb already builds there; the driver's build, the runner, the extension's packaging and CI are Windows-only; recordings differ by platform (`printf` float text, console comma zones D-011) | large: a platform is a milestone (toolchain, CI runner, recordings judged per platform) | a runner and a recording set per platform | medium: the recorded truth becomes three truths | QB64 users off Windows |
| 3 | **M6 as written** (second backend, runtime redesign) | very large | two backends kept equal | high | nobody visible (`study\37`) |
| 4 | **libqb repaired in place** through `tools\libqb\edits`: the 18 crash-class rows now, others as ruled (`study\37` §5.3) | small per row | the edits, kept as the clone moves | low | users who hit those crashes |
| 5 | **Offer the work upstream**: the `UPSTREAM.md` fixes as QB64pe issues or pull requests; the verification programs as test cases; the language server for QB64pe programs as it is | small | none | none; may be declined | the QB64pe community, and this project (fewer divergences to carry) |
| 6 | **Language work on the dialect**: warnings (unused variables and procedures, which the old IDE offers), then what users ask for | small per warning; a feature is a change | each is a divergence from QB64pe's acceptance set only if it rejects | a dialect fork if it goes past warnings | people who write new programs |
| 7 | **Robustness beyond the suites**: more real programs (QB64pe's 143 `qbasic_testcases` run from the clone with only the recordings kept here, `SOMEDAY.md` "Other ideas"; public-domain QB64 programs with a licence), a fuzz target for the lexer and parser, the string and array differential group | moderate | the programs and their recordings | low | everyone, invisibly |
| 8 | **Cut the standing cost** (§5.3): fewer rules, a thinner close, documents merged, the weekly CI job | small | less | low: a thinner close catches less; the full set at a merge stays | the user and every session |

Not an option: a text-mode IDE, self-hosting, a strict QB 4.5 mode (decided 2026-10-02; nothing since has made
them cheaper or wanted).

## 5. Recommendation

### 5.2 Under every purpose: make it usable by someone else (option 1, part of 5)

This is right under P1 (the author installs it on a second machine), P2 and P3, and it is small:

1. Release `qb64rust.exe` with libqb as its own asset beside the `.vsix`, with a one-page manual (the usage line,
   `-f:` settings, `--debug`, `-y`).
2. Open the QB64pe issues of `UPSTREAM.md` (option 5's first half): each has its measurement and its fix already
   written; it costs nothing to keep and is the honest thing to do with a found bug.

### 5.3 Under every purpose: cut the standing cost (option 8)

Each of these is a rule or a tool that earned its place while building and now costs every session:

1. *Decided otherwise: the rows stay; they are candidates for fixes behind a switch (`DECISIONS.md` 2026-10-26).*
2. **A thinner close per change**: tier 1 with fmt and clippy, the programs the change touches, the three pass
   lists with the pass cache, the call-site check. Drop from the per-change close: the demand reports (nothing is
   "not supported yet" any more), the count blocks and the wording check (regenerate at a merge), the test-time
   record (its finding, that runs were repeated, is made; keep the tool and run it when asked). The full set at a
   merge stays as it is: it is what found today's two problems.
3. **Merge the documents.** `STATUS.md`'s numbers blocks become one table of the last merge (the rest is in git);
   `GLOSSARY.md`, `DIVERGENCES-QB45.md` and `UPSTREAM.md` stay; `CLAUDE.md`'s twelve working rules can be eight
   (7 and 2 are one rule about the shell; 10 and 11 are one rule about paperwork cadence; 9's cadence goes to
   `tools\close\README.md` and the rule says "run `close.py`"). Archive studies `01`, `02`, `04`, `05` (the old
   compiler, codegen, IDE, build: no milestone needs them now); keep `03` while libqb is repaired, `06` while
   editor features are added.

### 5.4 Then, the direction the purpose picks

| Purpose | Order after §5.2 and §5.3 |
|---|---|
| P1 | option 4 (the crash-class rows), then option 10: declare 1.0 |
| P2 | option 7 (real programs, fuzzing) while option 2 is planned as a milestone per platform (Linux first: a GitHub runner exists and libqb's Linux path is the better exercised); option 6 only as users ask |
| P3 | the second half of option 5 (the verification programs as QB64pe tests; ask about the language server), option 4, then whatever the maintainers want |

Under none of them does M6 as written come next; `study\37`'s rewrite of it ("the runtime without QB64pe") is
`m6-toolchain` (built) and §5.2 item 2 here.

## 6. What the panel would change in how the work is run

3. **Measure before designing, keep doing that.** The project's best decisions (the debugger path, today's
   M6 verdict) came from a probe run before the panel sat. The worst risk now is the opposite: tooling added
   without a measured need.

## 7. What the user decides

2. **§5.2, the release work**: yes to both items, or which; the Marketplace, yes or no.
3. **§5.3, the cuts**: items 2, 3 and 5, yes or no.
4. **The direction after that** (§5.4): P2's row unless you choose otherwise.
