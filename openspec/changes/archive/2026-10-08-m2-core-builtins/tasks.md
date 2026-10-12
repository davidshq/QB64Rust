# Tasks

Order of the groups: 1 (the emitter split is a pure move, done before anything changes behaviour), 2 (measure
before designing), then 3 to 7 in order, then 8. Points marked "provisional until 2.1" in the design are corrected
in 2.1 before any code that depends on them. Each group regenerates the shrink-only lists
(`QB64RUST_UPDATE_LISTS=1 cargo test -p qb64rust-driver --test inputs`, entries may only go away) and adds the
programs it makes pass to `slice.list` / `pass.list`.

## 1. Emitter split (design D6)

- [x] 1.1 Split `crates\codegen-cpp\src\lib.rs` into modules by concern (names, declarations and allocation,
  procedures, operations, places, values), moving code only. Verify: every `cpp` snapshot unchanged
  (`cargo insta test` reports nothing), `cargo clippy --workspace --all-targets -- -D warnings` clean, tier 2
  (`slice.list` 131 of 131, `pass.list` 34 of 34) unchanged; `crates\README.md` names the modules.
  *Done 2026-10-07:* `lib.rs` (212 lines) keeps the entry points (`emit`, `dump`, `FRAGMENTS`) and the `Emitter`
  state; `names.rs`, `decl.rs`, `procs.rs`, `stmt.rs`, `place.rs`, `value.rs`, `builtins.rs` (the generic `call`,
  D5 adds to it) each add the emitter's methods of their concern. Code moved unchanged; only visibility widened to
  `pub(crate)`; `c_type`, `proc_name`, `var_name` re-exported. The unit tests moved with the name functions they
  test. No snapshot changed, clippy and fmt clean, `cargo test` green; tier 2 locally (reference clone): 131 of 131
  and 34 of 34.

## 2. Measurements before code (design D1)

