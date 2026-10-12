# Tasks

Order of the groups: 1, 2, 3, 5, 6, 7, then 4, then 8 and 9. Constants and `OPTION` (group 4) come after the IR
and the emitter have met jumps (design, Risks); `s18_const` and the `CONST` programs join the lists in 8.1.

## 1. Measurements before code (design D1)

- [x] 1.1 Write and run `verification\v17_*` with `qb64pe.exe` (`verification\run.sh`) for every question of D1:
  the pending-error rule (a store and a call made after a raising operation, placeholder values), `ELSEIF` and
  other header errors with `RESUME` and `RESUME NEXT`, jumps into and out of blocks, `NEXT`
  variables, `GOSUB`/`RETURN` across bodies, labels per body, `CONST` visibility and typing, `OPTION _EXPLICIT`
  placement and what counts as a declaration, operator corners. Also rerun the three probes of the Context as
  `v17_probe_*` so the table has a home in the repo. Record findings in `study\00` §5; correct D3–D10 and the spec
  deltas where they disagree, and list the corrections here. Verify: `run.sh` output recorded for each program;
  `python tools\repo_check\check_repo.py --untracked` clean (no local paths in the outputs); no loop in a program
  runs without a cap.
  *Done 2026-10-06:* 110 programs (`v17_a`–`v17_g`, `v17_probe_*`; `v17_e_const_int_float` added while writing
  up, to settle constant typing), and 5 more after review (`v17_c_next_suffix_single`, `v17_b_if_then_line`,
  `v17_b_if_then_label`, `v17_e_const_before_numsuffix*`) for claims that had no program. Findings in `study\00`
  §5, new items in §6. Corrections, each marked "Measured (1.1)" in the design:
  - D8, the pipeline and the error-handling deltas: a store with a placeholder happens (`x` is 0), but **a call does not do its work**:
    a procedure's entry is a check point (returns at once while an error is pending). `ELSEIF` uses the
    placeholder, and when it is false the error is serviced at the **next** statement that runs, which `RESUME`
    then re-runs (new scenarios "A SUB called with a raising argument", "ELSEIF condition raises"). `IF c GOTO x`
    jumps when `c` raises, so it lowers as a `Branch` and a separate `Jump`.
  - D8 and the control-flow delta: `_BYTE` → INTEGER temporaries, start/limit/step rounded to the temporary's type,
    the step's sign taken once; `_INTEGER64` loops wrap. **One `GOSUB` stack for the whole program** (a SUB's
    `RETURN` consumes main's entry); `RETURN label` in a procedure is a compile error. `RETURN label` with nothing
    pending crashes the old program on the next `GOSUB`: the emitter guards the decrement (decided by the user
    2026-10-06, `DIVERGENCES.md` D-003).
  - D6 and the constants delta: **a name used before its main `CONST` line is an error**, with any suffix (was: means what it would
    without the constant); procedure constants may shadow main ones; `CONST` twice with the same value is fine;
    integer-valued results are `_INTEGER64`, other floats DOUBLE; the evaluator's errors and the "not supported
    yet" corners listed.
  - D7 and the constants delta: **`OPTION _EXPLICIT` is program-wide** wherever it stands (was: for the rest of
    the file); a `FOR` variable is not a declaration; `OPTION EXPLICIT` is an error.
  - D3, D4, D5, D10 and the numeric-semantics delta: `FOR` variable and string-condition errors; a `TYPE` member
    as `FOR` variable "not supported yet"; float operands of `_ANDALSO`/`_ORELSE`/`_NEGATE` rounded; chained `IMP`
    "not supported yet"; the smallest integer `\ -1` crash left unspecified (listed under "Fix").
