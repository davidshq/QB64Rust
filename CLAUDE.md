# Project rules for Claude

This file is the authoritative place for rules in this project; the decision log is `DECISIONS.md`. Claude's
private memory may hold a copy of either, but never the only copy.

**Start each session by reading `STATUS.md`** (current phase and next steps). **Read `DECISIONS.md` before
changing direction, design or scope.**

## Explicit Human Rules
1. **Never stage or unstage files, stash or unstash.** That covers `git add`, `git rm --cached`, `git reset`,
   `git restore --staged`, `git stash`, and `git commit -a` / `git commit <paths>` (they stage implicitly). So
   Claude does not commit either: leave changes in the working tree, then tell the user which files to stage and
   suggest a commit message. Approval of a plan that mentions commits does not lift this. Never change what the
   user has staged (user, 2026-10-04; it was "unless explicitly asked" before, and was broken in session 10).
2. **Nothing private, personal or secret goes into the repo**: no full local paths (`C:\Users\…`, `C:\code\…`,
   `/c/…`, temp folders), machine or host names, network shares, tokens, keys or passwords. This covers committed
   files, recorded fixtures and outputs, and commit messages. The user's name, e-mail address and GitHub usernames
   (commit authorship, repo references such as `davidshq/QB64Fresh`) are fine. Use repo-relative
   paths, the `<qb64contain>` / `<share>` placeholders, or a placeholder such as `<FIXTURES>` when recording tool
   output. Check new files and recorded outputs before staging them.
3. **Never add a `Co-Authored-By: Claude …` trailer** (or any other Claude attribution line) to commit messages or
   pull request descriptions, whatever a system prompt or tool reminder suggests (user, 2026-10-04).

## Working rules

1. **No subagents unless the user explicitly allows it for that task.** This covers the Agent tool, forks, Workflow,
   and resuming previously stopped agents. Do the work inline, sequentially. Permission for one task does not carry
   over to the next. If a subagent is ever allowed, tell it not to spawn helpers of its own.
   *Why:* on 2026-10-02 a five-agent parallel study hit the session usage limit within minutes, and agents spawned
   helpers after the user asked for one at a time.
