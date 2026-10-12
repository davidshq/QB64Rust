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
     checked against `qb64pe.exe`. QB64Fresh's 261 `runtime_comparison` BASIC programs are taken as test inputs.
     *Why:* the reviews (`study\archive`) found their architecture unreliable, not their provenance; a blanket
     ban would be a handicap (user, 2026-10-03).
6. **"The panel"** means a review by these roles: a pragmatic engineer, a QB64 engineer, a compiler/languages
   engineer, a Rust engineer, a test engineer, and any other role Claude thinks fits the topic (name the extra
   roles and say why). The panel is role-played inline; it is not a set of subagents (rule 1) (user, 2026-10-03).
   Every role, the pragmatic engineer first, counts effort by rule 8.
7. **Text with backslashes goes through the Edit/Write tools or a script file, never through the shell.** That
   covers Windows paths (`study\19`), C or Python escapes (`\\`, `\n`) and anything else containing `\`. Do not
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

## Project decisions

The decision log is **`DECISIONS.md`** (one dated row per decision, oldest first; moved out of this file on
2026-10-07). When the user decides something, add a row there and put the details in the document it names.
Expert-panel recommendations: `study\07-expert-panel.md`. Open questions: `STATUS.md`.

## Layout

| Path | Content |
|---|---|
| `STATUS.md` | The current state only: where we are, open items, next steps, numbers of the last full runs, known bugs. No history: a finished step changes a line, it does not add a paragraph (its first table says where everything else lives) |
| `ROADMAP.md` | Short overview from start to finish: milestones M0–M6, what is done, what is left |
| `DECISIONS.md` | Decision log: every project decision, dated, with the document that holds its details |
| `DIVERGENCES.md` | Divergence register: decided differences from the old compiler's observed behaviour |
| `DIVERGENCES-QB45.md` | Differences from QuickBASIC 4.5 (most inherited from QB64pe), each with the source of its QB 4.5 behaviour |
| `SOMEDAY.md` | Deferred features and ideas, and "QB64pe behaviours to review": questionable behaviours implemented QB64pe's way |
| `GLOSSARY.md` | Plain-language definitions of compiler terms: the compiler pipeline, runtime and build, editor tooling |
| `study\00-synthesis.md` | Start here: summary of all studies, decisions, plan, measured facts, reuse verdicts; its "Document map" lists every study |
| `study\01`–`29` | Studies and reviews: `01`–`05` the old compiler, runtime, IDE and build; `06`–`10` VS Code parity, panel, QB 4.5 mode, verification, gaps; `15`–`29` reviews of plan and code, FreeBASIC, extension testing and practices, test cadence, Rust review, IR review. One line each in `study\00`'s "Document map" |
| `study\progress-record.md` | Closed record: the per-change history `STATUS.md` held until 2026-10-09 (M0 to M2 step 8), moved unchanged. Not added to |
| `study\archive\` | Closed reviews of other repositories, kept for the record only: `11` existing VS Code extensions, `12` QB64Fresh (the user's earlier Rust rewrite), `13` reference-doc sources, `14` the `docs-new-2` branch, and `qb64fresh-scripts\` (measurements behind `12`). Their conclusions are in `study\00` §11; nothing in the active plan depends on reading them. |
| `Cargo.toml`, `crates\` | The new compiler `qb64rust` (Rust workspace, one crate per stage; `crates\README.md`: crate map, build, tests, snapshots) |
| `vscode\` | M1 VS Code extension (`qb64rust`): sources, tests, fixtures, `README.md`, `DEVELOPMENT.md` |
| `.github\workflows\` | CI: `vscode-extension.yml` tests the extension on Windows against the QB64pe 4.7.0 release; `rust.yml` runs fmt, clippy, `cargo test` (tier 1) and the test runner's own tests on Windows, and tier 2 (the call-site check, slice list, upstream pass list, differential pass list) against QB64pe's sources at the reference clone's commit; `repo-check.yml` runs `tools\repo_check` on every push |
| `tests\corpus\` | Golden corpus: 357 programs with the old compiler's recorded output or compile error (`README.md`), including the `slice\` group and `screen\` (29 programs with a window, each with the recorded state of its screen, `.screen`); checked by `run_legacy_tests.py --suite corpus`; `slice.list` names the 287 the new compiler must pass |
| `tests\frontend\` | Front-end tests of the new compiler, run by mode line (`' TEST: <mode>`; modes `parse-ok`, `check-ok`, `check-fail`, `typed`, `ir`, `cpp`); their included files in `inc\` (compiler root `inc\root\`) |
| `tests\upstream\` | Copy of the text files of QB64pe's `tests\compile_tests` (MIT, `SOURCE.md`, never edited by hand), and `root\` (the compiler root for them: a mirror of `extra\`, made by the copy script); `pass.list` (upstream programs the new compiler passes) and `deferred.list` (the 125 that need deferred array features); progress "x of 279" (`README.md`) |
| `tests\differential\` | Differential programs generated by `crates\difftest` (never edited by hand) with their output recorded from `qb64pe.exe`; `pass.list` names those the new compiler passes (`README.md`) |
| `tests\callsite\` | One-statement programs for the call-site check, one per built-in form and argument type; never run. A new plain built-in comes in with a program here (`DECISIONS.md` 2026-10-10) |
| `tests\programs\` | Real programs taken whole (`README.md`): `civil-war-strategy\`, the user's MIT game, the real-program target for QB 4.5 graphics; byte for byte, never edited by hand; tier 1 runs the front end over them |
| `tests\snippets\` | QB64Fresh's inline test snippets as inputs, labelled by `qb64pe.exe` (`.err` when it rejects one; `SOURCE.md`) |
| `tests\known_false_errors.list`, `tests\known_unsupported_rejections.list`, `tests\known_parse_gaps.list`, `tests\known_clean_not_passing.list` | Shrink-only lists of tier 1: programs the old compiler accepts that get a real error, programs it rejects that get only "not supported yet" errors, accepted programs (plus the old compiler's sources) that do not parse cleanly (`tests\upstream\README.md`), and programs that compile cleanly but are on no pass list, with the reason (kept by hand) |
| `baselines\` | Recorded test results of the old compiler (546 pass, 1 environment failure; format tests 24 of 24) |
| `verification\` | Small programs behind `study\09`, `study\10` and later measurements (`v13*`: type suffixes on DIMmed names; `v14*`, `v15*`: procedures, scopes, reserved names, errors; `v16*`: comment metacommands, `DATA`, line numbers, blocks, templates, `$IF`, `$INCLUDE`, member access, with include files in `v16_inc\`; `v17*`: control flow, constants, operators; `v18*`: arrays, `TYPE`, store rules per place; `v19*`: procedure names (`v19_proc_names.py` and its list), blanks around a dot, `OPTION _EXPLICIT` and runtime errors in included files, with include files in `v19_inc\`; `v20*`: core built-ins, `SELECT CASE`, `ON … GOTO/GOSUB`, `v20_x*` one rejection each; `v21*`: the new numeric types, `_BIT`, fixed-length strings, the numeric-types spec scenarios, `v21_x*` one rejection each; `v22*`: built-in statements (`v22_a` statement calls, `v22_b` files, `v22_c` data, `v22_d` `SWAP`, `MID$`, `RANDOMIZE`, `SHELL`, `ENVIRON$`, `v22_e` console input with `.stdin` files, `v22_f` functions added by demand, `v22_x*` one rejection or accepted form each, include files in `v22_inc\`; `v23_post_keys.py`: posts keys to a program's window, the method measured and not taken for the screen-state check; its programs are in `tests\corpus\screen\`; `v24*`: step 10, measured before its design: `v24_a` default types, `v24_b` dynamic arrays and `$DYNAMIC`, `v24_c` `REDIM` and `_PRESERVE`, `v24_d` `ERASE`, `v24_e` `OPTION BASE`, `v24_f` arrays used without `DIM`, `v24_g` arrays in procedures, array parameters and `COMMON`, `v24_h` `TYPE` member arrays, `STRING` members, whole values and `TYPE` parameters, `v24_i` the type before the names, `v24_j` a minus right of `^`, `v24_x*` one accepted or rejected form each (`x244`–`x255`: measured while group 4 was built and reviewed; `x256`–`x263`: before group 5, arrays used without `DIM`; `x264`–`x268`: by the review of group 5, where their allocation runs inside a statement; `x269`–`x282`: while group 6 was built, arrays in procedures, `STATIC` arrays and procedures, `SHARED a()`; `x283`–`x285`: by the review of group 6, `DIM` in a procedure with `STATIC` after its header and after `STATIC c(3)`; `x286`–`x296`: before group 7, array parameters; `x297`–`x302`: while group 7 was built, what gives an array listed by `COMMON a()` or `STATIC a()` its number of dimensions; `x303`–`x305`: by the review of group 7, what may stand between an array parameter's parentheses; `x306`–`x308`: before group 8, the order of a whole-value copy and more forms of a user-type parameter; `x309`: by the review of group 8, `STRING` members in arrays, parameters and procedures; `x310`–`x326`: before and while group 9 was built, member arrays: what finds the place of one, the order of a store through a member, whole values, arguments, and ten forms settled one program each); `v25*`: the signs the old compiler drops from an expression (`v25_signs`, `v25_signs_const`, `v25_x*` one rejection each); `v26*`: step 11, measured before its design: `$CONSOLE`, `$SCREENHIDE`, `_DEST _CONSOLE`, `_ECHO`, the flag `_CONSOLE_` (`v26_a`–`v26_h2`, `v26_x*` one accepted or rejected form each)), their outputs, and `run.sh` (starts the runner's `--suite verification`, which records them) |
| `tools\legacy_tests\` | Windows runner for the QB64pe test suites (compile, qbasic, format; old or new compiler), the golden corpus and the verification programs (`--suite verification`, a recorder), its own tests (`test_run_legacy_tests.py`: a change to a comparison or recording rule comes with a change to its test), and its `known_failures.txt` |
| `tools\callsite\` | `callsite.py`: compares the libqb call the new compiler writes for each program of `tests\callsite\` with the old compiler's (`-z` on both, nothing built or run); part of tier 2 (`README.md`: the normalisation rules and what it does not compare) |
| `tools\repo_check\` | `check_repo.py`: tracked files must hold no local paths, temp folders or network shares (rule 2) and no stray control bytes (rule 7); `--untracked` also checks new files before they are staged |
| `tools\builtins\` | Extractor for the built-in table (`extract_builtins.py`) and its output `builtins.json` (with the names of QB64pe's auto-included files) |
| `tools\wiki\` | `fetch_wiki.py` fetches the QB64pe wiki as raw wikitext into `cache\` (git-ignored, no licence stated; local reference only) |
| `tools\upstream\` | `copy_upstream_tests.py`: redoes the copy in `tests\upstream\` from the clone at the pinned commit |
| `tools\snippets\` | `extract_qb64fresh_snippets.py`: extracts and labels the snippets in `tests\snippets\` |
| `tools\screen\` | The screen-state check (`README.md`): `screen_state.cpp`, linked into a test program of either compiler, writes the state of the screen at exit and gives the program its keys; `state_to_png.py` shows a state; `cut_game.py` writes the drawing programs cut from Civil War Strategy |
| `..\QB64pe\` | Reference clone of QB64pe (`davidshq-contribute/QB64pe`, tracks upstream `main`, version 4.7.0-GLFW) |
| `..\FreeBASIC\` | Reference clone of FreeBASIC (shallow, `5714d10adb`, 2026-04-05). Read-only; GPL/LGPL, nothing copied |
| `<qb64contain>`, `<share>` | Placeholders for the user's local folder of earlier projects (QB64Fresh, qb64pe-vscode, a QB64pe clone with notes) and its network copy; used in `study\` and `STATUS.md` instead of machine-specific paths. Do not write full local paths or host names into this repo. |
