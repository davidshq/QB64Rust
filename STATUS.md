# Status and next steps

Updated 2026-10-04 (session 11). Session-by-session history is in `git log`; measured facts are in `study\00`.

## Where we are

Roadmap (`study\07` Session 8, summarised in `study\00` §2): **M0 and M1 complete. M2 (front end) in progress: golden corpus, Rust workspace and end-to-end slice, procedures and error handling done.**

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
  against the QB64pe 4.7.0 release download. CI for Windows in `.github\workflows\vscode-extension.yml`, active
  on `davidshq/QB64Rust` (last run on `main` 2026-10-03: success). The Rust workspace has its own workflow
  since 2026-10-04 (`rust.yml`, `study\21`).
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

## M2: Rust workspace and end-to-end slice (2026-10-03, session 9)

OpenSpec change `m2-workspace-and-slice`: all tasks done; archived 2026-10-04 to
`openspec\changes\archive\2026-10-04-m2-workspace-and-slice\`, its specs now the main specs `openspec\specs\compiler\cli`,
`compiler\pipeline`, `language\numeric-semantics` and `testing\compiler-tests`. The new compiler `qb64rust` exists:
`Cargo.toml`, `crates\` (seven crates, one per stage; `crates\README.md`), Rust 1.88. Every stage has its final
shape: bytes in, lossless tree (our own, over bytes), typed tree with explicit conversions, ABI-neutral IR, C++
fragments for `qbx.cpp`, built through the reference clone's `Makefile` with overrides (libqb reused, clone
untouched, about 2 s per program). The subset: `$CONSOLE:ONLY`, `DIM` of scalars, assignments, implicit
variables, `PRINT` (`;`, `,`, auto-semicolon), `END`, literals, `+ - * /`, unary `-`, string `+`, `INSTR`.

- Tier 2: the 14 programs of `tests\corpus\slice.list` (7 new ones in `tests\corpus\slice\`, recorded with
  `qb64pe.exe`, and 7 of `runtime_comparison`) pass end to end, also with constant folding off. Full corpus once:
  31 pass, the rest are rejected with a diagnostic; no crash, no wrong executable (`tests\corpus\README.md`).
- Tier 1 (`cargo test`): unit and `insta` snapshot tests, `tests\frontend\` by mode line, the front end over every
  corpus program (round trip, no panic).
- `DIVERGENCES.md` (D-001, D-002: integer overflow wraps) and the numeric-semantics spec
  (`openspec\specs\language\numeric-semantics`).
- Measured on the way (`study\00` §5): a variable is a name plus a type (`verification\v13*`); `INSTR(0, …)` does
  not raise; negating an integer is believed `_INTEGER64`. The corpus runner now keeps the clone's tracked
  `internal\temp\temp.bin` (it deleted it; `qb64pe.exe` recreates it, `qb64rust` does not).

**Rust review of the workspace setup** (2026-10-04, `study\21`, accepted): no `_ =>` on semantic enums,
`disallowed-methods` for byte-to-`str` conversions, cast lints with `base::to_u32`, overflow checks in release
(the folder wraps with `wrapping_*`), `rust-version`, CI workflows `rust.yml` (tier 1) and `repo-check.yml`
(`tools\repo_check`, rules 2 and 7). Both workflows have not run on GitHub yet (not pushed).

## M2: procedures and error handling (2026-10-04, sessions 10–11)

OpenSpec change `m2-procedures-and-errors`: all tasks done; archived 2026-10-04 to
`openspec\changes\archive\2026-10-04-m2-procedures-and-errors\`, its specs now the main specs
`openspec\specs\language\procedures` and `language\error-handling` (and updates of `compiler\pipeline` and
`testing\compiler-tests`). Supported now (`crates\README.md`): `SUB`/`FUNCTION`, calls by
reference and by value, `EXIT`, `DECLARE`, `STATIC`, `SHARED`, `DIM SHARED`, reserved names, `SYSTEM`; labels,
`ON ERROR GOTO`, the three `RESUME` forms, `ERROR`, `ERR`, `ERL`, `CHR$`. `sema` reads the tree through typed
accessors (`ast.rs`) and keeps a symbol table for the language server.

- Tier 2: `slice.list` has 54 programs (the 12 of `slice\`, 5 of them new: `s08`–`s12`; 42 of
  `runtime_comparison`); all pass, also with folding off. Tier 1 now also requires an error for every `.err`
  program and checks reserved names against 1,030 measured forms (`verification\v15_builtin_names`).
- Full corpus once (`tests\corpus\README.md`): exactly the 54 pass, 216 are rejected with a diagnostic (20 of them
  `.err` programs), 5 known failures not run; no crash, no wrong executable; 4 min 47 s.
- Measured on the way (`study\00` §5, `verification\v14_*`, `v15_*`): argument passing, scopes in file order,
  `DECLARE` ignored, reserved names, `RESUME` granularity, `ERROR` values, no unary `+`.
- Noted for `TYPE`: an unsupported `TYPE` block gives follow-on errors on its member lines and member assignments
  (3 corpus programs).

**Next** (order accepted 2026-10-04 after the second review, `study\22` §5, reasons there; it replaces the order
of `study\20` §4):

1. OpenSpec change `m2-upstream-tests` (proposed): upstream test files copied into `tests\upstream\`; tier 1 over
   upstream, `qbasic_testcases` and the old compiler's sources; the "not supported yet" marker; ratchet lists;
   upstream progress reported as x of 279 (125 programs need deferred array features); the QB64Fresh snippets as
   labelled inputs; shared `Ty`/`BinOp`/`Conv`; a seeded mutation test.
2. Parser breadth, the whole language: blocks, control flow, labels, line numbers, `DATA`, comment metacommands,
   `$IF`/`$INCLUDE`, the `specialformat` statements; `sema` keyed by (`FileId`, offset). Panic hook at the start.
3. A thin language server (diagnostics, go to definition) in the extension.
4. Bug-compatibility decisions (`study\00` §6), then the differential tester, `Ty` as a type table, unsigned types.
5. Plain built-ins (249 of 455), table-driven, with generated tests.
6. Control flow through the pipeline, with `CONST`, `OPTION _EXPLICIT`, `DEFxxx`.
7. Arrays and `TYPE` (storage designed so member arrays fit later), then the IR review (`study\20` §3.4).

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
| Step 4 of "Next" | Decide the bug-compatibility choices in `study\00` §6 (32-bit INTEGER arithmetic and half-to-even rounding are already implemented as "keep") |
| M3 | Plain copy of `..\QB64pe\internal\c` at the pinned commit (`CLAUDE.md`) |
| M3 | Programs without `$CONSOLE:ONLY`, with an oracle such as a screen-state dump at exit compared between old and new compiler (`study\22` §3.2) |
| M4 | `qb64pe.bas` reached through its own include files, smallest first (`study\22` §4.2) |
| Help/hover work | Ask the QB64pe maintainers about the wiki licence before shipping any wiki text |
