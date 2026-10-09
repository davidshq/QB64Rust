# Status and next steps

Updated 2026-10-09 (session 33). Session-by-session history is in `git log`; measured facts are in `study\00`.

## Where we are

Roadmap (`study\07` Session 8, summarised in `study\00` §2): **M0 and M1 complete. M2 (front end) in progress: golden corpus, Rust workspace and end-to-end slice, procedures and error handling, upstream tests, control-flow slice, arrays-and-`TYPE` slice and parser breadth done and archived (2026-10-07): every program the old compiler accepts parses, and no false error is left. The core built-ins tranche with `SELECT CASE` and `ON … GOTO/GOSUB` (step 6 below) is done and archived (2026-10-08). The thin language server (step 7, OpenSpec change `m2-language-server`) is done and archived (2026-10-08), checked locally; its CI run on GitHub (task 5.1) is still to be seen after the next push. Step 8's bug-compatibility decisions are taken (2026-10-08); OpenSpec change `m2-numeric-types` groups 1–9 are done (2026-10-09); next: its group 10 (integration: full runs, CI, docs).**

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

Decided 2026-10-03: `_INTEGER64` overflow wraps too, like LONG (`DECISIONS.md`, `study\16` §8).

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
(`tools\repo_check`, rules 2 and 7). Both run green on GitHub since 2026-10-05.

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

## M2: upstream tests (2026-10-04, session 12)

OpenSpec change `m2-upstream-tests`: archived 2026-10-04 to
`openspec\changes\archive\2026-10-04-m2-upstream-tests\`, its specs now the main specs
(new: `openspec\specs\testing\upstream-tests`; updated: `compiler\cli`, `compiler\pipeline`,
`testing\compiler-tests`). Task 6.5, the last one, done 2026-10-05: the CI job `tier2` in `rust.yml` is green on
GitHub.

- `tests\upstream\`: QB64pe's compile tests copied (404 `.bas`, 836 text files, 2.5 MB, `SOURCE.md`); tier 1
  (`inputs.rs`) runs the front end over about 1,000 files: corpus, upstream, 406 QB64Fresh snippets labelled by
  `qb64pe.exe` (`tests\snippets\`), and from the clone `qbasic_testcases` and the old compiler's sources. No panic,
  exact round trip everywhere. `cargo test` takes about 3 s in debug.
- Diagnostics carry a "not supported yet" marker (`error: not supported yet: ...`, summary `N errors (M not
  supported yet)`). Baseline for parser breadth: **181 programs accepted by `qb64pe` still get a real error**
  (`tests\known_false_errors.list`), **82 rejected programs get only marked errors**
  (`tests\known_unsupported_rejections.list`); breakdown in `tests\upstream\README.md`.
- Seeded mutation test (`mutate.rs`): 5,500 mutants in under a second; 220,000 more with other seeds found nothing.
- **Upstream progress: 10 of 279** (`tests\upstream\pass.list`, `deferred.list` with the 125). `qb64rust` accepts
  the suite's `-f:` settings; the runner's `--list` and the new `--compile-tests` work for `--suite compile`.
- `Ty`, `BinOp`, `ConvKind` are defined once, in `sema`.
- Bugs found and fixed on the way: `#line` wrote `\` in a path as `\134`, which clang rejects (any absolute
  Windows path failed to build); build folders with spaces, `$` or `'` in the name broke `make` (every
  compile-suite program, `<test> - output.exe`); the runner's stale-file cleanup failed on kept build folders.
