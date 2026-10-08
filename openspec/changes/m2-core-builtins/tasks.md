# Tasks

Order of the groups: 1 (the emitter split is a pure move, done before anything changes behaviour), 2 (measure
before designing), then 3 to 7 in order, then 8. Points marked "provisional until 2.1" in the design are corrected
in 2.1 before any code that depends on them. Each group regenerates the shrink-only lists
(`QB64RUST_UPDATE_LISTS=1 cargo test -p qb64rust-driver --test inputs`, entries may only go away) and adds the
programs it makes pass to `slice.list` / `pass.list`.

## 1. Emitter split (design D6)

- [ ] 1.1 Split `crates\codegen-cpp\src\lib.rs` into modules by concern (names, declarations and allocation,
  procedures, operations, places, values), moving code only. Verify: every `cpp` snapshot unchanged
  (`cargo insta test` reports nothing), `cargo clippy --workspace --all-targets -- -D warnings` clean, tier 2
  (`slice.list` 131 of 131, `pass.list` 34 of 34) unchanged; `crates\README.md` names the modules.

## 2. Measurements before code (design D1)

- [ ] 2.1 Write and run `verification\v20_*` with `qb64pe.exe` (`verification\run.sh`) for every question of D1.
  Record findings in `study\00` §5; correct D3, D4, D7, D8 and the spec deltas (`builtin-functions`,
  `control-flow`) where they disagree, and list the corrections here. Anything that measures as wrong code in the
  old compiler is put to the user before it is designed. Verify: `run.sh` output recorded for each program;
  `python tools\repo_check\check_repo.py --untracked` clean; no loop in a program runs without a cap.
- [ ] 2.2 Write the slice programs of D9 (`s25…`), update `tests\corpus\slice\SOURCE.md`, record them with the old
  compiler (`--suite corpus --category slice --record`). Compare each result with the scenarios of the spec deltas;
  a scenario that disagrees is corrected (and listed here). Include the Q-001 (`ON 258 GOTO`) and Q-002 (recursion
  across a `SELECT`) cases. Verify: `run_legacy_tests.py --suite corpus --category slice` with `qb64pe.exe` passes
  every slice program.

## 3. Table-driven built-ins in `sema` (D2, D3)

- [ ] 3.1 `sema\src\builtins.rs`: the supported list `(name, Rule)`, the checker (arity from the table and the rule,
  argument kinds, slot conversion per slot type as measured, result type by rule), `size_of(Ty)` for `LEN`. Move
  `INSTR`, `CHR$` onto it and give `LBOUND`/`UBOUND` their row. Verify: every existing `typed`, `ir` and `cpp`
  snapshot unchanged; unit tests for the slot conversions (one per slot type, from 2.1) and for the arity check.
- [ ] 3.2 The coverage check of the testing delta in `crates\driver\tests\inputs.rs`: each supported built-in must be
  called in a `slice.list` program and in a `typed` front-end test. Verify: it passes for `INSTR`, `CHR$`, `LBOUND`,
  `UBOUND`, `ERR`, `ERL`; removing a name from its test makes it fail and name the built-in (tried by hand, not
  committed).

## 4. String built-ins (D4, D5)

- [ ] 4.1 `Plain` string functions: `LEFT$`, `RIGHT$`, `MID$`, `SPACE$`, `STRING$`, `LTRIM$`, `RTRIM$`, `_TRIM$`,
  `UCASE$`, `LCASE$`, `STR$`, `_TOSTR$`, with their emission. Verify: `typed` and `cpp` tests per group,
  `check-fail` for the measured rejections; slice program `s25…` passes in tier 2; the coverage check passes.
- [ ] 4.2 `LEN`, `ASC` (one and two arguments), `VAL` (with and without a type), `HEX$`, `OCT$`, `_BIN$` by their
  rules. Verify: `typed` tests per rule (including `LEN` of each place kind and the radix widths), `cpp` tests per
  emission rule, `check-fail` for the measured rejections; the string slice programs pass in tier 2; the upstream
  `val_*` and `func_tostr` programs that now compile are run in tier 2 and added to `pass.list` or, with the reason,
  to `tests\known_clean_not_passing.list`.

## 5. Math built-ins (D4, D5)

- [ ] 5.1 `ABS`, `INT`, `FIX` (`ResultOfArg`), `SIN`, `COS`, `TAN`, `ATN`, `SQR`, `LOG`, `EXP` (`FloatByArg`), `SGN`,
  `CINT`, `CLNG`, `CSNG`, `CDBL`, `_ROUND`, `_ATAN2`, `_HYPOT`, `_PI` (bare and with an argument). Verify: `typed`
  tests per rule and per argument type, `cpp` tests per emission rule, `check-fail` for the measured rejections; the
  math slice program passes in tier 2 (the error numbers of D1's edge values included); the coverage check passes.

## 6. `SELECT CASE` (D7)

- [ ] 6.1 `sema`: `SelectBlock` checked (selector kind, item kinds, `IS` operators, `TO`, `CASE ELSE`, `EVERYCASE`,
  the variable-selector rule as measured); the typed tree gains a `Select` statement. Verify: `typed` test,
  `check-fail` for a mismatched item and the other measured errors; the pipeline delta's new "Statement not compiled
  yet" scenario (`SWAP` in a `DO`) replaces the `SELECT CASE` one in `tests\frontend\blocks_not_compiled_yet.bas`.
- [ ] 6.2 Lowering to `Branch`/`Jump` with the static selector copy (Q-002) and the `EVERYCASE` flag; emission.
  Verify: `ir` snapshot shows the static hidden variable in a procedure and the branch chain; `cpp` snapshot;
  `ir::validate` clean on every accepted input; the `SELECT` slice programs (Q-002 included) and the six corpus
  `SELECT` programs pass in tier 2.

## 7. `ON … GOTO` and `ON … GOSUB` (D8)

- [ ] 7.1 `sema`, lowering and emission of `OnJumpStmt` with label targets (line-number targets stay marked).
  Verify: `typed`, `ir` and `cpp` tests; `check-fail` for a string `n` and a label of another body; the slice program
  (n = 0, count + 1, 258 for Q-001, -1) and corpus `226_on_goto`, `227_on_gosub` pass in tier 2.

## 8. Lists, progress and close

- [ ] 8.1 Full corpus and upstream once (tier 3 locally, release build): no crash, no wrong executable; numbers
  recorded in `tests\corpus\README.md`, `tests\upstream\README.md` and `STATUS.md`; every program of the census in
  `proposal.md` that still fails is listed with its reason in this task's record.
- [ ] 8.2 Documentation: `crates\README.md` (what compiles, the built-in rules), `study\00` §5 (done in 2.1) and §9,
  `DIVERGENCES-QB45.md` Q-001 and Q-002 "Pinned by" filled with the slice program names, `DIVERGENCES.md` and
  `DECISIONS.md` if 2.1 led to a decision. Verify: `python tools\repo_check\check_repo.py --untracked` clean.
- [ ] 8.3 CI: the `tier2` job's steps run locally against the QB64pe 4.7.0 release, all pass; tier-1 time recorded
  (budget a minute). Archive the change (`openspec archive`), specs merged; `STATUS.md` "Next" moves to step 7.
