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

- [x] 1.1 Write and run `verification\v24_a_*` (default types) and `v24_x*` for their rejections with `qb64pe.exe`
  (`verification\run.sh`). Record the findings in `study\00` §5; correct design D2 and the delta
  `language/default-types` where they disagree, and list the corrections here. Verify: an output is recorded for
  every program; every scenario of the delta that names a literal output agrees with a recorded one or is
  corrected; no loop runs without a cap.
  Done 2026-10-11: 11 `v24_a_*` programs and `v24_x01`–`x37`, all recorded (22 of the `x` programs are
  rejected, one fails the C++ build, 14 are accepted). Corrections:
  - **Scenario "Before and after" was wrong**: `a = 1.5: DEFINT A-Z: … PRINT a` prints 0, not 1.5. A name without
    a suffix is the name plus the default at its line, so `a` is `a%` after the statement. The requirement now
    says so; D2's "a variable that exists keeps its type (found by name before the default is asked)" holds only
    for a variable declared with `AS` and is rewritten.
  - **The table follows the file only** (D2's open question): a statement inside a block or a procedure goes on
    applying below it, after `END SUB` too; a procedure above a statement is not reached. The requirement's "as
    the old compiler does" is replaced by that rule with four scenarios; D2 says the first pass must apply the
    statements nested in blocks and procedure bodies as well.
  - **`_DEFINE` accepts `STRING * n`, `_BIT`, `_BIT * n`** (the proposal and the scenario expected them
    rejected). Rejected are a user type, `_MEM`, an unknown name, `_UNSIGNED` alone or with `STRING` or a float,
    `STRING * 0`. Requirement and scenario rewritten.
  - Added to the delta, not in it before: `_` as a letter of its own (`__a` is a valid name and is typed by it),
    backwards ranges, the malformed statements, a `FOR` variable (typed; a `STRING` one is an error), a `CONST`
    (not typed by the default), a FUNCTION called bare where the default has changed (found) and with another
    suffix (error).
  - Two probes were rewritten before recording counted: `v24_a_procs_subs` first compared SINGLE with LONG by
    `LEN` (both 4); `DEFINT _` at the end of a line is a line continuation (`v24_x20`, `x21`, kept as `x37`).
- [x] 1.2 The same for `v24_b_*` (dynamic arrays, `$DYNAMIC`), `v24_c_*` (`REDIM`, `_PRESERVE`), `v24_d_*`
  (`ERASE`) and `v24_e_*` (`OPTION BASE`); correct D3, D4, D7 and the requirements "Dynamic arrays", "REDIM",
  "REDIM _PRESERVE", "ERASE", "OPTION BASE". Decide `_BIT` arrays (D7) from what `v24_b` shows. Verify: as 1.1;
  the facts of `study\10` §1 that `v09_dim.bas` settled are confirmed or their difference is explained in `study\00`
  §5.
  Done 2026-10-11: 14 programs `v24_b_*` to `v24_e_*` and `v24_x38`–`x104`, all recorded; the C++ the old
  compiler writes for `DIM d(n)`, `REDIM`, `REDIM _PRESERVE` and a `_BIT` array was read beside them. Every fact
  `v09_dim.bas` settled is confirmed. Corrections:
  - **Error 5 is not the whole rule for bounds without elements** (`study\10` §1.7 said "count <= 0 → error 5"):
    a count of exactly 0 raises 5 **and the array is allocated empty**; a negative count goes on to error 257,
    which is critical: "Out of memory", the program ends, no handler. Scenario "Fewer than one element" (n = -3,
    handler prints 5) was wrong and is now two scenarios. D4 and D7 give the measured order.
  - **An error does not stop an allocation** (a bound that raises still allocates): `Allocate` is not guarded
    like a store. New in D4.
  - **`DIM a()` declares a scalar**, not an array without bounds (D3, requirement "Static arrays", scenario
    replaced). It goes on the "QB64pe behaviours to review" list of `SOMEDAY.md` in 4.3.
  - **Static or dynamic is settled by the first declaration**; later `DIM`s are run-time statements (table in
    D3). `DIM a(5)` then `DIM a(n)` compiles and raises 10, so `Allocate` may name a static array (D4's
    `validate` rule changed).
  - **`$DYNAMIC`, `$STATIC` and `OPTION BASE` follow the file**, through procedures, like the default types. An
    array used without `DIM` below `$DYNAMIC` is dynamic and allocated by the first statement that names it.
  - **`OPTION BASE`: only the plain number `0` keeps 0 to 0**; other constants that are 0 give a static array of
    no elements and error 5 at program start, which is matched (D3, D7). Only the word after `BASE` is read.
  - **`_BIT` arrays are in** (D7): plain packed storage. Requirement "Static arrays" no longer marks them.
  - `ERASE` of anything but a declared array is a compile error, an unused name included (no array is made).
  - `REDIM` with another type written with `AS` is a compile error; `REDIM SHARED _PRESERVE` is one too.
- [x] 1.3 The same for `v24_f_*` (implicit arrays) and `v24_g_*` (arrays in procedures, array parameters,
  `COMMON`); correct D5, D6 and the requirements "Implicit arrays", "Arrays in procedures", "COMMON", "Array
  parameters". Verify: as 1.1; the `study\00` §5 note that an implicit array in a SUB "raises 9 on every access" is
  explained or corrected.
  Done 2026-10-11: 9 programs `v24_f_*`, `v24_g_*` and `v24_x105`–`x181`, all recorded. Corrections:
  - **The "raises 9 on every access" note is explained and stands**: in a procedure the old compiler makes a
    local dynamic array and writes no allocation for it (read in its C++). Only a `SUB … STATIC` without
    `$DYNAMIC` gets a working one. D5 has the table; requirement "Implicit arrays" said "a static array of the
    scope it stands in" and is rewritten; scenario "Main array not visible" now names its output. **For the user
    in 1.4**: match this (as built) or allocate as QuickBASIC 4.5 does (a divergence).
  - **A block name in `COMMON` is a compile error**, not accepted and dropped (proposal, D6, requirement and its
    text corrected). The parser reads one today; 7.2 makes it an error.
  - **No pending list**: `STATIC a()` and `COMMON a()` declare a dynamic array that is not allocated; the `DIM`
    after them is an ordinary allocation that raises 10 the second time. D6 rewritten; the `COMMON` requirement
    said the array "gets its bounds from a later DIM", which hid that it is dynamic.
  - **Array arguments: kind and width must match, `_UNSIGNED` and `_OFFSET` are not compared** (D6 and the
    requirement said "exactly"). A `_BIT` array parameter is allowed.
  - **The number of dimensions of an array parameter** comes from body uses and arguments together (D6 had the
    body first, then calls judged against it): one unknown per parameter, solved after all bodies.
  - **Visibility of shared arrays follows the file** (a procedure above the declaration does not see it);
    `SHARED a()` needs the main array above it and makes none.
  - `ERASE` of an array parameter is static or dynamic at run time: `EraseParam` added to D4.
  - Compile errors here where the old compiler's C++ build fails: `STATIC v(n)`, `fill (x())`.
  - D5's name rule restated on the measured one: a name may be an array where it may be a variable.
- [x] 1.4 The same for `v24_h_*` (`TYPE`: member arrays with `REDIM`, `REDIM _PRESERVE` and `ERASE` of an unmarked
  one, `STRING` members, whole values, `TYPE` parameters); correct D8, D9 and the requirements "Member arrays",
  "Whole user-type values", "User-type parameters", "User types". **Write up case (a) or (b) of D8 with the
  evidence and put it, and any wrong code found in 1.1 to 1.4, to the user; record the answer in `DECISIONS.md`
  (and `DIVERGENCES.md` for a decided difference) and in D8.** Verify: D8 names the case and the decision; group 9
  below is rewritten, split off or cut to match it.
  Measured and written up 2026-10-11; **answered by the user the same day** (`DECISIONS.md` 2026-10-11):
  question 1, keep it as QB64pe for now and list it in `SOMEDAY.md`; question 2, do as QB64pe and note the
  QuickBASIC 4.5 difference (`DIVERGENCES-QB45.md` Q-009); the D9 additions stay. D5, D8, D9 and requirement
  "Whole user-type values" say so; nothing in `DIVERGENCES.md` changes (no new difference from QB64pe). 6 programs
  `v24_h_*` and `v24_x182`–`x226`, all recorded; findings in `study\00` §5.
  - **D8 is case (a): an unmarked member array is inline.** Evidence in D8 (`LEN` 22 for LONG, four LONGs,
    INTEGER; `REDIM` with the declared bounds clears, with any other bounds is a compile error; `REDIM
    _PRESERVE` does nothing; `ERASE` clears). No variable needs a descriptor, so group 9 stands as written; D8
    lists what the measurement added (bounds read at the `TYPE` block with `OPTION BASE` and no literal-0 rule;
    `REDIM`/`ERASE` lower to `Clear` or nothing; a whole member array as an array argument).
  - Corrections to the deltas: requirement "Member arrays" names the `REDIM`/`ERASE` behaviour instead of "as the
    old compiler does" and gains the store rule (D-004 at every index), `LEN`, `OPTION BASE` and six scenarios;
    "Whole user-type values" gains elements, members and self-assignment, `SWAP` with `STRING` members and the
    member-array copy; "User-type parameters" gains the entry check, FUNCTIONs, the parenthesised argument (an
    internal error of the old compiler: a compile error here) and "no user-type FUNCTION result"; scenario
    "Marked member" had the marker after `AS LONG`, where it is not read.
  - **Question 1, whole-value store with an index outside.** Old compiler: `a(9) = p` raises 9 and overwrites
    `a(0)`; `q = a(9)` raises 9 and copies `a(0)` into `q`. Proposed: nothing is copied when an error is
    pending, which is D-004's rule (a member store with a bad index) applied to the whole value; it would be a
    new row or an extension of D-004 in `DIVERGENCES.md`. 8.2 waits for the answer.
  - **Question 2, an array used without `DIM` in a SUB or FUNCTION.** Old compiler: a local array that nothing
    allocates, so every use raises 9 (only a `SUB … STATIC` gets a working one). QuickBASIC 4.5 makes a working
    0 to 10 array. Built as measured unless decided otherwise; allocating it would be a divergence. 5.1 and 6.1
    are written for "as measured" and change by a few lines either way.
  - Old-compiler behaviour matched as it is and listed for review in `SOMEDAY.md` when its group closes, no
    decision asked: `DIM a()` declares a scalar; under `OPTION BASE 1` a constant bound that is 0 but not
    written `0` compiles and raises 5 at program start; a negative element count ends the program with "Out of
    memory" where a count of 0 raises 5.
  - Still open from `design.md`: whether the D9 additions (`STRING` members, whole values, `TYPE` parameters)
    stay in this change. The measurement found nothing against it.

## 2. Default types (design D2; spec `language/default-types`)

- [x] 2.1 `sema`: the 27-entry table, advanced in file order in both passes; one function for "the default type of
  this name here" replaces the seven written-out SINGLEs (`check\decl.rs`, `check\places.rs`, `check\proc.rs`);
  `DEFINT`, `DEFLNG`, `DEFSNG`, `DEFDBL`, `DEFSTR` and `_DEFINE` are compiled, with the errors measured in 1.1.
  Verify: `tests\frontend\` `typed` tests for a variable before and after a statement, an array, a parameter, a
  FUNCTION result, a procedure before and after it, `_DEFINE` with an unsigned type; `check-fail` tests for each
  rejection; with no `DEFxxx` in a program every existing snapshot is unchanged.
  Done 2026-10-11. The table is `Checker::defaults`; pass 1 walks the whole file (blocks, procedure bodies,
  included files) and keeps the table at each procedure header, pass 2 sets it at each statement
  (`check\mod.rs`, `check\decl.rs`). Tests: `tests\frontend\deftype_typed.bas`, `deftype_errors.bas`,
  `deftype_explicit_errors.bas`; no existing snapshot changed. Beside the task's list:
  - the parser took only A to Z as a letter; `_` is one now (`DEFINT _, a`, measured);
  - a parameter without `AS` no longer types the plain name for its body (it is found as name plus default,
    which is what makes `p` after a `DEFINT` inside the SUB a new local, as measured);
  - an unknown type and `_MEM` after `_DEFINE … AS` are "not supported yet", as they are in `DIM` (the old
    compiler rejects both).
  7 of the 11 `v24_a_*` programs build and print what the old compiler printed; the other four wait on
  `COMMON`, `REDIM`, implicit arrays and a label named like a SUB. Of `v24_x01`–`x37`, 35 get the old
  compiler's verdict; `x30` and `v24_a_defstr` are held back only by that label form.
- [x] 2.2 Slice programs `tests\corpus\slice\s50_deftype*.bas` covering every scenario of the delta, recorded with
  `qb64pe.exe`, added to `slice.list` and `tests\corpus\slice\SOURCE.md`. Verify: tier 2 slice passes with the new
  programs; `runtime_comparison/235_deftype` passes and is listed.
  Done 2026-10-11: `s50_deftype` (main module, 22 numbered lines), `s50_deftype_procs` (procedures, 14),
  `s50_deftype_inner` (a statement inside a SUB, 6), each recorded from two equal runs. The new compiler printed
  all three as recorded on its first run. Tier 2 slice: 259 of 259, also with `QB64RUST_NO_FOLD=1`.
- [x] 2.3 Regenerate the shrink-only lists; run `qb64rust --report unsupported` over the corpus, upstream and
  `tests\programs`; put every program that became clean on `slice.list`, `pass.list` or
  `known_clean_not_passing.list` after running it in tier 2; update `crates\README.md` (what is supported). Verify:
  the lists only shrank; no `DEFxxx` or `_DEFINE` line is left in the three reports; the programs expected (corpus
  7, upstream 5 outside `deferred.list` naming one) against the programs that passed are stated here.
  Done 2026-10-11. The three shrink-only lists are unchanged (nothing to shrink: no entry named a `DEFxxx`).
  No `DEFxxx` or `_DEFINE` line is left in the three reports (`DEF` in Civil War Strategy is `DEF SEG`).
  **Expected against passed.** Corpus: 7 programs name `DEFINT`; 1 became clean and passes
  (`runtime_comparison/235_deftype`, on `slice.list`); the other 6 are the screen programs cut from Civil War
  Strategy (`g01`–`g06`), which draw (M3). Upstream: 5 outside `deferred.list`; 1 became clean and passes
  (`source_ordering/define`, on `pass.list`: 51 of 279); `operators_test/op_exp` waits on `DIM AS type` before the
  names (3.3), `image/curico_test` and `view/window_view_mandelbrot` on images, `mem/mem_checking_off` on `_MEM`.
  `fmt`, `clippy` and `cargo test` are green; the upstream pass list runs 51 of 51.

## 3. The array shape, `OPTION BASE`, `$DYNAMIC`, `DIM AS` lists (design D3)

- [x] 3.1 `Var::dims` becomes the shape of D3 (scalar, static with bounds, dynamic with an optional number of
  dimensions) in `sema`, the IR, `ir::validate`, both dumps and the emitter, with no new array kind yet. Verify:
  `cargo test` green with **no snapshot changed**; tier 2 slice unchanged (255 plus group 2's programs).
  Done 2026-10-11: `sema::Shape`, re-exported by the IR; `Var::bounds()` and `dim_count()` replace the field. No
  snapshot changed; tier 2 slice ran with the whole group (below).
- [x] 3.2 `OPTION BASE 0`/`1` as checker state folded into the lower bound, with the literal-0 rule and the
  placement rules measured in 1.2; `$DYNAMIC`/`$STATIC` in a comment as checker state (the declarations they turn
  dynamic stay marked until group 4, by their own message). Verify: `typed` tests showing bounds 1 to 3, 0 to 0 and
  2 to 4 under base 1; `check-fail` tests for each measured rejection; slice program `s51_option_base`, recorded,
  passes in tier 2; `runtime_comparison/236_option_base` passes and is listed.
  Done 2026-10-11: `Checker::option_base` and `dynamic_mode`; tests `option_base_typed.bas`,
  `option_base_errors.bas`; slice `s51_option_base`; `236_option_base` listed. A `DIM` below `$DYNAMIC` is marked
  "`DIM` below `$DYNAMIC` (a dynamic array)"; the marks on the two comments themselves are gone (three older
  snapshots lost those lines). The parser keeps what follows `OPTION BASE n` in the statement without an error,
  as the old compiler ignores it. **Found here, by comparing with the recordings:**
  - **a constant bound is a spelling, not a value** (`verification\v24_b_constant_bounds`, `_2`): a bound with an
    operator makes a dynamic array in the old compiler. Written into `study\00` §5, D3 and requirement "Static
    arrays"; the code change is in 4.1 and the difference is under "Known bugs" in `STATUS.md` until then.
  - so `DIM a(0 + 0)` under base 1 is not the "static array with no element" case (a `CONST` that is 0 is): the
    scenarios of requirement "OPTION BASE" were split accordingly. The new compiler answers the `CONST` case as
    the old one does (error 5 at program start, line 1); the `0 + 0` case waits for 4.1.
- [x] 3.3 `DIM AS type a, b(3)` lists in `DIM`, `DIM SHARED` and `STATIC` (in `REDIM` and `COMMON` when those come
  in), with "a suffix in the list" as the measured error. Verify: `typed` and `check-fail` tests; a slice program
  recorded and passing; the message "`DIM AS type` before the names" is gone from the three reports.
  Done 2026-10-11, measured first (`verification\v24_i_dim_as`, `v24_x227`–`x237`): every name of the list is
  declared as `name AS type`; a suffix on a name, also the list's own, and an `AS` clause on one are errors.
  `SHARED AS type` came with it (same code). Tests `dim_as_list_typed.bas`, `dim_as_list_errors.bas`; slice
  `s51_dim_as_list`. The message is gone from the three reports. **Expected against passed:** upstream
  `arrays/t659_variable_boolean_expr` and `operators_test/op_exp` became clean and pass (53 of 279); the other
  upstream programs naming the form wait on `REDIM` or arrays in procedures.
  **Found on the way, outside the task's list, and fixed:** `op_exp` printed one wrong line (`2 ^ -NOT x + 3`: 16
  for 5). The parser let a `NOT` inside a minus right of `^` reach past the next operator; the old compiler's
  rule (`study\02` §1.2 step D) is now in `parser\expr.rs`, measured by `verification\v24_j_power_unary` (27
  expressions, all equal now), with `power_negation_typed.bas`, `power_negation_errors.bas` (`2 ^ _NEGATE x` is
  rejected, as measured) and slice `s51_power_unary`.
  Group 3 closes with `fmt`, `clippy` and `cargo test` green, tier 2 slice 263 of 263 (also with
  `QB64RUST_NO_FOLD=1`) and the upstream pass list 53 of 53.
  **Review of groups 1 to 3 (2026-10-11), two fixes:**
  - **wrong code from 2.1:** a parameter without `AS` whose name a main-module `DIM SHARED … AS` types was read
    as the shared variable (`DIM SHARED x AS LONG` … `FUNCTION f (x): f = x * 2` printed 14 for 3), because 2.1
    stopped typing the plain name by such a parameter. Measured (`verification\v24_a_param_shared`): the
    parameter comes first while the default is the header's; below a `DEFxxx` in the body the name means the
    shared variable. `var_key` (`check\decl.rs`) does that now; test `deftype_param_shared_typed.bas`, slice
    `s50_deftype_param_shared`. The same rule holds for any variable of the procedure's own with the default
    type (a local `x!` first written with its suffix was read as the shared `x` before this change too: wrong
    code, older, fixed with it), not for a main variable bound by `SHARED x!`;
  - **a false error from 3.2:** `OPTION BASE 01` (accepted, base 1; `v24_x239`–`x243`: plain digits with zeros
    before them, `1&` and `&H1` rejected). Tests in `option_base_typed.bas` and `option_base_errors.bas`.
  Tier 2 slice after them: 264 of 264, also with `QB64RUST_NO_FOLD=1`. Found and not fixed (older than this
  change, "Known bugs" in `STATUS.md`): a unary plus is a parse error.

## 4. Dynamic arrays in the main module (design D3, D4, D7)

- [x] 4.1 `sema` and IR: `Allocate` (modes `Dim`, `Redim`, `Preserve`), `Clear`, `Release`; `DIM` with a bound that
  is not constant **by its spelling** (found in 3.2: a bound with an operator makes a dynamic array though its
  value is known; such `DIM`s are compiled as static arrays today and move to the dynamic side here, with a
  `typed` test for each spelling of `verification\v24_b_constant_bounds*` and the "Known bugs" line of
  `STATUS.md` removed), `DIM` under `$DYNAMIC`, `DIM a()`, `REDIM [SHARED]` with type adoption and its compile errors,
  `ERASE` of static and dynamic arrays; `ir::validate` checks of D4 with one broken hand-built program each; both
  dumps. Verify: `typed` and `ir` snapshot tests per statement and mode; `check-fail` tests per compile error of the
  requirements "Dynamic arrays", "REDIM" and "ERASE"; `validate` unit tests fail on each broken program.
  Done 2026-10-11. `declare_array` (`check\places.rs`) settles static or dynamic at the first declaration, from
  the spelling of each bound (`dim_bounds`, `bound_spelling`), and makes every later `DIM` or `REDIM` an
  `Allocate`; `redim` and `erase` are in `check\decl.rs`. The three statements are in the typed tree, the IR,
  `validate` (7 unit tests) and both dumps; the IR dump now lists the arrays (five older `ir` snapshots gained
  their `Array` lines). Tests: `dynamic_bounds_typed.bas` (the 27 spellings of the two measurements and `-&H5`),
  `redim_typed.bas`, `dynamic_arrays_ir.bas`, `redim_errors.bas` (22 errors), `redim_unsupported.bas`;
  `arrays_typed` shows `DIM f(-(2 ^ 2) TO 10 \ 3)` as a dynamic array now. The "Known bugs" line is gone.
  Beside the task's list:
  - **measured first, because I had read them off the old compiler's source** (`verification\v24_x244`–`x250`):
    two came out as read (bounds checked per dimension; a constant `DIM` again below `$DYNAMIC` runs), and **two
    did not**: `-&H5` is one number (I had taken it for an operator), and `SHARED` on a later `REDIM` is accepted
    and ignored (I had marked it). Both corrected; `study\00` §5, the delta and `SOMEDAY.md` have them;
  - a `REDIM` and an `ERASE` in a procedure of a `DIM SHARED` array compile here (measured,
    `v24_c_redim_shared`; nothing new is needed for them); a procedure's own arrays stay marked for group 6;
  - `array` is a `VarId`, not yet the `ArrayRef` of D4 (it comes with member arrays, group 9);
  - a typed array beside a scalar of its name typed alike is no longer marked (measured, `v24_x56`,
    `v24_b_dim_empty_then_dim`); `ERASE` of an array used without `DIM` is marked until group 5.
- [x] 4.2 Emitter: the not-allocated descriptor, `Allocate` with errors 5, 10 and 257 in the measured order,
  release of the old block after the new one exists, `Preserve` by flat position for numbers, strings, fixed-length
  strings and user types, `Clear` and `Release`; `LBOUND`/`UBOUND` and element access of an array not allocated.
  `_BIT` arrays here if 1.2 decided for them. Verify: `cpp` snapshot tests per statement; slice programs
  `s52_dynamic`, `s53_redim`, `s54_redim_preserve`, `s55_erase` (one scenario of the delta each at least), recorded
  with `qb64pe.exe`, pass in tier 2 with and without `QB64RUST_NO_FOLD=1`.
  Done 2026-10-11. `codegen-cpp\src\arrays.rs` (new) writes `Allocate` after `allocarray`'s sequence, read in
  `qb64pe.bas` 15193–15531; `decl.rs` the not-allocated descriptor, `release` and the packed size of `_BIT`
  arrays. `_BIT` arrays are in, static and dynamic: `getbits`/`getubits` to read, `setbits` to store, kept
  elements copied one by one in `_PRESERVE`. Tests: `dynamic_arrays_cpp.bas`, `bit_arrays_cpp.bas`,
  `bit_arrays_errors.bas` (in place of `bit_arrays.bas`, which pinned the mark). Slice programs `s52_dynamic`,
  `s52_dynamic_memory` (the "Out of memory" end), `s53_redim`, `s54_redim_preserve`, `s55_erase`, each recorded
  from two equal runs: **the new compiler printed all five as recorded on its first run**, also with
  `QB64RUST_NO_FOLD=1`. The 71 measurement programs of this group (`v24_b_*`, `v24_c_*`, `v24_d_*`, `v24_x38`–
  `x63`, `x80`–`x82`, `x86`–`x101`, `x104`, `x244`–`x253`), run with the new compiler: 38 build and print what
  the old compiler printed, 21 are rejected by both with a real error, none differs; 10 the old compiler
  accepts are still marked (arrays used without `DIM` in 8, a label named like a SUB, a plain scalar beside an
  array typed with `AS`) and 2 it rejects are marked (`$DYNAMIC` as a statement, `OPTION _EXPLICITARRAY`).
  - **Found with `_BIT` arrays in:** console `INPUT` into a `_BIT` element went through `sema` and failed the
    C++ build. Measured (`v24_x251`–`x253`): the old compiler rejects it ("INPUT cannot handle _BIT array
    elements") and takes `READ` and `INPUT #` into one. `sema` rejects the first now; the other two already
    printed as recorded (slice `s52_dynamic` line 19 pins `READ`).
  - Not done, by design D7's note: a dynamic array of the main module is not released when the program ends.
- [x] 4.3 Lists and documents as 2.3; `tools\callsite` programs for any libqb call newly written that the check
  can compare. Verify: the lists only shrank; `python tools\callsite\callsite.py` all equal; expected against passed
  stated here (corpus: `120_redim_no_preserve`, `203_redim_multiple`, `206_shared_redim`, `40_erase`,
  `201_erase_udt_array`; upstream: the `REDIM` programs not needing member arrays).
  Done 2026-10-11. `known_unsupported_rejections.list` shrank by two (`arrays/negative_array_size`, `_2`: they
  get the old compiler's "Invalid array bounds" now); the other two lists are unchanged. No plain `REDIM`,
  `ERASE`, `$DYNAMIC` or "bound that is not constant" line is left in the three reports (`REDIM _RETAIN`, 27
  upstream programs, stays in `SOMEDAY.md`). The call-site check is 256 of 256 equal; no program was added:
  the allocation is inline C++ and `getbits`/`setbits` are no built-in forms. **Expected against passed.**
  Corpus: 4 of the 5 became clean and pass (`40_erase`, `120_redim_no_preserve`, `201_erase_udt_array`,
  `203_redim_multiple`); `206_shared_redim` waits on `SHARED a()` (group 6). Upstream: 3 became clean and pass
  (`arrays/02_option_base1_only_affects_later_normal_arrays`, `36_redim_preserve_packed_bit_shrink_grow`,
  `erase_dynamic_string_array_reuse`: 56 of 279); the other `REDIM` programs wait on member arrays, arrays
  used without `DIM`, arrays in procedures or `REDIM _RETAIN`. Group 4 closes with `fmt`, `clippy` and `cargo
  test` green, tier 2 slice 273 of 273 (also with `QB64RUST_NO_FOLD=1`), the upstream pass list 56 of 56, the
  differential list 59 of 59 and the runner's tests 56 of 56.
  **Review of group 4 (2026-10-11), one fix:**
  - **a false syntax error, older than this change and made visible by it:** the parser took `PRESERVE` and
    `RETAIN` without their underscore as words of a `REDIM`, so `REDIM preserve(3)` was "expected a variable
    name". Measured (`verification\v24_x254`, `x255`): the old compiler rejects `$NOPREFIX`, the two words are
    names like any other (`REDIM preserve(3)` declares an array), and `REDIM PRESERVE x(5)` is "DIM: Expected
    ,". The parser reads only `_PRESERVE`, `_RETAIN` and `SHARED` now (`parser\decl.rs`); tests in
    `redim_typed.bas` and `redim_errors.bas`. `known_unsupported_rejections.list` shrank by two more
    (`runtime_comparison/28_redim`, `202_redim_preserve`, both `REDIM PRESERVE`: a syntax error now, as in the
    old compiler). No program of a pass list is touched (none could name the words before), so tier 2 was not
    run again; `fmt`, `clippy` and `cargo test` are green.
  - Read again and left as they are: the order of an allocation's checks against `allocarray`; the working
    variables as locals; that no path takes the address of a `_BIT` element (arguments are copies, `SWAP`, `LEN`
    and console `INPUT` are errors, `READ` and `INPUT #` go through the store).

## 5. Implicit arrays (design D5)

- [x] 5.1 `name(…)`, `LBOUND(name)` and `UBOUND(name)` for a name that may become an array by the rule of D5 make a
  static array `OPTION BASE` to 10; `OPTION _EXPLICITARRAY` makes it the measured error; a later `DIM` is the
  measured error. Verify: `typed` tests for one, two indexes, a suffix, a `DEFxxx` type, base 1; `check-fail`
  tests that each class of name D5 excludes **still** gets its old message (a `_` name, a built-in of another
  suffix, a name whose declaration was marked, a FUNCTION defined later); the "Implicit arrays" scenarios as a
  recorded slice program passing in tier 2.
  Done 2026-10-11. `implicit_array` (`check\places.rs`), called from `call`, `call_place` and `bound_fn`; the
  rest of the checker needed nothing (a later `DIM`, another number of indexes, `REDIM` and `ERASE` already
  went by the array found). Tests: `implicit_arrays_typed.bas`, `implicit_arrays_ir.bas`,
  `implicit_arrays_cpp.bas`, `implicit_arrays_errors.bas`, `implicit_arrays_explicitarray_errors.bas`,
  `implicit_arrays_explicit_errors.bas`; `arrays_errors.bas`, `member_access.bas`, `redim_unsupported.bas` and
  `unsupported.bas` lost the lines that pinned the mark (the third keeps it for a name used after a marked
  declaration). Slice `s55_implicit` (35 numbered lines, recorded from two equal runs): **the new compiler
  printed it as recorded on its first run**, also with `QB64RUST_NO_FOLD=1`.
  - **Measured first** (`verification\v24_x256`–`x263`, `study\00` §5), with five answers the design did not
    have: `OPTION _EXPLICIT` rejects such an array too, and both options apply to the whole program; a scalar
    typed with `AS` and a `CONST` of the name neither stand in the way nor type the array; `ERASE` clears it and
    is an error above the first use; in a SUB the name of an unshared main array, a parameter and a `DIM
    SHARED` scalar each make the local array; where the allocation below `$DYNAMIC` runs when the first use
    stands in the line of a block (`x262`).
  - **Beyond "a static array"**: below `$DYNAMIC` the dynamic array of D3, and in a procedure the local array
    that nothing allocates (D5's last row, decided 2026-10-11), with its per-call descriptor and the release
    of its lock in the emitter (`codegen-cpp\src\procs.rs`, after the old compiler's C++ read from `x263`).
    The 29 measurement programs of this group, run with the new compiler: 20 build and print what the old
    compiler printed or are rejected by both with a real error; 9 are still marked (`SUB … STATIC` and `STATIC
    y()` in 2, `DIM` of the array in the SUB, an array parameter, a first use in a block's line, `abs$(1)`,
    and the three the old compiler rejects that stay marks here: `f(3) = 5`, `rnd(1) = 2`, `len%(3) = 1`).
  - **Found by running `v24_x101` with the new compiler** (it printed 3 for 1): the allocation below `$DYNAMIC`
    is part of the statement that names the array first, so its error 10 skips that statement's store and
    `RESUME NEXT` goes on after the statement. `Allocate` got the flag `joined`; the lowering puts it first
    among the next statement's operations (`ir\src\lower.rs`). D3, D5, the requirement "Dynamic arrays" (a
    new scenario) and `SOMEDAY.md` say so. A first use in the line of a block is marked for that reason.
  - **Found by `tests\known_false_errors.list`**: a name with one leading underscore must keep its mark
    (`_CONNECTIONADDRESS$(…)` and `_ENCODEURL$(…)` are built-ins the table does not know by that spelling), so
    `_foo(1) = 2` is a mark, not the old compiler's "Invalid array name".
  - Not in this task: the scenario "In a SUB … STATIC" (no `STATIC` procedure compiles yet: group 6, with a
    line for it in its slice program), the whole array `nx()` (group 7).
  - My own slip, corrected: I ran one edit with backslashes through a shell heredoc (rule 7 of `CLAUDE.md`);
    the script stopped on its own check before writing, and `tools\repo_check` is clean.
  - **Review of the staged group (2026-10-11) found wrong output**, by programs built with both
    compilers: the joined allocation ran first in its statement, where the old compiler runs it where the name
    is evaluated. Measured (`verification\v24_x264`–`x268`, `study\00` §5) and marked "not supported yet"
    (`NoJoin`, `sema\src\check\mod.rs`): below `$DYNAMIC` a first use in a later item of `PRINT` or `READ`, in an
    item of `WRITE` or `PRINT #`, in a target of `INPUT` or `LINE INPUT`; and, wherever it stands, a first use in
    a bound of `REDIM` (the old compiler makes that array dynamic and allocates it as the `REDIM` does). Tests:
    `implicit_arrays_errors.bas` (six lines), `implicit_arrays_redim_errors.bas` (new). `s55_implicit.bas`
    lines 27 and 28 named new arrays in a second item: each is now two `PRINT` statements, the recorded output
    unchanged (old compiler, new compiler, and `QB64RUST_NO_FOLD=1`: pass). No program of the corpus, the
    upstream tests, the differential set, the snippets or `tests\programs` meets a new mark (`--report
    unsupported`), so the numbers of 5.2 stand; the full runs were not repeated. Design D5 "As built" and the
    requirement "Dynamic arrays" (two new scenarios) are corrected.
  - Found by the same review, **not fixed** (group 4's): a `DIM` or `REDIM` of several dynamic arrays is one
    statement in the old compiler (run twice: one error 10, a later array of the list is still allocated,
    `RESUME NEXT` goes on after the whole statement) and one statement per array here (two errors 10;
    `verification\v24_x267`). `STATUS.md`, "Known bugs".
  - The same slip again in the review's fix (a comment with `verification\v24_…` through a heredoc: three
    vertical tabs in `check\mod.rs`), found by `tools\repo_check --untracked` and repaired by a script file.
- [x] 5.2 Lists and documents as 2.3, and a check for wrong code: the full corpus and the full upstream suite run
  with the release build. Verify: no program builds and prints the wrong output; no program with an `.err` file
  starts compiling (`known_unsupported_rejections.list` only shrinks by programs that now get a real error);
  expected against passed stated here.
  Done 2026-10-11. **Full corpus** (349 programs): 293 pass (the 274 of `slice.list`, 19 `.err`), 50 rejected
  with a diagnostic, 1 `.err` with only marks (`218` `FRE`), 5 known failures; none builds and prints the wrong
  output (`tests\corpus\README.md`). **Full upstream suite** (404): 58 pass, 303 rejected, 42 `.err` with only
  marks, 1 known failure; none wrong (`tests\upstream\README.md`). `known_unsupported_rejections.list` shrank
  by two, both to the old compiler's own error ("Illegal string-number conversion" for `TRIM$(…)`, which is no
  built-in: an array with a string index): `runtime_comparison/16_ltrim_rtrim` and snippet
  `builtin_functions__trim_functions`; the other two lists are unchanged (0 and 0).
  **Expected against passed.** Expected: programs that only waited on an array used without `DIM`. Passed:
  none became clean, in the corpus (21 held back beside the screen programs, as before) or upstream (294 held
  back, as before). Every program that still shows the mark names the array after a declaration that is
  marked (member arrays, array parameters, arrays in procedures, `COMMON`, `_MEM`), where the follow-on rule
  keeps the name from becoming an array, or calls a FUNCTION of a `DECLARE LIBRARY` block or an auto-included
  file; those messages go with groups 6 to 9. Civil War Strategy: `hit(…)` and `summ(…)` are `COMMON SHARED`
  arrays (group 7). `pass.list` gained two programs that are group 4's and were not listed then
  (`arrays/negative_array_size`, `_2`: `.err` programs with the old compiler's error): 58 of 279. The
  call-site check gained no program (no libqb call is new: `free_mem_lock` is no built-in form).

## 6. Arrays in procedures (design D6, D7)

- [x] 6.1 `sema` and IR: `DIM` and `REDIM` in a SUB or FUNCTION (local, dynamic), `STATIC a(n)`, the pending list
  for `STATIC a()`, arrays of a `STATIC` procedure, `SHARED a()`, visibility of `DIM SHARED` and `REDIM SHARED`
  arrays by file order. Verify: `typed` and `ir` tests per form; `check-fail` tests per measured error ("Array
  already listed as STATIC", `DIM SHARED` in a procedure, and the rest of 1.3).
  Done 2026-10-11. `declare_array` and `implicit_array` (`sema\src\check\places.rs`), `shared_array`,
  `local_storage` and the declaration statements (`check\decl.rs`), `STATIC` after a header (`check\proc.rs`,
  `static_procs` and `static_scope` in `check\mod.rs`); design D6, "As built in group 6". Tests:
  `proc_arrays_typed.bas`, `proc_arrays_ir.bas`, `proc_arrays_errors.bas` (18 errors, one per SUB),
  `proc_arrays_unsupported.bas` (5 marks); `arrays_errors.bas`, `implicit_arrays_errors.bas`,
  `redim_unsupported.bas` and the snapshot of `proc_errors.bas` lost the lines that pinned the old marks.
  - **Measured first** (`verification\v24_x269`–`x282`, 14 programs, and the C++ the old compiler writes for
    `STATIC c(2)`; `study\00` §5), with these answers the design did not have:
    - **`STATIC a(n)` is no static array** (D3 and D6 said static): a dynamic array that lives for the program,
      allocated in the main module's start-up code, which runs again when an error handler is entered. Found
      by the recording of `s56_proc_arrays`: after `ERASE c` the old program raised error 9 where the first
      build here printed 0. The shape carries the bounds now (`Shape::Dynamic`, `at_start`); `ir::validate`
      checks it (one unit test with a broken program); both dumps show it.
    - **A procedure with `STATIC` after its header is static only down to its first `STATIC` statement**
      (`x279`, the old compiler's `dimstatic` flag). Built as measured; QuickBASIC 4.5 keeps everything:
      `DIVERGENCES-QB45.md` Q-010 and the review list of `SOMEDAY.md`. **For the user:** this follows the
      standing rule "as QB64pe" (`DECISIONS.md` 2026-10-08); say so if such a procedure should keep all of its
      variables instead (a row in `DIVERGENCES.md`).
    - `DIM a()` and `REDIM a()` in a procedure are scalars (`x273`); a listed array takes its number of
      dimensions from an index or `LBOUND` too (`x276`, `x282`); `SHARED a()` below an array the procedure has
      changes nothing, twice is accepted, and it names a main array used without `DIM` (`x274`, `x275`,
      `x277`); a `DIM` in a procedure of a shared main array is that array and raises 10 (`x270`, `x271`), in
      a `STATIC` procedure the compile error of the main module (`x281`).
  - **Changed against the task and the design:** `STATIC v(n)` with a bound that is not one number or one
    constant is "not supported yet", not a compile error: `STATIC v(2 + 3)` works in the old compiler (`x280`),
    so an error would reject programs it runs. The requirement and D6 say so.
  - `DIM SHARED` and `REDIM SHARED` in a procedure and `STATIC` in the main module are real errors now (they
    were marks); `SHARED a(3)`, `SHARED` of an array the main module has not above, and `STATIC` of an array
    the procedure has are the old compiler's errors.
  - Marked and why: `STATIC a(n)` beside a shared main array of the name (the old program ends when it starts,
    `x272`); `REDIM e AS LONG` without parentheses in a procedure (not measured there); `STATIC a() AS DOUBLE`
    then `DIM a(3)` (another array in the old compiler, `x126`: the existing mark for an array without a type
    beside one typed with `AS`).
  - Run with the new compiler, the 37 measurement programs of this group (`v24_g_local*`, `v24_g_shared`,
    `v24_f_implicit_sub`, `v24_b_metacommand_sub`, `v24_x120`–`x134`, `x176`, `x177`, `x263`, `x269`–`x282`;
    a line that calls a SUB as `name:` written as `CALL name:` in a copy, since that form is still marked):
    19 build and print what the old compiler printed, 14 are rejected by both with a real error, none
    differs; 3 the old compiler accepts are marked (`x126`, `x272`, `x280`) and 1 it fails to build is marked
    (`x121`). `v24_g_shared_order` waits on `COMMON` (group 7).
  - Not in this task: the known bug of group 4 (a `DIM` list of several dynamic arrays is one statement per
    array) now reaches every `DIM a(3), b(3)` in a procedure, where both arrays are dynamic. It shows only
    when such a `DIM` runs twice under a handler (two errors 10 for one). `STATUS.md`, "Known bugs".
  - **Review of the group (2026-10-11), fixed before the commit**, each measured first (`verification\v24_x283`–
    `x285`, `study\00` §5): in a procedure with `STATIC` after its header a plain `DIM` was taken for a `STATIC`
    statement by two checks of `declare_items` (they asked for the storage, which is `Static` there): `DIM k` on
    a `CONST` name was marked where the old compiler says "Name already in use" (`x283`), and `DIM w` on a `DIM
    SHARED` variable's name was marked where it makes the procedure's own, kept variable (`x284`); both ask for
    the declaring statement now. The mark on `DIM c(3)` after `STATIC c(3)` is gone: it is an `Allocate` and
    raises 10, as measured (`x285`). `ir::validate` rejects an array made on each call without its number of
    dimensions (one unit test), since the emitter no longer stops at one. Tests: `e17` in
    `proc_arrays_errors.bas`, `dimstatic` and `DIM sv` in `proc_arrays_typed.bas`, `u6` out of
    `proc_arrays_unsupported.bas`. `x284` and `x285` (with `CALL s` for `s:`) build here and print what the old
    compiler printed.
- [x] 6.2 Emitter: per-call descriptors, release on every way out of the procedure (strings first), program-long
  storage for `STATIC` arrays. Verify: `cpp` snapshot tests showing the release on `EXIT SUB`, at `END SUB` and
  after an error; slice program `s56_proc_arrays` (every scenario of "Arrays in procedures", a recursive procedure
  with a local array, an error inside one), recorded, passes in tier 2 with and without `QB64RUST_NO_FOLD=1`.
  Done 2026-10-11. Local arrays needed no new emitter code: group 5's per-call descriptor and its release in
  `freeK.txt` serve them, and `EXIT SUB`, `END SUB` and an error all pass `exit_subfunc` (`proc_arrays_cpp.bas`
  shows the three). New: `allocate_at_start` (`codegen-cpp\src\arrays.rs`) writes the allocation of a `STATIC
  a(n)` array into `maindata.txt` after its descriptor, and a listed array that nothing names gets a
  descriptor of one dimension (`decl.rs`). Slice `s56_proc_arrays` (26 numbered lines, recorded from two equal
  runs): the first build here differed in line 24 (`ERASE` of a `STATIC` array, above); after that correction
  it prints as recorded, also with `QB64RUST_NO_FOLD=1`.
- [x] 6.3 Lists and documents as 2.3. Verify: the message "arrays in a SUB or FUNCTION" and "`SHARED` of an array"
  are gone from the three reports; expected against passed stated here.
  Done 2026-10-11. Neither message, nor "`STATIC` after a procedure header", is left in the reports of the
  corpus, the upstream tests and `tests\programs` (the messages no longer exist in the compiler).
  `known_unsupported_rejections.list` shrank by one, to the old compiler's own error
  (`snippets/qb64fresh/shared_variables__shared_array`: "Array 'scores' not defined"); the other two lists are
  unchanged (0 and 0). **Expected against passed**, counted with the compiler of group 5 before the change:
  corpus, 1 program carried these marks (`runtime_comparison/206_shared_redim`, named in 4.3): it became clean
  and passes, on `slice.list` with `s56_proc_arrays` (276). Upstream, 3 programs carried "arrays in a SUB or
  FUNCTION": none became clean (`arrays/t001_whole_array_core`: whole arrays and `SYSTEM` with an exit code;
  `arrays/t003_whole_array_member_assign`: member arrays; `types/static_array`: `_MEM`), so `pass.list` stays
  at 58 of 279. Civil War Strategy: three `DIM` lines in SUBs carried "arrays in a SUB or FUNCTION" (`DIM
  hit(2)`, `DIM summ(40)` among them); they compile, and the 15 marks on `hit(…)` and `summ(…)` went with them
  (5.2 took the two for `COMMON SHARED` arrays; they are local): 1,721 marks in 34 messages by the report,
  where the same count gave 1,739 in 37 before.
  Group 6 closes with `fmt`, `clippy` and `cargo test` green, tier 2 slice 276 of 276 (also with
  `QB64RUST_NO_FOLD=1`), the upstream pass list 58 of 58, the differential list 59 of 59, the call-site check
  256 of 256 (no program added: the allocation is inline C++) and the runner's tests 56 of 56. The full corpus
  ran with the release build (`tests\corpus\README.md`); the full upstream suite did not: no upstream program
  changed from rejected to compiled, and tier 1 keeps every `.err` program rejected.

## 7. Array parameters and `COMMON` (design D6)

Before the group (2026-10-11), the two known bugs of `STATUS.md` were fixed, each measured first:
- **A `DIM` or `REDIM` of several dynamic arrays is one statement** (group 4's; found by the review of group
  5, `verification\v24_x267`). `declare_items` (`sema\src\check\decl.rs`) marks the `Allocate` of every array of
  the list but the last `joined`, so the lowering puts them into the last one's statement: one error 10, a later
  array of the list is still allocated, `RESUME NEXT` goes on after the whole list. `v24_x267` built with the new
  compiler prints what the old one printed. Four snapshots changed by it (`proc_arrays_*`, `redim_typed`).
- **A unary plus was a parse error where the old compiler drops the sign.** Measured (`verification\v25_signs`,
  `v25_signs_const`, `v25_x01`–`x10`, and 99 one-line forms tried with both compilers; `study\00` §5): the old
  compiler has no unary plus, it removes signs from the text of an expression: a `+` right after a `+` or a `-`,
  then two `-` right after any operator. In a `CONST` every run of `+` and `-` is one sign. Built in the parser
  (`syntax\src\parser\expr.rs`, `signs_after`, `sign_run`) and in `sema`'s `CONST` evaluator; the dropped tokens
  stay in the tree. Both measurement programs build and print what the old compiler printed (with the unsigned
  literal and the negative power in a `CONST` they also hold written another way: both are marked). Tests:
  `signs_typed.bas`, `unary_plus.bas` (8 more rejections), `const_errors.bas` (3 more). Two tests of the
  expression depth limit used 1,000 and 20,000 minus signs, which are now dropped in pairs: they use `NOT`.
- Each fix has a slice program, recorded from two equal runs and passing in tier 2, also with
  `QB64RUST_NO_FOLD=1`: `s52_dim_list` (7 numbered lines) and `s17_signs` (17). The requirement "Dynamic
  arrays" has the sentence and a scenario for the list; the signs have no requirement of their own (the
  operators' requirements are in `language/numeric-semantics`, which this change does not touch): their rule
  is in `study\00` §5 and in the tests.

- [x] 7.1 Array parameters: the parameter shape, the number of dimensions from the body, calls judged after every
  body, `Arg::Array`, the descriptor pointer in the emitter, `REDIM` and `ERASE` of a parameter. Verify: `typed`,
  `ir` and `cpp` tests; `check-fail` tests for each rejection, each with the call above and below the procedure;
  slice program `s57_array_params` (every scenario of "Array parameters"), recorded, passes in tier 2; upstream
  `arrays/array_arg_dimensions_transitive` and `arrays/comma_array_parameter` pass and join `pass.list`.
  Done 2026-10-11. `declare_array_param`, `array_arg` and `array_kind` (`sema\src\check\proc.rs`), `DimFact`
  and `solve_dims` (`check\mod.rs`), `settle_dims` and `open_dims` (`check\places.rs`); `Arg::Array` and
  `EraseParam` in the typed tree and the IR, with their checks in `ir::validate` (two unit tests); in the
  emitter an array parameter is the descriptor of the array passed (`ptrszint*`), and `erase_param`
  (`codegen-cpp\src\arrays.rs`) tests its flag. Design D6, "As built in group 7". Tests:
  `array_params_typed.bas`, `array_params_ir.bas`, `array_params_cpp.bas`, `array_params_errors.bas` (28
  errors; the calls stand above their SUBs in the main module and below them in `SUB below`). Slice
  `s57_array_params` (23 numbered lines, recorded from two equal runs): **the new compiler printed it as
  recorded on its first run**, also with `QB64RUST_NO_FOLD=1`. Both upstream programs pass and are on
  `pass.list`.
  - **Measured first**, beyond 1.3 (`verification\v24_x286`–`x296`, `study\00` §5), with these answers the
    design did not have:
    - An array made by passing a name that names none has **one dimension** (0 to 10), whatever the SUB does
      with its parameter (`x286`, `x287`, `x293`); below `$DYNAMIC` it is a dynamic array allocated as part of
      the calling statement (`x295`). **In a procedure it is a compile error** ("Expected (...)", `x292`), where
      the spec delta said "MAY be passed".
    - **An array listed by `COMMON a()` or `STATIC a()` may be passed before anything gave it a number of
      dimensions, and gets the parameter's** (`x288`, `x289`). So the unknown number is not only a
      parameter's: such an array joins the parameter's facts from the call on.
    - An array parameter and a scalar parameter of one name are two variables (`x290`); two array parameters of
      one name and type are "Name already in use" (`x291`). A number between a parameter's parentheses is
      accepted as measured in 1.3 (`a(1)`, `x157`): the parser rejected it and now takes numbers and commas.
  - **One number of dimensions per parameter, solved after the bodies.** Every index count, every `DIM` or
    `REDIM` of the parameter and every array passed is one fact, kept in file order; `solve_dims` reads them
    after the last body, so the error stands at the later of two that disagree, as measured (`x148`–`x151`,
    `x155`), and a call above the procedure is judged like one below. `LBOUND` and `UBOUND` of a parameter say
    nothing (`x153`).
  - Run with the new compiler, the 41 measurement programs of array parameters (`v24_g_params`, `v24_x135`–
    `x160`, `x178`–`x180`, `x286`–`x296`): 21 build and print what the old compiler printed, 20 are rejected by
    both with a real error (`x145`, an array in parentheses, which fails the old compiler's C++ build, among
    them); none differs and none is marked.
  - Marked and why (each not measured): an array parameter named `t$n()`, one typed `_UNSIGNED STRING`, two
    array parameters of one name and two types (`common_unsupported.bas`). A FUNCTION's name passed as `f()`
    keeps the mark of an array used without `DIM` where the old compiler says "Expected arrayname()".
- [x] 7.2 `COMMON` and `COMMON SHARED`: scalars, arrays through the pending list, a block name, the placement
  errors; `CHAIN` stays marked. Verify: `typed` and `check-fail` tests; slice program `s58_common` (every scenario
  of "COMMON"), recorded, passes in tier 2.
  Done 2026-10-11. `common` (`sema\src\check\decl.rs`) is `declare_items` with `Declarer::Common`: a scalar goes
  the way of `DIM`, `name()` lists a dynamic array without reading what stands between its parentheses, and
  `SHARED` works as in `DIM SHARED`. No pending list (design D6): the listed array is found by name and type
  like any other. `ast::CommonStmt` got `shared`, `block` and `as_clause`. Tests: `common_typed.bas`,
  `common_errors.bas` (8 errors), `common_unsupported.bas`. Slice `s58_common` (14 numbered lines, recorded
  from two equal runs) passes, also with `QB64RUST_NO_FOLD=1`.
  - **Found by the first run of `s58_common`** (the new compiler rejected it: "`ign` has 1 dimension, not 2"),
    then measured (`verification\v24_x297`–`x302`, `study\00` §5): **`LBOUND` and `UBOUND` of an array listed
    by `COMMON a()` say nothing about its number of dimensions** (`UBOUND(c)` then `DIM c(2, 5)` or `c(1, 2)`
    compiles), where those of one listed by `STATIC a()` make it one (`x282`, `x297`). An index gives the
    number in both, and a `DIM` or `REDIM` with another one after it is the compile error (`x299`, `x300`).
  - Run with the new compiler, the 22 measurement programs of `COMMON` (`v24_g_common`, `v24_a_common`,
    `v24_x161`–`x175`, `x181`, `x298`, `x299`, `x301`, `x302`) and `v24_g_shared_order`, which waited on it: 15
    build and print what the old compiler printed (two of them end with the runtime's error box, as recorded),
    8 are rejected by both with a real error; none differs.
  - Marked: `COMMON a()` of an array that exists already (not measured); `CHAIN`.
- [x] 7.3 Lists and documents as 2.3; `tests\programs\README.md` updated with what Civil War Strategy still needs.
  Verify: `COMMON`, `hit(...)`, `summ(...)`, "arrays in a SUB or FUNCTION" and `DEFINT` are gone from
  `qb64rust --report unsupported tests\programs`; expected against passed stated here.
  Done 2026-10-11. None of the five is left in the report of `tests\programs`: Civil War Strategy has 1,709
  marks in 33 messages (1,721 in 34 before; the 12 `COMMON` lines went), still no real error, though the
  follow-on rule no longer hides any: its first marked declaration is gone. The messages "array parameters"
  and "`COMMON`" no longer exist in the compiler. `known_unsupported_rejections.list` shrank by two, both to
  the old compiler's own error ("Whole array cannot be passed to a scalar SUB parameter"):
  `upstream/arrays/t045_function_scalar_array` and `t046_sub_scalar_array`; the other lists are unchanged (0
  and 0). **Expected against passed**, counted with the compiler as it was before the group: corpus, 1
  program carried these marks (`runtime_comparison/194_array_param_2d`): still marked (a FUNCTION typed with
  `AS`); the two new slice programs pass, and with the two of the bug fixes `slice.list` has 280. Upstream, 7 programs carried them: 2 became
  clean and pass (the two of 7.1), 5 wait on something else (`arrays/type_dynamic_string_member_array_param`
  and `arrays/type_member_arrays_as_parameters`: `STRING` members and member arrays; `offset/offset_test`:
  `_MEM`; `qb64pe/element` and `hasFunctionElement`: `CVL`, `STRING` members). With `t045` and `t046`
  `pass.list` has 62 of 279.
  Group 7 closes with `fmt`, `clippy` and `cargo test` green, tier 2 slice 280 of 280 (also with
  `QB64RUST_NO_FOLD=1`), the upstream pass list 62 of 62, the differential list 59 of 59, the call-site check
  256 of 256 (no program added: an array argument is a pointer, no libqb call) and the runner's tests 56 of
  56. The full corpus (354 programs) and the full upstream suite (404) ran with the release build: 299 and 62
  pass, every other program is rejected when compiling; none builds and prints the wrong output
  (`tests\corpus\README.md`, `tests\upstream\README.md`).
  **Review of the group (2026-10-11), one fix:**
  - **a false syntax error from 7.1:** the parser took only commas and numbers between an array parameter's
    parentheses, so `SUB s (a(n))`, `a(n + 1)`, `a(-1)` and `a(1 TO 5)` were rejected where the old compiler
    accepts them. Measured (`verification\v24_x303`–`x305`, `qb64pe.bas` 2882–2904, `study\00` §5): every word up
    to the `)` is skipped; a second `(` and the word `AS` there are syntax errors. The parser does that now
    (`syntax\src\parser\proc.rs`), and `ast::Param` reads the name and `BYVAL` from the words before the `(`.
    Tests: `skipped` in `array_params_typed.bas`, `array_params_parens_errors.bas` (new). `x303` built with the
    new compiler prints what the old one printed; `x304` and `x305` are rejected by both. No program of a pass
    list names the form, so tier 2 was not run again; `fmt`, `clippy` and `cargo test` are green.
  - Seven more programs built with both compilers and equal (a listed array passed while nothing gives it a
    number of dimensions, a parameter of a SUB never called, an array passed on through two SUBs, `REDIM` and
    `ERASE` of a parameter for a static and a dynamic array, 25 sign forms).

## 8. The rest of `TYPE` (design D9)

- [x] 8.1 `STRING` members: the member type, set-up and release per variable and per array element (main, local,
  `STATIC`, in `Allocate`, `Preserve`, `Clear`, `Release`), the compile errors measured in 1.4 (`LEN` of such a
  type). Verify: `typed`, `check-fail` and `cpp` tests; the two "String member" scenarios in a recorded slice
  program `s59_type_strings` passing in tier 2.
  Done 2026-10-11. `member_type` (`sema\src\check\places.rs`) takes `STRING`; `holds_strings` (`sema\src\lib.rs`)
  says whether a type has one somewhere in it, and `LEN` of a place of such a type is the measured error
  (`check\builtins.rs`). In the emitter a `STRING` member is the pointer to its string in the value's bytes
  (8 bytes, as in the old compiler); `parts`, `new_strings`, `free_strings`, `zero_value` and `each_element`
  (`codegen-cpp\src\decl.rs`) walk a type as the old compiler's `AppendLegacyInlineUDTVarOp` does. Every value
  gets its strings where it is made (a variable and a static array in `maindata.txt`, a local in `dataK.txt`, a
  dynamic array in its allocation, also the one of `STATIC a(n)`) and loses them where it goes (`mainfree.txt`,
  `freeK.txt`, `Release`, the old block of a `REDIM`); `Clear` and `clear.txt` empty them; `Preserve` copies the
  kept elements part by part. A store into a `STRING` member of an element copies the value first, as for a
  fixed-length one, and follows D-004. Design D9, "As built in group 8". Tests: `type_strings_typed.bas`,
  `type_strings_cpp.bas`, `type_whole_errors.bas` (4 `LEN` errors); `types_errors.bas` lost the forms that are
  valid now. Slice `s59_type_strings` (19 numbered lines, recorded from two equal runs): **the new compiler
  printed it as recorded on its first run**, also with `QB64RUST_NO_FOLD=1`.
  - Run with the new compiler, the measurement programs of `STRING` members (`v24_h_string_members`,
    `v24_x206`, `x207`, `x216`): `x207` and `x216` build and print what the old compiler printed, `x206` is
    rejected by both. `v24_h_string_members` is still rejected, for a reason outside the group: its
    `local: local` is read as a label with the name of a SUB, which is marked; with the two such lines split
    it prints what the old compiler printed.
- [x] 8.2 Whole values: `CopyWhole` for `q = p` between variables, elements and members, with its measured error
  rule; `SWAP` of two places of one user type; a `TYPE` parameter by reference. Verify: `typed`, `ir`, `cpp` and
  `check-fail` tests; slice program `s60_type_whole` (every scenario of "Whole user-type values" and "User-type
  parameters"), recorded, passes in tier 2 with and without `QB64RUST_NO_FOLD=1`.
  Done 2026-10-11. `whole_place` (`check\places.rs`) reads the place a whole value is taken from; `assign`
  (`check\mod.rs`) makes `StmtKind::CopyWhole { from, to }`, and `type_arg` (`check\proc.rs`) the `Arg::Ref` for
  a parameter of a user type. `Op::CopyWhole` in the IR, with its checks and those of such an argument in
  `ir::validate` (two unit tests). In the emitter `copy_whole` (`codegen-cpp\src\stmt.rs`), and a parameter of a
  user type is the address of the place passed (`void*`). `SWAP` needed nothing: it exchanged the bytes of two
  places of one user type already, and the bytes of a `STRING` member are its pointer. Tests:
  `type_whole_typed.bas`, `type_whole_ir.bas`, `type_whole_cpp.bas`, `type_whole_errors.bas` (16 more errors).
  Slice `s60_type_whole` (21 numbered lines, recorded from two equal runs): **printed as recorded on its first
  run**, also with `QB64RUST_NO_FOLD=1`.
  - **Measured first** (`verification\v24_x306`–`x308`, `study\00` §5), with an answer the design did not have:
    **the target is found before the source, and for a type with a `STRING` member somewhere in it both are
    found again for every part of the value** (a `STRING` member, a member of any other type, a member of a
    user type that holds no string; the parts of a member that holds strings in its place). A FUNCTION in an
    index is called once per part: `a(one(1)) = b(one(2))` prints `1 2` for a type of numbers and `1 2 1 2 1 2`
    for one of a LONG, a `STRING` and a `STRING * 2`. The old compiler writes both addresses into each
    `memcpy` and `qbs_set` of its copy. **Built as measured**, by the rule of 2026-10-08 (a deterministic
    behaviour is kept), and listed for review in `SOMEDAY.md`; the IR says so in `Op::CopyWhole`. No decision
    was asked for: it is the old compiler's behaviour, not a difference from it.
  - The copy is not guarded, as decided in 1.4: `a(9) = p` raises 9 and overwrites `a(0)` (slice line 8).
  - The compile errors: another user type and a number on the right of `=`; a place of another type, a number
    and a place in parentheses for a parameter of a user type; a whole value for a parameter that is none (the
    mark "a whole `TYPE` value as an argument" became this error). `FUNCTION make AS t1` was an error already.
  - Run with the new compiler, the measurement programs of whole values and parameters (`v24_h_whole`,
    `v24_h_type_params`, `v24_x208`–`x215`, `x306`–`x308`): 6 build and print what the old compiler printed, 7
    are rejected by both with a real error; none differs.
- [x] 8.3 Lists and documents as 2.3. Verify: "`STRING` members" and "assigning a whole `TYPE` value" are gone from
  the three reports; expected against passed stated here.
  Done 2026-10-11. Neither is left in `qb64rust --report unsupported` of `tests\corpus`, `tests\upstream` and
  `tests\programs`, nor is "a `TYPE` parameter"; the three messages no longer exist in the compiler. The
  shrink-only lists are unchanged (0, 0 and 55). **Expected against passed**, counted with the compiler as it
  was before the group: corpus, 3 programs carried these marks, all 3 are clean and pass
  (`runtime_comparison/57_type_string_field`, `200_type_assign_whole`, `242_types_simple`); with the two new
  slice programs `slice.list` has 285. Upstream, 29 programs carried them: 2 are clean and pass
  (`types/nested_udt`, `types/udt`), 27 wait on something else, by the first thing each still names: member
  arrays 19 (15 by that message, 4 by a type or an array that follows from one), `SYSTEM` with an exit code 3,
  `MKL$` and `CVL` 2 (`qb64pe/element`, `hasFunctionElement`), `REDIM _RETAIN` 1, a whole array `na()` 1
  (`types/udt_array`), an include from the old compiler's sources 1 (`qb64pe/file`). `pass.list` has 64 of 279.
  The upstream report now names member arrays in 156 programs (152 before: four were behind `STRING`
  members).
  Group 8 closes with `fmt`, `clippy` and `cargo test` green, tier 2 slice 285 of 285 (also with
  `QB64RUST_NO_FOLD=1`), the upstream pass list 64 of 64, the differential list 59 of 59, the call-site check
  256 of 256 (no program added: no new libqb call) and the runner's tests 56 of 56. The full corpus (356
  programs) and the full upstream suite (404) ran with the release build: 304 and 64 pass, every other program
  is rejected when compiling; none builds and prints the wrong output (`tests\corpus\README.md`,
  `tests\upstream\README.md`).
  **Review of the group (2026-10-11), nothing to fix:** one more program built with both compilers and equal
  (`verification\v24_x309`, 12 numbered lines): a `STRING` member of an element by reference, an array of such
  a type as an array parameter with `REDIM _PRESERVE` and `ERASE` in the SUB (a static and a dynamic argument),
  `STATIC k(2)` of it, a `DIM SHARED` variable, a recursive SUB with a local, a member of a parameter passed
  on, a whole value between two array parameters, `SWAP` of two elements, `EXIT SUB` with a local, a FUNCTION
  result stored into a member.

## 9. Member arrays (design D8; as decided in 1.4)

**Measured before and while the group was built (2026-10-11), 17 programs `verification\v24_x310`–`x326`, all
recorded; findings in `study\00` §5 and design D8, "As built in group 9".** What they changed in the plan:

- **`LBOUND`, `UBOUND` and `LEN` never find the place a member array is part of** (`x311`): `UBOUND(a(f(1)).v)`
  does not call `f`, `LBOUND(a(9).v)` raises nothing. `REDIM _PRESERVE` neither (`x312`). `ERASE` and `REDIM` do,
  once, and are not guarded: an index outside raises 9 and the member array of element 0 is cleared.
- **A string stored through a member is evaluated after the indexes, a number before them** (`x313`, `x316`):
  `a(setz(1)).s = z$` stores what `setz` put into `z$`, also for `z$ + "!"`, a `STRING * n` member and an element
  of a member array. The old compiler's string store is one call whose first argument is the place. **The code
  of groups before this one copied the string first** (`a(i).s = z$` stored the old value): corrected here, the
  spec deltas and the IR's rule say so, `s61` line 17 pins it.
- **A whole value goes element by element only where the elements are parts** (`x310`): each element of a
  `STRING` member array is a part, each element of one of a user type that holds strings gives its own parts,
  any other member array is one part.
- **A whole member array as an array argument works with a descriptor made for the call** (`x315`): in the
  procedure the bounds are the member's, `ERASE` clears, `REDIM` raises 10; `fill a(9).v()` raises 9 and the
  procedure is not entered. Built (9.2).
- `SWAP` takes an element of a member array, and a member of an element, like an argument by reference (`x314`):
  both compile now (the second was marked before).
- Ten forms settled by one program each (`x317`–`x326`): six compile errors (`t.m(1)` for a member the type does
  not have, an index after a member that is no array, `LBOUND` of one, `fill t.v` without parentheses, `REDIM AS
  LONG t.v(…)`, `PRINT t.v()`), `ERASE t.v()` and `REDIM SHARED t.v(…)` accepted, and the upper bound alone in a
  `REDIM … AS` going by `OPTION BASE` as it stands at the `REDIM`.

- [x] 9.1 `sema` and IR: bounds on a member in both declaration forms (with `OPTION BASE` as it is at the `TYPE`
  block, no literal-0 rule), `ArrayRef::Member`, elements through any chain, `LBOUND`/`UBOUND` of a member array
  (also `t.v()`), the store rule of D-004 at every index, `REDIM` and `ERASE` of a member array lowered to `Clear`
  and `REDIM _PRESERVE` to nothing with the compile errors of 1.4, the `_STATIC`/`_DYNAMIC` markers still marked;
  `DIVERGENCES.md` D-004 gains the member-array forms. Verify: `typed` and `ir` tests for `t.v(i)`, `a(i).v(j)`
  and `x(0).n(1).q(2)`; `check-fail` tests for a wrong index count, no index, a string index, a marked member, a
  bound that is not constant, and each rejected `REDIM` form.
  Done 2026-10-11. `Member` has `bounds`; `ArrayRef` (a variable, or a member of a place) names the array of
  an element, of `LBOUND`/`UBOUND`, of `Clear` and of an array argument, in the typed tree and the IR.
  `OPTION BASE` is collected per `TYPE` block by a walk before the blocks are read (`collect_option_bases`).
  `ir::validate` knows both kinds of array and that a member array is never a single value; its own test
  `member_arrays_are_named_as_arrays`. Tests `tests\frontend\member_arrays_typed.bas`, `_ir.bas`,
  `_errors.bas` (27 real errors, 10 marks: two of them, bounds and a `TYPE` size beyond 64 bits, were compiler
  crashes found by the review of the group, 2026-10-12).
  - **Narrower than written, each marked "not supported yet" by its own message and listed in `SOMEDAY.md`:**
    (1) **a bound of a member array that names a `CONST`** (accepted by the old compiler, `v24_x196`): the
    `TYPE` blocks are read before the program's `CONST`s, so a bound must be written with numbers alone, also
    as an expression of numbers; no upstream program has another. (2) A member array of an array element
    beside other targets of one `ERASE` or `REDIM` (`ERASE a(1).v, t.v`): the old compiler goes on with the
    other targets before the error of an index is handled, and each clear here is a statement of its own.
    (3) An element of a member array as a target of `INPUT`, `READ` or the `MID$` statement, as a member of an
    element already was: not measured. (4) `REDIM t.v(0) AS type` under `OPTION BASE 1`: not measured.
- [x] 9.2 Emitter: the inline layout (1.4, case (a): the member's offset plus the checked flat position times
  the element size), elements passed by reference, `Clear` of a member array, whole-value copy and `STRING`
  members inside member arrays, and a whole member array as an array argument if a descriptor made for the call
  is all it takes (D8; otherwise marked and named in `SOMEDAY.md`). Verify: `cpp` snapshot tests; slice program
  `s61_member_arrays` (every scenario of "Member arrays" and the member-array scenario of "LBOUND and UBOUND"),
  recorded, passes in tier 2 with and without `QB64RUST_NO_FOLD=1`; `runtime_comparison/197_type_array_field`
  passes.
  Done 2026-10-11. A read checks each index with `array_check` against the member's constant bounds; the store
  through a member is one routine for every such place (`store_through_member`, the byte distance added up
  index by index and the address formed last); `LBOUND`/`UBOUND` are constants, or picked by a dimension
  evaluated once into a `passN`; the array argument is a `passN` descriptor with the bounds in its declaration.
  `tests\frontend\member_arrays_cpp.bas`; `s61_member_arrays` (37 numbered lines) passes with and without
  folding, as does the whole slice list (287 of 287); `197_type_array_field` passes and is on `slice.list`.
  D-004 for member arrays cannot be a recorded program: command-line test `member_array_store_with_bad_index`
  (builds and runs, `#[ignore]`d, passes; `member_store_with_bad_index` still passes). The 44 member,
  whole-value and member-array programs of `verification\` (`v18_b_*`, `v18_h_member*`, `v24_h_*`,
  `v24_x182`–`x226`, `x306`–`x326`) through both compilers: 36 give the same result (the same output, or both
  reject), 4 differ only as D-004 says (`v24_h_member_store`, `v24_x313`, `v18_b_index_raises`,
  `v18_b_member_store`), 4 are rejected here by a mark (`x196` a `CONST` bound, `x312` a list of targets, and
  two for a label named like a SUB, which is no part of this group).
- [x] 9.3 Lists and documents as 2.3, with the full upstream suite run. Verify: "arrays as `TYPE` members" is gone
  from the corpus report and, outside `deferred.list`, from the upstream report except for marked members; no
  upstream program builds and prints the wrong output; expected (59 upstream programs naming member arrays)
  against passed stated here; `SOMEDAY.md` says what of the member-array layer is still deferred.
  Done 2026-10-11. The message is gone from both reports (the marked members have their own, 86 upstream
  programs). **Full upstream run (404 programs, release build, `--jobs 8`): 114 pass, none builds and prints
  wrong output**; 251 rejected with only "not supported yet", 38 `.err` programs with only marked errors, 1
  known failure. **Expected 59** (the proposal's count of programs outside `deferred.list` that name member
  arrays), **50 newly pass**, all in `arrays/`: 48 programs the old compiler builds, and
  `arrays/41_err_undefined_array` and `44_err_solved_bug_Incorrect_nr_of_args` (two `.err` programs, now
  rejected by a real error and off `known_unsupported_rejections.list`, 53 left). What holds the others back
  is in the demand report (the first lines: marked members, `$Unstable`, `_DEST`, `$CONSOLE`, `_MEM`).
  `pass.list` 64 to 114; `slice.list` 285 to 287; `tests\upstream\README.md` and `SOMEDAY.md` say the rest.

## 10. Integration

- [x] 10.1 Full runs with the release build against the reference clone: tier 2 slice (both fold settings), the
  full corpus, the upstream pass list and the full upstream suite, the differential pass list, the call-site
  check, the runner's own tests. Verify: none wrong at run time; every clean program is on a list; the numbers go
  into `STATUS.md` ("Numbers at the last full runs") and `tests\upstream\README.md` (a dated "Upstream progress"
  paragraph).
  Done 2026-10-12, at commit `3ba82c2` with a clean working tree, `--jobs 8 --build-cache`:
  - `cargo test --workspace --locked` passes (it holds "every clean program is on a list", the three shrink-only
    lists and the differential programs equal to the generator's); the 13 `--ignored` command-line tests that
    build an executable pass; the runner's own tests: 56 pass.
  - `slice.list`: 287 of 287 with folding and 287 of 287 with `QB64RUST_NO_FOLD=1` (about 3 min 20 s each).
  - Full corpus (357): 306 pass, 45 rejected at compile time (44 with an `.output`, the 29 screen programs
    among them, and the compile-only `239_lprint`), 1 `.err` program with only marked errors, 5 known failures;
    none wrong at run time.
  - Upstream: `pass.list` by itself 114 of 114; the full suite (404): 114 pass, 251 rejected at compile time, 38
    `.err` programs with only marked errors, 1 known failure; none wrong at run time.
  - Differential pass list: 59 of 59. Call-site check: 256 of 256 equal.
  - Every number equals the one after group 9, so the lists did not change.
- [x] 10.2 Documents: `STATUS.md` (step 10 done, step 11 next, its demand line refreshed from
  `--report unsupported`), `ROADMAP.md`, `crates\README.md`, `tests\corpus\README.md`, `study\00` (§5 complete;
  the document map if a study was added), `SOMEDAY.md`, `DECISIONS.md` (one row per decision taken in the
  change), `CLAUDE.md`'s layout table (the `verification\` row gains `v24*`). Verify:
  `python tools\repo_check\check_repo.py --untracked` clean; `openspec validate m2-defxxx-common-arrays --strict`
  passes; every task above is ticked or its remainder is named in `STATUS.md`.
  Done 2026-10-12. `STATUS.md`: step 10 done, step 11 next, the table of numbers from 10.1, the demand line from
  the report (upstream with `--include-root tests\upstream\root`: 96 without errors, 242 held back, 10 with other
  errors; the first lines as after group 9). `tests\upstream\README.md` and `tests\corpus\README.md`: the runs of
  10.1. `study\00`: §5 was complete (the `v24_*` and `v25_*` facts came in group by group); the M2 row of §2
  named the state of 2026-10-07 and now names the steps done, without numbers. No study was added.
  - **`ROADMAP.md` was not on disk and was never committed** (`study\26` §7 calls it "new, untracked"; the
    closing tasks of the changes before this one name it). Written again from `study\00` §2, `STATUS.md`
    ("Queued for later milestones") and `DECISIONS.md` (2026-10-07: milestones and done or not done only;
    2026-10-09: the formatter under "Not tied to a milestone yet"); it says so in its head.
  - Nothing to change, checked: `crates\README.md` (each group wrote its part; no progress numbers there),
    `SOMEDAY.md` (the four marked forms of the member-array layer and the behaviours kept as QB64pe are listed),
    `CLAUDE.md`'s layout table (the `verification\` row has `v24*` and `v25*`; 357 corpus programs, 287 on
    `slice.list`), `DECISIONS.md` (the change took one set of decisions, the three answers to task 1.4, row
    2026-10-11; everything else was built as measured under the standing rule "as QB64pe", or is D-004 applied
    to more places).
  - `check_repo.py --untracked`: 6711 files, 0 findings. `openspec validate m2-defxxx-common-arrays --strict`:
    valid. Every task is ticked; what stays marked is named in `STATUS.md` step 10 and in `SOMEDAY.md`.
