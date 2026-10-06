# Project rules for Claude

This file is the authoritative place for rules and decisions in this project. Claude's private memory may hold a
copy, but never the only copy.

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
   or `study\`). Memory may hold a pointer or copy in addition.
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
     Neither is imported or built on (decisions above: the extension is written fresh; QB64Fresh's front end and
     IR are unsound). A self-contained helper, test, grammar fragment or fixture may be copied when it saves time,
     provided it is read in full, fits the design here, and anything it asserts about QB64pe is checked against
     `qb64pe.exe`. QB64Fresh's 261 `runtime_comparison` BASIC programs are taken as test inputs.
     *Why:* the reviews (`study\archive`) found their architecture unreliable, not their provenance; a blanket
     ban would be a handicap (user, 2026-10-03).
6. **"The panel"** means a review by these roles: a pragmatic engineer, a QB64 engineer, a compiler/languages
   engineer, a Rust engineer, a test engineer, and any other role Claude thinks fits the topic (name the extra
   roles and say why). The panel is role-played inline; it is not a set of subagents (rule 1) (user, 2026-10-03).
7. **Text with backslashes goes through the Edit/Write tools or a script file, never through the shell.** That
   covers Windows paths (`study\19`), C or Python escapes (`\\`, `\n`) and anything else containing `\`. Do not
   pass it in a heredoc, inline `python -c`/`python - <<EOF`, `sed` or `printf` in the Bash tool, which collapses
   `\\` to `\`. Python then reads `\1`, `\v` or `\r` as control characters. If a shell edit with backslashes
   happened anyway, scan the touched files for bytes 0x00–0x1F (other than tab, LF and the CR of CRLF) before
   moving on.
   *Why:* on 2026-10-03 this corrupted files four times (`study\02` became `study` + 0x02, `verification\v13`
   became a vertical tab) and each needed a scan and a redo (user, 2026-10-03).

## Project decisions

| Date | Decision |
|---|---|
| 2026-10-02 | Ground-up rewrite of QB64pe. Start by understanding the existing codebase (done: `study\00`–`05`). |
| 2026-10-02 | Do not port the text-mode IDE. Reach IDE feature parity with a VS Code extension (`study\06-vscode-parity.md`). |
| 2026-10-02 | New compiler is written in **Rust**. |
| 2026-10-02 | **New error messages** (with columns, multiple errors); optionally emit the old QB64pe texts during a transition period. |
| 2026-10-02 | **No strict QuickBASIC 4.5 mode** (QB64pe has none). What one would need: `study\08-qb45-strict-mode.md`. |
| 2026-10-02 | Self-hosting is not a goal (follows from Rust). Compiling the old `qb64pe.bas` stays as a large test case. |
| 2026-10-02 | Recent upstream QB64pe features: keep `$USELIBRARY`, `$ERRORLOCATION`, GLFW runtime; defer the complex array work (TYPE member arrays, `_ARRAYCOPY`, whole-array assignment, `REDIM _RETAIN`) to `SOMEDAY.md`. |
| 2026-10-02 | Do not use the user's old `windsurf` branch (removed from the local clone). |
| 2026-10-02 | M1 extension is **written fresh** in `vscode\`; `vscode-qb64fresh` and `qb64pe-vscode` are reference only (overrides `study\archive\11`'s import recommendation). Refined 2026-10-03 by rule 5: small pieces of `vscode-qb64fresh` may be taken. |
| 2026-10-02 | Project name for ids, settings, commands and grammar scope: **`qb64rust`**. |
| 2026-10-02 | M1 scope is the core list only (grammar, build/run, old-compiler diagnostics, `-y` formatting, CP437 default). Snippets, keyword generation from `builtins.json`, and help are later. Planned as OpenSpec change `m1-vscode-extension-v0`. |
| 2026-10-02 | The earlier `rewrite-decision.md` (advised keeping QB64Fresh's front end, no empty-repo rewrite) was considered and overruled: the ground-up rewrite **stands** (`study\archive\12`: QB64Fresh front end not lossless, name-based IR, 19 of 331 outputs match). The other `<qb64contain>` notes are not brought into `study\`; `study\archive\12` summarises the one that mattered. |
| 2026-10-02 | Golden corpus: record the outputs of QB64Fresh's 261 `runtime_comparison` programs with the old compiler **at the start of M2** and add them to the conformance tests. Rust workspace is also set up at the start of M2. |
| 2026-10-02 | QB64pe wiki: fetched now with `tools\wiki\fetch_wiki.py` into the git-ignored `tools\wiki\cache\` (1,124 pages, main + Template namespaces). **Local reference only**: the wiki states no licence, so no wiki text is committed or shipped until the maintainers have been asked (when help/hover work starts). |
| 2026-10-02 | libqb comes into this repo at M3 as a **plain copy** of `..\QB64pe\internal\c` at the pinned commit (recorded in a file next to it), not a git subtree. Upstream changes are merged by hand when wanted. |
| 2026-10-02 | Pre-coding review (`study\15-pre-coding-review.md`): the Rust front end lexes **bytes, never `&str`**; an end-to-end vertical slice (PRINT of a few expression types through IR to C++ linked with libqb) is built **early in M2** to validate the IR/ABI design; the language server reports only errors the old compiler also reports until differential tests say otherwise; M1's backend interface stays minimal because the language server replaces check and format in M2. |
| 2026-10-03 | FreeBASIC (`github.com/freebasic/fbc`) cloned to `..\FreeBASIC\` as a second **read-only reference**; no code reused (GPL/LGPL). Lessons and six follow-ups: `study\16-freebasic.md` (follow-ups decided 2026-10-03, below). |
| 2026-10-03 | Extension needs **VS Code 1.100 or later** (`engines.vscode ^1.100.0`), raised from 1.85: allows ESM-only mocha 12 (no audit findings) and `TextDocument.encoding`. Tests stay on mocha + `@vscode/test-electron` like most large extensions, and run on stable and on 1.100 (`study\17-vscode-extension-testing.md`). |
| 2026-10-03 | Panel decision on the FreeBASIC follow-ups (`study\16` §8): **LONG overflow wraps** (generated code and constant folding; matches the old compiler's default build, differs from its `-O2`); test mode lines, typed-tree assertions, snapshot tests and one-error-per-statement recovery in M2; "cannot raise" flag only as a field defaulting to "may raise". |
| 2026-10-03 | Extension CI on **Windows** with GitHub Actions against the QB64pe `v4.7.0-GLFW` release; macOS/Linux later (`study\18` G). |
| 2026-10-03 | **`_INTEGER64` overflow wraps** too, like LONG (generated code and constant folding). Measured: same exposure as LONG, default build wraps, `-O2` does not (`verification\v12_wrap_int64`, `study\16` §8). |
| 2026-10-03 | **Test cadence** (`study\19`): the full golden corpus is not part of the edit loop. Tier 1 `cargo test` (incl. corpus front end only, no C++) on every edit; tier 2 a `--glob` slice before commits; tier 3 the full corpus in parallel in CI on each push/PR; tier 4 everything (`-O2`, legacy suites) nightly and before releases. The old compiler is rerun only for new corpus programs or a new reference version. |
| 2026-10-03 | Until M3's libqb copy, the new compiler builds through `..\QB64pe`'s `Makefile` with overrides (`m2-workspace-and-slice` design D8): its own fragments, `qbx.cpp` copy and exe stay outside the clone; **libqb objects may be built into the clone's git-ignored folders** (as `qb64pe.exe` does). Tracked files of the clone still never change (rule 3). |
| 2026-10-03 | Compiler structure (change `m2-workspace-and-slice`): Rust workspace at the repo root, **one crate per stage** (`crates\README.md`), Rust 1.88 pinned; **our own lossless tree over bytes** (not `rowan`, whose text is `str`); **one error per statement, at most 100 per run**; front-end tests in `tests\frontend\` with a first-line **mode line** `' TEST: <mode>` and `insta` snapshots; intentional differences from the old compiler go into **`DIVERGENCES.md`**. |
| 2026-10-04 | Review after the first slice and panel (`study\20-review-and-order.md`): direction unchanged; **order of work** set (`STATUS.md`, "Next"). Also: progress is reported on the corpus **and** the upstream expected-output tests; "compiles `qb64pe.bas` and the result passes the suite" is the **M4 exit criterion** (was a stretch); **no incremental (salsa-style) analysis**, a full reparse and recheck with cancellation instead; the IR stays and is reviewed after arrays and `TYPE`; one home per measured fact. |
| 2026-10-04 | Second review (`study\22`), accepted: **order of work replaced** (`study\22` §5, `STATUS.md`): upstream tests in tier 1 first, then parser breadth over the whole language (incl. `$IF`/`$INCLUDE`, `specialformat`), then a thin language server, before the differential tester, built-ins, control flow, arrays/`TYPE`. Upstream progress is reported as **x of 279** (125 of 404 need deferred array features). **QB64pe's `tests\compile_tests` text files are copied** into `tests\upstream\` (MIT; binary assets stay in the clone); **`qbasic_testcases` is never copied** (third-party, some Microsoft-copyright programs), only read from the clone. QB64Fresh's inline test snippets may be taken as inputs, labelled by `qb64pe.exe`. Programs without `$CONSOLE:ONLY` need a screen-state oracle (M3). |
| 2026-10-04 | Rust review of the workspace setup (`study\21-rust-review.md`), accepted: **no `_ =>` on semantic enums** in `sema`, `ir`, `codegen-cpp`; **"source is bytes" enforced** by clippy's `disallowed-methods` (`clippy.toml`); **cast lints** with `base::to_u32` and a 4 GiB source limit; **overflow checks in release**, BASIC wrapping written `wrapping_*`; `rust-version` in the manifest; **Rust CI** (tier 1, Windows, `-D warnings`) and a **repo check** for rules 2 and 7 on every push. Panic hook at step 3; `cargo-deny` when a real dependency arrives; no `clippy::pedantic` as a whole. |
| 2026-10-04 | Procedures and error handling (change `m2-procedures-and-errors`, `openspec\changes\archive\2026-10-04-m2-procedures-and-errors\design.md`): `sema` reads the tree through **hand-written typed accessors** (`syntax\src\ast.rs`, every child an `Option`); a **symbol table** (definition and references per variable, procedure, label; spans with `FileId`) is kept now for the language server; scope rules follow **file order**, as measured; a case the old compiler was not measured on is **"not supported yet", never guessed**; where the old compiler fails only in C++ (`SHARED h AS LONG` then `DIM h`), the new one rejects the program; `ERR` is typed LONG until unsigned types exist; tier 1 requires an error for every corpus `.err` program. |
| 2026-10-04 | **Tier 2 `.err` meaning for the new compiler:** an `.err` program passes when the compile fails, writes no executable and reports at least one error not marked "not supported yet"; the old message text is compared only when the compiler is `qb64pe`. The 56 upstream `.err` programs stay in the "x of 279" (change `m2-parser-breadth`, D11). |
| 2026-10-05 | Third review and panel (`study\23`), accepted: **order of work replaced** (`study\23` §4, `STATUS.md`): after the parser's block nodes, a **control-flow slice through to C++** (`IF`, `FOR`, `DO`, `WHILE`, `GOTO`, `GOSUB`, `CONST`, `OPTION _EXPLICIT`) comes before the rest of parser breadth, and the **IR review is held right after it** (was: after arrays and `TYPE`). **Follow-on errors:** after the first declaration reported "not supported yet", `sema` reports only "not supported yet" errors; name tracking only where the user asks for it on review. **A block that crosses an include boundary is "not supported yet".** The thin language server gives syntax errors, outline, folding, and go to definition for procedures and labels. |
| 2026-10-06 | **`RETURN label` with no `GOSUB` pending** raises error 3 and leaves the `GOSUB` stack intact (the old compiler underflows its counter and the next `GOSUB` crashes): `DIVERGENCES.md` D-003, change `m2-control-flow-slice` D8. |

