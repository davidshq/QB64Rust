# Tasks

Standing rules: CLAUDE.md. What this change does differently: every recording with `qb64pe.exe` (the family
`v61`) and the reading of the old compiler's C++ are batched in group 1, before the code; they may change the
delta specs and the design before anything is built.

## 1. Measurements and records

- [x] 1.1 The family `verification\v61_*`, recorded with `qb64pe.exe` (the verification suite), with one line in
  `verification\README.md`: `v61_a*` a procedure's variables (every type, strings, fixed-length strings, a `TYPE`
  and its members, `STATIC`, implicit, a FUNCTION's result name, arrays `DIM` and `STATIC`, recursion, a call to a
  second procedure after the first, `PEEK`/`POKE` through each); `v61_p*` parameters (a `TYPE` through a chain of
  three, an array parameter with static, dynamic, `COMMON` and a procedure's arrays, a scalar with `t 3`, `t (x%)`,
  `t x% + 1`, `t a(2)`, `t s.b`, a parameter of the main module's caller passed on; the forms that are the old
  compiler's internal error, each its own program); `v61_c*` `CALL ABSOLUTE` (a far return, a store through an
  argument, a string and a SINGLE argument, `ABSOLUTE` without `CALL`); `v61_m*` member names (design D4, written by
  a generator). Where a recording differs from the delta specs or the design, they are changed here.
  Verify: every program has its recorded output or error; `tools\repo_check\check_repo.py --untracked` finds
  nothing in them; `openspec validate qbasic-testcases --strict` passes after any edit.
- [x] 1.2 The old compiler's C++ (`qb64pe.exe -z`) of `v61_a1`, a `v61_p` program with a slot and a `v61_c`
  program, read for the order of a procedure's allocations against the save of `cmem_sp` (a `STATIC` one above
  all, design D1), the slot of an argument and the text of `CALL ABSOLUTE`. Design D1–D3 name the lines they rest
  on. Verify: no file of the clone is changed (`git -C ..\QB64pe status` is clean).
- [x] 1.3 `DECISIONS.md`: a row for the change (the user's word of 2026-10-26, "do it", on the three gaps). It
  names `openspec\changes\qbasic-testcases\design.md`, and says it ends "deferred until a program needs them" for
  a procedure's forms (`ROADMAP.md`, M4) and that a plain scalar variable passed to a parameter whose address is
  taken is an error, as the old compiler fails there. Verify: the row is there.

## 2. A procedure's variables

- [x] 2.1 `sema` (`check\places.rs`, `var_address`): `Local`, `Static` and `Result` roots are flagged as `Main`
  ones are; `codegen-cpp` (`cmem.rs`, `decl.rs`): their allocation from `dblock` in the order `v61_a` shows, a
  `STATIC` one kept across calls; arrays of a procedure with flag 4 (design D1). Verify: the `v61_a` programs pass
  with `qb64rust` (`--suite verification --glob "v61_a*"`); `tests\frontend` gains `check-ok` and `cpp` tests of a
  local and a `STATIC` one; tier 1 passes.

## 3. Parameters

- [x] 3.1 `sema`: the parameter flagged by `var_address`; the calls kept; `conventional_args` to a fixed point;
  the slot of an expression argument; the error for a plain scalar variable (design D2), with a `check-fail` test
  and its note. `ir`: the slot carried and `validate.rs` holding every address to a place in conventional memory.
  `codegen-cpp` (`value.rs`, `arg`): the slot allocated from `dblock` in the body's data. Verify: the `v61_p`
  programs pass; tier 1 passes.

## 4. CALL ABSOLUTE

- [x] 4.1 `sema` (`check\proc.rs`): `CALL ABSOLUTE` checked into `Stmt::CallAbsolute` (design D3); `ir` (the
  statement, its lowering, its validation, the dump); `codegen-cpp`: the old compiler's text. `ABSOLUTE` without
  `CALL` as `v61_c` shows. `SOMEDAY.md` row 249 keeps `CALL INTERRUPT[X]` only. Verify: the `v61_c` programs pass;
  the call-site check has a program `tests\callsite\call_absolute.bas` and passes; tier 1 passes.

## 5. Member names

- [x] 5.1 `sema` (`check\places.rs`): `MEMBER_NAMES_REFUSED` from the old compiler's table, every other valid name
  taken (design D4); any parse or check fault in a use the `v61_m` family shows is fixed here. Verify: the `v61_m`
  programs pass; `tests\frontend` `check-fail` for `print AS LONG` keeps its result; tier 1 passes.

## 6. The qbasic suite

- [x] 6.1 `tools\legacy_tests\run_legacy_tests.py`: `--jobs` for `--suite qbasic` with a compiler other than
  `qb64pe` (design D5), with its test in `test_run_legacy_tests.py`; the README's line for the suite. Verify: the
  runner's tests pass; `--suite qbasic --qb64 target\release\qb64rust.exe --jobs 8` passes all 143.
- [x] 6.2 `tools\close\close.py`: the step `qbasic` in the closing set and the merge set, with its test in
  `test_close.py`; `CLAUDE.md` working rule 9's two lists and `tools\close\README.md` name it. Verify:
  `close.py --plan` prints it; the tool's tests pass.

## 7. Lists and closing

- [x] 7.1 `SOMEDAY.md`: rows 56 (arrays of a procedure) and 249 (`CALL ABSOLUTE`) removed or narrowed; tier 1's
  shrink-only lists (`tests\known_*.list`) lose the qbasic programs that now compile clean. `tests\verification`:
  the `v61` programs on `pass.list` or, for a form left out, `known.list` with its reason. Verify: the tier-1 tests
  of the lists pass; no row says a form of this change is "not supported yet" unless it still is.
- [x] 7.2 The closing runs: `python tools\close\close.py --plan`, each command run by its own, then `--check`
  passes. Then the counts by `tools\legacy_tests\counts.py`, `STATUS.md`'s "Next", and `python
  tools\testtime\test_time.py`. Verify: `--check` passes.

## Workflow follow-up

- Archive the change once the user has reviewed it (`/opsx:archive`).
- Tell the user which files to stage and suggest a commit message (nothing is staged by Claude).
