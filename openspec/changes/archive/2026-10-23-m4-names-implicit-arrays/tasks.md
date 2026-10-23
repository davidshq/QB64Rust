# Tasks

Standing rules: CLAUDE.md. What this change does differently: every recording with `qb64pe.exe` (the family
`v55`) and the reading of the old compiler's C++ (`-z`) are batched in group 1, before the code. They may change
the delta specs and the design before anything is built.

## 1. Measurements and records

- [x] 1.1 The family `verification\v55_*` of design D9, recorded with `qb64pe.exe` (the verification suite), with
  one line in `verification\README.md`. Where a recording differs from a delta spec or the design (the header
  after an error of its allocation, a later `CASE`, `x()` in a SUB or under `OPTION BASE 1`, the `STATIC` bounds,
  a `STATIC` beside a dynamic shared array, which built-in names are errors where, the dotted declarations), the
  spec or design is changed here. A form the family shows to be accepted is taken out of the error rules of D6.
  Verify: every program has its recorded output or error; `tools\repo_check\check_repo.py --untracked` finds
  nothing in them; `openspec validate m4-names-implicit-arrays --strict` passes after any edit.
- [x] 1.2 The old compiler's C++ (`qb64pe.exe -z`) of `v24_x262`, `x264`, `x265`, `x266`, `x272`, `x280` and the
  `v55_a` programs, read for where each allocation stands among the statement's lines. Design D1, D2 and D4 are
  corrected where they differ: the position of each item's allocation, `WRITE`'s separator, the console
  `INPUT`'s index phase, the `CASE` and `LOOP` tests, the order of the start allocations. Verify: design D1 and D2
  each name the C++ lines they rest on; no file of the clone is changed (`git -C ..\QB64pe status` is clean).
- [x] 1.3 `DECISIONS.md`: a row for the change (step 2 of the plan of 2026-10-22), naming
  `openspec\changes\m4-names-implicit-arrays\design.md`. Verify: the row is there and names the design.

## 2. Names that call nothing

- [x] 2.1 `crates\builtins`: a test that every function spelling of `builtins.json` (each `mayhave` and `musthave`
  suffix, and the functions of `auto_include`) is found by `is_builtin_function` (design D6). Verify: `cargo test
  -p qb64rust-builtins`; the comment in `implicit_array` that the table "does not know every spelling" is gone.
- [x] 2.2 `sema`, `check\places.rs` and `check\io.rs`: "Invalid array name" for a name with one leading underscore
  that calls no built-in; "Expected variable =" for a built-in function's or a FUNCTION's name as a place;
  "Expected variable" as a target of `READ` (design D6), each only where `v55` records the error. Verify:
  `tests\frontend\names_no_array_errors.bas` (`check-fail`); `v24_x106`, `x112`, `x119`, `v22_x79`, `v26_x120`,
  `v28_x53`, `v33_x29`, `v44_x46`, `v49_x16` and the `v55_x` programs of these rules pass (`--suite
  verification`, `--list`); `cargo test -p qb64rust-sema`.
- [x] 2.3 `sema`, `check\proc.rs`, and `syntax\src\parser\mod.rs`: a statement that names no SUB, `CALL` of one, a
  built-in statement with a suffix it does not take, and a built-in function as a statement are errors (design
  D6). Verify: `tests\frontend\statement_names_errors.bas` (`check-fail`); the snapshot `proc_errors` loses the mark
  of `nothere` (the mark of `f`, a FUNCTION as a statement, stays: step 4); `v17_b_if_then_label`, `v20_x06`,
  `v22_x20`, `v33_x50` and the `v55_x` programs of these rules pass; `cargo test -p qb64rust-syntax -p
  qb64rust-sema`.
- [x] 2.4 `sema`, `check\flow.rs`: a jump to a built-in's name is "Invalid label!" for the targets `v55` records
  (design D6). Verify: `tests\frontend\builtin_label_errors.bas` (`check-fail`); `v27_x63`, `v29_x78`, `v29_x79`
  and the `v55_x` jump programs pass.
- [x] 2.5 `sema`, `check\decl.rs` `reserved_proc`: a procedure named like a built-in with a suffix it does not take
  is "Name already in use" where `v52_x28` and `v55` record it (design D6). Verify: the front-end test of
  `v52`'s procedures (`check-fail`); `v52_x28` and the `v55_x` procedure programs pass.
- [x] 2.6 The wording table: `python tools\diagnostics\old_wording.py`, which writes `crates\base\src\old_wording.tsv`
  with the notes of the new errors. Verify: `old_wording.py --check` passes; each program of 2.2–2.5 shows the
  old compiler's message as its note (`qb64rust -z`); `cargo test`, fmt, clippy.

## 3. Names that are arrays or variables

- [x] 3.1 `syntax`, `parser\decl.rs` and `parser\proc.rs`; `sema`, `check\proc.rs`: a declared name with blanks
  around `.` is one name, read without the blanks, and the call `a . b 3` finds the SUB `a.b` (design D7).
  Verify: a parser test of the declared names (`syntax\tests\parser.rs`); `tests\frontend\dot_blanks_decl_typed.bas`
  (`typed`); `v47_x17`, `v47_x18` and the `v55_x` dotted programs pass; `cargo test -p qb64rust-syntax -p
  qb64rust-sema`.
- [x] 3.2 `sema`, `check\builtins.rs` `not_an_assignment`: `WIDTH (n) = v` is a store into the implicit array
  `WIDTH` (design D8). Verify: `tests\frontend\width_array_typed.bas` (`typed`); `v27_x51` passes, and a slice
  program `tests\corpus\slice\s<next>_width_array` that runs `WIDTH (3) = 5: PRINT WIDTH(3)`, recorded with
  `qb64pe.exe` in task 1.1's batch, passes.