Expert-panel recommendations: `study\07-expert-panel.md` (its "Decisions for the user" are answered above). Open
questions: `STATUS.md`.

**Start each session by reading `STATUS.md`** (current phase and next steps).

## Layout

| Path | Content |
|---|---|
| `..\QB64pe\` | Reference clone of QB64pe (`davidshq-contribute/QB64pe`, tracks upstream `main`, version 4.7.0-GLFW) |
| `study\00-synthesis.md` | Start here: summary of all studies, decisions, plan, measured facts, reuse verdicts |
| `study\01`–`05` | Detailed studies: compiler front end, expressions/codegen, runtime, IDE/debugger, build/CI/tests |
| `study\06-vscode-parity.md` | IDE features mapped to VS Code mechanisms |
| `study\07-expert-panel.md` | Panel discussion and recommendations |
| `study\08-qb45-strict-mode.md` | Changes a strict QuickBASIC 4.5 mode would need (not planned) |
| `study\09-verification.md` | Study claims checked by running the old compiler |
| `study\10-gaps.md` | DIM/REDIM/STATIC/COMMON, PRINT/INPUT/WRITE emission, built-in table (M0 study gaps) |
| `study\15-pre-coding-review.md` | Review of the plan before coding started: five adjustments (bytes not strings, early vertical slice, diagnostics policy, minimal M1 backend, libqb copy) |
| `..\FreeBASIC\` | Reference clone of FreeBASIC (shallow, `5714d10adb`, 2026-04-05). Read-only; GPL/LGPL, nothing copied |
| `study\16-freebasic.md` | What to learn from FreeBASIC (module split, runtime-call tables, `-fwrapv`, lowering notes, test conventions), what not to take, and the panel review |
| `study\17-vscode-extension-testing.md` | How VS Code extensions are tested (runners, Node in the extension host, what popular extensions do) and how M1 compares |
| `vscode\` | M1 VS Code extension (`qb64rust`): sources, tests, fixtures, `README.md`, `DEVELOPMENT.md` |
| `.github\workflows\` | CI: `vscode-extension.yml` tests the extension on Windows against the QB64pe 4.7.0 release; `rust.yml` runs fmt, clippy and `cargo test` (tier 1) on Windows, and tier 2 (slice list, upstream pass list) against the QB64pe release; `repo-check.yml` runs `tools\repo_check` on every push |
| `study\18-vscode-extension-practices.md` | Other extension practices (bundling, manifest, workspace capabilities, status bar, notifications, CI) compared with five large extensions; proposals A–G |
| `study\19-test-cadence.md` | How often each test layer runs (four tiers), what a full run costs, and how the new compiler's runs are kept fast |
| `study\20-review-and-order.md` | Review of code and plan after the first slice (2026-10-04), the panel's outcome, and the accepted order of work |
| `study\21-rust-review.md` | Rust review of the workspace setup: lints adopted and not, CI, the repo check, how to apply them |
| `study\22-review-tests-and-order.md` | Second review (2026-10-04): test files of QB64pe and QB64Fresh and what is taken, the upstream yardstick; its order of work is replaced by `study\23` §4 |
| `study\23-review-and-order.md` | Third review (2026-10-05): path check of the codebase, the panel's outcome, the current order of work |
| `study\archive\` | Closed reviews of other repositories, kept for the record only: `11` existing VS Code extensions, `12` QB64Fresh (the user's earlier Rust rewrite), `13` reference-doc sources, `14` the `docs-new-2` branch, and `qb64fresh-scripts\` (measurements behind `12`). Their conclusions are in `study\00` §11; nothing in the active plan depends on reading them. |
| `verification\` | Small programs behind `study\09`, `study\10` and later measurements (`v13*`: type suffixes on DIMmed names; `v14*`, `v15*`: procedures, scopes, reserved names, errors; `v16*`: comment metacommands, `DATA`, line numbers, blocks, templates, `$IF`, `$INCLUDE`, member access, with include files in `v16_inc\`), their outputs, and `run.sh` |
| `tools\builtins\` | Extractor for the built-in table (`extract_builtins.py`) and its output `builtins.json` |
| `tools\wiki\` | `fetch_wiki.py` fetches the QB64pe wiki as raw wikitext into `cache\` (git-ignored, no licence stated; local reference only) |
| `<qb64contain>`, `<share>` | Placeholders for the user's local folder of earlier projects (QB64Fresh, qb64pe-vscode, a QB64pe clone with notes) and its network copy; used in `study\` and `STATUS.md` instead of machine-specific paths. Do not write full local paths or host names into this repo. |
| `SOMEDAY.md` | Deferred features and ideas |
| `STATUS.md` | Current phase, what is done, next steps |
| `tests\corpus\` | Golden corpus: 282 programs with the old compiler's recorded output or compile error (`README.md`), including the `slice\` group; checked by `run_legacy_tests.py --suite corpus`; `slice.list` names the 54 the new compiler must pass |
| `Cargo.toml`, `crates\` | The new compiler `qb64rust` (Rust workspace, one crate per stage; `crates\README.md`: crate map, build, tests, snapshots) |
| `tests\frontend\` | Front-end tests of the new compiler, run by mode line (`' TEST: <mode>`; modes `parse-ok`, `check-ok`, `check-fail`, `typed`, `ir`, `cpp`) |
| `tests\upstream\` | Copy of the text files of QB64pe's `tests\compile_tests` (MIT, `SOURCE.md`, never edited by hand); `pass.list` (upstream programs the new compiler passes) and `deferred.list` (the 125 that need deferred array features); progress "x of 279" (`README.md`) |
| `tests\snippets\` | QB64Fresh's inline test snippets as inputs, labelled by `qb64pe.exe` (`.err` when it rejects one; `SOURCE.md`) |
| `tests\known_false_errors.list`, `tests\known_unsupported_rejections.list`, `tests\known_parse_gaps.list` | Shrink-only lists of tier 1: programs the old compiler accepts that get a real error, programs it rejects that get only "not supported yet" errors, and accepted programs (plus the old compiler's sources) that do not parse cleanly (`tests\upstream\README.md`) |
| `tools\upstream\` | `copy_upstream_tests.py`: redoes the copy in `tests\upstream\` from the clone at the pinned commit |
| `tools\snippets\` | `extract_qb64fresh_snippets.py`: extracts and labels the snippets in `tests\snippets\` |
| `DIVERGENCES.md` | Divergence register: decided differences from the old compiler's observed behaviour |
| `tools\legacy_tests\` | Windows runner for the QB64pe test suites (compile, qbasic, format; old or new compiler) and its `known_failures.txt` |
| `tools\repo_check\` | `check_repo.py`: tracked files must hold no local paths, temp folders or network shares (rule 2) and no stray control bytes (rule 7); `--untracked` also checks new files before they are staged |
| `baselines\` | Recorded test results of the old compiler (546 pass, 1 environment failure; format tests 24 of 24) |
