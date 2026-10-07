# Status and next steps

Updated 2026-10-07 (session 19). Session-by-session history is in `git log`; measured facts are in `study\00`.

## Where we are

Roadmap (`study\07` Session 8, summarised in `study\00` §2): **M0 and M1 complete. M2 (front end) in progress: golden corpus, Rust workspace and end-to-end slice, procedures and error handling, upstream tests done.**

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
  and reports at least one error not marked "not supported yet"; they stay in the 279 (`CLAUDE.md`; implemented
  in `m2-parser-breadth` task 2.1).
- Your index: files staged before `.gitattributes` gained the `-text` lines were stored with LF (7 upstream `.bas`
  show `AM`); staging them again stores the bytes as upstream has them.

**Next** (order accepted 2026-10-05 after the third review and panel, `study\23` §4, reasons there; it replaces
the order of `study\22` §5):

1. Done 2026-10-05: `rust.yml` (with `tier2`) and `repo-check.yml` green on GitHub for `1289c20`;
   `vscode-extension.yml` last ran green on `main` (2026-10-03), nothing under `vscode\` changed since. Push the
   later commits and glance at their runs.
2. Done 2026-10-05: `m2-parser-breadth` groups 5 and 6 (member access, `DATA`, line numbers, blocks); state of
   the change below. The change now pauses.
3. **Next:** a control-flow slice through to C++, its own OpenSpec change (`m2-control-flow-slice`, proposed 2026-10-05, in progress): `IF`, `FOR`, `DO`,
   `WHILE`, `GOTO`, `GOSUB`/`RETURN`, `CONST`, `OPTION _EXPLICIT`, labels inside blocks and procedures, `RESUME
   NEXT` after an error in a block header, and the check of `NEXT` variables against their `FOR` (left out of the
   parser, design D4 "As built"). The block nodes and accessors exist (`ast.rs`); `sema` marks them in
   `check\blocks.rs` `block_parts`. First count the corpus and upstream programs blocked only by these. Then the IR
   review (keep, or merge into the typed tree; `study\20` §3.4). Fourth review (2026-10-05, order unchanged),
   written into the change `m2-control-flow-slice`: the IR's error rule is restated (a pending error and
   placeholder values, checked at named points; "skips the rest of the statement" was true only of `PRINT`) and
   measured first; operator typing lives in one function so the type table of step 6 rewrites one place;
   constants and `OPTION` are done after the IR and emitter tasks; the IR review leaves open how the IR names
   an array element or a `TYPE` member (step 8). Task 1.1 done 2026-10-06: 115 measurements `verification\v17_*`,
   findings in `study\00` §5 and §6; they corrected the design and five spec deltas (list in task 1.1): a procedure
   called while an error is pending returns at once, an `ELSEIF` error is serviced at the next statement, one
   `GOSUB` stack for the program, a name used before its `CONST` line is an error, `OPTION _EXPLICIT` is
   program-wide. Decided 2026-10-06: `RETURN label` with nothing pending is guarded (the old program crashes on
   the next `GOSUB`), `DIVERGENCES.md` D-003. Task 1.2 done 2026-10-06: slice programs `s13`–`s19` recorded
   (corpus now 282 programs); they agree with every spec scenario, coverage listed in the task; they join
   `slice.list` as they pass. Tasks 1.3 and 2.1 done 2026-10-06: `m2-parser-breadth` handed `CONST`/`OPTION`
   over; `sema`'s `check.rs` split into `check\` (`mod.rs`, `expr.rs`, `decl.rs`, `proc.rs`, `flow.rs`,
   `blocks.rs`; code moved only, no snapshot changed). Tasks 3.1 and 3.2 done 2026-10-06: every operator through
   to C++ (typing in `check\ops.rs`); `s17_operators` and 15 `runtime_comparison` programs pass, so `slice.list`
   has **70** programs (54 before); full corpus 84 pass, none wrong at run time; upstream **11 of 279**. Found on
   the way: `/` between two variables was emitted as `/*` (a C comment; the C++ failed to compile), fixed. Task
   4.1 done 2026-10-06: `CONST` and `OPTION` parse (`sema` still marks them); parse gaps 527 to 500, four `const/`
   `.err` programs pass, upstream **15 of 279**; `ROOT` (constant evaluator only) "not supported yet" in a `CONST`.
   Task 4.2 done 2026-10-06: the constant evaluator (`sema\consteval.rs`, `check\constants.rs`); floats in `f64`
   with an exactness check against the old `_FLOAT` path, "not supported yet" where it cannot be shown; a float
   beyond `_INTEGER64` is DOUBLE (corrects D6); `slice.list` **75**, full corpus 89, upstream **20 of 279**, none
   wrong; `s18_const` waits for `IF`/`FOR` (8.1). Task 4.3 done 2026-10-07: `OPTION _EXPLICIT` program-wide
   (pre-pass), `SHARED` must name a main variable of that type declared earlier (two new measurements); under the
   option an undeclared variable after anything marked "not supported yet" is only marked (follow-on rule, design
   D7 "As built"); false errors lose 12, `slice.list` 75 of 75, upstream still **20 of 279**. Tasks 5.1 and 5.2
   done 2026-10-07 (session 20): labels per body (a label is a typed statement; labels in procedures and inside
   blocks; four more measurements `verification\v17_d_*`), `GOTO`/`GOSUB`/`RETURN` typed, and `IF` (both forms),
   `FOR`, `DO`, `WHILE`, `EXIT FOR/DO/WHILE` as typed block statements with their errors (string conditions and
   limits, `NEXT` variable by identity, `FOR` variable a numeric scalar). Until the IR has jumps they stop at a
   gate before lowering ("not supported yet: … in code generation", `ir::not_lowered`); `--dump typed` shows them.
   Rejections with only marks 74 to 72; tier 2 75 of 75 and 20 of 20, unchanged. Next: task 6.1, the IR types.
4. `m2-parser-breadth` groups 7 to 9, with the blunt follow-on rule and block crossing marked (`study\23` §2.3,
   §2.4; design D10, D4).
5. A thin language server in the extension: syntax errors, outline, folding, go to definition for procedures and
   labels (`study\23` §2.6).
6. Bug-compatibility decisions (`study\00` §6), then the differential tester, `Ty` as a type table, unsigned types.
7. Plain built-ins (249 of 455), table-driven, with generated tests.
8. The rest of control flow (`SELECT CASE`, `ON … GOTO/GOSUB`, `DEFxxx`), then arrays and `TYPE` (storage designed
   so member arrays fit later).

State of **`m2-parser-breadth`** (started 2026-10-04;
   `openspec\changes\m2-parser-breadth\`). Done: task 1.1 (the wrong-code fix, comment metacommands), 1.2 (panic
   hook: "internal compiler error", exit code 3), and group 2: the tier-2 `.err` meaning in the runner (judged
   by the summary line), the third shrink-only list `tests\known_parse_gaps.list` (**673 of 1,139** files do not
   parse cleanly), and the 56 upstream `.err` programs run: 54 only marked, 2 pass only through a parse gap and
   are not added, so still **10 of 279** (`tests\upstream\README.md`). Task 3.1 done 2026-10-05: 94
   measurement programs `verification\v16_*` (M1–M8, four added by the reviews), findings in `study\00` §5;
   they changed eight design decisions (listed in task 3.1), among them: `$IF` and blocks must nest properly;
   only comment `$INCLUDE`; includes found next to the including file, then under a compiler root
   (`--include-root`, default the exe's folder, as the old compiler uses its own folder); depth 100, no cycle
   check. Group 4 done 2026-10-05 (no behaviour change): one `Tree` per file per inclusion in a
   `ParsedProgram`, `parse` takes a `Loader` (none loads anything yet), `sema` keyed by (`TreeId`, offset),
   `--dump tree` headed per tree. Group 5 done 2026-10-05: member access (`FieldExpr`), omitted arguments,
   `DATA`/`READ`/`RESTORE` (the old compiler's `DATA` scanner), line numbers, plain `GOTO`/`GOSUB`/`RETURN`, all
   parsed and marked by `sema`; **false errors 184 to 69**, parse gaps 677 to 654, two `.err` programs moved to
   the rejection list (task 5.1 says why); tier 2 unchanged. Group 6 done 2026-10-05: every block is a node
   (`IF` both forms, `FOR`, `DO`, `WHILE`, `SELECT CASE`, `TYPE`, `DECLARE LIBRARY`, `DEF FN`), one statement
   loop and block stack for the main module, procedures and blocks (`parser\blocks.rs`), recovery with one
   error per mistake, `EXIT` checked against the open blocks, `sema` marks every block and checks what is inside;
   19 more measurements; labels only at the start of a line (measured; it was wrong before). In entries (header
   comments not counted): **parse gaps 650 to 527**, no first gap a block any more; **false errors 66 to 59**;
   only-marked rejections 84 to 81; tier 2 unchanged, still **10 of 279** (task 6.4 has the details). The change
   pauses here for the control-flow slice (step 3 above) and resumes with statements, `specialformat` templates
   (new dependency `syntax -> builtins`), `$IF`, `$INCLUDE` with one tree per file, and the follow-on rule. Done
   when the parse-gap and false-error lists are empty.

## FreeBASIC reviewed (2026-10-03)

`..\FreeBASIC\` cloned (reference only, GPL/LGPL). Findings in `study\16-freebasic.md`. Reviewed by a panel and a second
pass (`study\16` §7). Measured finding about the old compiler (`verification\v11_wrap_o2`): LONG overflow gives
different results with and without the "optimize C++" setting (`-O2`), because the generated C++ is compiled
without `-fwrapv`. The six follow-ups in `study\16` §6 were decided by the panel (`study\16` §8): LONG overflow
wraps (decided now); test conventions and recovery policy go into M2; the rest are notes for M3, M5, M6.
M1 leftovers: done (see above).

## Known bugs

None open. Fixed 2026-10-07 (session 20): **deeply nested expressions overflowed the stack** (`x = 1 + 1 + …`
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
| Step 4 of "Next" | Decide the bug-compatibility choices in `study\00` §6 (32-bit INTEGER arithmetic and half-to-even rounding are already implemented as "keep") |
| M3 | Plain copy of `..\QB64pe\internal\c` at the pinned commit (`CLAUDE.md`) |
| M3 | Programs without `$CONSOLE:ONLY`, with an oracle such as a screen-state dump at exit compared between old and new compiler (`study\22` §3.2) |
| M4 | `qb64pe.bas` reached through its own include files, smallest first (`study\22` §4.2) |
| Help/hover work | Ask the QB64pe maintainers about the wiki licence before shipping any wiki text |
