# Tasks

Order: group 1 first (the measurements of `FIELD`, `LSET`, `RSET`), then 2 to 7 (design D10; 2 to 5 do not depend
on one another, 6 needs 5), then 8. Each of groups 2 to 7 ends with `cargo fmt --check`, `cargo clippy --workspace
--all-targets -- -D warnings` and `cargo test` (tier 1) green and with its tests, lists and `crates\README.md`
lines in place, so the change can stop after any group.

Standing rules for every task: work inline, no subagents; nothing staged or committed by Claude: each group ends
by naming the files to stage and a commit message; text with backslashes goes through the Edit and Write tools;
`python tools\repo_check\check_repo.py --untracked` is clean before a group is reported done. **Test cadence**
(`CLAUDE.md` working rule 9): while working, tier 1 and only the programs the work touches (`--glob`,
`--category`, or a list with `--pass-cache target\pass-cache`); tier 2, the run without folding and the full suites
run once, in group 8. CI is not a step of this change. A new measured fact goes into the `v30` paragraph of
`study\00` §5 in the task that measures it. A behaviour of the old compiler that looks questionable is implemented
its way and listed in `SOMEDAY.md` ("QB64pe behaviours to review"). A form that turns out not to parse today is
fixed in `crates\syntax` in the task that finds it, with a parser test.

## 1. Measurements before code

