# Project rules for Claude

This file is the authoritative place for rules and decisions in this project. Claude's private memory may hold a
copy, but never the only copy.

## Explicit Human Rules
1. Don't stage/unstage, stash/unstash, etc. files. unless explicitly asked to do. Instead, you should prompt the user to do this if it is necessary.
2. **Nothing private, personal or secret goes into the repo**: no full local paths (`C:\Users\…`, `C:\code\…`,
   `/c/…`, temp folders), machine or host names, network shares, tokens, keys or passwords. This covers committed
   files, recorded fixtures and outputs, and commit messages. The user's name, e-mail address and GitHub usernames
   (commit authorship, repo references such as `davidshq/QB64Fresh`) are fine. Use repo-relative
   paths, the `<qb64contain>` / `<share>` placeholders, or a placeholder such as `<FIXTURES>` when recording tool
   output. Check new files and recorded outputs before staging them.

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
| `.github\workflows\` | CI: `vscode-extension.yml` tests the extension on Windows against the QB64pe 4.7.0 release |
| `study\18-vscode-extension-practices.md` | Other extension practices (bundling, manifest, workspace capabilities, status bar, notifications, CI) compared with five large extensions; proposals A–G |
| `study\19-test-cadence.md` | How often each test layer runs (four tiers), what a full run costs, and how the new compiler's runs are kept fast |
| `study\archive\` | Closed reviews of other repositories, kept for the record only: `11` existing VS Code extensions, `12` QB64Fresh (the user's earlier Rust rewrite), `13` reference-doc sources, `14` the `docs-new-2` branch, and `qb64fresh-scripts\` (measurements behind `12`). Their conclusions are in `study\00` §11; nothing in the active plan depends on reading them. |
| `verification\` | Small programs behind `study\09` and `study\10`, their outputs, and `run.sh` |
| `tools\builtins\` | Extractor for the built-in table (`extract_builtins.py`) and its output `builtins.json` |
| `tools\wiki\` | `fetch_wiki.py` fetches the QB64pe wiki as raw wikitext into `cache\` (git-ignored, no licence stated; local reference only) |
| `<qb64contain>`, `<share>` | Placeholders for the user's local folder of earlier projects (QB64Fresh, qb64pe-vscode, a QB64pe clone with notes) and its network copy; used in `study\` and `STATUS.md` instead of machine-specific paths. Do not write full local paths or host names into this repo. |
| `SOMEDAY.md` | Deferred features and ideas |
| `STATUS.md` | Current phase, what is done, next steps |
| `tests\corpus\` | Golden corpus: 263 programs with the old compiler's recorded output or compile error (`README.md`); checked by `run_legacy_tests.py --suite corpus` |
| `tools\legacy_tests\` | Windows runner for the QB64pe test suites (compile, qbasic, format; old or new compiler) and its `known_failures.txt` |
| `baselines\` | Recorded test results of the old compiler (546 pass, 1 environment failure; format tests 24 of 24) |
