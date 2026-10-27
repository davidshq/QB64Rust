# 38. Where the project stands, and where it could go (2026-10-26)

Asked by the user at the merge of `mb5`, when every planned milestone but M6 was built: review where we are and
where we should go, with nothing taken as unchangeable; options, and how best to proceed. `study\37` answered the
narrower question of M6 the same day; this review takes that as read (§4 option 3) and looks at the whole.

The panel is role-played inline (`CLAUDE.md` working rule 6): a software architect (ARCH), the pragmatic engineer
(PRAG), the QB64 engineer (QB64), the compiler/languages engineer (COMP), the Rust engineer (RUST), the test
engineer (TEST), and two extra roles: a product owner (PROD: who the work is for, what reaches them, what a
release is) and a maintainer (MAINT: what it costs to keep this running for a year). Effort is Claude's time
(working rule 8); what is weighed is the cost that stays, the risk, and the user's time.

Evidence: `STATUS.md`, `ROADMAP.md`, `DECISIONS.md` (the first rows of 2026-10-02 in particular), `SOMEDAY.md`,
`UPSTREAM.md`, `study\00` §2 and §3, `study\06`, `study\37`, `vscode\README.md`, `tools\testtime\REPORT.md`, the
GitHub release and run lists, counts over the repository (below). No code was read for this review beyond what
`study\37` read.

## 1. Verdict in one paragraph