- [x] 1.1 Measure `FIELD`, `LSET` and `RSET` (design D3), one topic or one form per program
  (`verification\v30_j…` and `v30_x100` on), the C++ read with `qb64pe -z` outside the repository: the calls of
  `FIELD` with one and several parts; the targets accepted and rejected; the forms of the file number; `FIELD`
  on a file that is not random, with widths beyond the record, a second time on one file; `=` to a field
  variable; `GET` and `PUT` with no variable through fields; `GET` with a variable beside fields; `LSET` and
  `RSET` with a shorter and a longer value, on a fixed-length string, a `TYPE` value, a number, an array
  element. Verify: `verification\` holds the programs with their `.compile.txt` and `.out.txt` (recorded by
  `--suite verification`); the `v30` paragraph of `study\00` §5 has the facts; the scenarios of `FIELD`, `LSET`
  and `RSET` in `specs\language\record-files\spec.md` name forms and values the old compiler does give,
  corrected if not; design D3 says what was found in place of "the shape expected"; the reference clone's `git
  status --porcelain` is empty before and after.
- [x] 1.2 Record the row of `v30*` in `CLAUDE.md`'s layout table for the new programs, and tell the user if a
  form of `FIELD`, `LSET` or `RSET` has to stay "not supported yet" (D3), with the reason. Verify: the row names
  the new programs; `python tools\repo_check\check_repo.py --untracked` is clean.

## 2. `_IIF`, `_MIN`, `_MAX`, `_CLAMP`, `_SHL`, `_SHR`, `_ROL`, `_ROR`

- [x] 2.1 `ops::widest` with a unit test that reads `verification\v30_result_types.txt` (design D6). Verify:
  the test compares all 794 forms and passes; taking one rule out (the unsigned one of two of a width) makes
  it fail.
  - Done: `widest` and `rotated` in `sema\src\check\ops.rs`, the test
    `widest_and_rotated_give_the_measured_types` (794 forms and the 6 rejected ones); with the unsigned rule
    out it fails at `_MIN(a, b) | _BIT | _UNSIGNED _BIT`. **Found:** the rule works on what the arguments are
    *believed*, and there is no difference between held and believed beyond the usual one (a `_BIT` type held
    in its storage): the result of `_MIN(o, q)` is believed `_OFFSET` (`_MIN(o, o) ^ 2` is the old compiler's
    `_OFFSET` error), not `_INTEGER64` as design D6 said. Measured with 110 more forms read with `qb64pe -z`
    (literals, operator results, elements, members, parameters; the facts in `study\00` §5): one rule covers
    them all, and the two special cases of the old compiler's `Type_PromoteArithmeticType` (`_OFFSET`; a
    SINGLE beside a wider integer) are never taken, because each compares with a type value no argument has.
- [x] 2.2 `_MIN`, `_MAX`, `_CLAMP` (`Rule::Widest`) and `_IIF` (`ExprKind::Choice`, through `sema`, the IR, its
  validation and dump, and `codegen-cpp`), with their errors. Verify: `typed` and `ir` front-end tests pin the
  result types, the conversions and the choice; a `check-fail` test holds the rejected forms of `v30_x70`–`x81`,
  `x88`, `x94`, `x99` as the old compiler has them; the IR's validation rejects a `Choice` whose part was
  hoisted (a unit test builds one).
  - Done: `choice_typed`, `choice_ir`, `choice_errors` in `tests\frontend`. **Two things differ from the
    design.** (1) The arguments of `_MIN`, `_MAX` and `_CLAMP` and the two parts of `_IIF` are *not* converted
    in the tree: the old compiler writes them as they are and C++ converts them (`std::min<float>(q, s)`), and
    a cast written out would be a difference the call-site check reports. The IR's crate documentation names
    these as the operations that convert their operands themselves. (2) **Nothing in this compiler takes a
    part out of an expression** (the IR's values are `sema`'s expressions, written as one C++ expression each),
    so there is no hoisted part to reject. The validation checks what can go wrong instead (the condition an
    integer, the parts both strings or both numbers as the node is, each part a value of the body; unit test
    `choice` in `ir\src\validate.rs`), and the IR's documentation states the rule "a value is one expression".
    That only the part chosen runs is shown by lines 2 to 6 of the slice program.
  - `x99`, a whole array `a()` as a part of `_IIF` (accepted by the old compiler, element 0): an error here, as
    a whole array is in any expression; listed in `SOMEDAY.md` with task 8.2.
- [x] 2.3 `_SHL`, `_SHR` (plain rows) and `_ROL`, `_ROR` (`Rule::Rotate`, D7). Verify: `typed` tests pin the
  width for each integer type and for literals of three sizes; a `check-fail` test holds `v30_x84`–`x87` and a
  `_BIT` argument.
  - Done: `shifts_typed`, `shifts_errors`. `_SHL` and `_SHR` needed a slot kind, `Slot::UInt64` (the table's
    `_UNSIGNED _INTEGER64`). A `_BIT * 8`, `* 16`, `* 32` or `* 64` is accepted by `_ROL`, as the old
    compiler's test of the width in bits accepts it (`qb64pe.bas` 20865); every other `_BIT` type is the error.
- [x] 2.4 The eight names in a `CONST` are errors (`v30_x89`–`x93`). Verify: a `check-fail` test; the error is
  not "not supported yet".
  - Done with no code: `check\constants.rs` already gives "cannot be used in a `CONST`" for every function
    that is not one of the old evaluator's. The last lines of `choice_errors` and `shifts_errors` hold them.
- [x] 2.5 Call-site programs (one per function and argument type that changes the call: each float kind, two
  integer widths, signed beside unsigned, strings for `_IIF`) and a slice program for the eight, recorded with
  `qb64pe.exe`, with a line whose unchosen `_IIF` part would raise an error. Verify: the call-site check passes
  for the new programs (`--glob`); the slice program and the corpus programs `243_iif`, `108_clamp`,
  `78_min_max`, `159_bitwise_shl_shr`, `160_bitwise_rol_ror` pass and are on `tests\corpus\slice.list`; the
  coverage test of tier 1 passes; `crates\README.md` names the functions.
  - Done: 75 programs `tests\callsite\func_min_*`, `func_max_*`, `func_clamp_*`, `func_iif_*`, `func_shl_*`,
    `func_shr_*`, `func_rol_*`, `func_ror_*`, all equal; `slice\s78_choice_min_shifts` (24 lines) and the five
    corpus programs pass on their first run. Two changes on the way: the call-site tool's rule 5 also drops
    the `ull` suffix of a literal that is a whole argument (`func__shl(1ull,40)`; its self-test has the pair
    and three pairs it must keep apart), and the emitter writes a part of a choice and an argument of a call
    with a template argument without the parentheses it puts around an operation (`names::bare`), as the old
    compiler does. **Also found:** the upstream programs `iif/iif_test` and `rotations/test` compile cleanly
    now and pass, so they joined `tests\upstream\pass.list` (144). The slice program sets its `_UNSIGNED
    _INTEGER64` by `uq = 0: uq = uq - 1`: a literal above the `_INTEGER64` range is still "not supported yet:
    unsigned literals" (not of this change).

## 3. `EXIT SELECT`

- [x] 3.1 `EXIT SELECT` and `EXIT CASE` in `sema` and the IR (design D5), with their errors. Verify: an `ir`
  front-end test pins the jump for a plain `SELECT`, for `EVERYCASE` (both words), from inside an `IF` and a
  loop, and for a `SELECT` inside a `CASE`; a `check-fail` test holds `v30_x60`, `x61`, `x63`–`x65` as the old
  compiler has them; a slice program recorded with `qb64pe.exe` and `151_exit_select` pass and are on
  `slice.list`; `crates\README.md` names the statement.
  - Done: `StmtKind::ExitSelect { case }`; `exit_select_ir`, `exit_select_errors` (`x65`, which the old compiler
    accepts, is line 9 of the slice program); `slice\s79_exit_select` (12 lines) and `151_exit_select` pass on
    their first run. **Simpler than design D5:** `sema` keeps no stack of open `SELECT`s. The parser already
    rejects every form measured (`EXIT SELECT` with no `SELECT` open in the body, and any statement between
    `SELECT` and its first `CASE`), so the statement that reaches `sema` always stands in a `CASE`; the
    lowering keeps the stack it needs (`Lowerer::selects`).
  - **Found and measured** (`verification\v30_x66`, `study\00` §5): `EXIT CASE` is a jump to "the end of the
    next `CASE` body that closes", by a count, not to the end of its own `CASE`. The two are the same unless a
    `SELECT` with a `CASE` follows the statement inside its `CASE`; then the program jumps into that inner
    `SELECT` and goes on after its `END SELECT`. Implemented the old compiler's way (`Lowerer::case_end`),
    lines 11 and 12 of the slice program compare it, and it is listed in `SOMEDAY.md` ("QB64pe behaviours to
    review"). The mark of a match of `SELECT EVERYCASE` is still set after an `EXIT CASE`, so its `CASE ELSE`
    does not run.

## 4. `PRINT USING` and `LPRINT`

- [x] 4.1 `PRINT USING` to the console and to a file (design D1): `sema`, the IR with its validation and dump,
  `codegen-cpp`. For each rejected form of `v30_x01`–`x19` say in this task whether the parser or `sema`
  rejects it. Verify: an `ir` front-end test pins the statement for each value type, `TAB` and `SPC`, items
  before `USING`, a file number and a trailing separator; a `cpp` test pins the C++ of one statement with two
  values; a `check-fail` test holds the rejected forms.
  - Done: `StmtKind::PrintUsing` and `Op::PrintUsing` with `PrintTo` (screen, file, printer) and `UsingValue`
    (string, number with the type it is formatted as, blanks); `print_using_ir`, `print_using_cpp`,
    `print_using_errors`. **The parser changed** (`syntax\src\parser\print.rs`): `USING` was read only as the
    statement's first word; it is read after items too, with or without a separator before it (`PRINT "a";
    USING …`, `PRINT 1 USING …`), and its two new messages are real errors ("expected the format string after
    `USING`", "`LPRINT` takes no file number").
  - **Who rejects what:** the parser: `x02` (no `;` after the format), `x05` (no format), `x06` (no separator
    after the format), `x08` (two numbers with no separator), `x16` (as `x02`, to a file), `x17` (`USING
    #1`); `sema`: `x01` and `x07` (a comma after the format), `x03` (no value), `x04` (a number as the format),
    `x09` and `x10` (an empty value), `x11` and `x12` (a second `USING`), `x15` (a `TYPE` value, the error any
    expression gives). Accepted, as there: `x13`, `x14`, `x19`. `x18` (a whole array as a value, element 0
    there) is an error here, as a whole array is in any expression (`SOMEDAY.md`, with task 8.2).
  - **A number stays as it is** (as in group 2): the old compiler writes `print_using_single(f, 1.234E+3, …)`
    for a SINGLE literal, which is held as a C `double`; a conversion written out would be a cast the
    call-site check compares. The entry is named by `UsingValue::Num { as_ }`, from what the value is believed.
  - Measured on the way (24 forms, `study\00` §5): two values always need a separator, also next to a string
    literal; one trailing `,` is accepted; `TAB(n)` alone counts as a value; a file's statement leaves by its
    first label after the format. The spec's scenarios were extended by these.
