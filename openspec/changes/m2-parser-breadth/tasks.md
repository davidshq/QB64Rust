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

- [x] 7.1 Metacommands: `syntax::meta` splits `$NAME[:arg]` per `qb64pe.bas` (read, cite the lines); `$IF`/
  `$ELSEIF`/`$ELSE`/`$END IF`/`$LET`/`$ERROR` evaluated with `PpState` (per M6), `InactiveCode` node. `sema`:
  inactive code is skipped; active `$ERROR` is a real error. Verify: `parse-ok` tests with a `$IF` inside a block
  and an inactive block header; a `check-fail` test for a header in an active `$IF` closed outside it (M6, D8); the
  55 `$IF` files leave `known_parse_gaps.list` where nothing else is missing.
  *Done 2026-10-07 (session 23):* `syntax\src\pp.rs`: `directive` classifies a `$…` line as the old compiler does
  (trimmed, upper case, by prefix; `qb64pe.bas` 1836–1892, 3430–3534), `PpState` holds the `$LET` names (the ten
  predefined ones for Windows 64-bit, version 4.7.0) and the open levels, and `eval` is a port of `EvalPreIF`
  (28352–28536) with `SetPreLET` and `CompareVersions`, quirks included (unit tests). The parser evaluates each
  `$…` line in file order (`parser\meta.rs`); a branch not taken is one `InactiveCode` node, its end found by
  counting nested `$IF`s; an open `$IF` is a `PpIf` entry on the block stack, so a block closer meeting it, or a
  `$ELSE`/`$ELSEIF`/`$END IF` meeting an open block, is an error at that line, and the closer still closes its own
  block; `pop_block` removes the block's own entry under any `$IF` entries. A `$IF` line with an error still opens
  its level (as in the old compiler), so its `$END IF` gets no second error. `$IF`/`$LET` after `:` is an error
  (M6), an active `$ERROR` too; `$IF` without `$END IF` is reported at the end of the file. `parse_tree_with`
  takes and returns the `PpState`, ready for 8.1 (a `$LET` in an include reaches the including file). `sema`
  accepts the preprocessor lines and skips `InactiveCode`. The `$LET` value follows the prepass's normalisation
  (blanks dropped, quotes stripped), which is what decides which lines are compiled. Tests: slice program
  `s23_preprocessor` (recorded with `qb64pe.exe`; passes end to end), `tests\frontend\pp_if_errors.bas` (15
  errors, one per mistake), parser snapshot `preprocessor`. Lists: one parse gap gone
  (`afterlastline.bm`), `conditional_compilation__if_not_operator` left the only-marked rejections (now the real
  error); no other entry had a `$IF` as its first gap.
- [x] 7.2 Declarations (`decl.rs`): every `DIM`/`REDIM` form, `DEFxxx`, `_DEFINE`, `COMMON`, `ERASE`,
  `SUB … STATIC`, array parameters. `sema` marks what it does not compile. `CONST` and `OPTION` are parsed and
  compiled by `m2-control-flow-slice` (its design D2), not here; array bounds in `DIM`, `DIM SHARED`, `STATIC` and
  `SHARED` items (`DimItem::bounds`) by `m2-arrays-and-types` (its design D2), not here. `REDIM` and array
  parameters stay here.
  *Done 2026-10-07 (session 23):* `parser\decl.rs`: `DIM`/`REDIM`/`SHARED`/`STATIC`/`COMMON` with the type before
  the names (`DIM AS LONG a, b`: one type word, two after `_UNSIGNED`), `REDIM [_PRESERVE|_RETAIN] [SHARED]` with
  member arrays as items (`REDIM a(0).b(1 TO 3)`; found in 96 upstream array tests), `COMMON [SHARED] [/name/]`,
  `ERASE` (plain names, `a()`, member arrays as expressions), `DEFINT`/`DEFLNG`/`DEFSNG`/`DEFDBL`/`DEFSTR` and
  `_DEFINE … AS type` letter ranges (`LetterRange`), `AS STRING * n` everywhere (`n` a number or a constant).
  `parser\proc.rs`: array parameters (`a()`, `a( ,)`) and `STATIC` after a header. `parser\expr.rs`: a type name
  as an argument (`VAL(s, _UNSIGNED _INTEGER64)`, `_MEMGET(m, o, LONG)`, `_CAST(_BYTE, x)`) is a `TypeArg`. Lexer:
  a `.` between names with blanks around it is a `Dot` (measured: the old compiler drops those blanks, `a . s` is
  `a.s` for a member, a plain dotted name and in `ERASE`, `verification\v19_dot_blanks_*`); assignments take
  `a . s = 5`. `sema` marks every new form "not supported yet" (`REDIM`, `COMMON`, `ERASE`, `DEFxxx`, `_DEFINE`,
  the type before the names, fixed-length strings, array parameters, a `STATIC` header, a type argument, blanks
  around a dot), with one change in what it compiled before: `type_of` now marks `AS STRING * n` (a `DIM` with it
  was a parse mark before, so no program changed). Lists: parse gaps 465 → 281 lines, no new file; false errors
  one fewer (`t001_whole_array_core`); six only-marked rejections now get real errors the old compiler also gives
  (`194_array_param_2d`: `FUNCTION … AS LONG`; four `upstream/types/*_no_length`: no size after `*`; the `$IF NOT`
  snippet of 7.1). Tests: `tests\frontend\decl_forms.bas` (`parse-ok`, 28 marks and no real error), parser
  snapshot `declarations`; `unsupported_statement_message` now uses `LOCK`.
