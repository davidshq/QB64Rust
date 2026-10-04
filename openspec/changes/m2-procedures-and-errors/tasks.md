# Tasks

## 1. Grow the slice list (design D9, D10)

- [x] 1.1 Add the 17 corpus programs that already pass to `tests\corpus\slice.list` (27, 74, 75, 87, 92, 125, 130,
  132, 133, 144, 145, 163, 188, 208–211; `tests\corpus\README.md`). Change `crates\driver\tests\corpus.rs` to count
  the list's entries instead of asserting 14. Verify: `cargo test`; tier 2 with `--list tests\corpus\slice.list`
  passes 31 of 31.
- [x] 1.2 Tier-1 test: every corpus `.bas` with an `.err` file gets at least one error from the front end (D10).
  Verify: passes today (all 20 are rejected); temporarily deleting a parse error check makes it fail (checked once,
  then restored).

## 2. Measurements before code (D2, D3, D8)

- [x] 2.1 Measure with `qb64pe.exe` (programs and results in `verification\v14_*`, outputs recorded like the
  other `verification\` programs): `DIM SHARED` after a procedure; reserved names (`name`, `cls`, `print`, `len`,
  `left` / `left$`, a SUB named `chr`); `ON ERROR GOTO` inside a procedure; the compile errors of D8 (argument
  count, string for a number, `EXIT SUB` in main and in a FUNCTION, undefined and duplicate label, SUB used as a
  function, duplicate procedure); `ERROR` with 0, 256 and a float. Record the results in `design.md` (Context) and
  correct D2, D3, D8 where they disagree. Verify: no local paths in the recorded outputs.
- [x] 2.2 Write the five slice programs of D9 (`s08_byref` … `s12_error_in_print`) and update
  `tests\corpus\slice\SOURCE.md`. Each starts with `$CONSOLE:ONLY`, uses no `PRINT` comma, no control flow. Record
  them with the old compiler (`--suite corpus --category slice --record`). Compare each result with the scenarios
  of the two new specs; where they disagree, correct the **spec** and note it here. Verify: a second `--record`
  changes no file; `s12` shows the untrapped-error skip (pipeline scenario "Error inside a PRINT").
  Done: all results agree with the specs as corrected by task 2.1. Under `QB64PE_NOPROMPT=y` an untrapped error
  ends the program, so the corpus runner now reads a `.noprompt` sidecar (`s12` uses `continue`) and the pipeline
  scenario says so.

## 3. Procedures: front end (D1–D5, D11, D12)

- [x] 3.0 Typed accessors (D11): `crates\syntax\src\ast.rs` with a wrapper for each existing node kind; move
  `crates\sema\src\check.rs` to them (no `child_nodes().next().unwrap()` on statement or expression nodes left).
  Then the symbol table for what exists today (D12): main-module variables, definition and reference spans,
  `Symbols::at`, `dump_symbols`. Verify: `cargo test` with no snapshot changed; a `sema` snapshot of the symbol
  dump for the scenario "Definition and references of a variable"; a unit test of `Symbols::at` on a position
  inside, before and after a name.
- [x] 3.1 Parser: `ProcDef`/`ProcHeader`/`ParamList`/`Param`, `CallStmt` (with and without `CALL`), `ExitStmt`,
  `DeclareStmt`, `SharedStmt`, `StaticStmt`, the keyword list (`parser\keywords.rs`), block recovery of D1.
  Verify: parse snapshots for each form and each recovery case (nested SUB, missing `END SUB`, stray `END SUB`,
  wrong `END` kind); corpus round trip; a file that ends inside a SUB. Accessors (D11) for each new node kind,
  returning `None` for the children a recovery case leaves out (unit test per recovery case).
  Done. Also `ProcEnd` (the closing `END SUB`) and `DIM SHARED`. The keyword list is the old compiler's reserved
  words (33 of them checked as variable names with `qb64pe.exe`: all rejected). Without `CALL`, arguments that do
  not parse as expressions (`LOCATE , 5`) go into an `Error` node with no diagnostic, so built-in statements are
  reported by `sema` ("not supported yet") and a user SUB's by `sema` too (`s (1, 2)`).
- [x] 3.2 `sema`: procedure table (pass 1), scopes, parameters, result variable, calls in statements and
  expressions, argument passing of D4, reserved names (D3), `EXIT`, `DECLARE`, `STATIC`, `SHARED`, `DIM SHARED`,
  the errors of D8 as measured. Typed dump shows procedures, storage classes and each argument as `ref` or `temp`.
  Verify: `typed` tests for every argument form of D4 and every scope rule; `check-fail` tests for every error of
  D8; tier 1: programs 25, 47, 48, 122, 141, 190, 191 still rejected. Symbols (D12) for procedures, parameters,
  results, locals, `STATIC` and `SHARED` names; verify: symbol-dump snapshots for the scenarios "Same name,
  different symbols" and "Call before definition", and for a `SHARED` line that creates the main-module variable.
  Done (`tests\frontend\proc_*.bas`, `reserved_names*.bas`, `crates\sema\tests\symbols.rs`). Corrected on the
  way, measured: pass 2 checks in **file order** (D2); plain `SHARED g` is always the SINGLE `g!`, and `SHARED h
  AS LONG` types main's plain `h` for the code after it (`verification\v14_shared_plain_*`); a string variable in
  parentheses is still passed by reference (D4, `s08` output). Until task 4 lowers procedures, the driver stops a
  program that defines one with "code generation for SUB and FUNCTION is not supported yet" before lowering
  (`check_lowerable`; `--dump tree|typed` still work). The front end accepts all 12 procedure programs of 4.3
  (24 … 263). Fixed after review: reserved names measured for every keyword and built-in
  (`verification\v15_builtin_names.*`; names starting with `_` are never free, `WIDTH` is; checked by
  `crates\driver\tests\names.rs`), and a local `DIM` shadows a `DIM SHARED` name (`v14_dim_local_*`, D2). Also:
  there is no unary `+` (measured, `verification\v15_plus_*`: `PRINT +5`, `x = +n`, `2 * +n`, `+s$` are all
  compile errors; the slice parser had accepted it), and a procedure whose header has an error is entered as
  broken, so its calls add no second error (`tests\frontend\proc_header_errors.bas`).

## 4. Procedures: IR, emitter, end to end (D6, D7)

- [x] 4.1 IR: `Proc`, storage classes, `Body`, `Op::Call`, `Op::Exit`, `ValueKind::CallProc`, `Arg`. Verify:
  `ir` snapshot of a lowering-pair file with each argument form, a function in a PRINT, `EXIT`; `cargo tree -p
  qb64rust-ir` still has no `codegen-cpp`.
  Done (`tests\frontend\proc_lowering.bas`). Differences from D6's sketch: `Storage::Param(ProcId)` carries no
  index (the position is the index in `Proc::params`, as in `sema`); `Body` has only `stmts` until labels arrive
  in 5.2; `Proc` also carries the header and `END` lines for the `#line` directives (`sema::Proc` gained them
  too). Also `Op::System` (below).
