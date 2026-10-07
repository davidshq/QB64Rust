# Tasks

Order of the groups: 1 (the safety net protects everything after it), 2 (measure before designing), then 3 to 8
in order, then 9 and 10. Decisions marked "provisional until 2.1" in the design are corrected in 2.1 before any
code that depends on them.

## 1. Tier-1 safety net (design D8)

- [x] 1.1 `ir::validate` (`crates\ir\src\validate.rs`): placed labels, jump targets in their own body, main-module
  targets for `Return(Some)`, `Resume::To` and `SetHandler`, existing variables and procedures, storage classes of
  parameters and results, temporaries used only in their own body. The lowering calls it under `debug_assert!`
  (replacing the "label never placed" assertion). Verify: unit tests with one hand-built broken program per check;
  `cargo test` green.
  *Done 2026-10-07:* as listed, plus two checks the list did not name: a call's argument count matches the
  procedure's parameters, and a built-in call has one slot per table argument. A variable of a procedure
  (static, local, parameter, result, temporary) may be used only in that procedure's body, a main temporary only
  in the main module. The lowering runs it when `debug_assertions` are on and panics with every problem. 9 unit
  tests, one per check plus a sound program.
- [x] 1.2 `crates\driver\tests\inputs.rs`: every accepted input (no front-end error) is lowered, validated and
  emitted under `catch_unwind`, without compiling C++; a failure names the input and the first problem. Verify:
  `cargo test` green; the number of inputs lowered is printed and checked against a floor; a seeded bug (a jump
  to a label of another body) fails the test (tried by hand, not committed); tier-1 time stays within about a
  second of before.
  *Done 2026-10-07:* test `accepted_inputs_lower_and_emit`. **134 inputs** are accepted today (118 corpus programs
  without `.err`, 15 upstream, 1 snippet; the clone sets none), floor 130. All lower, validate and emit; no
  panic. Seeded bug (`Jump(top)` of a `DO … LOOP` pointing 1,000 labels further): both
  `every_input_goes_through_the_front_end` (the lowering's panic, with the problem) and the new test fail.
  `inputs` takes 2.1 s, as before.

## 2. Measurements before code (design D1)

- [x] 2.1 Write and run `verification\v18_*` with `qb64pe.exe` (`verification\run.sh`) for every question of D1.
  Record findings in `study\00` §5; correct D3, D4, D6, D7 and the spec deltas where they disagree, and list the
  corrections here. Decide the open questions (`STRING` members and string arrays; folding of `LBOUND`/`UBOUND`).
  Anything that measures as wrong code in the old compiler (D6 risk) is put to the user before it is designed.
  Verify: `run.sh` output recorded for each program; `python tools\repo_check\check_repo.py --untracked` clean; no
  loop in a program runs without a cap.
  *Done 2026-10-07:* 61 programs (`v18_a`–`v18_h`; 5 added after the first run: a program that stopped early was
  split, `v18_b_index_raises` and `v18_h_dotted_before_dim` to pin D-004's wording and the dotted-name order,
  `v18_d_multi_order`, `v18_e_bounds_empty_parens`). Findings in `study\00` §5, new items in §6. **Wrong code found
  and put to the user:** a member store into an element with a bad index writes element 0; decided: skip it, keep
  the order (D-004, `DECISIONS.md`). Corrections, each in the design's "Measured (2.1)":
  - D6 and the pipeline delta: the member-of-element store evaluates the **value before the index**; the first
    error of a statement wins (`error_handle.cpp`); a read with a bad index gives **element 0's value**, not 0; a
    bad index passed by reference never reaches the SUB (entry check).
  - D4 and the arrays delta: bounds are any constant expression, **floats rounded half to even**; `DIM a(-1)` is an
    error; a static array's `DIM` does nothing at run time; `DIM` twice is an error; **`LBOUND`/`UBOUND` are
    `_INTEGER64`** (the table says LONG) and **`LBOUND(x)` of a non-array makes an implicit array** (was: compile
    error), so it is "not supported yet"; `LBOUND(x())` is an error; arrays are a name plus a type (`c!(1)` beside
    `DIM c(3) AS LONG` is another, implicit array); each dimension is checked on its own.
  - D3 and the types delta: **types are known before the statements** (a type used above its block works); a
    `TYPE` in a SUB is accepted (marked); `a.b` for an **array** `a` of a type is an error, and `a.b` before
    `DIM a AS t` is a plain variable (file order); `q = p` and whole-type arguments are accepted (marked); whole
    values in expressions, `PRINT` and `p = 5` are errors; member suffix rules; layout without padding, `_FLOAT` 32
    bytes (D7).
  - Open questions decided: string arrays in (one blocked corpus program needs them, `43_array_string`), `STRING`
    members out (none of the 28 blocked programs needs one; they need per-variable setup); `LBOUND`/`UBOUND` not
    folded.
- [x] 2.2 Write the slice programs of D9 (`s20…`), update `tests\corpus\slice\SOURCE.md`, record them with the old
  compiler (`--suite corpus --category slice --record`). Compare each result with the scenarios of the spec
  deltas; correct the spec where they disagree. Verify: a second `--record` changes no file; every scenario of
  `language/arrays-and-types` and the new ones of `compiler/pipeline` is covered by a slice program or a planned
  frontend test (list which).
  *Done 2026-10-07:* `s20_arrays`, `s21_types`, `s22_store_rules`, recorded; a second `--record` changed no file.
  Every result agrees with the spec scenarios: **no spec correction**. Errors are raised only with a bad index and
  `CHR$` inside `INSTR` (the built-ins the compiler has). `s22`'s first recording had two by-reference cases with
  an index in range (`x(9)` for `DIM x(10)`); fixed to `x(11)` before the final recording. Coverage:
  - `language/arrays-and-types`: `s20` Bounds, Float bounds, Array and scalar of the same name, DIM run twice,
    Element passed by reference, Float index, Both bounds, Typed _INTEGER64; `s21` Members of each type, Nested
    type, Type used before its block, Local user-type variable, Dotted name without a TYPE, Dotted name before the
    DIM, Member of an element, Member passed by reference; `s22` Index out of range, Each dimension checked, Read
    with a bad index, Element with a bad index passed to a SUB, Dimension out of range; frontend `check-fail`
    Array in a SUB, Array used without DIM, Wrong number of indexes (4.3), Whole value in PRINT (4.1), Unknown
    member (4.2).
  - `compiler/pipeline` (new): `s22` Element store with a raising index, Element store with a raising value,
    Member store with a raising value, Value before index in a member-of-element store (the error reported;
    element 0 by the CLI test); `ir` snapshot Places lowered (5.1); CLI test (builds and runs, like D-003's)
    Member of an element with a bad index (6.2).
  - `testing/compiler-tests` (new): Lowering bug in an accepted program and Emitter panic, both tried by hand in
    1.2.
- [x] 2.3 `m2-parser-breadth`: take array bounds in `DIM`/`DIM SHARED`/`STATIC`/`SHARED` out of its task 7.2
  (pointing here). Verify: `openspec validate m2-parser-breadth` and `openspec validate m2-arrays-and-types` pass.
  *Done 2026-10-07:* both validate.

## 3. Parser (D2)

- [x] 3.1 `DimItem` with `ArrayBounds` (`(u)`, `(l TO u)`, several dimensions, `()`), accessor `DimItem::bounds`;
  `sema` still marks arrays. Verify: `parse-ok` test with every form; `known_parse_gaps.list` regenerated, the
  entries that left it counted here; no new entry.
  *Done 2026-10-07:* the parser reuses the `TYPE` field's `array_bounds`; `sema` marks a `DimItem` with bounds
  ("arrays", "`SHARED` of an array") until group 4. Tests: parser snapshot `dim_array_forms` (`DIM`, `DIM SHARED`,
  `STATIC`, `SHARED f()`), unit test `dim_bounds_accessor` (instead of a `parse-ok` file, which would not show
  the shape). **`known_parse_gaps.list` 500 → 461** (12 corpus, 12 snippets, 15 upstream programs left it); no
  new entry. Found on the way:
  - Fewer parser marks let one program reach a real error the error cap hid before: `qbasic/…/arcdemo.bas` ends
    in byte 0x1A (DOS end of file), which the old compiler's line reader drops at a line end (`qb64pe.bas`
    28060). Fixed in the lexer (whitespace before a line end or the end of the file; parser test
    `eof_byte_at_line_end`); no list change.
  - **False error, older than this change, not fixed here:** `SUB loc` is "name already in use" because `LOC` is
    a built-in; the old compiler accepts a SUB with that name and calls it (`s21` had one; renamed `locsub`). The
    reserved-name rule (`m2-procedures-and-errors` D3) was measured for variables only. Noted in `STATUS.md`
    "Known bugs".

## 4. `sema` (D3, D4, D5)

- [x] 4.1 User types: `TYPE` blocks in the main module, `Ty::User(TypeId)`, `Program::types`, `DIM … AS <type>`
  in every storage class, the marked member forms. Every exhaustive `match` on `Ty` handles `User`. Verify:
  `typed` test with a nested type; `check-fail` tests for each real error and marked form measured in 2.1.
- [x] 4.2 Places: `Place`, `ExprKind::Load`, `StmtKind::Assign { place }`, `Arg::Ref(Place)`; dotted names
  resolved as measured (M8); member read and write; a member passed by reference. Verify: `typed` tests for
  `p.x`, `p.a.b`, `a.b` as a plain name; the symbol table records members' uses where it records variables'.
- [x] 4.3 Arrays: `DIM` with constant bounds in the main module, element read and write, element by reference,
  indexes converted as measured, everything else of D4 marked. Verify: `typed` and `check-fail` tests.
- [x] 4.4 `LBOUND`/`UBOUND` (D4, as measured). Verify: `typed` test; `check-fail` for a non-array argument.
  *Done 2026-10-07 (4.1–4.4 together, one model):* new module `check\places.rs` (types in pass 1, arrays and
  their bounds, elements, members and dotted names, `load`, `LBOUND`/`UBOUND`); `Var::dims`, `Place`,
  `ExprKind::Load`/`Bound`, `Program::types`, `Scope::arrays`/`array_plain`. The no-wildcard lint found 13
  exhaustive `match`es on `Ty` in `sema` (all operand-only, `unreachable!`, except `FOR`, where a `TYPE` variable
  is now marked first). Tests: `typed` `arrays_typed`, `types_typed`; `check-fail` `arrays_errors` (20
  diagnostics, 9 real), `types_errors` (22, 13 real); symbol test `arrays_and_members` (an element's and a
  member's use is a reference of its variable). As built, beyond the design:
  - **Bounds** are typed as ordinary expressions and then evaluated by `places::constant`, not by the `CONST`
    evaluator: a static bound is a run-time expression (`^` is left-associative there), so only what `f64` gives
    exactly is taken (integers as the folding computes them; floats only from whole numbers or literals, rounded
    half to even); anything else is "a dynamic array". It works with folding off.
  - **`x()`** (no index) is "not supported yet" (whole arrays are deferred): a first version reported "has 1
    dimension, not 0", a real error for the wrong reason, which would have been a false error for a valid
    whole-array assignment (`arrA() = arrB()`); 10 upstream `.err` programs showed it.
  - Marked where not measured: a plain scalar beside an array whose plain name `DIM … AS` typed; an array beside
    a typed plain name; a string element or member in parentheses as an argument; a member type suffix on a
    `TYPE` member; `p.x` in a SUB that does not share the main `TYPE` variable `p`; a `TYPE` named like a keyword,
    with a dotted name, or twice; members named like keywords or twice; an empty `TYPE`.
  - Two existing snapshots changed as expected (`blocks_marked`, `blocks_recovery`: a valid top-level `TYPE` is
    no longer marked), two only by message text (`member_access`, `unsupported`).

## 5. IR (D5, D6)

- [x] 5.1 `ir::Place`, `ValueKind::Load`, `Op::Assign { place }`, `Arg::Ref(Place)`, `Ty::User`, the bounds
  node; the IR's doc comment and the pipeline spec state the store rule per place as measured; `ir::validate`
  checks places (an element's index count matches the array's dimensions; a member exists in its base's type).
  Verify: `ir` snapshot per place and per store rule; `cargo test` (tier 1 lowers every accepted input).
  *Done 2026-10-07:* as listed; `validate` also checks that a scalar place is no array, an index is `I64`, and a
  bound names an array. `Op::AssignAll` keeps `VarId` (the lowering's temporaries only). `ir` snapshot
  `places_ir` (every place form, loads, by-reference arguments, `LBOUND`/`UBOUND`). The store rules are one
  operation each in the IR (`Assign` with its place); their encodings are the emitter's, pinned by `places_cpp`.

## 6. Emitter (D7)

- [x] 6.1 Arrays: descriptors, static allocation, element read and write with `array_check`, the element store
  guard, `LBOUND`/`UBOUND`. Verify: `cpp` snapshot; tier 2 `s20…` (arrays) passes, also with
  `QB64RUST_NO_FOLD=1`.
- [x] 6.2 User types: layout, member read and write, members of elements, the member store rule. Verify: `cpp`
  snapshot; tier 2 for the `TYPE` slice programs.
- [x] 6.3 Elements and members passed by reference, in the measured order. Verify: tier 2 for the store-rule
  program; every slice program passes.
  *Done 2026-10-07 (6.1–6.3 together):* the shapes were taken from the old compiler's C++ for a sample program
  (`qb64pe -z`, read in the clone's ignored `internal\temp`): descriptors `ptrszint *__ARRAY_<TYPE>_<NAME>`
  allocated in `maindata.txt` with the dimensions in reverse order, `__UDT_<NAME>` byte blocks, members through
  typed pointers at constant offsets, element reads through `array_check`, `func_lbound`/`func_ubound`.
  `clear.txt` resets arrays and `TYPE` variables as the old compiler's static branch does. The member-of-element
  store of D-004 is a braced block (value first, then each index checked by hand with its own flag, store unless
  an index was bad or raised). `cpp` snapshot `places_cpp`; CLI test `member_store_with_bad_index` (builds and
  runs; `#[ignore]`d like D-003's, needs the clone) pins D-004, and the same program built with `qb64pe.exe`
  prints the old values. Tier 2: `s20`–`s22` pass on the first run, also with `QB64RUST_NO_FOLD=1`.

## 7. Lists and progress

- [x] 7.1 `slice.list` gains the new slice programs and every corpus program this change makes pass (tier 2 with
  the release build); `tests\upstream\pass.list` the upstream programs that pass; the three shrink-only lists
  regenerated. Verify: tier 2 `--list slice.list` all pass; upstream `--list pass.list` all pass; numbers recorded
  here.
- [x] 7.2 Full corpus and upstream once (tier 3 locally): no crash, no wrong executable, numbers recorded in
  `tests\corpus\README.md` and `tests\upstream\README.md`.
  *Done 2026-10-07 (7.1, 7.2):* full corpus **142 pass** (was 127): the 3 slice programs and 12
  `runtime_comparison` programs (`02_lbound_ubound`, `110_array_assign`, `121_type_nested`,
  `199_type_nested_deep`, `204_lbound_ubound_2d`, `255_type_print`, `261_many_dimensions`, `29_multi_dim`,
  `43_array_string`, `58_array_udt`, `71_dim_zero_bound`, `72_three_dim`), 12 of the 17 the proposal counted;
  every failure is a rejection, none wrong at run time. `slice.list` 113 → **128**, all pass (also the 15 new
  ones with `QB64RUST_NO_FOLD=1`). Upstream full run (36 s): **24 of 279** (+ `arrays/t659_array_assignment_control`),
  none wrong at run time; the other 10 candidates need array parameters, whole arrays or `_MEM`. Shrink-only
  lists: parse gaps 500 → 461, false errors 43 (unchanged), only-marked rejections 72 → 71
  (`205_dim_to_same` now gets the measured "cannot redefine" error).
  *Later the same day:* `30_type_udt` (`TYPE Point`) showed that the "named like a built-in" mark was too wide;
  measured `v18_h_type_named_*` and `v18_h_member_names*` (6 programs; `TYPE Point`/`cls` and members `left`,
  `len`, `color`, `name` accepted, `TYPE len`/`print`/`long` and a member `print` "Name already in use"; no
  general rule found, so exactly these are taken, the rest stays marked). `30_type_udt` passes (also without
  folding): **full corpus 144**, `slice.list` **129**, 13 `runtime_comparison` programs joined.

## 8. The value tree (D10)

- [x] 8.1 Count `value_of`'s work by the criterion of `study\25` §2.2 and decide; if "share", replace `ir::Value`
  and `ir::Place` by `sema`'s (`ir` re-exports them), with no snapshot change other than the IR dump's own
  format. Verify: `cargo test`, clippy, tier 2 unchanged; the decision recorded in the design ("As decided") and
  `DECISIONS.md`.
  *Done 2026-10-07:* **shared**: the 66 lines of `value_of`, `args_of` and `place_of` copied variants and did no
  work (count and reasons in design D10, "As decided"). The IR re-exports `sema`'s `Expr`, `ExprKind`, `Place`,
  `Arg`, `VarId`, `ProcId`; its own `Value`, `ValueKind`, `Const`, `Place`, `Arg` and id types are gone; trait
  `Facts` holds `may_raise`/`uses_strings`. No snapshot changed; `cargo test`, clippy clean; tier 2 `slice.list`
  128 of 128 and `pass.list` 24 of 24 after the refactor.

## 9. Documentation and close

- [x] 9.1 `crates\README.md` (what compiles), `tests\corpus\README.md`, `tests\upstream\README.md`,
  `DIVERGENCES.md` for any decided difference, `DECISIONS.md`, `CLAUDE.md` (layout `verification\v18*`), `STATUS.md`
  (step 4 done, numbers), `study\00` §5 (measured facts). Verify: `openspec validate m2-arrays-and-types`;
  `check_repo.py --untracked` clean.
  *Done 2026-10-07:* all listed files updated; `STATUS.md` names step 4 "closing" until 9.2. Both changes
  validate; the repo check is clean apart from untracked `*.exe.qb64rust\` build folders of earlier runs.
- [ ] 9.2 CI: the `tier2` job's steps run locally against the QB64pe 4.7.0 release, all pass. Archive the change
  when the user has seen the record.