- [x] 7.3 Control transfer (`flow.rs`): `GOTO`, `GOSUB`, `RETURN`, `ON … GOTO/GOSUB`, event `ON …` forms, `STOP`.
  *Done 2026-10-07 (session 23):* (`GOTO`, `GOSUB`, `RETURN` were done in 5.3.) `parser\errors.rs`: `ON expr
  GOTO|GOSUB target, ...` (`OnJumpStmt`, targets may be left out), `ON TIMER|KEY(n)|STRIG|PLAY|PEN|COM|UEVENT
  [(args)] GOSUB label` or `… sub_name` (`OnEventStmt`), the switches `TIMER ON`, `KEY(1) OFF`, `TIMER(t) FREE`
  (`EventSwitchStmt`; other statements starting with these words fall through), `ON ERROR GOTO _NEWHANDLER label`
  and `_LASTHANDLER`. `parser\mod.rs`: `STOP`, `RUN [number|label|file]`, `END code` and `SYSTEM code` (an
  expression in `EndStmt`/`SystemStmt`). `sema` marks them all, also an `ON ERROR GOTO` whose target starts with
  `_` (it was a false "label not defined" before). Lists: false errors two fewer (`_NEWHANDLER`, `_LASTHANDLER`),
  parse gaps 281 → 254 lines. Test `tests\frontend\flow_forms.bas` (`parse-ok`; the old compiler accepts it with
  `-z`, checked); parser snapshots updated (`SYSTEM 3`, `ON TIMER(1) GOSUB`, `ON n` now parse).
- [x] 7.4 I/O statements (`io.rs`) and assignment forms (`assign.rs`), D5 list.
  *Done 2026-10-07 (session 23):* the statements of the D5 list without a `specialformat` template (those with
  one, `OPEN`, `GET`/`PUT`, `SEEK`, `NAME … AS`, `TIME$ =`, `_CLIPBOARD$ =`, go to 7.5): `PRINT #n,` and `USING
  fmt;` as `FileNumber`/`UsingClause` nodes (so `PrintStmt::parts` is unchanged), `LPRINT [USING]`, `WRITE [#n,]`,
  `INPUT [;] ["prompt" ;|,] targets`, `INPUT #n,`, `LINE INPUT` (both forms), `CLOSE [[#]n, ...]`, `FIELD [#]n,
  w AS v, ...`, `LSET`/`RSET target = value`, `SWAP a, b` (`parser\io.rs`, `print.rs`); `sema` marks them all.
  `MID$(…) = …` and `LET` already parsed as assignments. Two rules found on the way: a word that is not reserved
  followed by `=` is an assignment before any statement dispatch (`close = 3` in `FUNCTION close` is accepted,
  `v19_proc_names`), except `END` and `SYSTEM`, which stay statements there (measured likewise); and a name
  starting with `__` is not reserved (`validname`, `qb64pe.bas` 28271–28274: only a single leading `_` is refused;
  `qb64pe.bas` has a parameter `__name$`), which takes `qb64pe.bas` off `known_false_errors.list`. Lists: parse
  gaps 254 → 227 lines. Test `tests\frontend\io_forms.bas` (`parse-ok`; the old compiler accepts it with `-z`,
  checked); parser snapshot `marked_parse_errors` updated.