- [x] 1.2 Write the slice programs of D11 (`s13_if` … `s19_header_errors`), update `tests\corpus\slice\SOURCE.md`,
  record them with the old compiler (`--suite corpus --category slice --record`). Compare each result with the
  scenarios of the five spec deltas; correct the **spec** where they disagree and note it here. Verify: a second
  `--record` changes no file; every scenario of `language/control-flow`, `language/constants` and the new ones of
  `language/numeric-semantics` and `language/error-handling` is covered by a slice program or a planned frontend
  test (list which).
  *Done 2026-10-06:* seven programs recorded (`s19` with a `.noprompt` of `continue` for its untrapped part); a
  second `--record` changed no file. Every result agrees with the spec scenarios: **no spec correction**. They
  raise errors only with `CHR$` (also inside `INSTR`), the built-ins the compiler has; the `ASC`/`LEN` forms stay
  in `verification\v17_*`. Found on the way: `INSTR(s, "")` is 1 (so a failed `CHR$` inside `INSTR` gives 1, not
  0); a nested single-line `IF`'s `ELSE` binds to the inner `IF`; `DO UNTIL` with a raising condition enters the
  body, as D8 predicts; a `FOR` header error leaves the variable at its old value also when untrapped. With
  `--cpp-opt` only `s17` differs (two LONG products, the known missing `-fwrapv`; `tests\corpus\README.md`).
  `s18_const` is a parse gap until 4.1 (`tests\known_parse_gaps.list`, allowed as a new input). `_BYTE` loops are
  not in `s15` (no `_BYTE` in the slice types; `v17_probe_for` has them). The programs join `slice.list` as they
  pass (3.2, 4.2, 7.1–7.3). Coverage:
  - `language/control-flow`: `s13` ELSEIF chain, Single-line IF with ELSE; `s14` LOOP UNTIL runs at least once;
    `s15` Limits rounded to the wider type, _INTEGER64 variable at its maximum, Limits evaluated once, Body changes
    the variable, Loop that does not run, INTEGER variable passes its range, SINGLE steps, EXIT FOR from inside an
    IF; `s16` Jump into an IF body, RETURN in a SUB during a main GOSUB, GOSUB inside a SUB; frontend `check-fail`
    Wrong order (5.2), GOTO from a SUB to a main-module label (5.1).
  - `language/constants`: `s18` Constant used in a later SUB, Procedure constant shadows a main constant, CONST
    inside a skipped block, Integer-valued constant is 64-bit, Right-associative power, Suffix rounds; frontend
    `check-fail` Name used before its CONST line, Assignment to a constant (4.2), OPTION inside a SUB, Undeclared
    variable (4.3); `check-ok` Declared variable and constant (4.3).
  - `language/numeric-semantics` (new): all seven in `s17`.
  - `language/error-handling` (new): `s19` IF condition raises, LOOP UNTIL condition raises, FOR header raises,
    WHILE condition raises; `s16` RETURN without GOSUB inside a SUB (case added after the first recording).
  - `compiler/pipeline` (new, for 7.3): `s19` ELSEIF condition raises, FOR limits computed with a placeholder
    value, A SUB called with a raising argument and A store made with a placeholder value (both with `CHR$`; the
    `ASC` forms of the scenarios are `v17_a_pending`); `ir` snapshots FOR loop lowered (6.2), Header error follows
    from the pending-error rule (7.3). D-003 (`RETURN label` with nothing pending) cannot be a slice program, the
    old program crashes: a frontend or CLI test in 7.2.
- [x] 1.3 `m2-parser-breadth`: take `CONST` and `OPTION` out of its task 7.2 (pointing here), and change the
  example of its pipeline delta's scenario "Statement not compiled yet" from `FOR` to `SELECT CASE`. Verify:
  `openspec validate m2-parser-breadth` and `openspec validate m2-control-flow-slice` pass.
  Done 2026-10-06: also its design D5's `decl.rs` list. Both validate; the new example
  (`SELECT CASE x: CASE 1: PRINT x: END SELECT`) gives a `SelectBlock` with a `CaseClause` holding the `PRINT`,
  and one "not supported yet" error at `SELECT`, checked with `qb64rust -z` and `--dump tree`.

## 2. `sema` split (D3)

