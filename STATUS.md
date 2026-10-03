# Status and next steps

Updated 2026-10-03 (session 6). Session-by-session history is in `git log`; measured facts are in `study\00`.

## Where we are

Roadmap (`study\07` Session 8, summarised in `study\00` §2): **M0 (baseline) complete. M1 is next.**

M0 delivered:
- Studies `study\00`–`10` and `15` (the other-repo reviews `11`–`14` are closed, in `study\archive\`).
- Old compiler built locally: `..\QB64pe\qb64pe.exe` (llvm-mingw toolchain in `..\QB64pe\internal\c\c_compiler`).
- Windows test runner `tools\legacy_tests\run_legacy_tests.py` (suites compile, qbasic, format; old or new
  compiler; `known_failures.txt` marks expected failures).
- Baseline in `baselines\`: 546 pass, 1 environment failure (`http/read_example`); format tests 24 of 24.
- `verification\`: 15 small programs with recorded outputs (`run.sh` reruns them with `QB64PE_NOPROMPT=y`).
- Built-in table `tools\builtins\builtins.json` (455 entries). Known gap: names from the auto-included BASIC files
  (`_TRUE`, `_FALSE`, color constants) are missing (`study\10` §3.4).
- QB64pe wiki cached locally by `tools\wiki\fetch_wiki.py` (1,124 pages, git-ignored; no licence stated).

Test hygiene learned in M0 (details `study\00` §5, §10): run programs with `QB64PE_NOPROMPT=y`; detect runtime
errors from output, not exit code (always 0); never use screen `PRINT` with a comma in a `$CONSOLE` program with
redirected stdout (hangs).

All open questions were answered on 2026-10-02 (`CLAUDE.md` decision table). **Nothing blocks M1.**

Open manual task for the user: the CP437 round trip in VS Code (`study\09`, last section); also M1 task 8.2.

## Next: M1, VS Code extension v0 on the old compiler

Planned as OpenSpec change `openspec\changes\m1-vscode-extension-v0` (proposal, design, four specs under
`editor\`, tasks; validates with `--strict`). Scope: grammar, build/run, diagnostics from `qb64pe -z -q -w`,
formatting via `-y`, CP437 default. Written fresh in `vscode\`, ids `qb64rust`. Nothing copied from `qb64pe-vscode`;
small pieces of `vscode-qb64fresh` allowed (`CLAUDE.md` rule 5). Measured old-compiler output formats are in the change's `design.md`.

**Implemented 2026-10-03 (session 7): 15 of 18 tasks.** `vscode\` holds the extension (README, DEVELOPMENT.md);
unit 31, grammar 4, integration 37 tests pass on VS Code stable and on 1.100 (the minimum since 2026-10-03,
`study\17`); `qb64rust-0.1.0.vsix` packages. Open:
task 8.1 and the rest of 8.2 need a person: the hand walkthrough in `verification\m1-extension.md` (install the
`.vsix`, steps 1–11). Practice proposals A–D from `study\18` are done (prepublish script,
virtual workspaces declared, language status item, fewer notifications); E and F wait until publishing; G is
done for Windows (`.github\workflows\vscode-extension.yml`, inactive until the repo has a GitHub remote). Then 8.3 (mark M1 done here, `study\00`, `CLAUDE.md` layout) and `/opsx:archive`.

## FreeBASIC reviewed (2026-10-03)

`..\FreeBASIC\` cloned (reference only, GPL/LGPL). Findings in `study\16-freebasic.md`. Reviewed by a panel and a second
pass (`study\16` §7). Measured finding about the old compiler (`verification\v11_wrap_o2`): LONG overflow gives
different results with and without the "optimize C++" setting (`-O2`), because the generated C++ is compiled
without `-fwrapv`. Added to the "Decide" list in `study\00` §6. **Open for the user:** accept or drop the six
follow-ups in `study\16` §6 (none affects M1).

## Queued for later milestones

| When | Item |
|---|---|
| Start of M2 | Rust workspace; record QB64Fresh's 261 `runtime_comparison` programs with the old compiler as golden outputs; create the divergence register and the numeric-semantics spec; build the end-to-end vertical slice first (`study\15` §2) |
| Before M3 codegen | Decide the bug-compatibility choices in `study\00` §6 |
| M3 | Plain copy of `..\QB64pe\internal\c` at the pinned commit (`CLAUDE.md`) |
| Help/hover work | Ask the QB64pe maintainers about the wiki licence before shipping any wiki text |
