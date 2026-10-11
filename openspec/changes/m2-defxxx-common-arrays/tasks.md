# Tasks

Order: group 1 first (measure before designing; it also settles the decision point of design D8 early), then 2
and 3 (independent of each other), 4 to 7 in order (they build on 3), 8 (stands alone), 9 (needs 3, 8 and the
user's decision of 1.4), then 10. Every group closes with `cargo fmt --check`, `cargo clippy` and `cargo test`
green and with its tests, lists and `crates\README.md` lines in place, so the change can stop after any group.

Standing rules for every task: work inline, no subagents; nothing staged or committed by Claude: each group ends
by naming the files to stage and a commit message; text with backslashes goes through the Edit and Write tools;
`python tools\repo_check\check_repo.py --untracked` is clean before a group is reported done. "Tier 2 slice" below
is `python tools\legacy_tests\run_legacy_tests.py --suite corpus --qb64 target\release\qb64rust.exe --list
tests\corpus\slice.list`, run also with `QB64RUST_NO_FOLD=1`.

## 1. Measurements before code (design D1)

- [ ] 1.1 Write and run `verification\v24_a_*` (default types) and `v24_x*` for their rejections with `qb64pe.exe`
  (`verification\run.sh`). Record the findings in `study\00` §5; correct design D2 and the delta
  `language/default-types` where they disagree, and list the corrections here. Verify: an output is recorded for
  every program; every scenario of the delta that names a literal output agrees with a recorded one or is
  corrected; no loop runs without a cap.
- [ ] 1.2 The same for `v24_b_*` (dynamic arrays, `$DYNAMIC`), `v24_c_*` (`REDIM`, `_PRESERVE`), `v24_d_*`
  (`ERASE`) and `v24_e_*` (`OPTION BASE`); correct D3, D4, D7 and the requirements "Dynamic arrays", "REDIM",
  "REDIM _PRESERVE", "ERASE", "OPTION BASE". Decide `_BIT` arrays (D7) from what `v24_b` shows. Verify: as 1.1;
  the facts of `study\10` §1 that `v09_dim.bas` settled are confirmed or their difference is explained in `study\00`
  §5.
- [ ] 1.3 The same for `v24_f_*` (implicit arrays) and `v24_g_*` (arrays in procedures, array parameters,
  `COMMON`); correct D5, D6 and the requirements "Implicit arrays", "Arrays in procedures", "COMMON", "Array
  parameters". Verify: as 1.1; the `study\00` §5 note that an implicit array in a SUB "raises 9 on every access" is
  explained or corrected.
- [ ] 1.4 The same for `v24_h_*` (`TYPE`: member arrays with `REDIM`, `REDIM _PRESERVE` and `ERASE` of an unmarked
  one, `STRING` members, whole values, `TYPE` parameters); correct D8, D9 and the requirements "Member arrays",
  "Whole user-type values", "User-type parameters", "User types". **Write up case (a) or (b) of D8 with the
  evidence and put it, and any wrong code found in 1.1 to 1.4, to the user; record the answer in `DECISIONS.md`
  (and `DIVERGENCES.md` for a decided difference) and in D8.** Verify: D8 names the case and the decision; group 9
  below is rewritten, split off or cut to match it.

## 2. Default types (design D2; spec `language/default-types`)

- [ ] 2.1 `sema`: the 27-entry table, advanced in file order in both passes; one function for "the default type of
  this name here" replaces the seven written-out SINGLEs (`check\decl.rs`, `check\places.rs`, `check\proc.rs`);
  `DEFINT`, `DEFLNG`, `DEFSNG`, `DEFDBL`, `DEFSTR` and `_DEFINE` are compiled, with the errors measured in 1.1.
  Verify: `tests\frontend\` `typed` tests for a variable before and after a statement, an array, a parameter, a
  FUNCTION result, a procedure before and after it, `_DEFINE` with an unsigned type; `check-fail` tests for each
  rejection; with no `DEFxxx` in a program every existing snapshot is unchanged.
- [ ] 2.2 Slice programs `tests\corpus\slice\s50_deftype*.bas` covering every scenario of the delta, recorded with
  `qb64pe.exe`, added to `slice.list` and `tests\corpus\slice\SOURCE.md`. Verify: tier 2 slice passes with the new
  programs; `runtime_comparison/235_deftype` passes and is listed.
- [ ] 2.3 Regenerate the shrink-only lists; run `qb64rust --report unsupported` over the corpus, upstream and
  `tests\programs`; put every program that became clean on `slice.list`, `pass.list` or
  `known_clean_not_passing.list` after running it in tier 2; update `crates\README.md` (what is supported). Verify:
  the lists only shrank; no `DEFxxx` or `_DEFINE` line is left in the three reports; the programs expected (corpus
  7, upstream 5 outside `deferred.list` naming one) against the programs that passed are stated here.

## 3. The array shape, `OPTION BASE`, `$DYNAMIC`, `DIM AS` lists (design D3)

- [ ] 3.1 `Var::dims` becomes the shape of D3 (scalar, static with bounds, dynamic with an optional number of
  dimensions) in `sema`, the IR, `ir::validate`, both dumps and the emitter, with no new array kind yet. Verify:
  `cargo test` green with **no snapshot changed**; tier 2 slice unchanged (255 plus group 2's programs).
- [ ] 3.2 `OPTION BASE 0`/`1` as checker state folded into the lower bound, with the literal-0 rule and the
  placement rules measured in 1.2; `$DYNAMIC`/`$STATIC` in a comment as checker state (the declarations they turn
  dynamic stay marked until group 4, by their own message). Verify: `typed` tests showing bounds 1 to 3, 0 to 0 and
  2 to 4 under base 1; `check-fail` tests for each measured rejection; slice program `s51_option_base`, recorded,
  passes in tier 2; `runtime_comparison/236_option_base` passes and is listed.
- [ ] 3.3 `DIM AS type a, b(3)` lists in `DIM`, `DIM SHARED` and `STATIC` (in `REDIM` and `COMMON` when those come
  in), with "a suffix in the list" as the measured error. Verify: `typed` and `check-fail` tests; a slice program
  recorded and passing; the message "`DIM AS type` before the names" is gone from the three reports.

## 4. Dynamic arrays in the main module (design D3, D4, D7)

- [ ] 4.1 `sema` and IR: `Allocate` (modes `Dim`, `Redim`, `Preserve`), `Clear`, `Release`; `DIM` with a bound that
  is not constant, `DIM` under `$DYNAMIC`, `DIM a()`, `REDIM [SHARED]` with type adoption and its compile errors,
  `ERASE` of static and dynamic arrays; `ir::validate` checks of D4 with one broken hand-built program each; both
  dumps. Verify: `typed` and `ir` snapshot tests per statement and mode; `check-fail` tests per compile error of the
  requirements "Dynamic arrays", "REDIM" and "ERASE"; `validate` unit tests fail on each broken program.
- [ ] 4.2 Emitter: the not-allocated descriptor, `Allocate` with errors 5, 10 and 257 in the measured order,
  release of the old block after the new one exists, `Preserve` by flat position for numbers, strings, fixed-length
  strings and user types, `Clear` and `Release`; `LBOUND`/`UBOUND` and element access of an array not allocated.
  `_BIT` arrays here if 1.2 decided for them. Verify: `cpp` snapshot tests per statement; slice programs
  `s52_dynamic`, `s53_redim`, `s54_redim_preserve`, `s55_erase` (one scenario of the delta each at least), recorded
  with `qb64pe.exe`, pass in tier 2 with and without `QB64RUST_NO_FOLD=1`.
- [ ] 4.3 Lists and documents as 2.3; `tools\callsite` programs for any libqb call newly written that the check
  can compare. Verify: the lists only shrank; `python tools\callsite\callsite.py` all equal; expected against passed
  stated here (corpus: `120_redim_no_preserve`, `203_redim_multiple`, `206_shared_redim`, `40_erase`,
  `201_erase_udt_array`; upstream: the `REDIM` programs not needing member arrays).

## 5. Implicit arrays (design D5)

- [ ] 5.1 `name(…)`, `LBOUND(name)` and `UBOUND(name)` for a name that may become an array by the rule of D5 make a
  static array `OPTION BASE` to 10; `OPTION _EXPLICITARRAY` makes it the measured error; a later `DIM` is the
  measured error. Verify: `typed` tests for one, two indexes, a suffix, a `DEFxxx` type, base 1; `check-fail`
  tests that each class of name D5 excludes **still** gets its old message (a `_` name, a built-in of another
  suffix, a name whose declaration was marked, a FUNCTION defined later); the "Implicit arrays" scenarios as a
  recorded slice program passing in tier 2.
- [ ] 5.2 Lists and documents as 2.3, and a check for wrong code: the full corpus and the full upstream suite run
  with the release build. Verify: no program builds and prints the wrong output; no program with an `.err` file
  starts compiling (`known_unsupported_rejections.list` only shrinks by programs that now get a real error);
  expected against passed stated here.

## 6. Arrays in procedures (design D6, D7)

- [ ] 6.1 `sema` and IR: `DIM` and `REDIM` in a SUB or FUNCTION (local, dynamic), `STATIC a(n)`, the pending list
  for `STATIC a()`, arrays of a `STATIC` procedure, `SHARED a()`, visibility of `DIM SHARED` and `REDIM SHARED`
  arrays by file order. Verify: `typed` and `ir` tests per form; `check-fail` tests per measured error ("Array
  already listed as STATIC", `DIM SHARED` in a procedure, and the rest of 1.3).
- [ ] 6.2 Emitter: per-call descriptors, release on every way out of the procedure (strings first), program-long
  storage for `STATIC` arrays. Verify: `cpp` snapshot tests showing the release on `EXIT SUB`, at `END SUB` and
  after an error; slice program `s56_proc_arrays` (every scenario of "Arrays in procedures", a recursive procedure
  with a local array, an error inside one), recorded, passes in tier 2 with and without `QB64RUST_NO_FOLD=1`.
- [ ] 6.3 Lists and documents as 2.3. Verify: the message "arrays in a SUB or FUNCTION" and "`SHARED` of an array"
  are gone from the three reports; expected against passed stated here.

## 7. Array parameters and `COMMON` (design D6)

- [ ] 7.1 Array parameters: the parameter shape, the number of dimensions from the body, calls judged after every
  body, `Arg::Array`, the descriptor pointer in the emitter, `REDIM` and `ERASE` of a parameter. Verify: `typed`,
  `ir` and `cpp` tests; `check-fail` tests for each rejection, each with the call above and below the procedure;
  slice program `s57_array_params` (every scenario of "Array parameters"), recorded, passes in tier 2; upstream
  `arrays/array_arg_dimensions_transitive` and `arrays/comma_array_parameter` pass and join `pass.list`.
- [ ] 7.2 `COMMON` and `COMMON SHARED`: scalars, arrays through the pending list, a block name, the placement
  errors; `CHAIN` stays marked. Verify: `typed` and `check-fail` tests; slice program `s58_common` (every scenario
  of "COMMON"), recorded, passes in tier 2.
- [ ] 7.3 Lists and documents as 2.3; `tests\programs\README.md` updated with what Civil War Strategy still needs.
  Verify: `COMMON`, `hit(...)`, `summ(...)`, "arrays in a SUB or FUNCTION" and `DEFINT` are gone from
  `qb64rust --report unsupported tests\programs`; expected against passed stated here.

## 8. The rest of `TYPE` (design D9)

- [ ] 8.1 `STRING` members: the member type, set-up and release per variable and per array element (main, local,
  `STATIC`, in `Allocate`, `Preserve`, `Clear`, `Release`), the compile errors measured in 1.4 (`LEN` of such a
  type). Verify: `typed`, `check-fail` and `cpp` tests; the two "String member" scenarios in a recorded slice
  program `s59_type_strings` passing in tier 2.
- [ ] 8.2 Whole values: `CopyWhole` for `q = p` between variables, elements and members, with its measured error
  rule; `SWAP` of two places of one user type; a `TYPE` parameter by reference. Verify: `typed`, `ir`, `cpp` and
  `check-fail` tests; slice program `s60_type_whole` (every scenario of "Whole user-type values" and "User-type
  parameters"), recorded, passes in tier 2 with and without `QB64RUST_NO_FOLD=1`.
- [ ] 8.3 Lists and documents as 2.3. Verify: "`STRING` members" and "assigning a whole `TYPE` value" are gone from
  the three reports; expected against passed stated here.

## 9. Member arrays (design D8; as decided in 1.4)

- [ ] 9.1 `sema` and IR: bounds on a member in both declaration forms (with `OPTION BASE`), `ArrayRef::Member`,
  elements through any chain, `LBOUND`/`UBOUND` of a member array, the store rule per place as measured, the
  `_STATIC`/`_DYNAMIC` markers still marked. Verify: `typed` and `ir` tests for `t.v(i)`, `a(i).v(j)` and
  `x(0).n(1).q(2)`; `check-fail` tests for a wrong index count, a string index, a marked member.
- [ ] 9.2 Emitter: the layout decided in 1.4 (inline elements, or a descriptor per variable with set-up, release
  and copy), elements passed by reference, `REDIM`, `REDIM _PRESERVE` and `ERASE` of a member array as measured,
  whole-value copy and `STRING` members inside member arrays. Verify: `cpp` snapshot tests; slice program
  `s61_member_arrays` (every scenario of "Member arrays" and the member-array scenario of "LBOUND and UBOUND"),
  recorded, passes in tier 2 with and without `QB64RUST_NO_FOLD=1`; `runtime_comparison/197_type_array_field`
  passes.
- [ ] 9.3 Lists and documents as 2.3, with the full upstream suite run. Verify: "arrays as `TYPE` members" is gone
  from the corpus report and, outside `deferred.list`, from the upstream report except for marked members; no
  upstream program builds and prints the wrong output; expected (59 upstream programs naming member arrays)
  against passed stated here; `SOMEDAY.md` says what of the member-array layer is still deferred.

## 10. Integration

- [ ] 10.1 Full runs with the release build against the reference clone: tier 2 slice (both fold settings), the
  full corpus, the upstream pass list and the full upstream suite, the differential pass list, the call-site
  check, the runner's own tests. Verify: none wrong at run time; every clean program is on a list; the numbers go
  into `STATUS.md` ("Numbers at the last full runs") and `tests\upstream\README.md` (a dated "Upstream progress"
  paragraph).
- [ ] 10.2 Documents: `STATUS.md` (step 10 done, step 11 next, its demand line refreshed from
  `--report unsupported`), `ROADMAP.md`, `crates\README.md`, `tests\corpus\README.md`, `study\00` (§5 complete;
  the document map if a study was added), `SOMEDAY.md`, `DECISIONS.md` (one row per decision taken in the
  change), `CLAUDE.md`'s layout table (the `verification\` row gains `v24*`). Verify:
  `python tools\repo_check\check_repo.py --untracked` clean; `openspec validate m2-defxxx-common-arrays --strict`
  passes; every task above is ticked or its remainder is named in `STATUS.md`.
