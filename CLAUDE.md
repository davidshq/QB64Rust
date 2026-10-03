# Project rules for Claude

This file is the authoritative place for rules and decisions in this project. Claude's private memory may hold a
copy, but never the only copy.

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
| `study\11-existing-vscode-extension.md` | Panel review of existing extensions: the user's `vscode-qb64fresh` (M1 base) and the community `qb64pe-vscode` (help, snippets only) |
| `study\12-qb64fresh-review.md` | Panel review of the user's earlier Rust rewrite QB64Fresh: use none of its code; take its test programs |
| `study\13-reference-docs.md` | Reference sources (QB64pe wiki API, QuickBASIC manuals, QB64Fresh specs): what to trust, where to get them |
| `study\14-docs-new-2-branch.md` | The user's QB64pe-fork branch `docs-new-2` (internals docs, rules): reference by commit, verify before use |
| `verification\` | Small programs behind `study\09` and `study\10`, their outputs, and `run.sh`; `qb64fresh\` holds the scripts and results behind `study\12` |
| `tools\builtins\` | Extractor for the built-in table (`extract_builtins.py`) and its output `builtins.json` |
| `<qb64contain>`, `<share>` | Placeholders for the user's local folder of earlier projects (QB64Fresh, qb64pe-vscode, a QB64pe clone with notes) and its network copy; used in `study\` and `STATUS.md` instead of machine-specific paths. Do not write full local paths or host names into this repo. |
| `SOMEDAY.md` | Deferred features and ideas |
| `STATUS.md` | Current phase, what is done, next steps |
| `tools\legacy_tests\` | Windows runner for the QB64pe test suites (compile, qbasic, format; old or new compiler) and its `known_failures.txt` |
| `baselines\` | Recorded test results of the old compiler (546 pass, 1 environment failure; format tests 24 of 24) |