- [x] 4.2 `LPRINT` with and without `USING` (D8): the destination of `Print` and `PrintUsing`; `LPRINT` on the
  list of built-ins no test runs, and the coverage test takes a statement with a node of its own by name; the
  test of `--report unsupported` in `driver\tests\cli.rs` takes `STOP`. Read the call-site tool's
  normalisation rules first and extend them, with their tests, if a statement of several calls needs it.
  Verify: call-site programs for `PRINT USING` (each value type, a file, `TAB`, items before, a trailing `;`)
  and for `LPRINT` (items, zones, `TAB`, `USING`, bare) pass (`--glob`); taking the `LPRINT` programs out makes
  `cargo test` fail and name it.
  - Done: 55 programs `tests\callsite\print_using_*` (42) and `lprint_*` (13), all equal on their first run.
    The tool compares every line of a statement already; one rule was extended: rule 2 also renumbers
    `skip_pu<N>` and `print_using_format<N>` (a recorded pair in its self-test).
  - **Not in the design:** `LPRINT` needs libqb's printer part linked (`DEP_PRINTER=y`, `qb64pe.bas` 27717
    and 13710). Added as `RuntimePart::Printer`, its flag at the old compiler's place (unit test
    `the_printer_flag`); without it a program with `LPRINT` would not link.
  - The coverage test: `OWN_NODE` (`LPRINT`) joins the rows of `STATEMENTS` for the slice and `ir` rules, and a
    name of it on `NEVER_RUN` must stand **as a statement** in a call-site program: with the 13 `lprint_*`
    programs moved away `cargo test` fails with "`LPRINT` (a built-in no test runs) is used as a statement in no
    program of tests/callsite" (the word alone would be found in `width_lprint.bas`, `WIDTH LPRINT`).
