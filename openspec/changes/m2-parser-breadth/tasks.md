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

- [x] 5.1 `Dot` token and `FieldExpr`; omitted arguments in `ArgList` and the `Option` accessor; `sema` marks
  `FieldExpr` "not supported yet". Verify: the member-access entries leave `known_false_errors.list` (about 120).
  Done 2026-10-05: `known_false_errors.list` 184 to 68 lines (none left from member access), parse gaps 677 to
  674. Also: an index or member assignment target (`a(1) = 2`, `a(1).b = 5`) is an `AssignStmt` with a
  `CallExpr`/`FieldExpr` target and `sema` (not the parser) marks it; a statement with a `,` outside parentheses
  is never an assignment (`s (5 / 2) = 2, 0` is a call); `ArgList::args` gives one `Option` per position and
  `sema` marks an omitted argument. Two entries added to `known_unsupported_rejections.list`, as task 2.3
  foresaw: `upstream/arrays/42_` and `43_err_solved_bug_Incorrect_nr_of_args` were rejected only through the
  parse gap at `f(10).a(5)`, not for the old compiler's reason (`UBOUND` argument count); now they get the
  marked `TYPE` errors. Test `tests\frontend\member_access.bas`, parser snapshot `member_access`.
- [x] 5.2 `DATA` mode in the lexer (per M2), `DataStmt` with items, `READ`, `RESTORE`. Lexer snapshots for the M2
  cases. Done 2026-10-05: one scanner, `syntax\src\data.rs`, follows the old compiler's rule (read in
  `qb64pe.bas` 25210–25310): a `"` opens a quoted item only as its first non-blank character, so `DATA a"b: x`
  ends at the `:`; text after a closing quote is a real error, as there. The word `DATA` starts data mode
  wherever it stands (an input has `… ELSE DATA 1,&h5`), except as a member name or with a suffix.
  `DataStmt::items` takes the file's bytes. Parse gaps 674 to 660. One entry came back into
  `known_false_errors.list`: `upstream/font/test.bas`, whose 584 `DATA` lines gave parser errors that filled the
  100-error cap and hid a real error at 36:15 (a `CONST` used as a string argument, a follow-on case for 8.2).
  Tests: lexer snapshot `data_text`, parser snapshot `data_read_restore`, unit tests in `data.rs`,
  `tests\frontend\data_read_restore.bas`.
- [x] 5.3 Line numbers (`LineNumber` node, per M3); `GOTO`/`GOSUB` to numbers. Verify: the `qbasic/` entries with
  line numbers leave `known_parse_gaps.list`.
  Done 2026-10-05: `parser\flow.rs`: `LineNumber` (a sibling statement, like `LabelDef`) for a `Number` at the
  start of a line, then labels; a number after a label or `:` is a real error (M3). Plain `GOTO`, `GOSUB`,
  `RETURN` with a label or number (the `ON …` forms stay in 7.3). The procedure loop takes the line prefix
  before looking for `END SUB` (`80 END SUB` closes the block), and the line number before looking for a
  `SUB`/`FUNCTION` header (review: `10 SUB s` is accepted, `10 SUB t` inside a `SUB` is the nested error;
  `verification\v16_m3_sub_header`, `v16_m3_nested_sub`). `sema` marks all four "not supported yet".
  Parse gaps 660 to 654; the 22 entries whose first gap was a line number all moved on to later families (`IF`,
  `ON … GOTO`) or left. Tests: parser snapshot `line_numbers_and_jumps`, `tests\frontend\line_numbers.bas`.

## 6. Blocks (D4)

- [x] 6.1 Block stack and recovery rules, with `IfBlock`, `IfStmt` (single-line) first; `check-fail` tests for each
  recovery rule.
- [x] 6.2 `ForBlock`/`NextStmt` (including `NEXT j, i`), `DoBlock`/`LoopStmt`, `WhileBlock`, `SelectBlock`,
  `EXIT FOR/DO/WHILE/SELECT`.
- [x] 6.3 `TypeBlock`, `DeclareLibraryBlock` (`BYVAL`, `ALIAS`, `CUSTOMTYPE`/`DYNAMIC`/`STATIC` forms found in the
  inputs), `DefFnBlock`/`DefFnStmt`.