- [x] 4.2 Emitter: `mainK`/`dataK`/`freeK`, prototypes, `main.txt` includes, `passN`, string parameter guard,
  result return, `STATIC` storage. Verify: `cpp` snapshot of the same file; compared by eye with the probe's
  fragments (Context) for the same constructs.
  Done (`tests\frontend\proc_lowering_cpp.bas`). Compared with `qb64pe.exe -z` output for `s08`–`s10` (scratch
  folder): prologue and epilogue verbatim, the same prototypes, `passN` placement (`maindata.txt` / `dataK.txt`),
  string guard and its `freeK` part, `qbs_maketmp` for a string result, `STATIC` variables in `global.txt` and
  `maindata.txt` with the procedure's prefix, `qbs_cleanup` after a call with string parameters. `Fragments`
  names are now `String`s (three files per procedure).
- [x] 4.3 Tier 2: add 24, 64, 69, 84, 85, 115, 124, 140, 192, 193, 196, 263, `s08`–`s10` to `slice.list`. Verify:
  the whole list passes, also with `QB64RUST_NO_FOLD=1`; `git -C ..\QB64pe status --porcelain` unchanged.
  Found in 3.2, not planned: `s08`–`s12` end with `SYSTEM`, which the subset does not have (the front end says
  "`SYSTEM` is not supported yet"); it is needed before they can pass. Remove `check_lowerable` (3.2) here.
  Done: 46 of 46 pass, also with folding off; clone unchanged; `check_lowerable` removed. `SYSTEM` without an
  exit code added end to end (`SystemStmt`, `StmtKind::System`, `Op::System`; emitted as the old compiler does:
  `if (sub_gl_called) error(271); close_program=1; end();`); `SYSTEM n` is "not supported yet" (parse snapshot
  `system_with_and_without_exit_code`).

## 5. Error handling (D1, D5–D7)

- [ ] 5.1 Parser and `sema`: `LabelDef`, `OnErrorStmt`, `ResumeStmt` (all four forms), `ErrorStmt`, `ERR`, `ERL`,
  `CHR$`; label table per body (main only), handler and resume targets resolved; `ON ERROR GOTO` inside a
  procedure resolves against main's labels, `RESUME label` inside a procedure and a label inside a procedure are
  errors (D8). Verify: parse snapshots;
  `check-fail` tests for the label errors of D8; `typed` test for `ERR`, `ERL`, `CHR$`; a symbol-dump snapshot
  with a label, its `ON ERROR GOTO` and `RESUME` references (D12).
- [ ] 5.2 IR and emitter: `SetHandler`, `Raise`, `Resume`, labels, `mainerr.txt` dispatch. Verify: `ir` and `cpp`
  snapshots of a lowering-pair file with each form.
- [ ] 5.3 Tier 2: add 03, 148, 228, 229, 230, 247, `s11`, `s12` to `slice.list`. Verify: the whole list passes,
  also with `QB64RUST_NO_FOLD=1`; clone unchanged.

## 6. Full corpus and documentation

- [ ] 6.1 Run the full corpus with `qb64rust` once (not a pass criterion). Verify: no crash, no wrong executable,
  every `.err` program rejected; record the counts in `tests\corpus\README.md`; any further program that now
  passes is added to `slice.list`.
- [ ] 6.2 Update `crates\README.md` (supported subset, `ast.rs` and the symbol table in the crate map), `tests\corpus\README.md` (slice group, list size),
  `STATUS.md`, `study\00` (§2 progress, §5 measured facts from the Context and task 2.1), `CLAUDE.md` (layout table:
  list size; decisions taken here). Verify: `openspec validate m2-procedures-and-errors --strict` passes.
