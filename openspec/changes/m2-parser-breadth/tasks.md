# Tasks

Every task that changes the parser or `sema` ends with: `cargo test`; `cargo clippy --workspace --all-targets --
-D warnings`; the three lists regenerated (`QB64RUST_UPDATE_LISTS=1 cargo test -p qb64rust-driver --test inputs`)
and their diff reviewed (entries only go away, except as D1 allows); tier 2 (`slice.list` and `pass.list`) still
passes with the release build. "Verify" below names what is checked on top of that.

## 1. Wrong code, panic hook (D2, D11)

- [x] 1.1 Lexer: `MetaComment` token kind (D2); parser: `MetaCommentStmt`; `syntax::meta::comment_directives`
  with the old compiler's rule (unit tests: `'$INCLUDE:'a.bi'`, `REM $FOO $DYNAMIC`, `'$Format:Off` gives
  nothing, `'$INCLUDEONCE` gives nothing, `$STATICx$DYNAMIC`). `sema`: a `MetaCommentStmt` with a directive is
  "not supported yet". Verify: the 4 wrong-code upstream programs (`include_once`, `include_paths`) are now
  rejected with a marked error; `print\auto_semicolon_insertion.bas` is unaffected.
- [x] 1.2 Panic hook and exit code 3; `QB64RUST_TEST_PANIC=1`; CLI spec scenario as a test in `cli.rs`.

## 2. Tier 2 `.err` meaning and the third list (D1, D11)

- [x] 2.1 `run_legacy_tests.py`: the `.err` meaning for compilers other than `qb64pe`. Verify: an upstream `.err`
  program that gets a real error passes; one that gets only marked errors fails with a reason that says so;
  `--qb64 qb64pe.exe` runs compare the text as before (one `.err` program checked each way).
  Done 2026-10-04: judged by the summary line (`N errors (M not supported yet)`, pass when N > M), not by
  `error:` lines, so a failed C++ build (clang prints `file:line:col: error:`) or an internal compiler error
  (exit code 3) never counts as a rejection; applies to the compile and the corpus suite.
- [x] 2.2 `inputs.rs`: `tests\known_parse_gaps.list` (D1), with the spec scenarios as tests. Generate it; record
  the count per set in `tests\upstream\README.md`. Done 2026-10-04: 673 of 1,139 files (the estimate of 790
  counted include files of the other sets too).
- [x] 2.3 Run the 56 upstream `.err` programs with `--list` against the release build; add the ones that pass to
  `pass.list`. Record the new "x of 279". Done 2026-10-04: 2 pass, but only through the parse gap at
  `f(10).a(5)` (task 5.1), so neither is added; still **10 of 279** (`tests\upstream\README.md`).

## 3. Measurements (M1–M8)

