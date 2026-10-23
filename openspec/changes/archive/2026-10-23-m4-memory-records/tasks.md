# Tasks

Standing rules: CLAUDE.md. What this change does differently: every recording with `qb64pe.exe` (the family
`v56`) and the reading of the old compiler's C++ (`-z`) are batched in group 1, before the code. They may change
the delta specs and the design before anything is built.

## 1. Measurements and records

- [x] 1.1 The family `verification\v56_*` of design D12, recorded with `qb64pe.exe` (the verification suite), with
  one line in `verification\README.md`. Where a recording differs from a delta spec or the design, the spec or
  design is changed here. That covers the order of `dblock`, a string's place after assignment, which types
  `VARPTR$` takes, `FOR` over a member with an index, `LEN` and `_MEM` of string members, and `STRING * n` with a
  float or string `CONST`. A form the family shows to be rejected moves to D11. Verify: every program has its
  recorded output or error; `tools\repo_check\check_repo.py --untracked` finds nothing in them;
  `openspec validate m4-memory-records --strict` passes after any edit.
- [x] 1.2 The old compiler's C++ (`qb64pe.exe -z`) of `v28_x01_accepted`, `v56_a`, `v56_b` and `v56_d`, read for
  each allocation in `dblock` (its order among the data fragment's lines), each address text of D2 and D3, and
  each `byte_element` of graphics `GET`/`PUT` of D5. Design D1, D2 and D5 are corrected where they differ, and each
  names the C++ lines it rests on. Verify: no file of the clone is changed (`git -C ..\QB64pe status` is clean).
- [x] 1.3 libqb read for strings in conventional memory: `qbs_new_cmem`, `qbs_set` and every libqb call the
  emitter uses that makes or replaces a descriptor (`SWAP`, `MID$`, parameters, `ERASE` of a string array). Design
  D1 lists each place and what the emitter must write there. Verify: the list is in D1, and each place is named by
  a line of `v56_a`.
- [x] 1.4 `DECISIONS.md`: a row for the change (step 3 of the plan of 2026-10-22). It names
  `openspec\changes\m4-memory-records\design.md` and says it replaces "arrays only until a program needs one" of
  2026-10-13. Verify: the row is there and names the design.

## 2. Variables in conventional memory

- [x] 2.1 `sema` (`lib.rs`, `Var::conventional`; `check\places.rs`, `var_address`): any variable of the main
  module named by `VARPTR` or `VARSEG` is made conventional, and the checks follow `qb64pe.bas` 21328–21560
  (design D1, D2). `ExprKind::VarPtr` and `VarSeg` become `ExprKind::Address { of, word }`. The dump shows the new
  kinds. Verify: `tests\frontend\varptr_kinds_typed.bas` (`typed`); `tests\frontend\m3_step2_marked.bas` loses
  the marks of scalars, strings, members and arrays of `TYPE`s and strings, and keeps those of procedures;
  `cargo test -p qb64rust-sema`.
- [x] 2.2 `ir` (`lib.rs`, `validate.rs`, `dump.rs`, `lower.rs`) and `codegen-cpp` (`decl.rs` `alloc`,
  `alloc_array`, `may_be_conventional`, `new_block`; `value.rs`): a conventional variable is allocated in `dblock`
  in the old compiler's order, and each address kind is written with the old compiler's text (design D1, D2).
  Verify: `tests\frontend\varptr_kinds_cpp.bas` (`cpp`); the call-site check shows every existing program's call
  unchanged; `v56_a1`–`a3`, `a6`, `a7` and `t01`–`t09` pass or are errors as D2 says (`--suite verification`,
  `--list`); `cargo test -p qb64rust-ir -p
  qb64rust-codegen-cpp`.
- [x] 2.3 `codegen-cpp`: every place found in task 1.3 keeps a conventional string's descriptor in conventional
  memory (design D1). Verify: `v56_a4` passes (assignment, `SWAP`, `MID$`, a SUB, `READ`, `LSET`), and `v56_a3`
  (a dynamic array of strings).
- [x] 2.4 `sema`, `check\expr.rs`: `SADD` and `VARPTR$` (design D3), with their errors. `ir` and `codegen-cpp`: their
  texts. `tests\callsite\`: one program each for `SADD`, `VARPTR$` and `VARPTR` of a scalar. Verify:
  `tests\frontend\sadd_varptr_str_errors.bas` (`check-fail`); `v28_x55`, `x56`, `x57`, `v56_b1`–`b5`, `t04`, `t05`, `x01`–`x05`, `x09`–`x11` pass; the call-site
  check compares the new programs equal.
- [x] 2.5 `sema`, `check\builtins.rs` (the `Rule::Radix` arms), and `codegen-cpp`: the width 0 for `VARPTR`, `VARSEG`,
  `SADD` and `_SCREENEXISTS` in `HEX$`, `OCT$` and `_BIN$` (design D4). Verify: `v28_x95`, `v56_c` pass; the
  `_SCREENEXISTS` programs of `known.list` lose that mark (each keeps a line with its next reason if one is left).
- [x] 2.6 `v28_x01_accepted` passes. Verify: `--suite verification --list` with it.

## 3. The array's place of block statements

- [x] 3.1 `sema`, `check\builtins.rs` `block_arg`: a plain variable, a string, a `TYPE` variable and member, an
  undeclared name and arrays of strings, fixed-length strings and `TYPE`s are taken (design D5); `StmtArg` gains
  the kinds. `codegen-cpp`: their `byte_element` texts as read in task 1.2. Verify:
  `tests\frontend\get_put_kinds_cpp.bas` (`cpp`); `tests\callsite\` programs for each kind compare equal; `v16_m5`,
  `v28_x12`, `x13`, `x20`, `x23`, `x59` and `v56_d` pass.
- [x] 3.2 `sema`, `check\builtins.rs` `bytes_arg` and `long_array_arg`: `und()`, `und$()` and `nd&()` make the
  implicit array (design D6). Verify: `tests\frontend\undeclared_block_arrays_typed.bas` (`typed`);
  `v30_x176`–`x178`, `v31_x01_accepted`, `v56_e` and `e2` pass.

## 4. Records

- [x] 4.1 `sema`, `check\places.rs` (`member_bounds`, `Bounds`, the type's layout): a static member array's bound
  that names a constant is read in pass 2, and the types it touches are laid out again before any body is checked
  (design D7). Verify: `tests\frontend\member_const_bounds_typed.bas` (`typed`: offsets, `LEN`, a type that holds
  such a type, a type used above its block); `v24_x196`, `v51_x09`, `x10`, `x11`, `x14` pass; the upstream `types/`
  programs on `pass.list` pass; `cargo test -p qb64rust-sema`.
- [x] 4.2 `sema`, `check\blocks.rs` `for_var`; `ir`, `lower.rs` `loop_block`; `codegen-cpp`: a `TYPE` member as the
  `FOR` variable through members only, and the errors of D8. Verify: `tests\frontend\for_member_ir.bas` (`ir`);
  `v17_c_for_type_member`, `v56_f1`, `f2`, `f4`, `x12`, `x13` pass, and `FOR r.m(i)` (`v56_f3`) stays marked.
- [x] 4.3 `sema`, `check\places.rs` `clear_member_array`: the targets of one `ERASE` or `REDIM` are ops of one
  statement with one error check after the last (design D9). Verify: `tests\frontend\member_erase_list_ir.bas`
  (`ir`); `v24_x312` passes.
- [x] 4.4 `sema`, `check\builtins.rs` `len_whole` and `check\mem.rs` `block_reference`: `LEN` of a whole member
  array of strings and `_MEM` of a value of a `TYPE` with a `STRING` member (design D10). Verify: `v48_x11`,
  `v48_x30`, `v56_g1`, `g2`, `x24` and `x25` pass; `tests\frontend\string_members_typed.bas` (`typed`).

## 5. Rejected forms

- [x] 5.1 `sema`, `check\decl.rs`, `check\proc.rs`, `check\places.rs`: the errors of D11 for `STRING * n` with a
  variable and for `f$n` as a SUB, a `CONST` and a member, and `STRING * n` with a float `CONST` built. Verify:
  `tests\frontend\memory_records_errors.bas` (`check-fail`); `v51_x75`, `v21_x54`, `x55`, `x57` and `v56_x14`–`x22`,
  `x29` pass.
- [x] 5.2 The wording table: `python tools\diagnostics\old_wording.py`, which writes
  `crates\base\src\old_wording.tsv` with the notes of the new errors. Verify: `old_wording.py --check` passes; each
  program of 2.4 and 5.1 shows the old compiler's message as its note (`qb64rust -z`); `cargo test`, fmt, clippy.

## 6. Lists and closing

- [x] 6.1 `SOMEDAY.md`: rows 22, 23, 60, 62–65, 93, 109, 214 and 235 are removed or narrowed to what stays marked
  (procedures, `_MEMGET` and the rest). Rows of "QB64pe behaviours to review" are added for the accidents kept:
  `VARSEG` 80 for every variable, `GET #1, , und$()` accepted where a declared array of strings is rejected, and
  `LEN` of a whole member array of strings. Verify: no row says a form of this change is "not supported yet" unless
  it still is.
- [x] 6.2 `tests\verification\known.list` and `pass.list`: every program that now passes moves to `pass.list`, and
  one whose first mark changed keeps its line with the new reason. The `v56` programs go on one list each. Verify:
  the tier-1 tests of the lists (each program named once); the demand report over `known.list`'s "not supported
  yet" programs no longer names a form of this change, except those `v56` left marked.
- [x] 6.3 The closing runs: `python tools\close\close.py --plan`, each command run by its own, then
  `tools\close\close.py --check` passes. Then the counts of the test READMEs by `tools\legacy_tests\counts.py`, and
  `STATUS.md`'s "Next" names step 4 of the plan (the statements and metacommands left, with the shrink-only
  lists). Then `python tools\testtime\test_time.py`. Verify: `--check` passes.
  Done (2026-10-23) with one exception, accepted by the user: `--check` passes every step but the run without
  folding, 486 of 487 twice (the second run with nothing beside it). The one failure is `screen/g12_game.files`,
  the timeout of `STATUS.md`'s "Still to watch" (exit 124, then `PermissionError` on the retry); run alone without
  folding it passes (3.0 s, the same build).

## Workflow follow-up

- Archive the change once the user has reviewed it (`/opsx:archive`).
- Tell the user which files to stage and suggest a commit message (nothing is staged by Claude).
