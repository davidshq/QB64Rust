# Status and next steps

Updated 2026-10-03 (session 8). Session-by-session history is in `git log`; measured facts are in `study\00`.

## Where we are

Roadmap (`study\07` Session 8, summarised in `study\00` §2): **M0 and M1 complete. M2 (front end) started: golden corpus done.**

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

## M1 done: VS Code extension v0 on the old compiler (2026-10-03, session 7)

OpenSpec change `m1-vscode-extension-v0`: all 18 tasks done; archived 2026-10-03 to
`openspec\changes\archive\2026-10-03-m1-vscode-extension-v0\`, its specs now the main specs in
`openspec\specs\editor\` (build-run, compiler-diagnostics, formatting, language-support). `vscode\` holds the extension (`README.md`,
`DEVELOPMENT.md`): grammar, build/run (commands and tasks), diagnostics from `qb64pe -z -q -w`, formatting via `-y`,
CP437 default. Written fresh, ids `qb64rust`. Measured old-compiler output formats are in the archived change's `design.md`.

- Tests: unit 40, grammar 4, integration 37 on VS Code stable and on 1.100 (the minimum, `study\17`); also passes
  against the QB64pe 4.7.0 release download. CI for Windows in `.github\workflows\vscode-extension.yml`, inactive
  until the repo has a GitHub remote.
- Walkthrough of the packaged `.vsix` (`verification\m1-extension.md`): 11 of 11 steps done; one expectation
  corrected (a program without `$CONSOLE` opens its own window, not the terminal).
- CP437 round trip (`study\09`): every byte survives except a lone 0x0D; a file containing 0x00 cannot be opened as
  text in VS Code at all, even with "Open Anyway".
- Practices (`study\18`): A–D done (prepublish, virtual workspaces, language status item, fewer notifications);
  E (bundling) and F (Marketplace metadata) wait until publishing; G done for Windows.
- Follow-ups from the walkthrough (2026-10-03): the `-x` progress bar no longer fills the output channel (each
  line shows its last redraw, progress lines dropped; `src\compiler\buildOutput.ts`, unit test on a recorded build,
  fixture `build_clean` added to `scripts\record-fixtures.sh`). Progress moved to the language status item
  instead (panel, 2026-10-03): "qb64pe: generating C++ n%", then "qb64pe: compiling C++…". Measured: generating
  C++ for `qb64pe.bas` (28,828 lines) takes 33 s, a small program 0.2 s; the terminal build task still shows the
  original bar. For the new compiler: report progress as structured
  messages (or LSP `$/progress`), not a `\r` bar. The heading "Qb64rust: Compiler Path" is left as is:
  it appears in the filtered search view that "Open Setting" opens, where VS Code shows the key's prefix; the
  extension's `name` already matches the prefix, so the tree view shows "Compiler Path". Removing the prefix
  from search results would need renamed keys (not wanted).
- Both checked by eye (2026-10-03, VS Code 1.140, build of `qb64pe.bas`): the settings tree view (Extensions >
  QB64) shows "Compiler Path" without the prefix. The status item shows "qb64pe: building…", then "qb64pe:
  generating C++ n%", "qb64pe: compiling C++…", and "qb64pe" again when done; the output channel has no bar lines.
  Measured with a pipe: qb64pe prints nothing for the first 31 s (the prepass, which draws no bar; the bar is drawn
  only in the main pass), then 2 %–100 % arrive within 2 s, then compiling C++ takes 17 s (50 s in all). So for
  most of the C++ generation the item said only "building…", and the percentage flashes by at the end. Fixed: the
  item switches to "qb64pe: generating C++…" when the line "Beginning C++ output from QB64 code..." arrives (0.2 s
  in; unit test added, 40 unit tests; seen by eye in the same build).

## M2: golden corpus (2026-10-03, session 8)

OpenSpec change `m2-golden-corpus`: all tasks done; archived 2026-10-03 to
`openspec\changes\archive\2026-10-03-m2-golden-corpus\`, its spec now the main spec `openspec\specs\testing\golden-corpus`. `tests\corpus\` holds 263 programs
(QB64Fresh's 261 `runtime_comparison` programs, unchanged, plus `verification\` v11 and v12) with the old
compiler's recorded output: 237 `.output`, 20 `.err`, 1 compile-only (`239_lprint`), 5 known failures
(`tests\corpus\README.md`). Checked by `run_legacy_tests.py --suite corpus` (about 11 min with the old compiler);
baseline `baselines\qb64pe-16f629784e-win64-corpus.json`. A second `--record` changed no file.

Found while recording:
- On Windows, `END` waits for a console key event; the runner feeds key presses (`press_any_key.py`).
- `PRINT` with a comma prints spaces forever when the console's stdout is redirected (80, 126, 187, 251): known
  failures, compiled but not run. Same cause: `CSRLIN`/`POS` always report 1, 1 (237).
- `CHAIN` to a missing program tries `<program folder>\qb64pe.exe -c` (238; path masked by `.normalize`).
- 20 programs use syntax QB64pe rejects (mostly `FUNCTION f (...) AS type`); kept as compile-error tests.
- `-O2` changes only `v11` (LONG) and the new `v12` (`_INTEGER64`): `_INTEGER64` overflow has the same exposure
  as LONG (`study\16` §8).

Test cadence decided (`study\19`): the full corpus is not part of the edit loop; four tiers from `cargo test` to
nightly.

Decided 2026-10-03: `_INTEGER64` overflow wraps too, like LONG (`CLAUDE.md`, `study\16` §8).

**Next:** the next M2 change: Rust workspace and end-to-end vertical slice
(`study\15` §2), divergence register and numeric-semantics spec.

## FreeBASIC reviewed (2026-10-03)

`..\FreeBASIC\` cloned (reference only, GPL/LGPL). Findings in `study\16-freebasic.md`. Reviewed by a panel and a second
pass (`study\16` §7). Measured finding about the old compiler (`verification\v11_wrap_o2`): LONG overflow gives
different results with and without the "optimize C++" setting (`-O2`), because the generated C++ is compiled
without `-fwrapv`. The six follow-ups in `study\16` §6 were decided by the panel (`study\16` §8): LONG overflow
wraps (decided now); test conventions and recovery policy go into M2; the rest are notes for M3, M5, M6.
M1 leftovers: done (see above).

## Queued for later milestones

| When | Item |
|---|---|
| Start of M2 | Rust workspace; create the divergence register and the numeric-semantics spec; build the end-to-end vertical slice first (`study\15` §2) |
| Before M3 codegen | Decide the bug-compatibility choices in `study\00` §6 |
| M3 | Plain copy of `..\QB64pe\internal\c` at the pinned commit (`CLAUDE.md`) |
| Help/hover work | Ask the QB64pe maintainers about the wiki licence before shipping any wiki text |
