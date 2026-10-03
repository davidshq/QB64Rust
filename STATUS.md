# Status and next steps

Updated 2026-10-02 after the fifth session (review of documents and tools).

## Where we are

Roadmap phase **M0 (baseline): complete.** Next is **M1**, from `study\07-expert-panel.md` Session 8.

M0 work:
- Codebase study (`study\00`–`06`), expert panel (`study\07`), strict-QB4.5 notes (`study\08`), `SOMEDAY.md`.
- Old compiler built locally: `..\QB64pe\qb64pe.exe` (llvm-mingw toolchain in `..\QB64pe\internal\c\c_compiler`).
- Windows test runner: `tools\legacy_tests\run_legacy_tests.py`.
- Baseline recorded: `baselines\` (546 pass, 1 environment-dependent failure).
- Upstream diff: not needed. Git history shows the "fork" features are upstream QB64pe work; decision recorded in
  `CLAUDE.md`.
- Verification checks (session 2): `study\09-verification.md`, programs and outputs in `verification\`
  (rerun with `verification\run.sh`). `run.sh` sets `QB64PE_NOPROMPT=y`, so programs that stop on an untrapped
  runtime error report it on stderr instead of opening a message box (exit code stays 0).
- Study gaps closed (session 3), all in `study\10-gaps.md`:
  - §1 DIM / REDIM / STATIC / COMMON / ERASE statement semantics; checked by `verification\v09_dim.bas`.
  - §2 PRINT / PRINT USING / WRITE / INPUT emission, files included; checked by `verification\v10_print.bas`.
  - §3 Built-in table extracted mechanically: `tools\builtins\extract_builtins.py` → `tools\builtins\builtins.json`
    (455 entries, argument/return types, categories, which ones the compiler special-cases).

Findings from session 3 worth remembering (details in `study\10`, decisions listed at the end of `study\09`):
- Screen `PRINT` with a comma in a `$CONSOLE` program with redirected stdout **never terminates** (the console
  `tab()` loops on `POS`, which reads the console window cursor). Do not use it in verification programs.
- `_LogMinLevel` and `_ScreenExists` are registered without a return type; they work by accident.

Session 5 (review of documents and tools; not yet committed, user commits):
- Runner: `--suite format` added (24 of 24 pass, `baselines\qb64pe-16f629784e-win64-format.json`);
  `known_failures.txt` (`KFAIL`, today only `http/read_example`); whole process tree killed on timeout; partial
  runs write `results-partial.json` / `results-<suite>.json` instead of overwriting `results.json`.
- `verification\run.sh` uses `QB64PE_NOPROMPT=y` and runs all programs; `v02c` re-recorded (no "Continue?").
- Stale or wrong statements fixed in `CLAUDE.md`, `SOMEDAY.md`, `study\00`, `01`, `02`, `03`, `05`, `07`, `08`,
  `09`, `11`, `12`, `14`; `measure.sh` header documents its TSV columns.
- Second pass over the code: runner exits 2 on an unknown category or empty selection, records an exception in
  one test as a FAIL instead of aborting, notes known failures that pass, escapes glob characters in test names,
  reads sidecar files as Latin-1, and keeps compiler paths outside the repo's parent folder out of the JSON.
  Extractor reports unclosed `clearid` blocks and stray statements (output unchanged). `run.sh` times out hung
  programs after 60 s (all 14 outputs reproduce exactly). `lsp_smoke.py` collects diagnostics for every URI;
  rerun confirmed `study\12` Q7.
- `study\00-synthesis.md` rewritten as the entry point covering `01`–`14`, `baselines\` and `verification\`.

Open manual task for the user: the CP437 round trip in VS Code (instructions in `study\09`, last section).

## Next: M1, VS Code extension v0 on the old compiler

From `study\07` (roadmap table, R4) and `study\06-vscode-parity.md`: syntax highlighting, build/run tasks,
diagnostics parsed from the old compiler's `-c` output, formatting via `-y`, CP437 as the default encoding.
No subagents (see `CLAUDE.md`).

First M1 check (open, from `study\11` recommendations 5 and 6): can the old compiler report BASIC-level errors
without a full C++ build (for example `-z`, or stopping early with `-c`)? This decides whether on-save diagnostics
cost seconds per save. Verify with a small program in `verification\`.

### Session 4: existing work reviewed (no M1 code written yet)

Four reviews, all written as panel discussions without subagents.

Placeholders used in these notes instead of machine-specific paths (the actual locations are known to the user):
- `<qb64contain>` — the local folder with the user's earlier projects: `QB64Fresh`, `qb64pe-vscode`, and a QB64pe
  clone holding untracked analysis notes.
- `<share>` — a network copy of `<qb64contain>` that also holds `vscode-qb64fresh` and the QuickBASIC manual texts.

- `study\11-existing-vscode-extension.md` — VS Code extensions. Recommendation: import the user's
  `vscode-qb64fresh` as the M1 base and add an old-compiler backend; from the community `qb64pe-vscode`
  (`<qb64contain>\qb64pe-vscode`) take only help, language configuration and snippets.
  **Newest copy of `vscode-qb64fresh`:** `<share>\vscode-qb64fresh` (same last commit as
  GitHub `davidshq/vscode-qb64fresh`, plus uncommitted integration tests and docs). There is no copy on this machine.
- `study\12-qb64fresh-review.md` — the user's earlier Rust rewrite QB64Fresh (`<qb64contain>\QB64Fresh`,
  branches `fixing-reapply` and `broken`). Output matches QB64pe on 19 of 331 tests; architecture conflicts with
  `study\07` R2/R5/R6. Build on none of its code. Take: its 261 `runtime_comparison` programs; its LSP as reading
  before M2; the process lessons in Session 5 and the `broken` addendum (one compiler API, one diagnostic type,
  thin dispatchers, small verified refactors, written phase contracts). Scripts and raw results:
  `verification\qb64fresh\` (locations come from environment variables; see each script's header).
- `study\13-reference-docs.md` — reference sources. The QB64pe wiki (1,035 articles) can be fetched fresh through
  its MediaWiki API; no licence stated. The Microsoft QuickBASIC manuals (text) are on
  `<share>\docs\` (copyrighted, never commit). QB64Fresh's specs are checklists, not sources.
  Found a gap in `builtins.json`: constants from auto-included files (`_TRUE`, `_FALSE`) are missing (`study\10` §3.4).
- `study\14-docs-new-2-branch.md` — branch `docs-new-2` on the QB64pe fork `davidshq-contribute/QB64pe`
  (`cb40765352`). About 60 machine-written internals docs plus QB64pe-specific rules. Use the internals docs as a
  map for M3/M5, never as a spec: 13 checks, 8 right (copied from code), 5 wrong (explanations and examples).

Material in `<qb64contain>\QB64pe` (untracked files in that clone, written 2026-10-02 before this repo's
studies, not yet brought in):
- `docs\rewrite-decision.md`: five-reviewer assessment of QB64Fresh; advised keeping QB64Fresh's front end and
  **against a new rewrite from an empty repository**. Its harness: `docs\analysis-scripts\rewrite-panel\`.
- `docs\maintainability-priorities.md`, `compiler-internals-notes.md`, `modularization-learnings.md`,
  `unit-testing-plan.md`.
- `reports\BASIC and QB64 test suites.md`, `reports\FreeBASIC and QB64pe codebases.md`, with notes in
  `research_notes\`.

Toolchain checked in session 4: Node 22.14 and npm 11.5 (Volta), Rust 1.88, VS Code installed, `tsc` not global
(per-project dependency is fine), MSYS2 UCRT64 gcc 15.2 (used for the QB64Fresh measurements).

## Open questions for the user (blocking M1)

1. Was `rewrite-decision.md`'s advice against an empty-repository rewrite considered, and does the ground-up
   rewrite stand? (`study\12` supports it given `study\07`'s architecture.)
2. Accept `vscode-qb64fresh` as the M1 base? Import into `vscode\` with its git history?
3. Project name for the rename of ids, settings and commands.
4. Record outputs of QB64Fresh's 261 `runtime_comparison` programs with the old compiler and add them to the
   conformance tests?
5. Rust workspace layout: set up now or at the start of M2 (recommended: M2).
6. Bring the useful parts of the `qb64contain` material above into `study\`?
7. Write a wiki fetcher (`tools\wiki\`) now, and ask the QB64pe maintainers about the wiki licence?

Other decisions: see `CLAUDE.md`. Bug-compatibility choices are listed at the end of `study\09`.