In 25 days the project built what it set out to build on day one: a compiler in Rust that matches QB64pe on every
program the suites hold (516 corpus, 402 of 404 upstream, 3,892 verification programs, one whole game played to
the same screens), a VS Code extension with checking, formatting, navigation, build and a line-level debugger, and
a test and release system around them. Today the only user is the author; the extension is installed from a GitHub release,
needs QB64pe installed beside it. The panel's recommendation is
in three parts: 
1. **write the purpose** (one paragraph, the user's; §5.1), because every option below is cheap or dear depending on it; 
2. **make the thing usable by someone else now**, which is small (§5.2); 
3. and **cut the standing cost** of process and test machinery that was right for building and is too heavy for maintaining
(§5.3). The directions after that (§4) are real choices, and the panel ranks them under each purpose it can
imagine (§5.4) rather than picking one for the user.

## 2. Where we are (measured 2026-10-26)

### 2.1 What exists

| | Measure |
|---|---|
| Age, size | 25 days (2026-10-02 to 2026-10-26), 193 commits on `main` and `mb5`, 71 archived OpenSpec changes |
| The compiler | 81,500 lines of Rust in 11 crates, 12,400 of them tests; `qb64rust` compiles, formats (`-y`), serves the editor (`lsp`) and the debugger (`dap`) |
| Parity | corpus 516 pass, 5 known failures; upstream 402 of 404 (2 out of scope); verification 3,892 pass, 63 known (machine-dependent); call-site check 2,649 of 2,649 equal; Civil War Strategy's scripted games held to line coverage; differential 59 of 59 |
| The extension | `vscode\`, 2,700 lines of TypeScript; release `vscode-v0.2.0` (2026-10-19, GitHub release, not the Marketplace); Windows x64; bundles `qb64rust` and the copy of libqb; **needs a QB64pe installation** for its C++ toolchain and `lldb-dap` |
| Tooling | 14,600 lines of Python in `tools\`: the test runner, the closing-run planner, coverage, call-site, the libqb copy, the built-in extractor, the wording table, the test-time record, the repo check |
| Documents | 1,780 lines across the eight top-level files (`CLAUDE.md` 215 with 12 working rules); 15 active studies, 20 archived; `SOMEDAY.md` holds about 165 "QB64pe behaviours to review" and 18 "other ideas" |
| Working time | 165 h of active Claude time; 20% of it waiting for test runs, plus 34 h of runs in the background (`tools\testtime\REPORT.md`) |
| Disk | `verification\` 3.3 GB on disk (the old compiler's 1,436 executables, git-ignored; 12,178 tracked text files); `.git` 118 MB |
| CI | the Rust job failed on `mb5`'s push on the same test fixed in this merge; the weekly `tier2` job failed on 2026-10-26 (it has run into its 45-minute limit before, `STATUS.md`); the extension and repo-check jobs pass |
| Upstream | `UPSTREAM.md` lists the fixes to offer QB64pe; none has been offered |

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
Without QB64pe installed they get nothing: no toolchain. On Linux or macOS they get nothing.

### 2.3 What was decided, and what was not

- Decided on 2026-10-02: 
   - a ground-up rewrite in Rust; 
   - no port of the text-mode IDE, VS Code instead; 
   - new error messages; 
   - QB64pe 4.7.0 as the dialect, behaviour as observed at a pinned commit. 
- Decided on 2026-10-08: 
   - a deterministic behaviour that changes results is kept, a crash, corruption or failed build is ixed.

Not decided, anywhere: 
- **who the compiler is for** and **what "done" is** for the project as a whole. 

The roadmap's milestones were the expert panel's architecture plan (`study\07` session 8) and each has a "done when" of its own, but there is no sentence that says, for example, 
   - "a QB64 programmer on any platform can install one thing and have an editor, compiler and debugger", 
   - or "the author's own programs build and debug in VS Code", 
   - or "a compiler that QB64pe's maintainers could adopt". 
Those three purposes lead to three different next steps.

### 2.4 What is fragile

- **The truth is a moving target.** Every recording is `qb64pe.exe` at `16f629784e` (4.7.0). QB64pe keeps
  releasing. There is no policy on whether the project follows (re-record, re-copy libqb, take new built-ins) or
  freezes its dialect at 4.7.0. Both are legitimate; freezing is the cheaper and makes the project independent.
- **Some recordings are the machine's.** Sound device rate, clipboard history, the console hand-over, the desktop's
  window size (`STATUS.md` "Known bugs", today's `v59_g_readback`). Each was handled when it appeared; the list
  grows by one every few days.
- **CI cannot run the screen programs** (no window on the runner), so tier 2 is local; the weekly job fails at its
  time limit. CI is declared not a gate (working rule 9), which is honest, but it means the only full check runs on
  one machine.
- **One real program.** Civil War Strategy is the only whole program beyond the test suites. The suites are
  measured against QB64pe's own behaviour, so they say "the same as QB64pe", not "programs people wrote work".
- **The 165 rows.** "QB64pe behaviours to review" waits on the user and grows with every change. By the 2026-10-08
  rule most of them are already decided (kept); only the 18 or so that end the program or touch memory need a
  ruling. The list is doing the work of a decision that was already made.
- **The process is sized for building.** Twelve working rules, a closing set of a dozen runs per change, a full
  set of an hour per merge, a test-time record read from the transcripts, count blocks in four READMEs, numbers
  blocks in `STATUS.md`. Each was added for a reason recorded in `CLAUDE.md`; together they are what a session
  spends its first and last hour on.

## 3. What the panel reads from this

**PROD:** The product exists and nobody has used it but its author. Every remaining question (M6, the 165 rows,
cross-platform, what to fix in libqb) would be answered faster by one week of a stranger using it than by another
review. The cheapest next step is to make that possible: no QB64pe install, a Marketplace listing or at least a
release that says what it is, a README that is true.

**ARCH:** The architecture is finished and sound: lossless tree, typed tree, ABI-neutral IR, one emitter pinned
to libqb. The IR is the asset. It would let the project do things QB64pe structurally cannot (more than one
backend, analyses, language changes), and none of them is worth doing until there is a reason from outside.

**COMP:** What the new front end uniquely enables is language work: warnings (unused variables, which the old IDE
has as a dialog), a module story, modern conveniences. That is a fork of the dialect and should be driven by
users who ask, not by us. The one piece that is pure gain and no fork: better diagnostics and warnings on the
existing dialect.

**QB64:** The community's compiler is QB64pe. A second compiler that matches it exactly is useful to that
community mainly as (a) a check on QB64pe (the verification families found real bugs: `UPSTREAM.md`), (b) an
editor and debugger for QB64pe programs, and (c) a path to platforms or features QB64pe cannot reach. (a) and (b)
are ready to offer today and cost nothing to keep.

**RUST:** The Rust is in good order (the reviews of `study\archive\21`, `30` found nothing structural). The risk
in the code is not design but coverage of inputs we never saw: there is no fuzzing and one real program.

**TEST:** The suites are the strongest part of the project and also its heaviest. The three full suites with
coverage are right at a merge. The rest of the closing set (demand reports, count blocks, wording table, test-time
record) was built to keep a fast-moving build honest, and can be thinned now that the build is done.

**MAINT:** For a year of maintenance the costs that stay are: the pinned QB64pe clone (3.3 GB with its
executables, needed for every recording), the libqb copy with its edits, the toolchain dependency, the documents.
A policy on following QB64pe releases decides most of this.

**PRAG:** Nothing here is urgent. The project could stop today and be a complete thing. So the order is: write the
purpose; do the small release work that is right under every purpose; cut the standing cost; then the direction
the purpose picks.

## 4. The options

Each is independent; several can run together. Effort is Claude's time; the columns that matter are the last
three.

| # | Option | Claude's time | Cost that stays | Risk | Who gains |
|---|---|---|---|---|---|
| 1 | **Usable without QB64pe**: the compiler as its own release asset with a short manual; Marketplace listing (`study\37` §5.2) | small | one toolchain pin to keep | low | anyone who would try it |
| 2 | **Linux and macOS**: libqb already builds there; the driver's build, the runner, the extension's packaging and CI are Windows-only; recordings differ by platform (`printf` float text, console comma zones D-011) | large: a platform is a milestone (toolchain, CI runner, recordings judged per platform) | a runner and a recording set per platform | medium: the recorded truth becomes three truths | QB64 users off Windows |
| 3 | **M6 as written** (second backend, runtime redesign) | very large | two backends kept equal | high | nobody visible (`study\37`) |
| 4 | **libqb repaired in place** through `tools\libqb\edits`: the 18 crash-class rows now, others as ruled (`study\37` §5.3) | small per row | the edits, kept as the clone moves | low | users who hit those crashes |
| 5 | **Offer the work upstream**: the `UPSTREAM.md` fixes as QB64pe issues or pull requests; the verification programs as test cases; the language server for QB64pe programs as it is | small | none | none; may be declined | the QB64pe community, and this project (fewer divergences to carry) |
| 6 | **Language work on the dialect**: warnings (unused variables and procedures, which the old IDE offers), then what users ask for | small per warning; a feature is a change | each is a divergence from QB64pe's acceptance set only if it rejects | a dialect fork if it goes past warnings | people who write new programs |
| 7 | **Robustness beyond the suites**: more real programs (public-domain QB64 programs with a licence), a fuzz target for the lexer and parser, the string and array differential group | moderate | the programs and their recordings | low | everyone, invisibly |
| 8 | **Cut the standing cost** (§5.3): fewer rules, a thinner close, documents merged, the 165 rows closed by the rule that already decides them, a policy on QB64pe releases | small | less | low: a thinner close catches less; the full set at a merge stays | the user and every session |
| 9 | **Freeze the dialect at 4.7.0** and stop tracking QB64pe: no re-recordings, the clone kept only for the toolchain until option 1 removes that too | none | far less | the dialect ages; built-ins QB64pe adds later are absent | the maintainer |
| 10 | **Declare 1.0 and stop building**: M5b is the end; maintenance only | none | the minimum | none | the author's time |

Not an option: a text-mode IDE, self-hosting, a strict QB 4.5 mode (decided 2026-10-02; nothing since has made
them cheaper or wanted).

## 5. Recommendation

### 5.1 First, the purpose (the user, one paragraph)

Write into `ROADMAP.md`'s head who the project is for and what "done" means for it. The panel offers three
candidates; the user may have a fourth:

- **P1, the author's tool**: the author's programs (Civil War Strategy and others) are written, built and debugged
  in VS Code with this compiler, on Windows. *Then* options 1 (without the Marketplace), 4, 8 and 9 finish the
  project; 2, 3, 6 are not needed.
- **P2, a QB64 for anyone**: a QB64 programmer installs one thing on any platform and has an editor, compiler and
  debugger. *Then* 1, then 2, with 7 beside them; 4 and 6 as users ask; 5 as goodwill.
- **P3, a contribution to QB64pe**: the project's value is what it gives back: a reference implementation that
  found QB64pe's bugs, a test corpus, an editor and debugger for QB64pe programs. *Then* 5 first, 1 for the
  editor, 9 is wrong (the project must track QB64pe), 2 only if QB64pe's community asks.

### 5.2 Under every purpose: make it usable by someone else (option 1, part of 5)

This is right under P1 (the author installs it on a second machine), P2 and P3, and it is small:

1. Release `qb64rust.exe` with libqb as its own asset beside the `.vsix`, with a one-page manual (the usage line,
   `-f:` settings, `--debug`, `-y`).
2. Open the QB64pe issues of `UPSTREAM.md` (option 5's first half): each has its measurement and its fix already
   written; it costs nothing to keep and is the honest thing to do with a found bug.

Whether to list on the Marketplace is the user's (it is public in a way a GitHub release is not).

### 5.3 Under every purpose: cut the standing cost (option 8)

Each of these is a rule or a tool that earned its place while building and now costs every session:

1. **Close the 165 rows by the rule that already decides them.** The 2026-10-08 decision keeps every
   deterministic result-changing behaviour. Move the rows to a closed record (`study\archive\behaviours-kept.md`)
   and keep in `SOMEDAY.md` only the 18 that end the program, hang or touch memory, which the same rule says to
   fix (option 4). "Waiting on the user" then means something again.
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
4. **Decide the policy on QB64pe releases** (option 9 or its opposite). The panel's default: **freeze at 4.7.0
   until a user asks for a later built-in**, then take that one by the existing path (extract, record, compile).
   Re-recording the whole suite for every QB64pe release is the single largest standing cost and buys nothing
   until someone needs what the release added.
5. **Stop the weekly `tier2` CI job** or give it a runner that can open a window. A job that fails every week on
   its limit teaches the next session to ignore red.

### 5.4 Then, the direction the purpose picks

| Purpose | Order after §5.2 and §5.3 |
|---|---|
| P1 | option 4 (the crash-class rows), then option 10: declare 1.0 |
| P2 | option 7 (real programs, fuzzing) while option 2 is planned as a milestone per platform (Linux first: a GitHub runner exists and libqb's Linux path is the better exercised); option 6 only as users ask |
| P3 | the second half of option 5 (the verification programs as QB64pe tests; ask about the language server), option 4, then whatever the maintainers want |

Under none of them does M6 as written come next; `study\37`'s rewrite of it ("the runtime without QB64pe") is
`m6-toolchain` (built) and §5.2 item 2 here.

## 6. What the panel would change in how the work is run

1. **A milestone needs a person in its "done when".** M0–M5b each ended when a measurement passed. The next
   ones should end when someone can do something they could not before. That is the one structural lesson of
   this review.
2. **Reviews at a merge are enough; studies at a milestone are enough.** Twelve reviews in 25 days found "nothing
   astray" eleven times. The cadence can be one review per milestone.
3. **Measure before designing, keep doing that.** The project's best decisions (the debugger path, today's
   M6 verdict) came from a probe run before the panel sat. The worst risk now is the opposite: tooling added
   without a measured need.

## 7. What the user decides

1. **The purpose** (§5.1): P1, P2, P3, or your own paragraph.
2. **§5.2, the release work**: yes to both items, or which; the Marketplace, yes or no.
3. **§5.3, the cuts**: each of the five, yes or no; in particular the policy on QB64pe releases (freeze at
   4.7.0, or track).
4. **The direction after that** (§5.4), which follows from 1 unless you choose otherwise.

The questions of `study\37` §8 (what M6 becomes) are answered by 1 and 2 here.