- [x] 2.1 Write and run `verification\v20_*` with `qb64pe.exe` (`verification\run.sh`) for every question of D1.
  Record findings in `study\00` §5; correct D3, D4, D7, D8 and the spec deltas (`builtin-functions`,
  `control-flow`) where they disagree, and list the corrections here. Anything that measures as wrong code in the
  old compiler is put to the user before it is designed. Verify: `run.sh` output recorded for each program;
  `python tools\repo_check\check_repo.py --untracked` clean; no loop in a program runs without a cap.
  *Done 2026-10-07:* `v20_a_slots`, `v20_b_result_types`, `v20_c_string_edges`, `v20_d_val`, `v20_e_radix`,
  `v20_f_math_edges`, `v20_g_len`, `v20_h_select`, `v20_i_on_goto` and 30 one-question rejection programs
  `v20_x01`–`x30`, all run with `run.sh` (outputs recorded; the only loops are two `FOR`s with fixed bounds); types
  read also from the C++ (`qb64pe -z`) and libqb's signatures. Findings in `study\00` §5. **Corrections:**
  - D2: two more rules, `Exp` (EXP's own typing) and `Convert(Ty)` (`CINT`, `CLNG`, `CSNG`, `CDBL`, `_ROUND`);
    each call gets a held type (the C++ type of the libqb call) and a believed type (the old compiler's), as
    operators already do.
  - D3: `_FLOAT` slots are not converted (the argument is passed in its own type); the LONG slot is the store rule,
    confirmed (low 32 bits after rounding to `_INTEGER64`).
  - D4: `VAL` with an integer type is `_INTEGER64` for every integer type; the `HEX$` width 0 applies to a 64-bit
    argument that is not a place; `LEN` of a literal or a non-place expression is a compile error; `CSNG` of an
    integer held DOUBLE, believed SINGLE; `STRING$(n, s$)` noted.
  - D7: items are converted to the selector's type (an integer selector through `qbr_double_to_long`); a raising
    selector or item runs that `CASE`'s body; the `EVERYCASE` flag is per call in a procedure.
  - D8: `n` is converted by its own rule (`qbr_float_to_long` for a float, the low 32 bits of an integer), the label
    tests use `OnError::UseValue`, the negative check comes after them.
  - Spec `builtin-functions`: `LEFT$(s, -1)` raises nothing (an empty string); the example error list uses
    `CHR$(256)` instead; the "Float argument" scenario prints `ababcd` (it said `abcd`); new scenarios for a value
    beyond LONG and a negative length; `VAL` with an integer type and `CLNG` in the result types.
  - Spec `control-flow`: items converted to the selector's type (new scenario), a raising item (new scenario), the
    value of a raising `n`, a float `n` (new scenario).
  Put to the user (possible wrong code in the old compiler): six oddities; **all kept as measured** (user,
  2026-10-08; `DECISIONS.md`, `study\00` §6).
- [x] 2.2 Write the slice programs of D9 (`s25…`), update `tests\corpus\slice\SOURCE.md`, record them with the old
  compiler (`--suite corpus --category slice --record`). Compare each result with the scenarios of the spec deltas;
  a scenario that disagrees is corrected (and listed here). Include the Q-001 (`ON 258 GOTO`) and Q-002 (recursion
  across a `SELECT`) cases. Verify: `run_legacy_tests.py --suite corpus --category slice` with `qb64pe.exe` passes
  every slice program.
  *Done 2026-10-08:* five programs, split by subject: `s25_string_builtins`, `s26_val_len_radix`,
  `s27_math_builtins`, `s28_select_case` (Q-002: `rec(1) = 2`), `s29_on_goto` (Q-001: `ON 258 GOTO` continues).
  Recorded with `--record` (two equal runs each); nothing depends on the old compiler's undefined behaviour. Every
  scenario of the spec deltas agrees with the recorded output, as corrected in 2.1; no further correction. Found on
  the way: an `_INTEGER64` selector is not truncated (`CASE 2` does not match 4294967298). The slice category with
  `qb64pe.exe`: 29 of 29.

## 3. Table-driven built-ins in `sema` (D2, D3)

- [x] 3.1 `sema\src\builtins.rs`: the supported list `(name, Rule)`, the checker (arity from the table and the rule,
  argument kinds, slot conversion per slot type as measured, result type by rule), `size_of(Ty)` for `LEN`. Move
  `INSTR`, `CHR$` onto it and give `LBOUND`/`UBOUND` their row. Verify: every existing `typed`, `ir` and `cpp`
  snapshot unchanged; unit tests for the slot conversions (one per slot type, from 2.1) and for the arity check.
  *Done 2026-10-08:* `sema\src\builtins.rs` (public: `Rule`, `SUPPORTED`, `lookup`, `supported`, `rule`,
  `slot_count`, `radix_width`; the held and believed result types per rule) and `sema\src\check\builtins.rs` (the
  checker, which needs the private `Checker`: arity, kinds, slot conversion, `LEN`, `VAL`); `size_of` in
  `sema\src\lib.rs`. Rules as corrected in 2.1, plus two the C++ showed: `IntFix` (`INT`/`FIX` take the argument
  uncast, `ABS` casts it: `ResultOfArg`) and `StringFill` (`STRING$`'s second slot takes a string too). Rows now:
  `INSTR`, `CHR$` (`Plain`), `LBOUND`, `UBOUND` (`Fixed(_INTEGER64)`, their own node, arity and type from the row),
  `ERR`, `ERL` (`Fixed`; the bare-name rule replaces their special case). `ir::validate` counts slots by
  `slot_count`. No `typed`, `ir` or `cpp` snapshot changed; one `check-fail` message did (`ERR(1)`: "`ERR` with
  arguments"). Tests: `crates\sema\tests\builtins.rs` (LONG and STRING slots, optional slots, arity, kinds, bare
  names) and unit tests in `builtins.rs` (overloads, `HEX$` widths, conversion held types); the DOUBLE, `_FLOAT`
  and any-numeric slots get theirs with their first functions in 4.1 and 5.1.
- [x] 3.2 The coverage check of the testing delta in `crates\driver\tests\inputs.rs`: each supported built-in must be
  called in a `slice.list` program and in a `typed` front-end test. Verify: it passes for `INSTR`, `CHR$`, `LBOUND`,
  `UBOUND`, `ERR`, `ERL`; removing a name from its test makes it fail and name the built-in (tried by hand, not
  committed).
  *Done 2026-10-08:* test `every_supported_builtin_is_covered`: a word match (case-insensitive, with the suffix; a
  bare name does not match one with a suffix, `ERR` not `ERROR` or `ERR%`) on the code of each program, comments,
  `REM` lines and string literals left out (`code_only`, tested by `coverage_words`). Passes for the six. Tried by
  hand: `ERL` renamed in `error_handling_typed.bas` (its only `typed` test) fails with "`ERL` is called in no
  `typed` test of tests/frontend"; restored.

## 4. String built-ins (D4, D5)

- [x] 4.1 `Plain` string functions: `LEFT$`, `RIGHT$`, `MID$`, `SPACE$`, `STRING$`, `LTRIM$`, `RTRIM$`, `_TRIM$`,
  `UCASE$`, `LCASE$`, `STR$`, `_TOSTR$`, with their emission. Verify: `typed` and `cpp` tests per group,
  `check-fail` for the measured rejections; slice program `s25…` passes in tier 2; the coverage check passes.
  *Done 2026-10-08 (with 4.2, which `s25` also needs):* rows added (`STRING$` is `StringFill`); the emitter's
  `builtins.rs` writes every rule at once (D5), `table_call` casting an any-numeric argument to its C type
  (`qbs_str(((int16)(5)))`: an INTEGER literal is a C `int`, and libqb's overloads differ, e.g. `func_abs`).
  `Slot` and `slots` are public, re-exported with the rest of `sema::builtins` by `ir`. A supported function used
  as a statement is an error (`v20_x06`). Tests `builtins_strings_typed`, `builtins_strings_cpp`, `builtins_errors`
  (check-fail). Twelve corpus programs and the upstream `func_tostr/mmLiteral`, `mmVariable` compiled cleanly and
  pass in tier 2; added to `slice.list` and `pass.list`.
- [x] 4.2 `LEN`, `ASC` (one and two arguments), `VAL` (with and without a type), `HEX$`, `OCT$`, `_BIN$` by their
  rules. Verify: `typed` tests per rule (including `LEN` of each place kind and the radix widths), `cpp` tests per
  emission rule, `check-fail` for the measured rejections; the string slice programs pass in tier 2; the upstream
  `val_*` and `func_tostr` programs that now compile are run in tier 2 and added to `pass.list` or, with the reason,
  to `tests\known_clean_not_passing.list`.
  *Done 2026-10-08:* rows `LEN` (`Len`: a place's size is a constant from `sema::size_of`, which the emitter now
  uses too, replacing its own size table), `ASC`, `VAL`, `HEX$`/`OCT$`/`_BIN$` (`Radix`). Tests
  `builtins_len_val_typed`, `builtins_len_val_cpp` (the C++ matches the old compiler's call for call), more
  `check-fail` cases (`LEN(5)`, `LEN(i + 1)`, `LEN()`, `ASC` with three arguments, `HEX$("a")`, `VAL(5)`,
  `VAL(…, STRING)`). `LEN("a")` alone is marked, not an error: the parser reads `LEN` as a statement word (it
  starts `LEN =` in `OPEN`); the program is rejected either way. Tier 2: `s25`, 18 corpus programs, `v11_wrap_o2`,
  `v12_wrap_int64` and the six upstream `basic/val_*` and `console_only/print_console_null_char` pass; all added
  to the lists. (8.3: two of the `val_*` fail against the 4.7.0 release and moved to
  `known_clean_not_passing.list`.)

## 5. Math built-ins (D4, D5)

- [x] 5.1 `ABS`, `INT`, `FIX` (`ResultOfArg`), `SIN`, `COS`, `TAN`, `ATN`, `SQR`, `LOG`, `EXP` (`FloatByArg`), `SGN`,
  `CINT`, `CLNG`, `CSNG`, `CDBL`, `_ROUND`, `_ATAN2`, `_HYPOT`, `_PI` (bare and with an argument). Verify: `typed`
  tests per rule and per argument type, `cpp` tests per emission rule, `check-fail` for the measured rejections; the
  math slice program passes in tier 2 (the error numbers of D1's edge values included); the coverage check passes.
  *Done 2026-10-08:* rows as listed (`ABS` `ResultOfArg`, `INT`/`FIX` `IntFix`, `EXP` `Exp`, the five conversions
  `Convert`, `SGN`/`_ATAN2`/`_HYPOT`/`_PI` `Plain`). Tests `builtins_math_typed`, `builtins_math_cpp` (every libqb
  entry as the old compiler picks it; `ABS(-32768%)` through `func_abs((int16)…)`), more `check-fail` cases; in
  `crates\sema\tests\builtins.rs` the DOUBLE, `_FLOAT` and any-numeric slot tests left from 3.1 and the held and
  believed result types. `tests\frontend\unsupported.bas` used `LEN` and `SQR` as unsupported examples: now `RND`
  and `_MIN`. Tier 2: `s26`, `s27` (every error number and placeholder of D1's edge values), 16 corpus programs and
  upstream `print/auto_semicolon_insertion` pass; added to the lists. `known_unsupported_rejections.list` lost one
  entry (`snippets/qb64fresh/error_detection__function_wrong_args`: now a real arity error). The coverage check
  passes for all 43.

## 6. `SELECT CASE` (D7)

- [x] 6.1 `sema`: `SelectBlock` checked (selector kind, item kinds, `IS` operators, `TO`, `CASE ELSE`, `EVERYCASE`,
  the variable-selector rule as measured); the typed tree gains a `Select` statement. Verify: `typed` test,
  `check-fail` for a mismatched item and the other measured errors; the pipeline delta's new "Statement not compiled
  yet" scenario (`SWAP` in a `DO`) replaces the `SELECT CASE` one in `tests\frontend\blocks_not_compiled_yet.bas`.
  *Done 2026-10-08:* `StmtKind::Select { selector, copied, every, cases, else_, end_line }` with `Case` and
  `CaseItem` (`Is(op, value)`, `Range`). The rules are the old compiler's (`qb64pe.bas` 6790–7209, read after
  2.1): a plain variable is read at each test, any other selector converted to its copy type (LONG for an integer
  of 32 bits or fewer, `_INTEGER64`, the believed float type, a string); each item converted to the selector's type
  (a float item for an integer selector through the store rule to LONG, or rounded to `_INTEGER64` for an
  `_INTEGER64` selector), both sides in C's common type. Tests `select_typed`, `select_errors` (check-fail; the
  statements inside are still checked); `blocks_not_compiled_yet` now uses `SWAP` in a `DO`, with a parser
  snapshot of that tree (`swap_in_a_do`); `blocks_marked` and `blocks_recovery` lost their `SELECT CASE` marks.
  `EXIT SELECT`/`EXIT CASE` (which the parser reads) stay marked: not in this change.
- [x] 6.2 Lowering to `Branch`/`Jump` with the static selector copy (Q-002) and the `EVERYCASE` flag; emission.
  Verify: `ir` snapshot shows the static hidden variable in a procedure and the branch chain; `cpp` snapshot;
  `ir::validate` clean on every accepted input; the `SELECT` slice programs (Q-002 included) and the six corpus
  `SELECT` programs pass in tier 2.
  *Done 2026-10-08:* `Lowerer::select` as design D7, with one change: the copy is a global variable in the main
  module (not a `Temp`, which cannot hold a string) and a `STATIC` one in a procedure, named `<n>SELECT` (a leading
  digit, so no BASIC name meets it); the `EVERYCASE` flag is a `Temp` (per call in a procedure, as measured). Items
  joined by `_ORELSE`, ranges by `_ANDALSO` (the old `||`/`&&`). No emitter change was needed. Tests `select_ir`,
  `select_cpp`; every accepted input lowers and validates. Tier 2: `s28` (Q-002 pinned) and the six corpus
  programs pass; added to `slice.list`.

## 7. `ON … GOTO` and `ON … GOSUB` (D8)

- [x] 7.1 `sema`, lowering and emission of `OnJumpStmt` with label targets (line-number targets stay marked).
  Verify: `typed`, `ir` and `cpp` tests; `check-fail` for a string `n` and a label of another body; the slice program
  (n = 0, count + 1, 258 for Q-001, -1) and corpus `226_on_goto`, `227_on_gosub` pass in tier 2.
  *Done 2026-10-08:* `OnJumpStmt` accessors (`value`, `keyword`, `targets`), `StmtKind::OnJump`, `Lowerer::on_jump`
  as D8 corrected in 2.1. The negative check is two statements of their own after the tests (`Branch(n >= 0 →
  end)`, `Raise 5`), so the fall-through never jumps past the `ON` statement's event check: an error in `n` is
  serviced there, as in the old compiler. `ON … GOSUB` is one statement per target (skip, `Gosub`, jump to the
  end). An omitted target (`ON n GOTO a, , b`) is marked (not measured). No emitter change. Tests
  `on_goto_typed`, `on_goto_ir`, `on_goto_cpp`, `on_goto_errors` (a string `n`, a label of another body, a line
  number). Tier 2: `s29` (Q-001 pinned), `226_on_goto`, `227_on_gosub` pass; added to `slice.list`.

## 8. Lists, progress and close

- [x] 8.1 Full corpus and upstream once (tier 3 locally, release build): no crash, no wrong executable; numbers
  recorded in `tests\corpus\README.md`, `tests\upstream\README.md` and `STATUS.md`; every program of the census in
  `proposal.md` that still fails is listed with its reason in this task's record.
  *Done 2026-10-08:* release build against the reference clone. Corpus (292 programs): 208 pass (192 of
  `slice.list`, 16 `.err`), 75 rejected with a diagnostic, 4 `.err` programs with only marks, 5 known failures;
  no crash, no internal compiler error, no wrong executable. Upstream (404 programs, 64 s): 44 pass, exactly
  `pass.list` as it stood then; none wrong. (8.3 took two of them out again: **42 of 279**, from 34.) The census was counted without names (58 corpus, 11 upstream),
  so it cannot be matched one by one: 61 corpus programs joined (54 of them `runtime_comparison`, plus `s25`–`s29`,
  `v11`, `v12`) and 10 upstream. Every corpus program still rejected that uses a construct of the change is
  blocked by something outside it (checked with `--dump typed`): file statements (`176_open_random`,
  `177_open_binary`, `179_file_lock`, `213_chdir`, `214_mkdir_rmdir`, `216_kill_nonexistent`,
  `219_fileexists_direxists`), `DATA`/`READ` (`220_read_type_mismatch`, `223_read_past_eof`), `ENVIRON$`
  (`212_environ_dollar`), `COMMAND$` (`217_command_dollar`), `_CEIL` (`77_ceil_round`), `EXIT SELECT`
  (`151_exit_select`), the `MID$` statement (`42_mid_assign`); `16_ltrim_rtrim` is an `.err` program (the old
  compiler rejects its `TRIM$`) that gets only a mark.
- [x] 8.2 Documentation: `crates\README.md` (what compiles, the built-in rules), `study\00` §5 (done in 2.1) and §9,
  `DIVERGENCES-QB45.md` Q-001 and Q-002 "Pinned by" filled with the slice program names, `DIVERGENCES.md` and
  `DECISIONS.md` if 2.1 led to a decision. Verify: `python tools\repo_check\check_repo.py --untracked` clean.
  *Done 2026-10-08:* `crates\README.md` (the 43 functions, the three files of the checker and the emitter, the
  rule model, `SELECT`/`ON`; what is still marked), `study\00` §9 (the table's facts found wrong and how `sema`
  uses the table), §6 (the six kept oddities), Q-001 pinned by `s29_on_goto`, Q-002 by `s28_select_case`.
  `DIVERGENCES.md` unchanged (nothing diverges from QB64pe); `DECISIONS.md` has the 2026-10-08 row. Also updated:
  `CLAUDE.md` layout (corpus 292, `slice.list` 192, `verification\v20*`), `STATUS.md`. The repo check
  (`--untracked`) is clean.
- [x] 8.3 CI: the `tier2` job's steps run locally against the QB64pe 4.7.0 release, all pass; tier-1 time recorded
  (budget a minute). Archive the change (`openspec archive`), specs merged; `STATUS.md` "Next" moves to step 7.
  *CI steps done 2026-10-08:* the job's two steps run locally with the release build (working tree on `a84d589`)
  against the `v4.7.0-GLFW` release, downloaded as the job does: corpus `slice.list` **192 of 192** (396 s),
  upstream `pass.list` **42 of 42** (62 s). First run: upstream 42 of 44: `basic/val_default_large_integer_decimal`
  and `val_typed_large_integer_decimal` print other digits against the release, and the release's own
  `qb64pe.exe` fails them too (they expect the `VAL` of libqb after 4.7.0, which the reference clone has); taken
  out of `pass.list` (added this session, never in CI) into `tests\known_clean_not_passing.list` with that reason.
  Tier 1: about 10 s (budget a minute). Archived 2026-10-08 at the user's word, specs merged by `openspec archive`.
