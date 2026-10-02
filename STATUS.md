# Status and next steps

Updated 2026-10-02 at the end of the third session.

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
  (rerun with `verification\run.sh`). Programs that stop on an untrapped runtime error open a native message box
  the user has to click; `run.sh` skips them by default. Do not add such programs to default runs.
- Study gaps closed (session 3), all in `study\10-gaps.md`:
  - §1 DIM / REDIM / STATIC / COMMON / ERASE statement semantics; checked by `verification\v09_dim.bas`.
  - §2 PRINT / PRINT USING / WRITE / INPUT emission, files included; checked by `verification\v10_print.bas`.
  - §3 Built-in table extracted mechanically: `tools\builtins\extract_builtins.py` → `tools\builtins\builtins.json`
    (455 entries, argument/return types, categories, which ones the compiler special-cases).

Findings from session 3 worth remembering (details in `study\10`, decisions listed at the end of `study\09`):
- Screen `PRINT` with a comma in a `$CONSOLE` program with redirected stdout **never terminates** (the console
  `tab()` loops on `POS`, which reads the console window cursor). Do not use it in verification programs.
- `_LogMinLevel` and `_ScreenExists` are registered without a return type; they work by accident.

Not yet committed (user commits): `.gitignore`, `tools\`, `baselines\`, `STATUS.md`, recent `CLAUDE.md` edits,
`verification\`, `study\09-verification.md`, a note in `study\03`, `study\10-gaps.md`.

Open manual task for the user: the CP437 round trip in VS Code (instructions in `study\09`, last section).

## Next: M1, VS Code extension v0 on the old compiler

From `study\07` (roadmap table, R4) and `study\06-vscode-parity.md`: syntax highlighting, build/run tasks,
diagnostics parsed from the old compiler's `-c` output, formatting via `-y`, CP437 as the default encoding.
No subagents (see `CLAUDE.md`).

Before starting, agree with the user on:
1. Where the extension lives (a folder in this repo, e.g. `vscode\`, or a separate repo) and the toolchain
   (TypeScript + `npm`; check what is installed).
2. Whether the Rust workspace layout (crates for the M2 front end) is set up now or at the start of M2.

Decisions still open: none blocking; see `CLAUDE.md`. Bug-compatibility choices are listed at the end of
`study\09`.
