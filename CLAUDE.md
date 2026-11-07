# Project rules for Claude

This file is the authoritative place for rules in this project; the decision log is `docs\DECISIONS.md`. Claude's
private memory may hold a copy of rules, but never the only copy.

**The project's documents are in `docs\`**, all but `README.md` and this file. 

A document named without a folder (`STATUS.md`, `DECISIONS.md`, `SOMEDAY.md`, …), here or anywhere in the repo, is the one in `docs\`.

**Start each session by reading `docs\STATUS.md`** (current phase and next steps). **Read `docs\DECISIONS.md` before
changing direction, design or scope.**

## Explicit Human Rules
1. **Never stage or unstage files, stash or unstash.** 
  - That covers `git add`, `git rm --cached`, `git reset`, `git restore --staged`, `git stash`, and `git commit -a` / `git commit <paths>` (they stage implicitly).
  - **One exception: move or rename a tracked file with `git mv`**, never a plain `mv`, so that git keeps it as a
   rename and its history follows.
  - Never change what the user has staged.
  - **Enforced by a hook** (`tools\hooks\guard_tool.py`, `.claude\settings.json`, 2026-10-31): staging, stashing,
   resetting and a plain `mv`/`rm` of a tracked file are refused before they run. A refusal is not an error to work
   around: do what its message says.
2. **Nothing private, personal or secret goes into the repo**
  - No full local paths (`C:\Users\…`, `/c/…`, temp folders), machine or host names, network shares, tokens, keys or passwords. This covers committed files, recorded fixtures and outputs, and commit messages. 
  - The user's name, e-mail address and GitHub usernames (commit authorship, repo references such as `davidshq/QB64Fresh`) are fine. 
  - Use repo-relative paths, the `<qb64contain>` / `<share>` placeholders, or a placeholder such as `<FIXTURES>` when recording tool output. Check new files and recorded outputs before staging them.
3. **Never add a `Co-Authored-By: Claude …` trailer** (or any other Claude attribution line) to commit messages or
   pull request descriptions.

## Working rules

1. **No subagents unless the user explicitly allows it for that task.**
  - Do the work inline, sequentially. 
  - Permission for one task does not carry over to the next. If a subagent is ever allowed, tell it not to spawn helpers of its own.
   *Why:* on 2026-10-02 a five-agent parallel study hit the session usage limit within minutes, and agents spawned
   helpers after the user asked for one at a time.
2. **Where things live.**
   - **This repo (`QB64Rust`) is the project.** Paths in this file are relative to it. The parent folder (`..`)
     has symlinks `CLAUDE.md`, `SOMEDAY.md` (to `docs\SOMEDAY.md`) and `study` pointing here, so sessions started
     in the parent see the same files; edit the real files here, not copies.
   - **`<qb64contain>` is `..\..\qb64contain`**, beside the parent folder, not the parent itself. It holds QB 4.5
     (`<qb64contain>\qb45\install`), which `tools\qb45\qb45.py` finds there by default (`tools\qb45\README.md`).
   - **`..\QB64pe\` is a read-only reference clone.** Do not modify it. Study notes and design documents go in
     `study\`.
   - **`..\QB64pe-latest\` is a second clone, of upstream's `main`** (`QB64-Phoenix-Edition/QB64pe`, cloned
     2026-11-02 at `8070eebd2`). It shows what upstream has fixed since the reference. It is not the parity
     target: tools and recordings use `..\QB64pe\` at `16f629784e`. Pull it to update it; change nothing else.
   - **Never save anything to memory only.** Any rule, decision or fact worth keeping goes into this repo (this
     file, `DECISIONS.md`, or `study\`). Memory may hold a pointer or copy in addition.
   - **A game port's records go in the game's repo, not here** (user, 2026-11-06). The games are
     `..\world-war-2`, `..\civil-war-battleset`, `..\civil-war-strategy` and `..\wars-of-napoleon`. A port's
     plan or study, its decisions, its status, its to-do items and its someday ideas go in the game's `docs\`
     or `TODO.md`. They do not go in this repo's `study\`, `DECISIONS.md`, `STATUS.md` or `SOMEDAY.md`. What
     a port finds about the compiler still comes here: a bug, a corpus or verification program, a
     `DIVERGENCES-QB45.md` or `QB64PE-DIVERGENCES.md` row, a lesson for `PORTING-PDS.md`. Civil War Strategy
     as this repo's own test target (`tests\programs\civil-war-strategy`, `screen\g01`–`g12`) stays here.
     *Why:* the ports' plans and rulings had filled this repo's documents (`DECISIONS.md` 2026-11-06).
3. **Code from other projects.** Two tiers:
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
4. **The panel, and how effort is counted.**
   - **"The panel"** means a review by these roles: a pragmatic engineer, a QB64 engineer, a compiler/languages
     engineer, a Rust engineer, a test engineer, and any other role Claude thinks fits the topic (name the extra
     roles and say why). The panel is role-played inline; it is not a set of subagents (rule 1) (user,
     2026-10-03). **When a decision is put to the user, the panel's view comes with it**, unasked, with its
     recommendation (user: "always give the panel thoughts", 2026-10-26).
   - **Count effort in Claude's time, not a person's**, in the panel and everywhere else. When weighing whether
     something is worth doing (a recommendation, a plan, a panel review, "now or later"), estimate how long Claude
     takes to write and check it, not how long an engineer would. Work that is a day for a person is often minutes
     here, so "not worth the effort", "not used often enough to repay the work" and "wait until it next needs a
     change" are not reasons by themselves. What still counts: a risk to correctness, a cost that stays (more code
     to maintain, disk, CI time), machine time of a run, the user's time (reading, deciding, staging), and scope
     the user has not asked for. Say which of these applies, or recommend doing it.
     *Why:* on 2026-10-10 the panel advised leaving `verification\run.sh` outside the test runner because full
     runs are rare; the user: "it isn't going to take time", and about 80% of the pragmatic engineer's timing
     objections would go away (user, 2026-10-10).
5. **Text with backslashes goes through the Edit/Write tools or a script file, never through the shell.** That
   covers Windows paths (`study\archive\19`), C or Python escapes (`\\`, `\n`) and anything else containing `\`. Do not
   pass it in a heredoc, inline `python -c`/`python - <<EOF`, `sed` or `printf` in the Bash tool, which collapses
   `\\` to `\`. Python then reads `\1`, `\v` or `\r` as control characters. If a shell edit with backslashes
   happened anyway, scan the touched files for bytes 0x00–0x1F (other than tab, LF and the CR of CRLF) before
   moving on.
   *Why:* on 2026-10-03 this corrupted files four times (`study\02` became `study` + 0x02, `verification\v13`
   became a vertical tab) and each needed a scan and a redo (user, 2026-10-03).
6. **Testing.** Run `cargo test -p <crate>` on what you touched while working. Run `just quick` in the background
   before handing over files to stage, and say what ran, what stood and why. `just full` runs in a worktree of the
   commit, started by the user; `just merge` there before a merge into `main`. Read a run's output from its log; never start a run
   again to read it. (`tools/check/README.md`; the OpenSpec change `testing-reset`, `DECISIONS.md` 2026-10-31.)
   - **Large runs go in the suites VM, not on the host** (user, 2026-11-05). A large run is a whole suite, a
     whole verification family, or more than about 20 programs, by `check` or by calling
     `tools\legacy_tests\run_legacy_tests.py` directly. Run it with `check full` or `check <step> --where vm`, so
     that `suites.ps1` takes it to `qb64rust-suites`. A step named alone without `--where vm` runs on the host.
     If the run cannot be done there (an option `check` does not pass, such as `--trace-values`), ask the user
     first. On the host stay `cargo test -p`, `just quick` and a few programs named one by one.
   - **Never run a program on the host that can make Windows ask the user something.** This covers network
     programs: one that opens a port makes Windows Defender Firewall ask for permission, and every new build is a
     new executable, so it asks again. These are the programs whose `.external` sidecar starts with `network:`
     (for example `s141_tcp_loopback` and `v63_d1_statuscode_closed`). The runner skips them on a Windows host
     and runs them in the suites VM (`QB64RUST_ALLOW_NETWORK=1`, set by `check`). Claude never sets that
     variable on the host; the user may, for example to record network programs with `qb64pe`. A new program
     that calls `_OPENHOST` or `_OPENCLIENT` needs the sidecar; `inputs::network_programs_are_marked` checks it.
   - **A game port's own `tests\play.py` is not such a run** (user, 2026-11-06; D7 of Civil War Battleset's `docs\port-study.md`). The scripts of
     World War 2 and Civil War Battleset run on the host, on a desktop of their own that the user never sees,
     with `--jobs 4`. They open no window and run no network program.
   - **The hidden desktop does not hide `SHELL`.** A program that runs `SHELL` starts a `cmd.exe`, and its
     console window shows on the user's desktop. So a game port's scripts may run on the host only when no
     `SHELL` can run under them (a hook in the port's `SCRIPT.BM`, or `SHELL _HIDE`). Before a long run, check
     it with one script on the host that reaches a `SHELL`. The same check goes for any other program run on
     the host that may start a child process.
     *Why:* on 2026-11-06 Civil War Battleset's `make_scripts.py` and `play.py record` ran for about two and a
     half hours on the host. Nearly every battle ends in `SHELL "copy maxskors temp.$$$"`, and each one flashed a
     terminal window at the user while they worked (user: "they seem to [open terminal windows]").
   *Why:* on 2026-11-05 a session ran the whole corpus and two verification families on the host with
   `--trace-values`, eight jobs at once, while the user was working; network programs ask the user to allow them
   through the firewall (user: "that seems stupid").

7. **Documents: what is read when, and when the paperwork is done.**
   - **`study\archive\` is read only when needed**: when a live document points there for a fact or a reason the
     task in hand needs, or when the user asks. It is not read to get oriented, and its files are never edited
     (paths inside them may be out of date). A study keeps its number when it is archived: a `study\NN` that is
     not in `study\` is in `study\archive\`. When only part of a study is archived, the part goes to a file of the
     same name in `study\archive\`, under the same headings, and each half names the other at its top: a section
     not found in the active file is in the archived one. A study is archived once its recommendations are all
     decided and built, so that what came of it lives in `DECISIONS.md`, this file or `ROADMAP.md`; a study of the
     old system stays active while a milestone still needs it, and a review stays active while the user has not
     answered it. *Why:* by 2026-10-18 `study\` held 32 documents, two thirds of them reviews whose outcomes were
     already recorded elsewhere (user, 2026-10-18).
   - **The paperwork around a change goes at the merge cadence, not per change** (`study\35` §3 item 3, user,
     2026-10-18). The end of a change asks the user nothing about the behaviours it kept QB64pe's way: each is a row
     in `SOMEDAY.md`'s "QB64pe behaviours to review" and stays silent until the user asks for them. Reviews, and
     the edits of `ROADMAP.md` and of `STATUS.md` beyond its "Next" and known bugs, come once per merge into
     `main`. `study\00` §5 takes no new facts: a fact measured from now on lives in its family's recorded output,
     with the family's line in `verification\README.md` saying what it shows; a change's `design.md` names the
     programs it rests on. *Why:* every change ended with rulings to ask for, and `study\00` §5 had grown to about
     3,000 lines that no session reads whole (`study\35` §2).
8. **Where QB64pe's own result depends on the machine, do not build to match it bit for bit** (`study\35` §3
    item 4, user, 2026-10-18). When the old compiler's value comes from the CPU or the libraries `qb64pe.exe` was
    built with (x87 instructions, the C runtime's math), take a well-defined value (correctly rounded, as D-026)
    with a `QB64PE-DIVERGENCES.md` row, and say so in the design in a paragraph, not a module. The fidelity already built
    (the software `_FLOAT`, the certification of the math constants) stays; this rule does not remove it.
9. **American English spelling in new text** (user, 2026-10-28): "license", "behavior", "color", "-ize". This
    covers new prose, comments, identifiers, messages and file names, and any line already being edited for
    another reason. Existing text is not swept to change its spelling; that is a `SOMEDAY.md` row. Text copied
    from elsewhere (upstream files, licenses, recorded outputs) keeps its own spelling.
10. **Write plainly and specifically** (user, 2026-11-02). The reader is technical but may not have the context.
    - Use short sentences with one idea each. About 25 words at most is a rule of thumb, not a limit.
    - Say what happened or what is meant first. Put references (`D7`, a `DECISIONS.md` row, a study number) after
      it, never in place of it.
    - Keep the specifics: file names, program names, line numbers, counts, exact messages. Plain means clear, not
      general. "18 programs where we report the error at the SUB line" beats both "some exceptions" and
      "18 rows (D5)".
    - Use normal word order and common words. Avoid chains of parentheses and semicolons.
    - Commit messages: a subject of about 50 characters that completes "If applied, this commit will …", and a
      body wrapped at 72 characters that says what changed and why.

    This covers new text: documents, comments, commit messages and replies. Existing text is rewritten when it
    is next edited for another reason, not in a sweep.
    *Why:* the project's documents grew dense and hard to read, and each session copied that style (user,
    2026-11-02). The rule asks for specifics so that plain writing does not turn vague (plain-language guidance:
    clarity and precision go together).
11. **OpenSpec is for changes that need a design** (user, 2026-11-02). Use an OpenSpec change when a change has
    an open design question, spans several crates or sessions, or adds a mechanism (a tool, a test kind, a
    runtime part). Otherwise build directly: one measured behavior, small code, no open question. A direct change
    still gets its usual records:
    - a `DECISIONS.md` row, and a `QB64PE-DIVERGENCES.md` row if it differs from QB64pe;
    - the merged spec under `openspec\specs\` edited in place, so that it never says the opposite of the code;
    - its tests, including recorded programs for the behavior, not only unit tests.

    *Why:* a change's proposal, design and tasks cost the user reading time, and for a small change the
    `DECISIONS.md` row and the spec edit carry the same facts. Examples built directly: the 40-character limit on
    names and `_BIT` parameters (2026-11-02).

## Project decisions

The decision log is **`docs\DECISIONS.md`** (one dated row per decision, oldest first; moved out of this file on
2026-10-07). When the user decides something, add a row there and put the details in the document it names; a minor
row (document arrangement, a change archived, a one-time exception) goes to `DECISIONS-SMALL.md` instead.
Expert-panel recommendations: `study\07-expert-panel.md`. Open questions: `STATUS.md`.

## Layout

Where things are: **`docs\LAYOUT.md`**, one row per file or folder. Read it when looking for something; update its row
when a file or folder is added, moved or changes purpose.