- [x] 3.3 `sema`, `check\places.rs` `implicit_array`: `x()` of a name that is no array makes the implicit array of
  one dimension (design D5). Verify: `tests\frontend\whole_implicit_typed.bas` (`typed`); `v48_x04`, `v48_x14` and
  `v55_b` pass.

## 4. Where an implicit array is allocated

- [x] 4.1 `sema`: `StmtKind::Allocate`'s `joined` becomes `join: Join` (`None`, `First`, `Before(n)`); the
  statements of `NoJoin::Item` number their parts and `implicit_array` gives `Before(n)` (design D1); the dump
  shows the join. Verify: `tests\frontend\implicit_items_typed.bas` (`typed`: each statement kind); `cargo test -p
  qb64rust-sema`.
- [x] 4.2 `ir` and `codegen-cpp`: `Op::Print`, `Op::PrintUsing`, `Op::Write`, `Op::Read` and `Op::Input` carry
  `allocs: Vec<(usize, Allocation)>`; the lowering moves an allocation of `Join::Before(n)` into the next op, and
  the emitter writes it just before part `n` (a console `Input` in its index phase) (design D1); the IR's
  validation and dump take the field. Verify: `tests\frontend\implicit_items_ir.bas` (`ir`) and
  `implicit_items_cpp.bas` (`cpp`); the call-site check shows every existing program's call unchanged;
  `v24_x264`, `x265` pass; `cargo test -p qb64rust-ir -p qb64rust-codegen-cpp`.
- [x] 4.3 `sema`, `check\blocks.rs`: the allocations of a header are collected; those of `IF`, `FOR`, `WHILE`,
  `DO` at the top and `SELECT CASE` join the block's first statement, and those of `ELSEIF`, `LOOP` and `CASE` go to
  `Branch::allocs`, `LoopTest::allocs` and `Case::allocs`; `ir`, `lower.rs` (`if_block`, `loop_block`, `select`)
  puts them first in that part's test, and `loop_block` writes the first test of a loop tested at the top apart
  when an allocation joins it (design D2). Verify: `tests\frontend\implicit_headers_ir.bas` (`ir`);
  `v24_x262` and `v55_a` pass.
- [x] 4.4 `sema`: an array first named in a bound of `REDIM` is dynamic and allocated in the `REDIM`'s mode in the
  main module, and is the procedure's unallocated implicit array in a procedure (design D3); `NoJoin` is gone if
  nothing marks with it any more. Verify: `tests\frontend\implicit_redim_bound_typed.bas` (`typed`); `v24_x266`,
  `x268` pass.
- [x] 4.5 `SOMEDAY.md`: row 322 (the allocation as part of the statement that names it) states the item and header
  positions, as a row of "QB64pe behaviours to review". Verify: the row names `v24_x262`, `x264`, `x265`.

## 5. STATIC arrays

- [x] 5.1 `sema`, `check\places.rs`: `STATIC` bounds whose value is known when compiling are taken; a bound naming
  a parameter or local of the procedure is an error saying the old compiler's C++ build fails; the forms `v55_c`
  records are taken as recorded, the rest stay marked (design D4). Verify: `tests\frontend\static_bounds_errors.bas`
  (`check-fail`) and `static_bounds_typed.bas` (`typed`); `v24_x121`, `x280` and `v55_c` pass.
- [x] 5.2 `sema` (`static_array_free`), `ir` (`Program::start_allocations`) and `codegen-cpp` (written after
  `per_program_var`): `STATIC a(n)` beside a shared main-module array is a second start allocation of that array
  (design D4); `SOMEDAY.md` row 331 says it compiles as the old compiler's, no longer marked. Verify:
  `tests\frontend\static_shared_cpp.bas` (`cpp`); `v24_x272` and `v55_d` pass; `cargo test -p qb64rust-ir -p
  qb64rust-codegen-cpp`.

## 6. Lists and closing

- [x] 6.1 `SOMEDAY.md`: the rows this change closes are removed, or become rows of "QB64pe behaviours to review"
  where a quirk is kept: 48, 50, 123, 182, 203, 217, 220, 227, 271. A form that stays marked keeps its row with
  what `v55` found. Verify: no row says a form of this change is "not supported yet" unless it still is.
- [x] 6.2 `tests\verification\known.list` and `pass.list`: every program that now passes moves to `pass.list`, and
  one whose first mark changed keeps its line with the new reason. The `v55` programs go on one list each.
  `v39_a_handles` and `v45_sample`, which have no mark left, are run and get the line their result calls for.
  `tests\known_unsupported_rejections.list` loses `error_detection__undefined_sub_call` and
  `inline_conditional_functions__iif_string`. Verify: the tier-1 tests of the lists (each program named once;
  the shrink-only lists); the demand report over `known.list`'s "not supported yet" programs no longer names an
  implicit array, a name of D6 or a dotted declaration, except the forms `v55` left marked.
- [x] 6.3 The closing runs: `python tools\close\close.py --plan`, each command run by its own, then
  `tools\close\close.py --check` passes; the counts of the test READMEs by `tools\legacy_tests\counts.py`;
  `STATUS.md`'s "Next" names step 3 of the plan (memory and record forms); `python tools\testtime\test_time.py`.
  Verify: `--check` passes.

## Workflow follow-up

- Archive the change once the user has reviewed it (`/opsx:archive`).
- Tell the user which files to stage and suggest a commit message (nothing is staged by Claude).