- [x] 4.3 Slice programs recorded with `qb64pe.exe`: formats for numbers and strings, several values and the
  format starting again, the run-time errors under a handler, `PRINT USING` to a file read back; `LPRINT` held
  as code that is not reached. Verify: they and `186_print_using` pass and are on `slice.list`; `239_lprint`
  compiles with no diagnostic; `crates\README.md` names the statements.
  - Done: `slice\s80_print_using_numbers` (16 lines), `s81_print_using_strings` (17), `s82_print_using_file`
    (with six `LPRINT` statements below its `SYSTEM`); they and `186_print_using` pass on their first run;
    `239_lprint` compiles, builds and is on `slice.list` as the compile-only program its `.norun` makes it.
    The upstream programs `func_tostr/double`, `float` and `single` compile cleanly now and pass:
    `tests\upstream\pass.list` has 147.
  - The spec's scenario of an `_UNSIGNED _INTEGER64` sets its value by `u~&& = 0: u~&& = u~&& - 1`: a literal
    above the `_INTEGER64` range is "not supported yet: unsigned literals" (not of this change; told to the
    user).

## 5. `GET` and `PUT` on a file, `LOCK`, `UNLOCK`

- [x] 5.1 The file form of `GET` and `PUT` (design D2): `StmtRule::File`, the three calls by the third
  argument, the bytes of one place as a statement argument, the rejections. Verify: an `ir` front-end test pins
  the argument for a numeric variable, a fixed-length string, a variable-length string, a `TYPE` variable and
  member, an element, a whole array, and no variable, each with and without a position and after `CALL`; a
  `check-fail` test holds the rejected forms of `v30_x20`–`x48`; the graphics tests of the name do not change.
  - Done: `StmtRule::File` (a second row of `GET` and of `PUT` in `STATEMENTS`), `Slot::Bytes`,
    `StmtArg::Bytes(place)`; `file_get_put_ir`, `file_get_put_errors`. The statement is checked by the general
    code of a statement with a template; only the variable has a function of its own (`bytes_arg`). The
    graphics tests: `m3_step2_errors` is unchanged (a statement that starts with a point is not tried as the
    file form); `m3_step2_marked` lost the two lines that held the file form as "not supported yet".
  - **Measured on the way** (`verification\v30_x67`): a name alone as the variable is a scalar, also when
    only an array has the name; and the position is an `_INTEGER64` slot (`study\00` §5).
  - One form stays marked: `GET #1, , und()` for an array that is not declared ("not supported yet"; the old
    compiler's answer is not measured). `GET #1, , "lit"`, the old compiler's internal error, is the plain
    "needs a variable" error here, as the design says.
- [x] 5.2 `LOCK` and `UNLOCK` as plain rows (D4). Verify: an `ir` test pins the mask of the three forms and of
  `LOCK #1,`; a `check-fail` test holds `v30_x50`–`x56`'s rejected forms.
  - Done, in the same two tests. The template's plan gives the old compiler's masks with no change (0, 1, 3,
    and 0 for `LOCK #1,`): the `ir` test pins the slots the mask is made of, and the 14 call-site programs
    compare the number. One check was added: `LOCK #1, TO 4` matches the template with the first record left
    out, and is the old compiler's error (`x50`); `sema` rejects it for these two statements. The slots of
    the records are `_INTEGER64`, the table's type (a new arm of `Slot::from_table`).
- [x] 5.3 Call-site programs for every accepted form of 5.1 and 5.2, and slice programs recorded with
  `qb64pe.exe`: a binary file (sizes, positions, past the end, a gap, strings, a whole array, a `TYPE`
  variable, the errors), a random file (records, the gap, strings with their length, error 59 and 63), locks
  (the errors of `v30_i_lock` that one program shows). Each program's file names start with its own name.
  Verify: the call-site check passes for the new programs (`--glob`); the slice programs and `175_get_put`,
  `177_open_binary`, `179_file_lock` pass and are on `slice.list`; `crates\README.md` names the statements.
  - Done: 64 programs `tests\callsite\get_file_*` (30), `put_file_*` (20), `lock_*` (8), `unlock_*` (6), all
    equal, among them a `TYPE` variable and whole arrays of fixed-length strings and of a `TYPE`, which the
    emitter spells as the old compiler does. Elements and members have no program (the tool does not compare
    element addressing); they are in the `ir` test and in `s83`. `slice\s83_binary_file` (16 lines),
    `s84_random_file` (12), `s85_file_locks` (11) and the three corpus programs pass on their first run. The
    upstream program `arrays/put_get_write_read_fixed_string` compiles cleanly now and passes:
    `tests\upstream\pass.list` has 148.

## 6. `FIELD`, `LSET`, `RSET`

- [x] 6.1 `FIELD`, `LSET` and `RSET` as task 1.1 found them (design D3): `sema`, the IR, `codegen-cpp`; the
  forms the old compiler rejects are errors. Verify: `ir` front-end tests pin each statement; a `check-fail`
  test holds the rejected forms measured in 1.1; `FIELD`, `LSET` and `RSET` no longer reach the "not supported
  yet" branch of `check\mod.rs`, or only in the forms 1.2 named.
  - Done: `StmtKind::Field { file, parts }` and `StmtKind::Lset { right, target, value }` with their `Op`s;
    `field_lset_ir`, `field_lset_errors` (50 rejected forms, each a real error). The parser got two messages
    of its own so that they are no "not supported yet" (`CALL` followed by one of the three words; a `;`
    between the parts of `FIELD`). The two tests that used `LSET` as "a statement not compiled yet"
    (`blocks_not_compiled_yet`, `unsupported`) take `STOP`, and the main spec that names it has a delta
    (`specs\compiler\pipeline`).
  - **One form stays "not supported yet", which task 1.2 did not name: a whole array `e()` as the string**
    (`FIELD #1, 4 AS e()`, `LSET e() = "x"`). The old compiler accepts it and takes the array's first element
    with no bounds check (`v30_x109`, `x153`), which for a dynamic array that is not allocated reads whatever
    lies there. It is listed in `SOMEDAY.md` twice (left marked; a behaviour to review) and told to the user.
- [x] 6.2 Call-site programs for each accepted form, and a slice program recorded with `qb64pe.exe`: records
  written through fields and read back, `GET` and `PUT` with no variable, `LSET` and `RSET` with shorter and
  longer values, the run-time errors measured. Verify: the call-site check passes for the new programs
  (`--glob`); the slice program passes and is on `slice.list`; the coverage test of tier 1 passes;
  `crates\README.md` names the statements; `SOMEDAY.md` lists what stayed marked.
  - Done: 23 programs `tests\callsite\field_*` (13), `lset_*` (6), `rset_*` (4), all equal on their first run
    (elements and members as targets are in the `ir` test and the slice program: the tool does not compare
    element addressing). `slice\s86_field_lset` (28 lines) passes on its first run, the order `fn ix wd` of
    line 17 included. The coverage test takes the three names by `OWN_NODE`.

## 7. `CHAIN`, the corpus programs, the lists

- [x] 7.1 `CHAIN` as a plain row on the list of built-ins no test runs (design D8). Verify: call-site programs
  for a literal, a variable and an expression pass; a `check-fail` test holds `v30_x95`–`x98`'s rejected forms;
  a slice program holds `CHAIN` as code that is not reached; the coverage test passes.
  - Done: a row of `STATEMENTS` and a name of `NEVER_RUN`; six programs `tests\callsite\chain_*`, all equal;
    `chain_ir`, `chain_errors`; `slice\s87_chain_not_reached`. `common_unsupported`, which held `CHAIN
    "other.bas"` as "not supported yet", lost that line.
- [x] 7.2 `238_chain`: run it once with the new compiler through the corpus runner. Verify: it is on
  `slice.list` if its output is equal, else on `tests\known_clean_not_passing.list` with the reason; this task
  says which, and `tests\corpus\README.md` says the same.
  - **Equal: it is on `slice.list`.** Its `.normalize` rule already takes the folder out of `cmd`'s message,
    and both compilers' programs print the same three lines for a program that is not there. So one corpus
    program does reach `sub_chain`, with a name that does not exist; no test chains to a program that does.
- [x] 7.3 Regenerate the shrink-only lists and run the demand report. Verify: `qb64rust --report unsupported
  tests\corpus` names no program held back by a "not supported yet" error but for forms tasks 1.2 and 6.2
  named; `tests\known_unsupported_rejections.list` did not grow; `tests\corpus\slice.list` holds every corpus
  program this change set out for, or this task says why not.
  - Done: the report says "389 programs: 388 without errors, 0 with only "not supported yet" errors, 1 with
    other errors" (the one is `s77_exeicon`, whose icon lies in its `.files` folder, which only the runner
    copies beside it; as before this change). `QB64RUST_UPDATE_LISTS=1` changed none of the four lists.
    `slice.list` holds all twelve corpus programs the proposal names and the ten new slice programs: 384.