- [x] 7.5 `specialformat` templates: grammar values in `builtins` (build-time parse, tests over all 172), `syntax ->
  builtins`, `BuiltinStmt` matching (D6, per M5). Verify: `parse-ok` tests for every graphics form found in the
  inputs; a `check-fail` test for a template mismatch; the `LINE`/`PSET`/`CIRCLE`/`PUT`/`GET`/`SCREEN`/`OPEN`
  entries leave `known_parse_gaps.list`.
  *Done 2026-10-07 (session 23):* `builtins\src\template.rs`: the grammar (`Item::Arg`, `Punct`, `Choice` of word
  and punctuation sequences, `Optional`), `parse` and `print`; `build.rs` includes the same file and fails the
  build on a template that does not parse or print back; unit test over all 172. Deviation from the design: the
  grammar is parsed on demand (`statement_templates`) rather than emitted as values by `build.rs`, which only
  checks it; the check at build time is what D6 asked for. New dependency `syntax -> builtins`.
  `parser\template.rs`: at a statement whose word is a built-in SUB with a template (with its required suffix:
  `TIME$ =`), the forms are matched on the tokens ahead by backtracking (arguments first, so `LINE …, B` takes `B`
  as the colour and `LINE …, BF, BF` the second as the box word, as measured in M5; choices and entries in table
  order; an argument's extent from the shape of the expression grammar, `expr_extent`), then built as
  `BuiltinStmt` (name, `FormWord`, punctuation, `FormArg`). No match: a real error at the furthest token a form
  reached, naming the forms; but a reserved word followed by `=` (`key = 1`) is left to the assignment path, which
  reports the reserved name as before. `DEF SEG` goes through `DEF`'s template. A word followed by `=` is an
  assignment first (7.4) unless its template starts with `=` (`TIME$ =`, `_CLIPBOARD$ =`). Also, for the last
  entries: `_MEMPUT`/`_MEMFILL … value AS type` (`MemStmt`; stubs without a template), `_ARRAYCOPY source TO
  target` (`ArrayCopyStmt`, sides as `DimItem`s with slices and member arrays; deferred feature, parsed and
  marked), and `ROOT` as an operator inside `CONST` (level of `^`, `const_eval.bas` 157–158; `sema` marks it).
  `sema` marks every `BuiltinStmt` by its name. **`known_parse_gaps.list` is empty** (461 entries at the start of
  this session). Tests: `tests\frontend\template_forms.bas` (`parse-ok`, 54 forms; the old compiler accepts it with
  `-z`, checked), `builtins` unit tests `every_template_parses` and `template_shapes`, `ast` test
  `call_statement_parts` and parser snapshots `call_arguments_the_parser_cannot_read`, `def_fn`, `const_errors`
  updated.

## 8. Included files and follow-on errors (D9, D10)

- [x] 8.1 The driver's file loader (path resolution per M7: including file's folder, then the compiler root),
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
  *Done 2026-10-07 (session 23):* parser: `program.rs` `Includer` (source map, loader, trees by id, include map,
  `$INCLUDEONCE` files, depth); `Parser` has a second lifetime for it. At a comment `$INCLUDE` the parser loads
  the file, parses it at once as a tree of its own with the current `PpState` and takes back the state it leaves;
  missing file, unreadable file and depth over 100 are errors at the include (one error, naming the deepest file);
  `$INCLUDEONCE` makes later inclusions of the file empty. Block crossing (D4): blocks open at an include are
  marked `crossed`; in an included file a closer without its block, a block open at its end, and a `$IF` open at
  its end are "not supported yet", as is the including file's crossed block left unclosed. `sema`: pass 1 reads
  types and procedures through includes (`expand`), the label pre-pass and pass 2 descend into an included tree
  at its include statement; a SUB from a file included inside a SUB or block is an error (M7); a missing label is
  now a real error (the "program with an `$INCLUDE`" mark is gone); `$INCLUDEONCE` is accepted. Measured first
  (`verification\v19_explicit_*`): an `OPTION _EXPLICIT` in an included file applies to the whole program, also
  before the include, so the pre-pass reads every tree. Measured too (`v19_include_runtime_error`): a runtime error
  in an included file reports "Line: 3 (in raise.bi)"; codegen now emits `evnt(line, line, "file")` and `#line`
  with the file's name for included statements and labels (labels carry their file in `sema` and the IR); the
  new compiler prints the same message. Driver: `FileLoader` (the including file's folder, then the compiler root;
  `.\`/`./` dropped; absolute paths as written; one `FileId` per canonical path), `--include-root <dir>` (default:
  the executable's folder; CLI test `include_root`), `frontend_with`. Found while doing it: the design's root
  `tests\upstream` cannot resolve `'tests/compile_tests/extra/…'`, since the copy lives at
  `tests\upstream\compile_tests`; `copy_upstream_tests.py` now mirrors `extra\` into
  `tests\upstream\root\tests\compile_tests\extra\` (rerun at the pinned commit: no other file changed), and the
  runner (`--include-root tests/upstream/root` for the new compiler), tier 1 and the front-end tests use such a
  root. Tier 1 (`inputs.rs`): one loader per input, every tree round-trips, parse gaps of included trees count
  (named by file), and `upstream/qb64pe/*` (which include the old compiler's sources as `../../../source/…`)
  resolve from the clone's copy of their folder and join the clone-dependent sets (`list_diff` now leaves a
  clone-dependent entry out of "new" too, not only out of "stale", so tier 1 without the clone, as in CI, passes;
  checked with `QB64RUST_QB64PE_ROOT` pointing nowhere). Tier 2: `include_once/test`,
  `include_paths/include_fixed_compile_location`, `include_multiple` and `include_relative` pass. Lists: parse gaps
  still empty, also with `qb64pe.bas` parsed through all its includes; `upstream/qb64pe/element*` and
  `hasFunctionElement` now reach `_CHR_CR` (auto-included constants, task 8.2). Tests:
  `tests\frontend\include_ok.bas` (`typed`: `$LET` across files, `$INCLUDEONCE`, guarded self-include, root-only
  file, shared variable, SUBs from an include), `include_errors.bas` (missing file, nested include not looked up
  in the main file's folder, `FOR`/`NEXT` across files, SUB inside SUB, self-include loop), include files in
  `tests\frontend\inc\`; `comment_metacommands` snapshot (a missing file is now a real error). Tier-1 time: about
  14 s for `cargo test --workspace` (`inputs` 6 s).
- [x] 8.2 The follow-on rule (D10, changed 2026-10-05): after the first unsupported declaration `sema` reports only
  "not supported yet" errors; the two spec scenarios as `check-fail` tests. It replaces the narrower rule of
  `m2-control-flow-slice` 4.3 (`check\decl.rs` `undeclared`, design D7 "As built"). The auto-include name list from
  `extract_builtins.py` (re-run it; `builtins.json` gains the names with their source file); `_GL` and built-in
  assignment targets marked. Verify: the type-error and reserved-name entries leave `known_false_errors.list`; the
  new entries in `known_unsupported_rejections.list` are listed here by declaration kind and shown to the user,
  who decides whether any kind gets name tracking instead.
  *Done 2026-10-07 (session 23):* `sema`: a declaration statement (`DIM`, `REDIM`, `COMMON`, `SHARED`, `STATIC`,
  `CONST`, `DEFxxx`/`_DEFINE`) marked "not supported yet" in pass 2, a `TYPE` block marked in pass 1 (when pass 2
  reaches it) or inside a body, and a `DECLARE LIBRARY` block turn the rule on; from then on `error()` drops real
  errors (the statement still fails). `undeclared`'s narrower rule is gone. `extract_builtins.py` re-run: it now
  lists the names of the auto-included files (`beforefirstline.bi` 310 constants, `afterlastline.bm` 3 functions,
  the `$COLOR` files' constants with their condition) in `builtins.json` `auto_include` (the table entries are
  unchanged); `builtins` generates `AUTO_INCLUDE_NAMES` (unconditional ones) and `is_auto_include_name`; `sema`'s
  reserved-name check marks such a name, and `SUB _GL`, "not supported yet". Built-in assignment targets
  (`TIME$ =`, `_CLIPBOARD$ =`) were done by 7.5. Found by the full upstream run on the way: QB64pe's precompiler
  flags (`_CONSOLE_`, `_EXPLICIT_`, `_EXPLICITARRAY_`, `_ASSERTS_`, `_DEBUG_`, `_SOCKETS_`, set from the whole
  program, `qb64pe.bas` 1713–1722) were unknown names, so `$IF _CONSOLE_ THEN` was false and
  `precomp-flags/consoleonly` compiled and printed the wrong output; now a `$IF`/`$ELSEIF` naming one is marked by
  `sema` ("the precompiler flag … in `$IF`"; the parser takes such a branch silently, the program being rejected).
  Lists: **`known_false_errors.list` empty** (the last family, "cannot store a string in a number variable" after
  an unsupported declaration, 15 entries, and the auto-include names); new entries in
  `known_unsupported_rejections.list` by kind: none from the follow-on rule; one from the auto-include marks,
  `snippets/qb64fresh/constants__builtin_chr_str_constants_registered` (rejected by the old compiler for
  `_STR_CR`, the second name on its last line, after `_STR_LF`, an auto-include name, which is now marked; the
  statement stops at its first mark). Tests: `pp_if_errors.bas` (precompiler flags), `builtins` unit test
  `auto_include_names`, `pp` unit test `precompiler_flags`; `explicit_after_unsupported` (real errors after a
  marked declaration dropped, its comment rewritten). Four error tests lost real-error cases to the rule
  (`types_errors`, `proc_errors`, `blocks_marked`, `unsupported`: a marked `TYPE`, `SHARED`, `DECLARE LIBRARY` or
  fixed-length `DIM` stood early); their marked declarations now stand last, and every case they checked before is
  in their snapshots again.
- [x] 8.3 The last entries of `known_parse_gaps.list` and `known_false_errors.list`, one by one, until both are
  empty. Anything that turns out not to fit is decided with the user, not left in the lists.
  *Done 2026-10-07 (session 23):* nothing was left after 7.5 (parse gaps), 8.1 and 8.2 (false errors); both lists
  are empty, also with `qb64pe.bas` and every clone set read (clone present).

## 9. Documentation and close

- [x] 9.1 `crates\README.md` (what parses, what compiles, the new dependency, the third list), `tests\upstream\
  README.md` (lists and counts, upstream progress), `DIVERGENCES.md` for any decided difference, `CLAUDE.md`
  layout (`verification\v16*`), `STATUS.md` (step 2 done, numbers).
  *Done 2026-10-07 (session 23):* all listed files updated, also `study\00` (§2 M2 row, §5 measured facts of
  `v19_*`), `ROADMAP.md` (done items; its numbers line left for the user's decision on `study\26` §7), the design
  ("As built" notes for D6, D9, D10), `tests\corpus\slice\SOURCE.md`. `DIVERGENCES.md` unchanged: no difference was
  decided. Open for the user: `CALL peek` and `FUNCTION poke` pass `qb64pe -z` but fail its C++ build, while the
  new compiler accepts them (not registered as a divergence until decided).
- [x] 9.2 Measure tier-1 time in debug and record it in `crates\README.md`; full corpus once with the release build
  (no crash, no wrong executable). Verify: `openspec validate m2-parser-breadth --strict` passes;
  `python tools\repo_check\check_repo.py --untracked` passes.
  *Done 2026-10-07 (session 23):* tier 1 9.5–9.9 s (`inputs.rs` 6.1 s). Release build against the QB64pe 4.7.0
  release: `slice.list` 131 of 131 and `pass.list` 34 of 34, both also with `QB64RUST_NO_FOLD=1`; full upstream run
  34 pass, 369 rejected with a diagnostic, 1 known failure, none wrong; full corpus 147 pass, 131 rejected with a
  diagnostic, 4 `.err` programs with only marked errors, 5 known failures, no crash, no wrong executable (270 s).
  `openspec validate m2-parser-breadth --strict` valid; repo check clean (3,608 files).
