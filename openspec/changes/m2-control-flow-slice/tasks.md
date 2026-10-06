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
- [ ] 1.2 Write the slice programs of D11 (`s13_if` … `s19_header_errors`), update `tests\corpus\slice\SOURCE.md`,
  record them with the old compiler (`--suite corpus --category slice --record`). Compare each result with the
  scenarios of the five spec deltas; correct the **spec** where they disagree and note it here. Verify: a second
  `--record` changes no file; every scenario of `language/control-flow`, `language/constants` and the new ones of
  `language/numeric-semantics` and `language/error-handling` is covered by a slice program or a planned frontend
  test (list which).
- [ ] 1.3 `m2-parser-breadth`: take `CONST` and `OPTION` out of its task 7.2 (pointing here), and change the
  example of its pipeline delta's scenario "Statement not compiled yet" from `FOR` to `SELECT CASE`. Verify:
  `openspec validate m2-parser-breadth` and `openspec validate m2-control-flow-slice` pass.

## 2. `sema` split (D3)

- [ ] 2.1 Split `crates\sema\src\check.rs` by family into `check\` modules (expressions, declarations,
  procedures, flow and error handling, blocks), moving code only. Verify: `cargo test` with no snapshot changed;
  `cargo clippy --workspace --all-targets -- -D warnings` clean; no module over about 600 lines.

## 3. Operators (D4)

- [ ] 3.1 `sema`: comparisons (numeric with the float-narrowing rule, strings as `StrCompare`), `NOT`, `AND`, `OR`,
  `XOR`, `EQV`, `IMP`, `_ANDALSO`, `_ORELSE`, `_NEGATE`, `\`, `MOD`, `^`, with the types and conversions of D4 and
  `UnOp` replacing `Neg`; folding for comparisons and logic on integers, never for `\`/`MOD` by 0 or `^`. The
  typing rules are one function in `check\expr.rs` (D4), the only place that matches on `Ty` to type an operator.
  Verify:
  `typed` frontend tests for each operator family (types and conversion nodes shown); unit tests of folding,
  including wrap at the 32- and 64-bit limits; `check-fail` for a string compared with a number.
- [ ] 3.2 IR and emitter for the operators (`qb_safe_idiv`, `qb_safe_mod`, `pow2`, the string comparison calls,
  `-(a==b)` forms, `&&`/`||` for the short-circuit pair). Verify: `cpp` snapshot per family; tier 2 `s17_operators`
  passes, also with `QB64RUST_NO_FOLD=1`; `crates\README.md` lists the operators as supported.

## 4. Constants and `OPTION _EXPLICIT` (D2, D6, D7)

- [ ] 4.1 Parser: `ConstStmt`/`ConstItem`, `OptionStmt`, accessors. Verify: parse snapshots for each form
  (several items, suffixes, `OPTION BASE 1`, `OPTION _EXPLICITARRAY`); `QB64RUST_UPDATE_LISTS=1 cargo test -p
  qb64rust-driver --test inputs` removes entries from `tests\known_parse_gaps.list` and adds none.
- [ ] 4.2 `sema\consteval.rs` and constant scopes (D6): evaluation, typing as measured in 1.1, visibility from
  the line on, procedure constants, uses as literal nodes, assignment to a constant an error, functions "not
  supported yet". Verify: unit tests of the evaluator (each operator, `^` associativity, suffix rounding, overflow);
  `check-fail` tests for the D10 constant errors; `s18_const` passes in tier 2; the symbol table records constants
  as their own kind (snapshot).
- [ ] 4.3 `OPTION _EXPLICIT` and `_EXPLICITARRAY` (D7), placement as measured. Verify: `check-ok` and `check-fail`
  tests for each declaration kind of 1.1; the scenarios of "OPTION _EXPLICIT" as frontend tests.

## 5. Blocks in `sema` (D3, D5)

- [ ] 5.1 Labels per body (D5): pre-pass per body, labels inside blocks and procedures, jumps checked against the
  body, `ON ERROR GOTO` only to main labels. `GOTO`, `GOSUB`, `RETURN`, `RETURN label` typed. Verify: `check-fail`
  tests (duplicate label in one body, jump to another body's label, `ON ERROR GOTO` to a SUB label); `check-ok`
  for the same name in two bodies; symbol-table snapshot with a label in a SUB.
- [ ] 5.2 `IF` (both forms), `FOR`, `DO`, `WHILE`, `EXIT FOR/DO/WHILE` as typed block statements; `NEXT`
  variables compared (D3); `block_parts` reduced to the blocks still marked. Verify: `typed` snapshots per block
  kind; `check-fail` for a string condition, a wrong `NEXT` variable and order, a non-scalar `FOR` variable;
  regenerated lists lose entries and gain none; the tier-1 `.err` test still passes.

## 6. IR (D8)

- [ ] 6.1 IR types: lowering labels, `Storage::Temp`, `Jump`, `Branch`, `AssignAll`, `Gosub`, `Return`; the
  module documentation states the pending-error rule of D8 with its check points (`PRINT` items, `Jump`, `Gosub`,
  `Branch` by `on_error`), corrected by 1.1 if the measurement disagrees; the IR dump shows them. Verify:
  `cargo test`;
  `cargo clippy` clean (no `_ =>` on the new enums).
- [ ] 6.2 Lowering of every row of the D8 table, `ELSEIF` with the `on_error` value fixed by 1.1. Verify: `ir`
  frontend snapshots per row, including a `GOTO` into a block, `NEXT j, i`, an `EXIT FOR` from inside an `IF`, and
  a `FOR` in a procedure (its temporaries are per call).

## 7. Emitter (D9)

- [ ] 7.1 Jumps, branches with the error-pending rule, lowering labels, user labels in procedures, `Temp`
  declarations, `AssignAll`. Verify: `cpp` snapshots; tier 2 `s13_if`, `s14_loops`, `s15_for` pass, also with
  `QB64RUST_NO_FOLD=1`.
- [ ] 7.2 `GOSUB`/`RETURN` with `retK.txt` per body, `RETURN label` with the guarded decrement (D-003),
  `error(3)` in procedures. Verify: `cpp` snapshot; tier 2 `s16_goto_gosub` passes; a frontend or slice test runs
  `RETURN label` with nothing pending, then a `GOSUB` and `RETURN`, and gets error 3 and no crash.
- [ ] 7.3 Header errors end to end. Verify: tier 2 `s19_header_errors` passes; the pipeline scenarios "FOR loop
  lowered", "Header error follows from the pending-error rule", "A store made with a placeholder value" and "FOR
  limits computed with a placeholder value" are pinned by an `ir` snapshot and by `s19`.

## 8. Lists and progress

- [ ] 8.1 Add to `tests\corpus\slice.list` every corpus program that now passes tier 2 (the count expects up to
  53, listed in `tests\corpus\README.md` with the reason for any that does not pass), and to
  `tests\upstream\pass.list` every upstream program that passes (up to 12 expected). Regenerate the three
  shrink-only lists. Verify: tier 2 with both lists passes, also with `QB64RUST_NO_FOLD=1`; CI job `tier2` green
  after the push; the list diffs only remove entries.
- [ ] 8.2 Full corpus once with the release build: no crash, no wrong executable, every `.err` program rejected;
  record counts and time in `tests\corpus\README.md`. Measure tier-1 time and record it in `crates\README.md`
  (budget a minute). Verify: the numbers are in both files.

## 9. Documents

- [ ] 9.1 `crates\README.md` (what compiles now), `tests\upstream\README.md` ("x of 279"), `STATUS.md` (the change
  done, next step the IR review, which leaves the place question open until arrays and `TYPE`), `DIVERGENCES.md` if 1.1 led to a decided divergence, `CLAUDE.md` decisions (the
  flat IR with jumps, `CONST` evaluator "keep"), `study\00` §6 (the `WHILE` header loop and `CONST` `^` listed for
  step 6). Verify: `python tools\repo_check\check_repo.py --untracked` clean; `openspec validate
  m2-control-flow-slice` passes.