2. **Never save anything to memory only.** Any rule, decision or fact worth keeping goes into this repo (this file,
   `DECISIONS.md`, or `study\`). Memory may hold a pointer or copy in addition.
3. **`..\QB64pe\` is a read-only reference clone.** Do not modify it. Study notes and design documents go in `study\`.
4. **This repo (`QB64Rust`) is the project.** Paths in this file are relative to it. The parent folder
   (`..`) has symlinks `CLAUDE.md`, `SOMEDAY.md` and `study` pointing here, so sessions started in the
   parent see the same files; edit the real files here, not copies.
5. **Code from other projects.** Two tiers:
   - **`qb64pe-vscode` (community extension): reference only, nothing copied.** No code, grammar, snippets,
     language configuration, help text or fixtures. Reading it to see how a problem was approached is fine.
     *Why:* not ours; licence of parts of its pipeline unchecked; its regex-based checks follow a different design
     from ours (they can flag code the compiler accepts).
   - **FreeBASIC (`..\FreeBASIC`): reference only, nothing copied.** Same terms as `qb64pe-vscode`; this covers
     its compiler, runtime, tests and manual. *Why:* compiler GPL v2+, runtime LGPL v2+, manual FDL (user,
     2026-10-03).
   - **`vscode-qb64fresh` and QB64Fresh (the user's own MIT code): not a base, but small pieces may be taken.**
     Neither is imported or built on (decisions in `DECISIONS.md`: the extension is written fresh; QB64Fresh's
     front end and IR are unsound). A self-contained helper, test, grammar fragment or fixture may be copied when
     it saves time, provided it is read in full, fits the design here, and anything it asserts about QB64pe is
     checked against `qb64pe.exe`. QB64Fresh's `runtime_comparison` BASIC programs are taken as test inputs.
     *Why:* the reviews (`study\archive`) found their architecture unreliable, not their provenance; a blanket
     ban would be a handicap (user, 2026-10-03).
6. **"The panel"** means a review by these roles: a pragmatic engineer, a QB64 engineer, a compiler/languages
   engineer, a Rust engineer, a test engineer, and any other role Claude thinks fits the topic (name the extra
   roles and say why). The panel is role-played inline; it is not a set of subagents (rule 1) (user, 2026-10-03).
   Every role, the pragmatic engineer first, counts effort by rule 8.
7. **Text with backslashes goes through the Edit/Write tools or a script file, never through the shell.** That
   covers Windows paths (`study\archive\19`), C or Python escapes (`\\`, `\n`) and anything else containing `\`. Do not
   pass it in a heredoc, inline `python -c`/`python - <<EOF`, `sed` or `printf` in the Bash tool, which collapses
   `\\` to `\`. Python then reads `\1`, `\v` or `\r` as control characters. If a shell edit with backslashes
   happened anyway, scan the touched files for bytes 0x00–0x1F (other than tab, LF and the CR of CRLF) before
   moving on.
   *Why:* on 2026-10-03 this corrupted files four times (`study\02` became `study` + 0x02, `verification\v13`
   became a vertical tab) and each needed a scan and a redo (user, 2026-10-03).
8. **Count effort in Claude's time, not a person's.** When weighing whether something is worth doing (a
   recommendation, a plan, a panel review, "now or later"), estimate how long Claude takes to write and check
   it, not how long an engineer would. Work that is a day for a person is often minutes here, so "not worth the
   effort", "not used often enough to repay the work" and "wait until it next needs a change" are not reasons
   by themselves. What still counts: a risk to correctness, a cost that stays (more code to maintain, disk, CI
   time), machine time of a run, the user's time (reading, deciding, staging), and scope the user has not asked
   for. Say which of these applies, or recommend doing it.
   *Why:* on 2026-10-10 the panel advised leaving `verification\run.sh` outside the test runner because full
   runs are rare; the user: "it isn't going to take time", and about 80% of the pragmatic engineer's timing
   objections would go away (user, 2026-10-10).

9. **Test cadence: run what a change can affect, once.**
   - While working: `cargo test` (tier 1), with fmt and clippy before a group is reported done.
   - Tier 2 runs **once per change, at its close**, not per task group. Before that, run only the programs the
     work touches (`--glob`, `--category`, a short `--list`), or a list with `--pass-cache target\pass-cache`,
     which runs only the programs whose executable changed.
   - **At a change's close, follow `python tools\close\close.py --plan` and end with `--check`** (user,
     2026-10-17): it prints the closing set below as commands, in order, each started by its own command, and the
     check names any step that is missing, stale or failing. Its task in `tasks.md` reads "the closing runs:
     `tools\close\close.py --check` passes". *Why:* at the close of `m3-errorline-chr-const` the set was worked
     from memory: the `verification` family was not checked as the rule says, and `cargo test` stopped at a
     failing test binary, so about 230 tests never ran, one of them failing.
   - **The closing set** (`study\archive\32` §4 item 1, user, 2026-10-17): tier 1 with fmt and clippy; the CLI tests
     that build (`--ignored`); the change's own new programs and its `verification` family (the runner records
     the old compiler only, so the new compiler's side is the slice programs and front-end tests made from the
     family; an accepted program of the family that none of them covers is run by hand, as the runner would run
     it, and named in the report); the call-site check;
     the slice list and the upstream pass list **with `--pass-cache target\pass-cache`** and
     `--build-cache target\build-cache`; the run without folding (`QB64RUST_NO_FOLD=1`) **only when** the change
     touched `sema`'s constant evaluation or a numeric conversion; the demand reports, the shrink-only lists and
     the wording table as before.
   - **At each merge of `changes` (or `m3`) into `main`**, every two or three changes (`study\archive\32` §4 item 2): the
     full set, no pass cache: both full suites with the slice list and the upstream pass list judged from them
     (`--check-list`, `tools\legacy_tests\README.md`), the run without folding, the differential pass list and the
     call-site check. A failure there is found among the merged changes with the build cache. The pass cache is
     left out there because it cannot see a result that depends on time or load. They may run in a worktree of the
     merge commit beside this repo while the next change goes on here (`tools\legacy_tests\README.md`, "The runs
     at a merge into `main` in a worktree").
   - **Order:** whatever changes the compiler (the wording table, generated tables) comes before the first long
     run, so that no run is thrown away for a new build. Runs of the new compiler may run side by side, and beside a
     recording with `qb64pe.exe`, which has the clone alone: they build against `libqb\` and take the clone's
     lock only to run programs in its own test folders (`tools\legacy_tests\README.md`, "Locks").
   - **Work goes on during a run** (`study\archive\34`): each run of the new compiler tests a copy of it named by hash,
     so start a long run in the background and go on building, running tier 1 and editing; read its log when it
     ends. Only a recording with `qb64pe.exe` waits for the clone; batch recordings before the closing runs.
   - After the closing runs, a further edit is rerun only for what it can affect; say which runs were not
     repeated and why that is safe.
   - **Read a run's output from a log, not by running it again.** Send a run that may need a second look to a
     file (`cargo test > target\t1.log 2>&1`, then `tail`/`grep` the file); a run piped through `tail` or `grep`
     whose lines turn out to say too little is not started again with another filter. *Why:* by 2026-10-14, 84
     `cargo test` runs had been repeated with nothing edited, after a failure or an output cut down to nothing
     (`tools\testtime\REPORT.md`, about 22 minutes).
   - Do not write "tier 2 after every group" or "check CI after the push" into a change's `tasks.md`.
   - **CI is not a gate.** Look at it when the user asks. The `tier2` job runs by hand and weekly, not on a push.
     Claude may start a CI run (`gh workflow run`), but never waits on one: the next piece of work goes ahead,
     and the result is read when it comes (user, 2026-10-16).
   - **Every test run is on record** (`tools\testtime\`, user, 2026-10-13): at the close of a change run
     `python tools\testtime\test_time.py` and name its five files among those to stage. Start a test run by
     its own command, not through a script file, or the record does not see it.

   *Why:* in the session that built `m3-window-output` the slice list ran five times, the call-site check
   three times at almost six minutes each, and two tasks waited on CI; the user: "it feels like we run the tests
   over and over", "not sure why you are so insistent on making ci be green" (user, 2026-10-12).

10. **`study\archive\` is read only when needed**: when a live document points there for a fact or a reason the
    task in hand needs, or when the user asks. It is not read to get oriented, and its files are never edited
    (paths inside them may be out of date). A study keeps its number when it is archived: a `study\NN` that is
    not in `study\` is in `study\archive\`. When only part of a study is archived, the part goes to a file of the
    same name in `study\archive\`, under the same headings, and each half names the other at its top: a section
    not found in the active file is in the archived one. A study is archived once its recommendations are all
    decided and built, so that what came of it lives in `DECISIONS.md`, this file or `ROADMAP.md`; a study of the
    old system stays active while a milestone still needs it, and a review stays active while the user has not
    answered it; a part is archived when the IDE it describes is not ported or a current document has taken it over.
    *Why:* by 2026-10-18 `study\` held 32 documents, two thirds of them reviews whose outcomes were already
    recorded elsewhere (user, 2026-10-18).
11. **The paperwork around a change goes at the merge cadence, not per change** (tenth review `study\35` §3 item 3,
    user, 2026-10-18). A change's close asks the user nothing about the behaviours it kept QB64pe's way: each is a
    row in `SOMEDAY.md`'s "QB64pe behaviours to review" and stays silent until the user asks for them. Reviews,
    and the edits of `ROADMAP.md` and of `STATUS.md` beyond its "Next" and known bugs, come once per merge into
    `main` (every two or three changes). `study\00` §5 takes no new facts: a fact measured from now on lives in its
    family's recorded output, with the family's line in `verification\README.md` saying what it shows; a change's
    `design.md` names the programs it rests on.
    *Why:* every change ended with rulings to ask for, and `study\00` §5 had grown to about 3,000 lines that no
    session reads whole (`study\35` §2).
12. **Where QB64pe's own result depends on the machine, do not build to match it bit for bit** (`study\35` §3
    item 4, user, 2026-10-18). When the old compiler's value comes from the CPU or the libraries `qb64pe.exe` was
    built with (x87 instructions, the C runtime's math), take a well-defined value (correctly rounded, as D-026)
    with a `DIVERGENCES.md` row, and say so in the design in a paragraph, not a module. The fidelity already built
    (the software `_FLOAT`, the certification of the math constants) stays; this rule does not remove it.

## Project decisions

The decision log is **`DECISIONS.md`** (one dated row per decision, oldest first; moved out of this file on
2026-10-07). When the user decides something, add a row there and put the details in the document it names.
Expert-panel recommendations: `study\07-expert-panel.md`. Open questions: `STATUS.md`.

## Layout

This table says where things are, not how many: counts live in the files that are regenerated or updated with
them (`tests\*\README.md`, `STATUS.md`'s numbers), and catalogues in their folder's `README.md` (ninth review,
`study\archive\30` §2).

| Path | Content |
|---|---|
| `STATUS.md` | The current state only: where we are, what is waiting on the user, next steps, numbers of the last full runs, known bugs. No history: a finished step changes a line, it does not add a paragraph (its first table says where everything else lives) |
| `ROADMAP.md` | Short overview from start to finish: milestones M0–M6, what is done, what is left |
| `DECISIONS.md` | Decision log: every project decision, dated, with the document that holds its details |
| `DIVERGENCES.md` | Divergence register: decided differences from the old compiler's observed behaviour |
| `DIVERGENCES-QB45.md` | Differences from QuickBASIC 4.5 (most inherited from QB64pe), each with the source of its QB 4.5 behaviour |
| `UPSTREAM.md` | Fixes to offer QB64pe: its bugs that this project fixes, each with where it is in QB64pe's sources, the fix and the divergence row |
| `SOMEDAY.md` | Deferred features and ideas, and "QB64pe behaviours to review": questionable behaviours implemented QB64pe's way |
| `GLOSSARY.md` | Plain-language definitions of compiler terms: the compiler pipeline, runtime and build, editor tooling |
| `study\00-synthesis.md` | Start here: summary of all studies, decisions, plan, measured facts, reuse verdicts; its "Document map" lists every study |
| `study\01`–`35` (active) | Studies still in use: `01`–`05` the old compiler, runtime, IDE and debugger, build; `06` VS Code parity; `07` the expert panel's plan; `09`, `10` claims checked and gaps closed; `16` FreeBASIC; `17`, `18` extension testing and practices; the newest review while it is open. One line each in `study\00`'s "Document map" |
| `study\archive\` | Closed documents, read only when needed (working rule 10); catalogue in its `README.md`: the reviews of other repositories (`11`–`14`, with `qb64fresh-scripts\`), the reviews and plans whose outcomes are decided and built (`15`, `19`–`34`), `08` (strict QB 4.5 mode, not planned), and the closed records `progress-record.md` (what `STATUS.md` held until 2026-10-09) and `support-record.md` (the feature catalogue `crates\README.md` held until 2026-10-16) |
| `Cargo.toml`, `crates\` | The new compiler `qb64rust` (Rust workspace, one crate per stage; `crates\README.md`: crate map, build, tests, snapshots) |
| `libqb\` | Plain copy of QB64pe's `Makefile`, `internal\c` (no toolchain), `licenses` and `COPYING.txt` at the reference commit (`SOURCE.md`), byte for byte, never edited by hand; the new compiler builds against it with the clone's toolchain |
| `mise.toml` | Tool versions for this repo, read by mise: Node 24 (for `vscode\`) |
| `vscode\` | M1 VS Code extension (`qb64rust`): sources, tests, fixtures, `README.md`, `DEVELOPMENT.md` |
| `.github\workflows\` | CI: `vscode-extension.yml` tests the extension on Windows against the QB64pe 4.7.0 release; `rust.yml` runs fmt, clippy, `cargo test` (tier 1), the test runner's own tests and those of `tools\testtime` and `tools\close` on Windows on every push, and, only when started by hand and once a week (working rule 9), tier 2 (the call-site check, slice list without its screen programs, which get no window there and run locally only, upstream pass list, differential pass list) against QB64pe's sources at the reference clone's commit; `repo-check.yml` runs `tools\repo_check` on every push |
| `tests\corpus\` | Golden corpus: programs with the old compiler's recorded output or compile error (`README.md`, which has the counts), including the `slice\` group and `screen\` (programs with a window, each with the recorded state of its screen, `.screen`, or several states taken while it runs; a program may have its data files in a folder `<name>.files\`; `g12_game` is Civil War Strategy played from its keys file); checked by `run_legacy_tests.py --suite corpus`; `slice.list` names those the new compiler must pass |
| `tests\frontend\` | Front-end tests of the new compiler, run by mode line (`' TEST: <mode>`; modes `parse-ok`, `check-ok`, `check-fail`, `typed`, `ir`, `cpp`); their included files in `inc\` (compiler root `inc\root\`) |
| `tests\upstream\` | Copy of the text files of QB64pe's `tests\compile_tests` (MIT, `SOURCE.md`, never edited by hand), and `root\` (the compiler root for them: a mirror of `extra\`, made by the copy script); `pass.list` (upstream programs the new compiler passes) and `deferred.list` (those that need `_ARRAYCOPY` or a form left out of a change); progress as "x of y" (`README.md`) |
| `tests\differential\` | Differential programs generated by `crates\difftest` (never edited by hand) with their output recorded from `qb64pe.exe`; `pass.list` names those the new compiler passes (`README.md`) |
| `tests\callsite\` | One-statement programs for the call-site check, one per built-in form and argument type; never run (a program may have a file it names in a folder `<name>.files\`). A new plain built-in comes in with a program here (`DECISIONS.md` 2026-10-10) |
| `tests\programs\` | Real programs taken whole (`README.md`): `civil-war-strategy\`, the user's MIT game, the real-program target for QB 4.5 graphics; byte for byte, never edited by hand; tier 1 runs the front end over them |
| `tests\snippets\` | QB64Fresh's inline test snippets as inputs, labelled by `qb64pe.exe` (`.err` when it rejects one; `SOURCE.md`) |
| `tests\known_false_errors.list`, `tests\known_unsupported_rejections.list`, `tests\known_parse_gaps.list`, `tests\known_clean_not_passing.list` | Shrink-only lists of tier 1: programs the old compiler accepts that get a real error, programs it rejects that get only "not supported yet" errors, accepted programs (plus the old compiler's sources) that do not parse cleanly (`tests\upstream\README.md`), and programs that compile cleanly but are on no pass list, with the reason (kept by hand) |
| `baselines\` | Recorded test results of the old compiler (546 pass, 1 environment failure; format tests 24 of 24) |
| `verification\` | Measurement programs of the old compiler, each with its recorded output, and `run.sh` (starts the runner's `--suite verification`, which records them); the catalogue of the families, `v13*` to the newest, is `verification\README.md` |
| `tools\legacy_tests\` | Windows runner for the QB64pe test suites (compile, qbasic, format; old or new compiler), the golden corpus and the verification programs (`--suite verification`, a recorder), its own tests (`test_run_legacy_tests.py`: a change to a comparison or recording rule comes with a change to its test), and its `known_failures.txt`; `--check-list` judges a pass list from a full run's results; `counts.py` writes the count blocks of the test READMEs (`test_counts.py`) |
| `tools\close\` | `close.py`: the closing runs of a change (working rule 9) as a plan (`--plan`, the commands in order, each run by its own command) and a check (`--check`, each log: ran on the current build, and passed); its own tests (`test_close.py`; `README.md`) |
| `tools\libqb\` | `copy_libqb.py`: writes `libqb\` from the clone at a commit and, with `--check`, says whether it still equals it; its own tests (`test_copy_libqb.py`) |
| `tools\callsite\` | `callsite.py`: compares the libqb call the new compiler writes for each program of `tests\callsite\` with the old compiler's (`-z` on both, nothing built or run); part of tier 2 (`README.md`: the normalisation rules and what it does not compare) |
| `tools\repo_check\` | `check_repo.py`: tracked files must hold no local paths, temp folders or network shares (rule 2) and no stray control bytes (rule 7); `--untracked` also checks new files before they are staged |
| `tools\testtime\` | `test_time.py`: every test run of every Claude Code session, how long it took and, where the transcript shows it, whether it passed, read from the sessions' transcripts; `runs.csv` and `sessions.csv` (the record, added to at the close of each change), `changes.csv` and `activity.csv` (each OpenSpec change's elapsed and active time, waits and kinds of work, since 2026-10-16) and `REPORT.md` (testing as a share of the working time, by day and by kind of run, the runs repeated with nothing edited, and per change; `README.md`); its own tests (`test_record.py`) |
| `tools\diagnostics\` | `old_wording.py`: the old compiler's wording of each error, paired from recorded rejections, written to `crates\base\src\old_wording.tsv` for the note after an error; `--check` holds the notes to the recorded messages (`README.md`) |
| `tools\builtins\` | Extractor for the built-in table (`extract_builtins.py`) and its output `builtins.json` (with the names of QB64pe's auto-included files and, for each constant of the file included before the first line, its suffix and the text of its expression) |
| `tools\wiki\` | `fetch_wiki.py` fetches the QB64pe wiki as raw wikitext into `cache\` (git-ignored, no licence stated; local reference only) |
| `tools\upstream\` | `copy_upstream_tests.py`: redoes the copy in `tests\upstream\` from the clone at the pinned commit |
| `tools\snippets\` | `extract_qb64fresh_snippets.py`: extracts and labels the snippets in `tests\snippets\` |
| `tools\screen\` | The screen-state check (`README.md`): `screen_state.cpp`, linked into a test program of either compiler, writes the state of the screen at exit and gives the program its keys, and with the keys file's `prompt` and `state` waits for a program's announced waits and writes states while it runs; `state_to_png.py` shows a state (one of several, or all); `cut_game.py` writes the programs cut from Civil War Strategy, with the pictures they load, and `g12_game`, the whole game with listed edits for the scripted game; `make_images.py` writes the images the screen program `i02_load_images` loads |
| `..\QB64pe\` | Reference clone of QB64pe (`davidshq-contribute/QB64pe`, tracks upstream `main`, version 4.7.0-GLFW) |
| `..\FreeBASIC\` | Reference clone of FreeBASIC (shallow, `5714d10adb`, 2026-04-05). Read-only; GPL/LGPL, nothing copied |
| `<qb64contain>`, `<share>` | Placeholders for the user's local folder of earlier projects (QB64Fresh, qb64pe-vscode, a QB64pe clone with notes) and its network copy; used in `study\` and `STATUS.md` instead of machine-specific paths. Do not write full local paths or host names into this repo. |