## 8. Closing runs and documents

- [x] 8.1 The closing runs, each once, with `--jobs 8 --build-cache` and without `--pass-cache`: tier 1 and
  the tests that build (`--ignored`), the runner's own tests, the call-site check, the slice list with and
  without folding (`QB64RUST_NO_FOLD=1`), the upstream and differential pass lists, the two full suites.
  Verify: every run passes, or a failure is fixed and only what the fix can affect is rerun, with a note of
  which runs were not repeated and why that is safe; `STATUS.md` has the numbers.
  - Done, 2026-10-13, each run once and every one passed: tier 1 (303 tests), the 16 tests that build, the
    runner's 68 tests, the call-site check (1145 of 1145), the slice list (384 of 384 in 89 s; without folding
    384 of 384 in 144 s), the upstream pass list (148 of 148), the differential pass list (59 of 59), the full
    corpus (409: 403 pass, the 5 known failures, `218_fre` with only marked errors as before) and the full
    upstream suite (404: 148 pass, 255 rejected or marked, 1 known failure; none wrong at run time). Nothing
    was edited after them but documents, the record of test runs and this file, which no run reads.
  - **After the closing runs, by a review of the change (2026-10-13):** a FUNCTION's name with empty
    parentheses as the variable of `GET` or `PUT` or as the string of `FIELD`, `LSET` or `RSET` was "not
    supported yet" and is an error now, as measured (`verification\v30_x171`–`x175`, "Expected (...)"; five
    lines in `file_get_put_errors.bas` and `field_lset_errors.bas`); an undeclared array in `GET` and `PUT`
    was measured (`x176`–`x178`: accepted) and stays marked. `design.md` got its "As built" section. Rerun:
    fmt, clippy and tier 1 (pass). Not rerun: tier 2 and the full suites. That is safe: the edit turns one
    error into another on two paths that already ended in an error, so no program that compiled before
    reaches it, and no C++ is written differently.