- [x] 3.1 Write and run `verification\v16_*` for M1–M8 with `qb64pe.exe` (`run.sh`), record outputs, and note
  the findings in `study\00` §5. Update this design where a finding changes a decision (and say so in the
  decision). Done 2026-10-05: 94 programs (4 added by the reviews; include files in `v16_inc\`). Eight decisions
  changed: (1) `$IF` and blocks nest properly, `$IF` is an entry on the parser's block stack (D8, spec); (2) only
  comment `$INCLUDE` exists (D9, spec); (3) include depth 100, not 32 (D9, spec); (4) lookup in the including
  file's folder, then the compiler root (`--include-root`, default the exe's folder), never the main file's folder
  or the working directory (D9, both specs); (5) no cycle check (D9, spec); (6) `DATA` ends only at `:` (D3);
  (7) `a(2) .b` is member access (D3); (8) every block crossing tried is rejected, crossing files is accepted (D4).
  Open question closed.

## 4. Many files at once: keys, loader, trees (D9, D10 keys)

- [x] 4.1 `TreeId`; `Node` carries it; `ParsedProgram`; `parse` takes a `Loader` (a no-op loader in tests);
  `--dump tree` prints every tree. No behaviour change. Verify: snapshots unchanged except the dump header.
  Done 2026-10-05: `syntax\src\program.rs` (`Tree`, `ParsedProgram`, `Loader`, `NoLoader`, `parse`);
  `Node::key()`; the loader takes the `SourceMap` as an argument and source bytes are shared (`Arc<[u8]>`), see D9.
  No snapshot changed; the header (`tree 0: p.bas`) has a CLI test (`tree_dump`), since no snapshot dumps a tree
  through the command line. Tier 2 unchanged: slice 54 of 54, upstream 10 of 10.
- [x] 4.2 `sema`: `check` takes the `SourceMap` and the `ParsedProgram`; every node-keyed map keyed by
  (`TreeId`, offset); `text()` through the `SourceMap`. Verify: no snapshot changes; `grep` finds no map keyed by
  a bare `u32` offset in `sema`. Done 2026-10-05: `proc_of_def`, `label_of_def` keyed by `Node::key()`; the
  skipped statements (parse errors, error cap) kept per tree; `dump_symbols` takes the `SourceMap`. Verified as
  stated.

## 5. Lexer and expressions (D3, D7)

- [ ] 5.1 `Dot` token and `FieldExpr`; omitted arguments in `ArgList` and the `Option` accessor; `sema` marks
  `FieldExpr` "not supported yet". Verify: the member-access entries leave `known_false_errors.list` (about 120).
- [ ] 5.2 `DATA` mode in the lexer (per M2), `DataStmt` with items, `READ`, `RESTORE`. Lexer snapshots for the M2
  cases.
- [ ] 5.3 Line numbers (`LineNumber` node, per M3); `GOTO`/`GOSUB` to numbers. Verify: the `qbasic/` entries with
  line numbers leave `known_parse_gaps.list`.

## 6. Blocks (D4)

- [ ] 6.1 Block stack and recovery rules, with `IfBlock`, `IfStmt` (single-line) first; `check-fail` tests for each
  recovery rule.
- [ ] 6.2 `ForBlock`/`NextStmt` (including `NEXT j, i`), `DoBlock`/`LoopStmt`, `WhileBlock`, `SelectBlock`,
  `EXIT FOR/DO/WHILE/SELECT`.
- [ ] 6.3 `TypeBlock`, `DeclareLibraryBlock` (`BYVAL`, `ALIAS`, `CUSTOMTYPE`/`DYNAMIC`/`STATIC` forms found in the
  inputs), `DefFnBlock`/`DefFnStmt`.
- [ ] 6.4 `sema` marks every block kind; statements inside are still checked. Verify: `parse-ok` tests per block
  kind; the IF/FOR/DO/SELECT entries leave `known_parse_gaps.list`.

## 7. Statements (D5, D8)

- [ ] 7.1 Metacommands: `syntax::meta` splits `$NAME[:arg]` per `qb64pe.bas` (read, cite the lines); `$IF`/
  `$ELSEIF`/`$ELSE`/`$END IF`/`$LET`/`$ERROR` evaluated with `PpState` (per M6), `InactiveCode` node. `sema`:
  inactive code is skipped; active `$ERROR` is a real error. Verify: `parse-ok` tests with a `$IF` inside a block
  and an inactive block header; a `check-fail` test for a header in an active `$IF` closed outside it (M6, D8); the
  55 `$IF` files leave `known_parse_gaps.list` where nothing else is missing.
- [ ] 7.2 Declarations (`decl.rs`): every `DIM`/`REDIM` form, `CONST`, `DEFxxx`, `_DEFINE`, `COMMON`, `ERASE`,
  `OPTION`, `SUB … STATIC`, array parameters. `sema` marks what it does not compile.
- [ ] 7.3 Control transfer (`flow.rs`): `GOTO`, `GOSUB`, `RETURN`, `ON … GOTO/GOSUB`, event `ON …` forms, `STOP`.
- [ ] 7.4 I/O statements (`io.rs`) and assignment forms (`assign.rs`), D5 list.
- [ ] 7.5 `specialformat` templates: grammar values in `builtins` (build-time parse, tests over all 172), `syntax ->
  builtins`, `BuiltinStmt` matching (D6, per M5). Verify: `parse-ok` tests for every graphics form found in the
  inputs; a `check-fail` test for a template mismatch; the `LINE`/`PSET`/`CIRCLE`/`PUT`/`GET`/`SCREEN`/`OPEN`
  entries leave `known_parse_gaps.list`.

## 8. Included files and follow-on errors (D9, D10)

- [ ] 8.1 The driver's file loader (path resolution per M7: including file's folder, then the compiler root),
  `--include-root` (default the exe's folder; CLI spec), comment `$INCLUDE` (replaces the marking of 1.1 for
  `$INCLUDE`; a bare `$INCLUDE:` stays an error, M1), `$INCLUDEONCE`; errors for missing files and depth (100, no
  cycle check; the error names the deepest file); blocks crossing an include boundary (M7). `run_legacy_tests.py`
  passes `--include-root tests/upstream` to `qb64rust`, and the loader in `inputs.rs` (tier 1) uses the same root.
  Verify: a `check-ok` test for the guarded self-include (`v16_m7_self_guarded`); the upstream `include_once\*` and
  `include_paths\*` programs (including `include_fixed_compile_location`, `include_multiple`) pass tier 2 and go
  into `pass.list`; `qb64pe-source/` files are parsed through `qb64pe.bas`'s includes (clone present).
- [ ] 8.2 Unknown names after unsupported declarations (D10); the auto-include name list from
  `extract_builtins.py` (re-run it; `builtins.json` gains the names with their source file); `_GL` and built-in
  assignment targets marked. Verify: the type-error and reserved-name entries leave `known_false_errors.list`; any
  new entry in `known_unsupported_rejections.list` is named here with its reason.
- [ ] 8.3 The last entries of `known_parse_gaps.list` and `known_false_errors.list`, one by one, until both are
  empty. Anything that turns out not to fit is decided with the user, not left in the lists.

## 9. Documentation and close

- [ ] 9.1 `crates\README.md` (what parses, what compiles, the new dependency, the third list), `tests\upstream\
  README.md` (lists and counts, upstream progress), `DIVERGENCES.md` for any decided difference, `CLAUDE.md`
  layout (`verification\v16*`), `STATUS.md` (step 2 done, numbers).
- [ ] 9.2 Measure tier-1 time in debug and record it in `crates\README.md`; full corpus once with the release build
  (no crash, no wrong executable). Verify: `openspec validate m2-parser-breadth --strict` passes;
  `python tools\repo_check\check_repo.py --untracked` passes.
