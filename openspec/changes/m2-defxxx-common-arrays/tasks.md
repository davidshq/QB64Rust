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

- [ ] 9.1 `sema` and IR: bounds on a member in both declaration forms (with `OPTION BASE` as it is at the `TYPE`
  block, no literal-0 rule), `ArrayRef::Member`, elements through any chain, `LBOUND`/`UBOUND` of a member array
  (also `t.v()`), the store rule of D-004 at every index, `REDIM` and `ERASE` of a member array lowered to `Clear`
  and `REDIM _PRESERVE` to nothing with the compile errors of 1.4, the `_STATIC`/`_DYNAMIC` markers still marked;
  `DIVERGENCES.md` D-004 gains the member-array forms. Verify: `typed` and `ir` tests for `t.v(i)`, `a(i).v(j)`
  and `x(0).n(1).q(2)`; `check-fail` tests for a wrong index count, no index, a string index, a marked member, a
  bound that is not constant, and each rejected `REDIM` form.
- [ ] 9.2 Emitter: the inline layout (1.4, case (a): the member's offset plus the checked flat position times
  the element size), elements passed by reference, `Clear` of a member array, whole-value copy and `STRING`
  members inside member arrays, and a whole member array as an array argument if a descriptor made for the call
  is all it takes (D8; otherwise marked and named in `SOMEDAY.md`). Verify: `cpp` snapshot tests; slice program
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