- [x] 2.1 Split `crates\sema\src\check.rs` by family into `check\` modules (expressions, declarations,
  procedures, flow and error handling, blocks), moving code only. Verify: `cargo test` with no snapshot changed;
  `cargo clippy --workspace --all-targets -- -D warnings` clean; no module over about 600 lines.
  Done 2026-10-06: `mod.rs` 437 lines (entry points, `Skips`, `Scope`, `Checker`, shared helpers, the statement
  dispatcher, metacommands, `assign`, `print`), `expr.rs` 417 (expressions, `store`, conversions, folding helpers),
  `decl.rs` 308 (names, reserved names, variables by scope, `DIM`/`STATIC`/`SHARED`), `proc.rs` 252 (pass 1,
  calls, arguments, `EXIT SUB`/`FUNCTION`), `flow.rs` 159 (labels, `ON ERROR`, `RESUME`, `ERROR`), `blocks.rs` 99
  (`block`, `block_parts`). Moved by a line-range script that checked every source line arrives once; only
  imports, `pub(super)` where another module calls an item, and one misplaced doc comment
  (`first_token_span`'s) changed. `cargo test` passes with no snapshot changed, clippy and `cargo fmt --check`
  clean.

## 3. Operators (D4)

- [x] 3.1 `sema`: comparisons (numeric with the float-narrowing rule, strings as `StrCompare`), `NOT`, `AND`, `OR`,
  `XOR`, `EQV`, `IMP`, `_ANDALSO`, `_ORELSE`, `_NEGATE`, `\`, `MOD`, `^`, with the types and conversions of D4 and
  `UnOp` replacing `Neg`; folding for comparisons and logic on integers, never for `\`/`MOD` by 0 or `^`. The
  typing rules are one function in `check\expr.rs` (D4), the only place that matches on `Ty` to type an operator.
  Verify:
  `typed` frontend tests for each operator family (types and conversion nodes shown); unit tests of folding,
  including wrap at the 32- and 64-bit limits; `check-fail` for a string compared with a number.
  Done 2026-10-06, together with 3.2 (the IR reuses `sema`'s `BinOp`, and the emitter matches every variant, so
  neither can lag behind). As built: the typing function `op_typing` and the folding live in their own module
  `check\ops.rs` (pure functions, no `Checker`), not in `check\expr.rs`, which would otherwise have passed 2.1's
  600 lines; `expr.rs` calls them. `\` and `MOD` also stay unfolded for the smallest value of the type by -1
  (unspecified, the old program crashes), so a folded and an unfolded build agree. Tests: `typed`
  `operators_compare`, `operators_logic`, `operators_div_mod_pow`; `check-fail` `operators_errors` (string against
  number, string operands, `NOT`/`_NEGATE`/`_ANDALSO` of a string, the `IMP` chain with and without parentheses,
  and `5 IMP (3 IMP 0)` accepted); 7 unit tests in `ops.rs` (folding at the 32- and 64-bit limits, `\`/`MOD`
  by 0 and `MIN \ -1` not folded, the typing of each family, strings). Two snapshots changed as expected:
  `unsupported` (line 5 now stops at `LEN`; line 8 got `SQR(2)` so the `IF` line still shows an inner error) and
  `member_access` (`s (5 / 2) = 2, 0` is now a valid call, as in upstream `t659_sub_call_comparisons`).
- [x] 3.2 IR and emitter for the operators (`qb_safe_idiv`, `qb_safe_mod`, `pow2`, the string comparison calls,
  `-(a==b)` forms, `&&`/`||` for the short-circuit pair). Verify: `cpp` snapshot per family; tier 2 `s17_operators`
  passes, also with `QB64RUST_NO_FOLD=1`; `crates\README.md` lists the operators as supported.
  Done 2026-10-06: `ValueKind::Unary` replaces `Neg`, `ValueKind::StrCompare` added; `\`, `MOD` and `^` make a
  statement `may_raise` (pinned by the `ir` test `operators_ir`); the IR dump marks only `+ - *` as wrapping. `cpp`
  snapshot `operators_cpp` (one line per family). **Bug found and fixed:** `/` was emitted as `(a/b)`, so `*__A/*__B` (two variables) opened a C comment
  and the C++ did not compile (the program was rejected, no wrong code); `/` is now followed by a space, as in the
  old compiler (`a/ b`; `lowering_pairs_cpp` snapshot changed by that space). Tier 2: `s17_operators` passes with
  and without folding. The full corpus run then passed 15 more `runtime_comparison` programs (comparisons, logic,
  `\`, `MOD`, `^`), so `slice.list` grew from 54 to 70 (`s17` and those 15), all passing with and without folding;
  no corpus program fails at run time. Upstream: `noprompt/noprompt-continue-fatal` (`1 \ 0` is fatal) passes,
  **11 of 279**.

## 4. Constants and `OPTION _EXPLICIT` (D2, D6, D7)

- [x] 4.1 Parser: `ConstStmt`/`ConstItem`, `OptionStmt`, accessors. Verify: parse snapshots for each form
  (several items, suffixes, `OPTION BASE 1`, `OPTION _EXPLICITARRAY`); `QB64RUST_UPDATE_LISTS=1 cargo test -p
  qb64rust-driver --test inputs` removes entries from `tests\known_parse_gaps.list` and adds none.
  *Done 2026-10-06:* `parser\decl.rs` (`const_stmt`, `option_stmt`), accessors `ConstStmt::items`,
  `ConstItem::name`/`value`, `OptionStmt::word`/`base`; `sema` marks both statements "not supported yet" until 4.2
  and 4.3. Snapshots `const_forms`, `const_errors`, `option_forms`, `option_errors` (`crates\syntax\tests\parser.rs`);
  three older tests that used `CONST` as their unsupported statement now use `REDIM`. `OPTION` is not reserved, so
  it is a statement only when a word follows (`option = 3` stays an assignment); the spellings `EXPLICIT` and
  `EXPLICITARRAY` parse, and `sema` rejects them (no `$NOPREFIX`). `ROOT`, an operator of the old constant
  evaluator only (`study\02` §7), is "not supported yet" inside a `CONST` value (it was the one new false error,
  `upstream/const/math_funcs`). Lists: parse gaps 527 to 500 (28 entries gone, none new; the others' first gap moved
  on), only-marked rejections 81 to 77, false errors unchanged at 59. Four `const/` `.err` programs now pass tier
  2: upstream **15 of 279** (`tests\upstream\README.md`).
- [x] 4.2 `sema\consteval.rs` and constant scopes (D6): evaluation, typing as measured in 1.1, visibility from
  the line on, procedure constants, uses as literal nodes, assignment to a constant an error, functions "not
  supported yet". Verify: unit tests of the evaluator (each operator, `^` associativity, suffix rounding, overflow);
  `check-fail` tests for the D10 constant errors; `s18_const` passes in tier 2; the symbol table records constants
  as their own kind (snapshot).
  *Done 2026-10-06:* `sema\src\consteval.rs` (arithmetic on values, 7 unit tests) and `sema\src\check\constants.rs`
  (walk, scopes, uses); `Program::consts`, `SymbolKind::Const`. Decisions in D6 "As built": floats in `f64` with an
  exactness check against the old `_FLOAT` path, else "not supported yet"; **corrected:** a float beyond
  `_INTEGER64` range is DOUBLE (measured `1E+19 / 1`, `1E+30`), only an integer power beyond it is "not supported
  yet" (spec delta updated, scenario "Float beyond _INTEGER64" added); an unknown name in a value is "not
  supported yet" (may be a `$COLOR` constant), a variable an error. Tests: `check-fail` `const_errors` (every D10
  constant error) and `const_unsupported`, `typed` `const_uses`, symbol snapshot `constants_are_their_own_kind`;
  `unsupported.bas` now uses `OPTION BASE 1` as its unsupported statement. **`s18_const` cannot run in tier 2 yet**:
  it needs `IF` and `FOR` (groups 5–7), so it joins in 8.1 as the header says; every line of it without a block
  was compiled and its output matches the recording exactly. Tier 2: `slice.list` 70 to **75** (`26_const`,
  `104_const_expr`, `207_const_string`, `241_const_simple`, `256_const_use`; also with folding off); full corpus 89
  pass, none wrong at run time; upstream **20 of 279** (`const/comma`, `const/const_sub`, `const/not_string`, the
  two `const/type_mismatch_string_*`), none wrong. Lists: false errors lose 4 entries (two others now fail later),
  only-marked rejections lose 3; nothing added.
- [x] 4.3 `OPTION _EXPLICIT` and `_EXPLICITARRAY` (D7), placement as measured. Verify: `check-ok` and `check-fail`
  tests for each declaration kind of 1.1; the scenarios of "OPTION _EXPLICIT" as frontend tests.
  *Done 2026-10-07:* pre-pass `has_option_explicit` (`check\mod.rs`), `option_stmt` and `undeclared`
  (`check\decl.rs`); details in D7 "As built". Three more measurements: `v17_f_explicit_shared_before_dim` and
  `v17_f_explicit_shared_other_type` (both "not defined": `SHARED` needs the main variable of that type declared
  earlier in the file; scenario added to the spec delta), `v17_f_explicit_const_in_sub` (a SUB's own `CONST`, a
  main `CONST` in a SUB, a FUNCTION read by its name: accepted). Tests: `check-ok` `explicit_declared` (every
  declaration kind, OPTION after a statement, twice, after a colon, with `_EXPLICITARRAY`; `x&` after `DIM x AS
  LONG`) and `explicit_array_only`; `check-fail` `explicit_undeclared` (every error of 1.1 and the two new ones,
  `OPTION EXPLICIT`), `explicit_in_sub`, `explicit_in_block`, `explicit_after_unsupported` (the follow-on rule,
  marks by the parser and by `sema`); `unsupported.bas` now shows `OPTION BASE` marked on its word. The `FOR`
  variable waits for 5.2 (added there). Lists: false errors lose 12 entries (the follow-on rule turns the
  first errors of `$INCLUDE`/`TYPE` programs into marks), nothing added. Tier 2: `slice.list` 75 of 75; upstream
  unchanged at **20 of 279** (the same 20), no program built that gives wrong output.

## 5. Blocks in `sema` (D3, D5)

- [x] 5.1 Labels per body (D5): pre-pass per body, labels inside blocks and procedures, jumps checked against the
  body, `ON ERROR GOTO` only to main labels. `GOTO`, `GOSUB`, `RETURN`, `RETURN label` typed. Verify: `check-fail`
  tests (duplicate label in one body, jump to another body's label, `ON ERROR GOTO` to a SUB label); `check-ok`
  for the same name in two bodies; symbol-table snapshot with a label in a SUB.
  Done 2026-10-07 (D5 "As built"): `tests\frontend\labels_errors.bas`, `labels_bodies.bas` (typed, stronger than
  `check-ok`), `labels_ir.bas`, `labels_in_blocks.bas`; symbol test `label_in_a_sub`; CLI test
  `jumps_wait_for_the_ir`. Four more measurements (`verification\v17_d_on_error_main_to_sub_label`,
  `v17_d_on_error_sub_label_both`, `v17_d_resume_sub_label`, `v17_d_resume_main_label_from_sub_both`), written into
  the error-handling delta. Lists unchanged; tier 2 75 of 75 and 20 of 20.
- [x] 5.2 `IF` (both forms), `FOR`, `DO`, `WHILE`, `EXIT FOR/DO/WHILE` as typed block statements; `NEXT`
  variables compared (D3); `block_parts` reduced to the blocks still marked. Verify: `typed` snapshots per block
  kind; `check-fail` for a string condition, a wrong `NEXT` variable and order, a non-scalar `FOR` variable, an
  undeclared `FOR` variable under `OPTION _EXPLICIT` (`v17_f_explicit_for`, left from 4.3);
  regenerated lists lose entries and gain none; the tier-1 `.err` test still passes.
  Done 2026-10-07 (D3 "As built"): `tests\frontend\blocks_typed_if.bas`, `blocks_typed_loops.bas`,
  `blocks_errors.bas`, `explicit_for.bas`; `blocks_not_compiled_yet.bas` now uses the spec's `SELECT CASE` example
  (task 1.3 changed the spec, not the test). `known_unsupported_rejections.list` 74 to 72 (`NEXT j` after `FOR i`,
  a reserved name in an `IF` condition: now real errors); the other two lists unchanged. Tier 2 75 of 75 and 20 of
  20 (the blocks stop at the IR gate, so nothing new passes before 6.2 and 7.1).

## 6. IR (D8)

- [x] 6.1 IR types: lowering labels, `Storage::Temp`, `Jump`, `Branch`, `AssignAll`, `Gosub`, `Return`; the
  module documentation states the pending-error rule of D8 with its check points (`PRINT` items, `Jump`, `Gosub`,
  `Branch` by `on_error`), corrected by 1.1 if the measurement disagrees; the IR dump shows them. Verify:
  `cargo test`;
  `cargo clippy` clean (no `_ =>` on the new enums).
  Done 2026-10-07 (D8 "As built"): `ir\src\lib.rs` (`When`, `OnError`, `Label::name` an `Option`, the rule with
  procedure entry as a check point and the "next statement that runs" case of 1.1); the dump writes lowering
  labels `@n`. `cargo test`, `cargo clippy --all-targets -D warnings` and `cargo fmt --check` clean.
- [x] 6.2 Lowering of every row of the D8 table, `ELSEIF` with the `on_error` value fixed by 1.1. Verify: `ir`
  frontend snapshots per row, including a `GOTO` into a block, `NEXT j, i`, an `EXIT FOR` from inside an `IF`, and
  a `FOR` in a procedure (its temporaries are per call).
  Done 2026-10-07 (D8 "As built"): `ir\src\lower.rs`; `ir` snapshots `tests\frontend\lower_if.bas`,
  `lower_loops.bas`, `lower_for.bas` (also a raising limit), `lower_jumps_proc.bas`. The gate moved from the IR to
  the emitter (`driver::check_backend(fe, to_cpp)`): `--dump ir` and the `ir` mode see every block, `--dump cpp`,
  `-z` and builds still stop with "`…` in code generation" until 7.1 (CLI test renamed `jumps_wait_for_the_emitter`).
  `sema` gained the `IF`/`ELSEIF` and `END IF` lines (typed snapshots `blocks_typed_if`, `blocks_typed_loops`,
  `labels_in_blocks` show them). Lists unchanged; tier 2 75 of 75 and 20 of 20.

## 7. Emitter (D9)

- [x] 7.1 Jumps, branches with the error-pending rule, lowering labels, user labels in procedures, `Temp`
  declarations, `AssignAll`. Verify: `cpp` snapshots; tier 2 `s13_if`, `s14_loops`, `s15_for` pass, also with
  `QB64RUST_NO_FOLD=1`.
  Done 2026-10-07 (D9 "As built"): `codegen-cpp\src\lib.rs`; `Op::may_raise` moved from the lowering into
  `ir\src\lib.rs` for the jump guard. The gate is gone (`driver::check_backend` and `not_emitted` removed; the
  CLI test `jumps_wait_for_the_emitter` replaced, 7.2). `cpp` snapshot `tests\frontend\emit_blocks_cpp.bas`. Tier 2
  `s13`–`s19` pass, also with `QB64RUST_NO_FOLD=1`.
- [x] 7.2 `GOSUB`/`RETURN` with `retK.txt` per body, `RETURN label` with the guarded decrement (D-003),
  `error(3)` in procedures. Verify: `cpp` snapshot; tier 2 `s16_goto_gosub` passes; a frontend or slice test runs
  `RETURN label` with nothing pending, then a `GOSUB` and `RETURN`, and gets error 3 and no crash.
  Done 2026-10-07: `cpp` snapshot `tests\frontend\emit_gosub_cpp.bas` (main and SUB, the same label in both,
  `RETURN label`, a `FOR` in the SUB). `s16_goto_gosub` passes. D-003 is a CLI test,
  `return_label_with_nothing_pending` (builds and runs; output `error 3`, then two `GOSUB`/`RETURN` round trips,
  exit 0). It is `#[ignore]`d like the other tests that need the clone, so CI does not run it; it passed by hand.
  It cannot be a slice program: the corpus holds only output recorded with `qb64pe.exe`, whose program crashes
  here.
- [x] 7.3 Header errors end to end. Verify: tier 2 `s19_header_errors` passes; the pipeline scenarios "FOR loop
  lowered", "Header error follows from the pending-error rule", "A store made with a placeholder value" and "FOR
  limits computed with a placeholder value" are pinned by an `ir` snapshot and by `s19`.
  Done 2026-10-07: no emitter change needed beyond 7.1. `ir` snapshot `tests\frontend\lower_header_errors.bas` (a
  raising `WHILE` condition as a Skip branch, a store with a raising value, a raising `FOR` limit in the header's
  `AssignAll` before the `Jump`, a raising `ELSEIF` as a UseValue branch); "FOR loop lowered" is also
  `lower_for.bas`. `s19_header_errors` passes, also with `QB64RUST_NO_FOLD=1`. Tier 1 green (`fmt --check`,
  `clippy -D warnings`); tier 2 `slice.list` 75 of 75 with and without folding, upstream `pass.list` 20 of 20.
  The lists are not extended yet (8.1).

## 8. Lists and progress

- [x] 8.1 Add to `tests\corpus\slice.list` every corpus program that now passes tier 2 (the count expects up to
  53, listed in `tests\corpus\README.md` with the reason for any that does not pass), and to
  `tests\upstream\pass.list` every upstream program that passes (up to 12 expected). Regenerate the three
  shrink-only lists. Verify: tier 2 with both lists passes, also with `QB64RUST_NO_FOLD=1`; CI job `tier2` green
  after the push; the list diffs only remove entries.
  Done 2026-10-07: `slice.list` 75 → 113 (`s13`–`s16`, `s18`, `s19`, 32 of `runtime_comparison`; 52 joined over
  the change against the 53 counted, the one missing cannot be named since the count kept no names; every program
  still rejected is blocked by something outside the change, `tests\corpus\README.md`). `pass.list` 20 → 23
  (`source_ordering/goto_gosub`, `arrays/t659_explicit_call_control`, `arrays/t659_sub_call_comparisons`; 13
  joined over the change against 12 expected). The three shrink-only lists were already exact (regenerating
  changed nothing; the earlier tasks kept them current). Tier 2: corpus 113 of 113 and upstream 23 of 23, both also
  with `QB64RUST_NO_FOLD=1`. CI `tier2` closed 2026-10-07 by running the job's steps locally against the
  `v4.7.0-GLFW` release, upstream programs from the `tests\upstream` copy: corpus 113 of 113 (235 s), upstream
  23 of 23 (34 s).
- [x] 8.2 Full corpus once with the release build: no crash, no wrong executable, every `.err` program rejected;
  record counts and time in `tests\corpus\README.md`. Measure tier-1 time and record it in `crates\README.md`
  (budget a minute). Verify: the numbers are in both files.
  Done 2026-10-07: 127 pass (113 `.output`, 14 `.err`), 144 rejected with a diagnostic, 6 `.err` programs with
  only marked errors (every `.err` program rejected, no executable), 5 known failures, 0 crashes or wrong
  executables; 4 min 24 s. Full upstream run: 23 pass, 41 s. Tier 1: 4.9–5.4 s (`inputs.rs` 2.4 s).

## 9. Documents

- [x] 9.1 `crates\README.md` (what compiles now), `tests\upstream\README.md` ("x of 279"), `STATUS.md` (the change
  done, next step the IR review, which leaves the place question open until arrays and `TYPE`), `DIVERGENCES.md` if 1.1 led to a decided divergence, `CLAUDE.md` decisions (the
  flat IR with jumps, `CONST` evaluator "keep"), `study\00` §6 (the `WHILE` header loop and `CONST` `^` listed for
  step 6). Verify: `python tools\repo_check\check_repo.py --untracked` clean; `openspec validate
  m2-control-flow-slice` passes.
  Done 2026-10-07: `crates\README.md` lists `OPTION _EXPLICIT` and the control flow as supported, and only what is
  still marked; `tests\upstream\README.md` already said 23 of 279 (8.1), a row for the lists added (43 false
  errors, 72 only-marked, 500 parse gaps); `STATUS.md` and `ROADMAP.md` show the change done and the IR review
  next; `DIVERGENCES.md` unchanged (1.1 led only to D-003, entered with 7.2); `CLAUDE.md` decision of 2026-10-07
  (flat IR with jumps, `CONST` evaluator "keep"); `study\00` §6 already listed the `WHILE` loop and `CONST` `^` and
  now points to step 7 (the order of `study\24` renumbered step 6), §2's M2 row and §13's gap updated. Repo check
  clean (3,312 files), `openspec validate m2-control-flow-slice` valid.