- **Found, fixed 2026-10-04 (`m2-parser-breadth` task 1.1: comment `$INCLUDE`/`$STATIC`/`$DYNAMIC` are now "not
  supported yet", a malformed comment `$INCLUDE` an error, by the old compiler's rule): wrong code.** Comment metacommands (`'$INCLUDE: 'x.bm'`, `REM $INCLUDE`) are ignored as
  comments, so 4 upstream programs build and print the wrong output (`include_once`, `include_paths`). This breaks
  "never wrong code"; first item of step 2 below: mark them "not supported yet" until `$INCLUDE` is implemented.
- **Decided 2026-10-04 (was an open question):** the runner compared an `.err` program's compiler output with the
  old text, so the 56 upstream `.err` programs (counted in the 279) could not pass tier 2 with the new messages.
  For compilers other than `qb64pe`, an `.err` program now passes when the compile fails, writes no executable
  and reports at least one error not marked "not supported yet"; they stay in the 279 (`DECISIONS.md`; implemented
  in `m2-parser-breadth` task 2.1).
- Your index: files staged before `.gitattributes` gained the `-text` lines were stored with LF (7 upstream `.bas`
  show `AM`); staging them again stores the bytes as upstream has them.

## M2: control-flow slice (2026-10-05 to 2026-10-07)

OpenSpec change `m2-control-flow-slice`: all tasks done; archived 2026-10-07 to
`openspec\changes\archive\2026-10-07-m2-control-flow-slice\` (record in its `tasks.md`), its specs now the main
specs (new: `openspec\specs\language\control-flow`, `language\constants`; updated: `compiler\pipeline`,
`language\error-handling`, `language\numeric-semantics`). Supported now
(`crates\README.md`): every operator, `CONST` (the old evaluator, as measured), `OPTION _EXPLICIT`, `IF`, `FOR`,
`DO`, `WHILE`, `EXIT`, `GOTO`, `GOSUB`, `RETURN`, labels in every body. The IR is flat with explicit jumps and
states the pending-error rule (a raising header behaves as in QB64pe, measured by `verification\v17_*`).
`slice.list` 54 → 113, upstream 10 → 23 of 279. The CI job `tier2`'s steps, run locally against the QB64pe
4.7.0 release (2026-10-07): 113 of 113 and 23 of 23.

## M2: arrays-and-`TYPE` slice (2026-10-07)

OpenSpec change `m2-arrays-and-types`: all tasks done (9.2 on 2026-10-07, session 23: the CI `tier2` steps against
the QB64pe 4.7.0 release, 129 of 129 and 24 of 24); archived 2026-10-07 to
`openspec\changes\archive\2026-10-07-m2-arrays-and-types\` (record in its `tasks.md`), its specs now the main specs
(new: `openspec\specs\language\arrays-and-types`; updated: `compiler\pipeline`, `testing\compiler-tests`). Supported now (`crates\README.md`): static arrays of the main module (numeric, `STRING`, `TYPE`
elements), elements by reference, `LBOUND`/`UBOUND`, `TYPE` with numeric and nested members, members by
reference, dotted plain names. Measured first (`verification\v18_*`, 67 programs, `study\00` §5). Decided by the
user: a member store into an element with a bad index stores nothing (`DIVERGENCES.md` D-004). The IR now shares
`sema`'s value tree (design D10). Tier 1 lowers, validates and emits every accepted input (`ir::validate`).
`slice.list` 113 → 129, full corpus 127 → 144, upstream 23 → 24 of 279.

## M2: language server (2026-10-08, session 25)

OpenSpec change `m2-language-server`, archived to `openspec\changes\archive\2026-10-08-m2-language-server\` (record
in its `tasks.md`, details settled while applying in its `design.md` "As built"); its specs are now the main specs
(new: `openspec\specs\editor\language-server`; updated: `compiler\cli`, `editor\compiler-diagnostics`,
`editor\language-support`). `qb64rust lsp` (new crate `crates\lsp`, `lsp-server` and `lsp-types`, the workspace's first
dependencies, so `cargo deny` came with them: `deny.toml`, CI job `deny`) gives, from the parser alone: syntax
errors as you type (100 ms debounce, a parse worker, cancellation), the outline (procedures, `TYPE` with members,
labels), folding of every block, `$IF` region and comment run, and go to definition for procedures, labels and
line numbers across included files. Included files come from open documents first, then the disk, by the
compiler's lookup. The document's text is parsed as the bytes of its encoding, with tables generated from
`iconv-lite` (`tools\encodings\gen_tables.js`); positions are UTF-16 columns, the only encoding VS Code's client
accepts (checked, task 1.1). The extension starts it when `qb64rust.path` (or `PATH`) names the binary, in a
trusted workspace; its diagnostics (source `qb64rust`) live beside the old compiler's (`qb64pe`).

- Does not do: anything `sema` knows (variables, constants, "not supported yet" of statements), hover,
  completion, rename, references, semantic highlighting, formatting; the server is not shipped inside the `.vsix`.
  It does not watch the disk: an included file that is not open and changes on disk is read again at the
  includer's next parse (its next edit).
- Review fixes (session 25): closing one includer no longer clears an include's diagnostics that another open
  program still shows; a request after a failed parse gets an empty answer instead of waiting for good; an
  overtaken restart of the client no longer sets the state; a relative `qb64rust.includeRoot` without a workspace
  folder is ignored.
- Tests: `cargo test -p qb64rust-lsp` (unit; symbols and folding snapshots and every go-to-definition form;
  16 protocol scenarios in-process; the walks over 696 corpus and upstream programs in 1.6 s), two CLI tests of
  the subcommand; the extension: unit 47, integration 42 with the server on VS Code stable and on 1.100 (37 and 5
  skipped without it). Measured (`study\00` §5): parsing `qb64pe.bas` with its includes takes 111 ms in release.
- CI: `vscode-extension.yml` builds `qb64rust` first and runs the integration suites with it; not yet run on
  GitHub (task 5.1, left open when the change was archived: check the run after the next push).

**Next** (steps 1–5: order accepted 2026-10-07 after the fourth review, `study\24` §4; from step 6 on: reordered
2026-10-07 after the fifth review, `study\26` §6, reasons there). One line per step; the per-task record of a change
is its `tasks.md`, and `git log`.

1. Done 2026-10-05: CI green (`rust.yml` with `tier2`, `repo-check.yml`; `vscode-extension.yml` unchanged since
   its last green run).
2. Done 2026-10-05: `m2-parser-breadth` groups 5 and 6 (member access, `DATA`, line numbers, blocks). The change
   pauses until step 5.
3. Done 2026-10-07: `m2-control-flow-slice` (archived); the IR review (`study\25`): **IR kept**, the jump and
   error model final, the value-tree copy decided after step 4, the place question stated for step 4 (§3).
4. Done 2026-10-07 (session 23): `m2-arrays-and-types` task 9.2; archived. The place question and the value-tree
   question of the IR review are answered (design D5, D10).
5. Done 2026-10-07 (session 23): `m2-parser-breadth` groups 7 and 8 (preprocessor, declarations, control
   transfer, I/O statements, `specialformat` templates, `$INCLUDE` with one tree per file, the follow-on rule,
   auto-include names) and group 9; `known_parse_gaps.list` and `known_false_errors.list` are empty. Both changes
   archived 2026-10-07. The decisions of `study\26` §9 taken the same day (`DECISIONS.md`); its tier-1 check
   "clean programs are listed" is in (`tests\known_clean_not_passing.list`).
6. Done 2026-10-08 (session 24): OpenSpec change `m2-core-builtins`, all tasks done; archived to
   `openspec\changes\archive\2026-10-08-m2-core-builtins\` (record in its `tasks.md`), its specs now the main specs
   (new: `openspec\specs\language\builtin-functions`; updated: `compiler\pipeline`, `language\control-flow`,
   `testing\compiler-tests`): the emitter split by concern; `verification\v20_*` measured first (`study\00` §5); `sema`'s
   built-ins table-driven (`sema\src\builtins.rs`, one rule per kind of special-casing, held and believed result
   types); 43 built-in functions; `SELECT CASE`/`EVERYCASE`; `ON … GOTO/GOSUB`; slice programs `s25`–`s29`; tier-1
   coverage check for built-ins. Decided 2026-10-07: `ON n` above 255 falls through and the `SELECT` copy stays
   static, both as QB64pe (`DIVERGENCES-QB45.md` Q-001, Q-002, now pinned by `s29`, `s28`); 2026-10-08: six
   old-compiler oddities found by the measurements are kept for now (`DECISIONS.md`, `study\00` §6). CI `tier2`
   steps against the QB64pe 4.7.0 release: 192 of 192, 42 of 42.
7. Done 2026-10-08 (session 25), checked locally: the thin language server (`study\23` §2.6; design points
   `study\27` §6): OpenSpec change `m2-language-server`, archived, section above. Left: see the CI run on GitHub
   after the next push (task 5.1).
8. **Next:** bug-compatibility decisions done 2026-10-08 (session 26; all of `study\00` §6, `DECISIONS.md`,
   `DIVERGENCES.md` D-005–D-013, `DIVERGENCES-QB45.md` Q-003–Q-008). Left: OpenSpec change `m2-numeric-types`
   (proposed 2026-10-08): the differential tester, the full numeric type set and fixed-length strings
   (`Ty` stays an enum with an explicit rank; no type table, `study\27` §5), and the decided fixes D-005–D-009 and the
   `CONST` `^` warning. D-010–D-012 are libqb's and wait for the runtime copy (M3); D-013 for its two built-ins.
   **Group 1 (measurements) done 2026-10-08 (session 27):** `verification\v21_*` (`study\00` §5), the spec deltas
   and design corrected (listed in its `tasks.md` 1.5), four oddities decided by the user (`DECISIONS.md`); then the
   user's rule "do what QB64pe does" (questionable behaviours listed in `SOMEDAY.md` for review), so task 1.6
   measured and brought in the forms first left "not supported yet" (`verification\v21_f_*`). **Group 2 (the
   differential tester) done 2026-10-08 (session 28):** `crates\difftest` generates 59 programs into
   `tests\differential` (every operator on every pair of 17 numeric types, stores, unary, `PRINT`/`STR$`, operands
   as variables and as literals), recorded with `qb64pe.exe`; tier 1 checks they are fresh and recorded; CI runs
   `tests\differential\pass.list`. The full programs cannot compile before the new types do, so the six-type
   subset is kept as the recorded group `old6` (48 programs, task 2.7): 27 pass and are on the list; the rest stop
   where D7's literal rules (group 4) apply.
   **Group 3 (`Ty` without a derived order) done 2026-10-09 (session 29):** the 17 variants of design D3 and their
   methods, no behaviour change (no snapshot changed; tier 2 192, 42, 27 as before); record in `tasks.md` 3.1.
   **Group 4 (declarations and literals) done 2026-10-09 (session 30):** every `AS` spelling and suffix of the new
   numeric types is declared, sized by `LEN` and emitted; their *values* are "not supported yet" behind one gate in
   the checker (`Ty::is_gated`) until groups 5 and 6 write their rules. Suffixed literals and `CONST`s are held as
   C++ types their digits and believed the suffix's type (design D7), which made the 21 remaining `old6` programs
   and upstream `const/expression` pass; `STRING * n` and `t$n` parameters (design D6). Tier 2 192, 43, 48. Record
   in `tasks.md` 4.1–4.3.
   **Group 5 (typing, conversions and emission) done 2026-10-09 (session 31):** values of every new numeric type
   compute as the old compiler's C++ does (`held`, the markup, the `_OFFSET` rules, `conversion`, the `_BIT` store
   mask), corrected against the differential programs: **all 59 full differential programs pass** (107 with `old6`;
   the `fold` ones with folding off too). Five corrections to `study\02` and the design (a `_BIT` believed its own
   type, the unsigned markup, unary markup, `EQV`/`IMP` as `~a^b`, `ll`/`ull` for wide `_BIT` literals; `study\00`
   §5), four accidents to `SOMEDAY.md`. Slice programs `s30_new_types`, `s31_unsigned_ops`. Uses that group 8
   brings stay "not supported yet" (`Checker::later`). Tier 2 194, 43, 107. Record in `tasks.md` 5.1–5.4.
   **Groups 6 (`_BIT`) and 7 (fixed-length strings) done 2026-10-09 (session 32):** `_BIT` needed only its tests
   (`bit_stores_cpp`, `bit_arrays`, `s32_bit`, CLI test `wide_bit_scalars_do_not_overlap`; D-009 pinned).
   `STRING * n` and `name$n` variables of every storage class, static arrays and `TYPE` members of them, built as
   the old compiler builds them (fixed `qbs` descriptors over NUL-filled bytes, `qbs_set` stores), passed to `STRING`
   parameters by reference; `s33_fixed_strings` and corpus `198_type_fixed_string` pass (Q-005 pinned). Still "not
   supported yet": the `MID$` statement (step 9) and `f$n` FUNCTIONs (task 8.2). Tier 2 199, 43, 107. Record in
   `tasks.md` 6.1, 7.1, 7.2. The tier-2 runner gained `--jobs N` and `--build-cache DIR` for local runs
   (`crates\README.md` "Tests").
   **Groups 8 (the new types everywhere else) and 9 (the decided fixes) done 2026-10-09 (session 33):** arrays,
   members, parameters (by reference across signedness also for elements and members, measured
   `verification\v21_g_passing_places`), FUNCTION results with `f$n`, `FOR`, `SELECT CASE`, constants used with
   another suffix (measured `v21_g_const_suffix`) and the special-cased built-ins as `evaluatefunc` writes them;
   nothing is gated any more (`Checker::later` gone). D-005 to D-008 pinned; the chained-`^` warning, printed only
   with `-w`. `s34_types_procs` and upstream `const/hex_literals` pass. Tier 2 198, 44, 107. Record in `tasks.md`
   8.1–9.4. Found on the way and fixed by the user's decision: two calls of a FUNCTION `f$n` in one expression read
   memory the FUNCTION had released (`fs$5("ab") + fs$5("cd")` was `cd   cd   `); it now returns a copy
   (`DIVERGENCES.md` D-014).
   Review fixes (session 34): the D-014 copy is now made before the epilogue releases the static pool (it was made
   after); `name$n` is read as a procedure's name in one place (`Checker::proc_fixed_name`), only for variable
   names and calls, so `CONST`, `TYPE` members, labels and SUB headers named `f$n` stay "not supported yet"
   (measured, `verification\v21_x50`–`x57`: QB64pe rejects all eight; beside a SUB or a FUNCTION of another type
   `x$n` is "Name already in use", as qb64rust says); the extension reads a warning line with a column
   (`file:line:col:`) too.
   **Next: group 10, integration** (full runs with the release build, the CI `tier2` steps against the 4.7.0
   release, docs).
9. Built-in statements and functions by demand (`study\27` §3): a built-in statement operation in the IR, then
   sequential file I/O, `DATA`/`READ`/`RESTORE`, `SWAP`, `RANDOMIZE`/`RND`/`TIMER`, console `INPUT`/`LINE INPUT`,
   `SHELL`/`COMMAND$`/`ENVIRON$`, each measured first; the remaining plain functions as the corpus, upstream or a
   user names them.
10. `$CONSOLE` with `_DEST _CONSOLE` and `$SCREENHIDE`, measured first (`study\26` §5); check once that the GitHub
    Windows runner can start the hidden window.
11. `DEFxxx`, then the rest of arrays and `TYPE`: `REDIM`, dynamic arrays, `OPTION BASE`, plain member arrays
    (`study\26` §4; their `_STATIC`/`_DYNAMIC` markers stay in `SOMEDAY.md`).

Numbers at the last full runs (2026-10-08, session 24, release build against the reference clone):
`slice.list` 192 of 192, full corpus 208 pass and none wrong at run time, upstream **42 of 279** (44 pass against
the clone; two need the `VAL` of libqb after the 4.7.0 release, `tests\upstream\README.md`; none wrong);
shrink-only lists: **0 false errors, 0 parse gaps**, 65 only-marked rejections (`tests\upstream\README.md`).
Tier 1: about 10 s. Upstream's number is expected to stay near 42 until steps 8, 10 and 11 (its blockers are
types, `_DEST` and the rest of arrays, `study\27` §4); the corpus is the signal for steps 7–9.

**`m2-parser-breadth`** (2026-10-04 to 2026-10-07; archived 2026-10-07 to
`openspec\changes\archive\2026-10-07-m2-parser-breadth\`, record in its `tasks.md`, its specs now the main specs:
updated `compiler\cli`, `compiler\pipeline`, `testing\upstream-tests`): groups 1–6 as
before (wrong-code fix for comment metacommands, panic hook, tier-2 `.err` meaning, parse-gap list, `v16_*`
measurements, one tree per file, member access, `DATA`, line numbers, blocks). Session 23, groups 7 and 8: the
preprocessor (`syntax\src\pp.rs`, a port of `EvalPreIF`; `$IF` and blocks nest; slice program `s23`), every
declaration form, `ON … GOTO/GOSUB` and the event forms, the I/O statements, built-in statements read by their
`specialformat` template (new dependency `syntax -> builtins`), `$INCLUDE` (the parser loads and parses included
files with the shared `PpState`; `--include-root`; runtime errors in an included file named as QB64pe names
them), the follow-on rule, and the names of QB64pe's auto-included files; all marked "not supported yet" in
`sema` unless already compiled. Measured on the way (`verification\v19_*`, `study\00` §5): procedure names
(the known bug below), blanks around a dot, `OPTION _EXPLICIT` in an included file, a runtime error in an included
file. Found and fixed on the way: the precompiler flags (`_CONSOLE_` ...) were unknown names and made upstream
`precomp-flags/consoleonly` print the wrong output (now marked); the design's include root could not resolve two
upstream tests (now `tests\upstream\root`, made by the copy script). All tasks done.

## FreeBASIC reviewed (2026-10-03)

`..\FreeBASIC\` cloned (reference only, GPL/LGPL). Findings in `study\16-freebasic.md`. Reviewed by a panel and a second
pass (`study\16` §7). Measured finding about the old compiler (`verification\v11_wrap_o2`): LONG overflow gives
different results with and without the "optimize C++" setting (`-O2`), because the generated C++ is compiled
without `-fwrapv`. The six follow-ups in `study\16` §6 were decided by the panel (`study\16` §8): LONG overflow
wraps (decided now); test conventions and recovery policy go into M2; the rest are notes for M3, M5, M6.
M1 leftovers: done (see above).

## Known bugs

None open.

Fixed 2026-10-07 (session 23): **a SUB named like a built-in function was a false error** (`SUB loc`: "name already
in use"). Measured for every keyword and built-in (`verification\v19_proc_names`, 1,948 programs): a SUB name is
taken only by a built-in statement without a required suffix, a FUNCTION name only by a built-in function without
one (plus keywords and `_` names, as for variables); `sema`'s `reserved_proc`, checked against every measured form
by `names.rs`. In an expression such a SUB's name still means the built-in function. Slice program
`s24_proc_names`.

Fixed 2026-10-07 (session 20): **deeply nested expressions overflowed the stack** (`x = 1 + 1 + …`
with 20,000 terms, 3,000 nested parentheses; the debug build already at 250 terms, since a left-associative chain
is parsed in a loop but nests one tree level per operator). Now: the parser counts each expression's tree height
and marks one deeper than 1,000 levels "not supported yet" (`parser\expr.rs` `MAX_EXPR_DEPTH`); the compiler runs
on a 64 MiB thread (`driver::with_stack`; a debug build needs about 4 KiB per level, the Windows main thread has
1 MiB), so both limits fit at once. Measured: the old compiler overflows its own stack at 150–200 nested
parentheses and under 50 nested calls, takes flat chains of 3,000 operators; of 1,780 `.bas`/`.bi`/`.bm` inputs
only one goes past 40 levels. CLI test `deep_expressions`. Found on the way and fixed: the parser's look-ahead
(`nth`) rescanned from the current token, so classifying `a(1).b.b… = 1` was quadratic (48 s for 20,000
members in debug); it is now constant time.

## Queued for later milestones

| When | Item |
|---|---|
| M3 | Plain copy of `..\QB64pe\internal\c` at the pinned commit (`DECISIONS.md`) |
| M3 | Programs that draw (without `$CONSOLE:ONLY` and not writing to `_DEST _CONSOLE`), with an oracle such as a screen-state dump at exit compared between old and new compiler (`study\22` §3.2); `$CONSOLE` + `_DEST _CONSOLE` programs are M2 work (step 10 of "Next", `study\26` §5) |
| M4 | `qb64pe.bas` reached through its own include files, smallest first (`study\22` §4.2) |
| Help/hover work | Ask the QB64pe maintainers about the wiki licence before shipping any wiki text |