- [x] 6.4 `sema` marks every block kind; statements inside are still checked. Verify: `parse-ok` tests per block
  kind; the IF/FOR/DO/SELECT entries leave `known_parse_gaps.list`.

  Group 6 done 2026-10-05, in one piece: `parser\blocks.rs` (one statement loop for the main module, procedures
  and blocks; the block stack; closers; recovery), `proc.rs` (procedures through that loop, `EXIT`, `DECLARE
  LIBRARY` with `ALIAS`/`BYVAL`), 32 node kinds with accessors in `ast.rs`, `sema` marking (`check.rs`,
  `block_parts`); the tree shape and the decisions taken are in design D4 "As built". 19 more measurements
  (`verification\v16_m4_*`, `v16_m3_*_after_colon`; `study\00` §5 "Blocks, more", "Labels"). No first parse gap is
  a block any more: **parse gaps 650 to 527**, **false errors 66 to 59**, **only-marked rejections 84 to 81**
  (`corpus/…/19_if_elseif_else` and the two `error_detection__unclosed_*` snippets now get the real error the old
  compiler gives); with the error cap lifted, no input file gets a block error the old compiler does not give.
  Found on the way and fixed: labels were taken after a `:` (`WHILE …: increaseUDTArrays: WEND` in `qb64pe.bas`
  is a call; measured); reserved names with a suffix other than `&` were extrapolated as taken (`name$`, `not$`
  are variables; now "not supported yet" until measured); a label with a SUB's name made `ON ERROR GOTO` to it a
  real error. Review of the group (same day): statements after `THEN 10:`, `GOTO 20:` or `ELSE 30:` were put after
  the `IF` instead of into the branch (measured `v16_m4_line_if_jump_colon`; it would have been wrong code once `IF`
  compiles), a stray `ELSE` within a statement was marked instead of a real error, two impossible loop exits
  could have spun, and messages counted lines wrong after a lone CR; all fixed. Three entries came back into `known_false_errors.list`, all task 8.2 families found behind gaps that
  are gone now: `upstream/putimage/putimage_test` (`_TRUE`), `qbasic/open_gl/3d_model_viewer` (`_GL`),
  `qbasic/misc/mzupd2` (`TIME$ = …`, a built-in assignment target). Upstream `.err` programs: none gets a real
  error yet; still **10 of 279**. Tier 2 unchanged: slice 54 of 54, upstream 10 of 10. Tests: parser snapshots
  `if_blocks`, `single_line_if`, `loops`, `select_case`, `type_block`, `declare_library`, `def_fn`,
  `block_recovery`, `labels_only_at_line_start`, the two block spec scenarios; accessor unit tests in `ast.rs`;
  `tests\frontend\blocks_*.bas` (four `parse-ok`, three `check-fail`). Not done here: `NEXT` variables against the
  `FOR` variables (a `sema` check for the control-flow slice); a `^Z` (0x1A) at the end of
  `qbasic/qb45com/action/arcdemo` is "expected a statement" (lexer, task 8.3; hidden behind the error cap so far).

**Pause here** (order of work, `study\23` §4, accepted 2026-10-05): after group 6 the control-flow slice goes
through to C++ as its own OpenSpec change, with the IR review after it. Groups 7 to 9 resume afterwards.

## 7. Statements (D5, D8)

- [ ] 7.1 Metacommands: `syntax::meta` splits `$NAME[:arg]` per `qb64pe.bas` (read, cite the lines); `$IF`/
  `$ELSEIF`/`$ELSE`/`$END IF`/`$LET`/`$ERROR` evaluated with `PpState` (per M6), `InactiveCode` node. `sema`:
  inactive code is skipped; active `$ERROR` is a real error. Verify: `parse-ok` tests with a `$IF` inside a block
  and an inactive block header; a `check-fail` test for a header in an active `$IF` closed outside it (M6, D8); the
  55 `$IF` files leave `known_parse_gaps.list` where nothing else is missing.
- [ ] 7.2 Declarations (`decl.rs`): every `DIM`/`REDIM` form, `DEFxxx`, `_DEFINE`, `COMMON`, `ERASE`,
  `SUB … STATIC`, array parameters. `sema` marks what it does not compile. `CONST` and `OPTION` are parsed and
  compiled by `m2-control-flow-slice` (its design D2), not here.
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
  cycle check; the error names the deepest file); a block crossing an include boundary is "not supported yet"
  (D4, changed 2026-10-05; `check-fail` tests from `v16_m7_*`: `FOR` closed in an include, `SUB` closed in one).
  `run_legacy_tests.py`
  passes `--include-root tests/upstream` to `qb64rust`, and the loader in `inputs.rs` (tier 1) uses the same root.
  Verify: a `check-ok` test for the guarded self-include (`v16_m7_self_guarded`); the upstream `include_once\*` and
  `include_paths\*` programs (including `include_fixed_compile_location`, `include_multiple`) pass tier 2 and go
  into `pass.list`; `qb64pe-source/` files are parsed through `qb64pe.bas`'s includes (clone present). Also (from
  `m2-control-flow-slice` 4.3): `sema`'s `OPTION _EXPLICIT` pre-pass (`check\mod.rs` `has_option_explicit`) reads
  only the main tree; it must read every tree, with a `check-fail` test where the only `OPTION _EXPLICIT` stands
  in an included file (measure it with `qb64pe.exe` first).
- [ ] 8.2 The follow-on rule (D10, changed 2026-10-05): after the first unsupported declaration `sema` reports only
  "not supported yet" errors; the two spec scenarios as `check-fail` tests. It replaces the narrower rule of
  `m2-control-flow-slice` 4.3 (`check\decl.rs` `undeclared`, design D7 "As built"). The auto-include name list from
  `extract_builtins.py` (re-run it; `builtins.json` gains the names with their source file); `_GL` and built-in
  assignment targets marked. Verify: the type-error and reserved-name entries leave `known_false_errors.list`; the
  new entries in `known_unsupported_rejections.list` are listed here by declaration kind and shown to the user,
  who decides whether any kind gets name tracking instead.
- [ ] 8.3 The last entries of `known_parse_gaps.list` and `known_false_errors.list`, one by one, until both are
  empty. Anything that turns out not to fit is decided with the user, not left in the lists.

## 9. Documentation and close

- [ ] 9.1 `crates\README.md` (what parses, what compiles, the new dependency, the third list), `tests\upstream\
  README.md` (lists and counts, upstream progress), `DIVERGENCES.md` for any decided difference, `CLAUDE.md`
  layout (`verification\v16*`), `STATUS.md` (step 2 done, numbers).
- [ ] 9.2 Measure tier-1 time in debug and record it in `crates\README.md`; full corpus once with the release build
  (no crash, no wrong executable). Verify: `openspec validate m2-parser-breadth --strict` passes;
  `python tools\repo_check\check_repo.py --untracked` passes.