- [x] 8.2 The documents: `STATUS.md` (the state, step 4 done, the next step), `ROADMAP.md`, `crates\README.md`,
  `tests\corpus\README.md`, `study\00` §5, `SOMEDAY.md`, `DIVERGENCES-QB45.md` where a kept behaviour differs
  from QuickBASIC 4.5, `CLAUDE.md`'s layout table (the corpus and slice counts, `v30*`). Verify: `python
  tools\repo_check\check_repo.py --untracked` is clean; `openspec validate m3-console-files --strict` passes.
  - Done; also `tests\upstream\README.md` (148), `tools\callsite\README.md` (the two extended rules, the
    count), `DECISIONS.md` (a row for the choices made while building) and `DIVERGENCES-QB45.md` Q-013 (`LSET`
    of a `TYPE` value) and Q-014 (a field string after `=`). The strict validation asked for shorter
    requirement texts: the `FIELD` requirement of `language/record-files` is three requirements now.
- [x] 8.3 The record of test runs: `python tools\testtime\test_time.py`. Verify: its three files are named
  among those to stage, with a commit message for the change.
  - Done: `tools\testtime\runs.csv`, `sessions.csv` and `REPORT.md` (193 sessions, 3731 runs on record). The
    files to stage and the commit message are in the session's last message to the user.

## Workflow follow-up

- Archive the change (`/opsx:archive`) after the user has staged and committed it; the specs of
  `language/print-using` and `language/record-files` become main specs, and the purpose of `language/file-io`
  gets a pointer to `language/record-files`.
