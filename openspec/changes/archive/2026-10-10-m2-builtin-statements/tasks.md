# Tasks

Order of the groups: 1 (the operation and the plain statements, measured first), 2 (the call-site tool, so that
every later group can use it), then 3 to 7 in order, then 8. Groups 3 to 7 each start with their own measurement
task (design D1). Corrections that a measurement makes to the design or the spec deltas are listed in the task
that found them. Anything measured as wrong code in the old compiler (a crash, memory corruption, a failed build)
is put to the user before it is designed; a behaviour that is only questionable is implemented QB64pe's way and
listed in `SOMEDAY.md` "QB64pe behaviours to review" (`DECISIONS.md` 2026-10-08). Each group regenerates the
shrink-only lists (`QB64RUST_UPDATE_LISTS=1 cargo test -p qb64rust-driver --test inputs`, entries may only go
away), adds the programs it makes pass to `slice.list` and `tests\upstream\pass.list`, updates the support list
in `crates\README.md`, and keeps `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`,
`python -m ruff check .`, tier 1 and tier 2 green. Text with backslashes goes through the Edit and Write tools
(`CLAUDE.md` rule 7); no subagents (rule 1).

## 1. The built-in statement operation and plain statements

- [x] 1.1 Write and run `verification\v22_a_*` (design D1, "Statement calls") with `qb64pe.exe`
  (`verification\run.sh`), and read the C++ of each statement with `qb64pe -z`: `KILL`, `MKDIR`, `RMDIR`, `CHDIR`,
  `NAME`, `ENVIRON`, with a raising argument, a failing call, `RESUME`, wrong kinds and counts of arguments, inside
  a procedure; one rejection per `v22_x*` program. Verify: outputs recorded beside each program; findings in
  `study\00` §5; `python tools\repo_check\check_repo.py --untracked` clean.
- [x] 1.2 Correct design D2 (the error rule of a statement call) and the deltas `language/builtin-statements` and
  `compiler/pipeline` where 1.1 disagrees, and list each correction here. Verify: `openspec validate
  m2-builtin-statements --strict` passes.
  Corrections (2026-10-10): none to the rule. D2's error rule is measured, no longer provisional: the call is
  made after a raising argument and the first error is serviced (`sub_environ` does not test for a pending error
  and raises 5 again; no visible difference). Added to `language/builtin-statements`: the `CALL` form (`CALL
  KILL("f")` is accepted, `v22_x16`), a scenario for a wrong number of arguments, and one for `RESUME` running
  the statement and its arguments again. `compiler/pipeline`: no change. Read for later groups: an integer
  argument of a number slot is written as it is and a float through `qbr(…)`, for a LONG and an `_INTEGER64`
  slot alike (`SEEK`); the new compiler's explicit casts are the same C++ conversion (a matter for the
  call-site check's normalisation, task 2.2).
- [x] 1.3 `syntax\src\ast.rs`: accessors for `BuiltinStmt` (name, arguments, words in order), with unit tests for
  `OPEN … FOR OUTPUT AS #1`, `SHELL _HIDE c$`, `NAME a$ AS b$` and a statement with an optional argument left out.
  Verify: `cargo test -p qb64rust-syntax` green; no snapshot of a tree changes.
- [x] 1.4 `sema`: `STATEMENTS` and `StmtRule::Plain` in `builtins.rs` (design D3), the statement side of
  `check\builtins.rs`, `StmtKind::Builtin` in the typed tree and its dump; rows for `KILL`, `MKDIR`, `RMDIR`,
  `CHDIR`, `NAME`, `ENVIRON`; their marks removed from `check\mod.rs`. Tests: the unit test that every row is in
  the table once; `tests\frontend` `typed` and `check-fail` programs for each statement and each error of 1.1.
  Verify: `cargo test` green; the new snapshots reviewed.
- [x] 1.5 `ir`: `Op::Builtin` and `StmtArg` (design D2), the lowering, `dump`, `validate` (slots against the rule),
  `may_raise`, the error rule in the crate documentation. Tests: `ir` front-end programs for a statement with a
  value, with two values, and in a procedure; a `validate` unit test for a statement with a wrong slot. Verify:
  `cargo test` green.
- [x] 1.6 `codegen-cpp`: `stmt_call` for `Plain` (design D5); an absent optional slot written `NULL` for functions
  and statements (design D4). Tests: `cpp` front-end programs; the changed `cpp` snapshots of optional function
  slots reviewed one by one. Verify: `cargo test` green; tier 2 (`slice.list`, upstream and differential pass
  lists) unchanged in result.
- [x] 1.7 The coverage check reads `STATEMENTS` (delta `testing/compiler-tests`): each statement in a `slice.list`
  program and an `ir` front-end test. Add `tests\corpus\slice\s35_file_system` (the six statements; their effect shown by the errors a second
  call raises under a handler, since the file functions come in group 3), recorded with `qb64pe.exe`. Verify: `cargo test`
  fails when a row is added without the tests (tried once, then reverted) and passes with them; `s35` passes in
  tier 2.
  Done (2026-10-10). Found on the way: **the old compiler's rule for a template is one rule for every statement**
  (`qb64pe.bas` `seperateargs`: which arguments and choices are C arguments, `NULL` for an absent one, the bits
  of the `passed` mask). It is ported once (`crates\builtins\src\passing.rs`, checked against the C++ of `OPEN`,
  `NAME`, `SEEK`, `RANDOMIZE`, `SHELL` and five `LINE` forms), so `StmtRule::Plain` is the table entry plus that
  rule; D3's `Open` and `OpenOld` variants are expected not to be needed (settled in task 3.2). The table's
  `arg_types` list the C arguments in order, choices included. A row added without tests (`SEEK`) failed the
  coverage check with both messages, then reverted. `s35` passes, and with it the corpus programs
  `214_mkdir_rmdir` and `216_kill_nonexistent` (3 programs newly on `slice.list`: 201). Tier 2 after the `NULL`
  change: slice 201 of 201, upstream 47 of 47, differential 59 of 59; the five changed `cpp` snapshot lines are
  `0` to `NULL` in `func_pi`, `func_instr` (twice), `func_mid` and `qbs__tostr`.

## 2. The call-site check (trial, design D8)

- [x] 2.1 `tools\callsite\callsite.py` with a `README.md`: run both compilers with `-z` on each
  `tests\callsite\*.bas`, extract the last statement's lines from each, normalise, compare, report (no local
  path in the output; scratch folder outside the repo). Unit-style self-test in the script (`--self-test`): the
  normaliser on recorded line pairs, and seeded mistakes (a missing rounding call, a wrong mask, swapped
  arguments) reported as different. Verify: `python tools\callsite\callsite.py --self-test` passes; `ruff` clean;
  `git status --porcelain` unchanged in this repo and in the reference clone after a run.
- [x] 2.2 `tests\callsite\`: one program per statement and argument type for the six statements of group 1 (a
  literal, a variable, an expression, a float where a number is taken). Verify: the tool reports every one
  `equal`, or the difference is a real bug that is fixed, or a normalisation rule is added and listed in the
  README with its reason.
- [x] 2.3 Note the interim state here (programs compared, equal, rules added) so that group 3 can extend it.
  Verify: the counts in this task match the tool's summary line.
  Interim state (2026-10-10): `23 programs: 23 equal, 0 differ, 0 not compared` (`KILL` 8, `MKDIR`, `RMDIR`,
  `CHDIR`, `ENVIRON` 3 each, `NAME` 4: literals, variables, expressions with built-in function calls, a
  fixed-length string, `KILL (s)`, `CALL KILL(s)`). Four normalisation rules, none naming a built-in
  (`tools\callsite\README.md`): the statement frame; numbered temporaries and labels renumbered; redundant
  parentheses; an integer cast around a whole call argument (`questions.md` Q1). Two corrections to design D8:
  **user variables are compared as written**, not renamed in order of first use (both compilers name them by
  the old scheme, and renaming hides two swapped variables); **array elements are not passed in the programs**
  (their index is spelled with other conversions, `questions.md` Q2). No difference found was a bug. The tool
  must not run while tier 2 runs: it empties the clone's `internal\temp`, which a `qb64rust` build's `make` reads
  (three differential programs failed to build once for that reason, and passed when rerun alone).

## 3. Sequential files

- [x] 3.1 Write and run `verification\v22_b_*` (design D1, "Files") and read the C++ for each form. Verify:
  outputs recorded; findings in `study\00` §5; repo check clean.
- [x] 3.2 Correct design D2, D4, D5 and the delta `language/file-io` where 3.1 disagrees; list each correction
  here; put wrong code to the user. Verify: `openspec validate m2-builtin-statements --strict` passes.
  Corrections (2026-10-10): D2: `Op::Write` has `newline` (`WRITE 1,` is accepted and ends without a line end).
  D3: `OPEN` in both forms is `Plain` by the ported template rule; no `Open`/`OpenOld` variants; `SEEK`'s
  position slot is `_INTEGER64`. D4: `LOF`, `LOC`, `SEEK` are held `_INTEGER64` (libqb returns `int64`), believed
  LONG; `FREEFILE` and `_CWD$` are bare only. D5: the exact shape of file `Print`, `Write` and `Input` as read
  (a comma is the `tab` flag of the item before it; the type code is the target's old type value). Delta
  `language/file-io`: scenarios added for 14-column zones, the missing automatic `;` in `PRINT #`, `WRITE`'s
  trailing comma and `;`, a value outside the target's range (error 6, 0 stored, later targets skipped), the
  rejected targets, and `INPUT #1, a()` (accepted by the old compiler, "not supported yet" here). No wrong code
  was found; questionable behaviours for `SOMEDAY.md` (task 8.4): 2.5 read into an `_INTEGER64` is 3 while into a
  LONG it is 2; `OPEN … FOR OUTPUT ACCESS READ` raises 53; `WRITE` does not double a quote inside a string.
- [x] 3.3 `OPEN` (both forms), `CLOSE`, `SEEK`: `OPEN` and `SEEK` as `StmtRule::Plain` (3.2), `StmtRule::Close`;
  `ast.rs` accessor for `CloseStmt`; emission. Tests: `typed`, `ir`, `cpp` and `check-fail` front-end programs;
  `tests\callsite` programs for every mode, access and lock word and each argument type. Verify: `cargo test`
  green; the call-site tool reports them equal.
  Call-site programs (2026-10-10): 37 for `OPEN` (6 modes, 9 access and lock forms, 4 record lengths, 8 file
  numbers, 4 names, 6 of the old form), 10 for `CLOSE`, 10 for `SEEK`; all equal. **Rule 5 added**: the `ll`
  suffix of an integer literal that is a whole call argument (`sub_seek(1,2ll)`), as expected. Six programs of
  the first run differed in how an expression inside an argument is spelled, not in the call: an INTEGER operand
  widened by a cast (`((int32)(*__INTEGER_I))*2`), an `ll` literal as an operand (`q*2ll+1ll`,
  `func_lof(n)+1ll`). By the answer to `questions.md` Q2 (no rules for operand spellings) they were rewritten
  with operands of one width; `tools\callsite\README.md`, "What it does not compare", says so.
- [x] 3.4 The file functions as rows: `EOF`, `LOF`, `LOC`, `SEEK`, `FREEFILE`, `_FILEEXISTS`, `_DIREXISTS`, `_CWD$`
  (design D4; bare names for `FREEFILE` and `_CWD$` as measured). Tests: a `typed` program; call-site programs.
  Verify: `cargo test` green apart from the coverage check, which names exactly these functions until 3.7's slice
  program exists.
  Call-site programs (2026-10-10): 22 (`EOF` 4, `LOF` 3, `LOC` 3, `SEEK` 3, `FREEFILE`, `_FILEEXISTS` 4,
  `_DIREXISTS` 3, `_CWD$`), each as the right side of an assignment to a variable of the function's held type;
  all equal. `n = LOF(1)` was written and taken out again: the difference is the store's narrowing cast
  (`*__LONG_N=((int32)(func_lof(1)))`), not the call. State of the tool: `102 programs: 102 equal, 0 differ, 0
  not compared`, five normalisation rules, none naming a built-in.
- [x] 3.5 `PRINT #` and `WRITE` (file and console): `Op::Print` with `to`, `Op::Write`, `check\io.rs`, `ast.rs`
  accessors for `WriteStmt`, `codegen-cpp\src\io.rs` (design D5). Tests: `ir` and `cpp` programs for every item
  kind, separators and the empty forms (`PRINT #n, USING` stays marked). Verify:
  `cargo test` green; every existing `PRINT` snapshot unchanged.
- [x] 3.6 `INPUT #` and `LINE INPUT #`: `Op::Input` with `Source::File`, accessors for `InputStmt` and
  `LineInputStmt`, the type code per target type with a unit test against the codes read in 3.1, target places by
  their store rules. Tests: `ir`, `cpp`, `check-fail` (a target that is no place, a numeric `LINE INPUT` target).
  Verify: `cargo test` green.
- [x] 3.7 Slice programs `s36_files` (write, append, read back, every item kind), `s37_file_errors` (52, 53, 55,
  62 and the others measured, under a handler), `s38_file_functions`, recorded with `qb64pe.exe`; corpus and
  upstream programs that now pass added to their lists. Verify: tier 2 green on the three lists; the corpus count
  in this task (programs newly on `slice.list`).
  Done (2026-10-10): `s36`–`s38` and 19 corpus programs (`05`, `52`–`54`, `59`, `60`, `86`, `95`–`97`, `174`,
  `176`, `178`, `180`–`183`, `213`, `219`) newly on `slice.list` (22 programs: 223); no upstream program waited
  only on files. Tier 2 with the release build: slice 223 of 223, upstream 47 of 47, differential 59 of 59.

**Group 3 is closed (2026-10-10).** Carried to group 8, where the documents are brought up to date once:
- `CLAUDE.md` "Layout": rows for `tests\callsite\` and `tools\callsite\` and `v22*` in the `verification\` row,
  once the call-site check has its verdict (no rows if it is dropped).
- For the call-site check's verdict: it has found no bug so far; the tool drops every `qbs_cleanup` line as
  frame, so a statement that forgets its cleanup compares `equal`, and the programs avoid three spellings that
  are not the call (elements, operands of mixed width, a narrowing store), all listed in
  `tools\callsite\README.md`, "What it does not compare". `questions.md` Q1 and Q2 are answered (`DECISIONS.md`
  2026-10-10): rule 4 stays, elements stay out of the programs.

## 4. `DATA`, `READ`, `RESTORE`

- [x] 4.1 Write and run `verification\v22_c_*` (design D1, "Data"), with include files under `verification\v22_inc\`
  where needed. Verify: outputs recorded; findings in `study\00` §5; the unmeasured remark on bare `DATA` in
  `ast.rs` replaced by the fact.
  Done (2026-10-10): `v22_c_order`, `v22_c_types`, `v22_c_errors`, `v22_c_restore`, `v22_inc\data.bi`, and
  `v22_x65`–`x90` (one rejection or one accepted form each); the C++ read with `qb64pe -z`.
- [x] 4.2 Correct design D6 and the delta `language/data-read` where 4.1 disagrees; list each correction here.
  Verify: `openspec validate m2-builtin-statements --strict` passes.
  Corrections (2026-10-10). D2: **`Read` has no pending-error check** (the old compiler writes none; libqb's
  readers return 0 while an error is pending and put the position back when they raise, so later targets are set
  to 0 and the failing item is read again by the next `READ`); `Op::Restore { at, label }` carries the label's
  name beside the position, for the old compiler's `data_at_LABEL_<NAME>`. D6: the order is as expected (file
  order, procedures and included files in place, also a block that never runs and lines after `SYSTEM`); bare
  `DATA` is one empty item; a `RESTORE` label has no scope and a name two bodies have is an error. Deltas:
  `language/data-read` (scenarios for the order, bare `DATA`, an item that is no number or too large staying
  unread, a quoted number being no number, the FUNCTION's own name, labels of other bodies, an ambiguous label;
  the forms left "not supported yet": a line number, `READ (a)`, `READ a()`), `language/builtin-statements` (`READ`
  taken out of "stops at the first item that raises"), `compiler/pipeline` (`READ` checks nowhere; `RESTORE`
  names the label too). No wrong code found. Questionable, for `SOMEDAY.md` (task 8.4): 2.5 read into an
  `_INTEGER64` is 3 and into a LONG 2 (as `INPUT #`); a quoted number is no number; `READ a()` sets only `a(0)`.
  **A false accept found and fixed on the way**: the name of the FUNCTION a statement stands in was taken as a
  target of `INPUT #` and `LINE INPUT #` (group 3) and of `READ`; the old compiler rejects all three (`v22_x87`–
  `x89`), and so does `target_place` now (`files_errors.bas`, `data_errors.bas`).
- [x] 4.3 `sema`: collect the items in the measured order into the program's data; `READ` targets; `RESTORE` with
  and without a label (a line-number target stays marked); the three marks removed. Tests: `typed` programs (data
  across a procedure and an included file under `tests\frontend\inc\`), `check-fail` (unknown label, a target that
  is no place, text after a closing quote). Verify: `cargo test` green.
  Done (2026-10-10): `data_typed.bas` (with `inc\data.bi`), `data_errors.bas`, `data_marked.bas` (the forms left
  "not supported yet").
- [x] 4.4 `ir` and `codegen-cpp`: `Program::data`, `Op::Read`, `Op::Restore`, `validate`, the data fragment and the
  per-type read calls. Tests: `ir` and `cpp` programs; call-site programs for `READ` into each type and both
  `RESTORE` forms. Verify: `cargo test` green; the tool reports them equal.
  Done (2026-10-10): `data_ir.bas`, `data_cpp.bas` (its `global.txt` lines and offsets equal the old compiler's
  for the same program: 0, 23, 27, size 29), a `validate` unit test. Call-site programs: 17 for `READ` (13 numeric
  types, a string, a fixed-length string, several targets, implicit variables) and 4 for `RESTORE`; all equal
  with no new rule (`123 programs: 123 equal`). Not in the programs, because the store is spelled differently and
  is not the call: `_BIT` targets (`&1` against `&0x1ull`), members and elements.
- [x] 4.5 Slice program `s39_data` (every measured item form, out of data under a handler, `RESTORE` to labels),
  recorded; lists updated. Verify: tier 2 green; the corpus count in this task.
  Done (2026-10-10): `s39_data` and 10 corpus programs (`44`, `46`, `116`, `143`, `220`–`223`, `240`, `252`)
  newly on `slice.list` (11 programs: 234); upstream `console_only/data` and `source_ordering/data_positioning`
  on `pass.list` (49). The four `v22_c_*` programs print the same with both compilers (`_ERRORLINE` and the
  line-number lines taken out, which are not compiled yet).

## 5. `SWAP`, the `MID$` statement, `RANDOMIZE`, `RND`, `TIMER`

- [x] 5.1 Write and run `verification\v22_d_*` for these (design D1, "The rest", first half). Verify: outputs
  recorded; findings in `study\00` §5.
  Done (2026-10-10): `v22_d_swap`, `v22_d_mid`, `v22_d_random`, `v22_d_randomize_noseed`, `v22_x91`–`x137`; the
  C++ read with `qb64pe -z`; `qb64pe.bas` 11377 for the `SWAP` rule.
- [x] 5.2 Correct the delta `language/builtin-statements` (SWAP, MID$, RANDOMIZE) and `language/builtin-functions`
  where 5.1 disagrees; list each correction here. Verify: strict validation passes.
  Corrections (2026-10-10). `SWAP`: "the same type" is the same but for signedness, any two strings go together,
  a `_BIT` is rejected ("Cannot SWAP bit-length variables"), the FUNCTION's own name is no operand; left "not
  supported yet": an operand in parentheses, two whole arrays (`SWAP x(), y()`), a member of an element; scenarios
  added. `MID$`: what changes nothing; a member of an element left marked. `RANDOMIZE`: `USING` sets the state and
  the plain form mixes the seed into it (`RANDOMIZE 5` twice does not repeat). `language/builtin-functions`: no
  change (`RND` SINGLE; `TIMER` believed SINGLE and held DOUBLE, an override beside `LOF`'s; both bare, as the
  spec says). No wrong code found. **Put to the user, `questions.md` Q3**: `SWAP` with an element whose index is
  out of range raises 9 and exchanges with the array's first element; implemented QB64pe's way meanwhile.
- [x] 5.3 `SWAP` (`StmtRule::Swap`, accessor for `SwapStmt`, the swap entry per type) and the `MID$` statement
  (`StmtRule::MidAssign`; the parser's node for `MID$(…) = …` checked first: `tests\known_clean_not_passing.list`
  and the mark `MID$(...)` show how it parses today). Tests: `ir`, `cpp`, `check-fail` (different types, a literal
  operand, a target that is no string place); call-site programs per type. Verify: `cargo test` green; equal.
  Done (2026-10-10). `MID$(…) = …` parses as an assignment to a call of `MID$`; `assign` hands it to
  `mid_assign`. `SwapStmt` had its accessor. Tests: `swap_mid_ir.bas`, `swap_mid_cpp.bas`, `swap_mid_errors.bas`,
  `swap_mid_marked.bas`, two arms of `validate`. Three older tests used `SWAP`, `RND` or the `MID$` statement as
  their example of a mark and now use `LSET` or `_IIF` (`blocks_not_compiled_yet.bas`, `unsupported.bas`,
  `fixed_strings_errors.bas`). Call-site programs: 12 for `SWAP`, 5 for `MID$`; all equal.
- [x] 5.4 `RANDOMIZE` (row, with `USING`; the seedless form as measured or marked), `RND` and `TIMER` (rows, bare
  and with an argument; the bare-name property of design D4 replacing the `_PI` special case, `_PI`'s snapshots
  unchanged). Tests: `typed`, `ir`, `check-fail`; call-site programs. Verify: `cargo test` green; equal.
  Done (2026-10-10). The bare-name property was there already (`check\expr.rs`: a supported function whose slots
  are all optional), so `RND` and `TIMER` are two rows and one held-type override. **`RANDOMIZE` without a seed is
  marked "not supported yet"**: libqb prints `Random-number seed (-32768 to 32767)? ` and reads a line through
  `qbs_input`, which waited for ever on the null device; it is taken up with console input (task 6.4), where the
  marked form becomes the plain call. `CALL RANDOMIZE(5)` is marked too (a template statement through `CALL`).
  Tests: `random_typed.bas` and the `swap_mid_*` files. Call-site programs: 6 for `RANDOMIZE`, 8 for `RND` and
  `TIMER`; all equal with **rule 6** (a `passed` mask written `0|1`). A number of another type in a `double` slot
  is kept out of the programs (the new compiler's `(double)` cast is compared on purpose; README). State of the
  tool: `154 programs: 154 equal, 0 differ, 0 not compared`, six rules.
- [x] 5.5 Slice programs `s40_swap_mid` and `s41_random` (seeded sequences; `TIMER` only in comparisons), recorded;
  lists updated. Verify: tier 2 green; the corpus count in this task.
  Done (2026-10-10): `s40`, `s41` and 6 corpus programs (`41`, `42`, `56`, `232`–`234`) newly on `slice.list` (8
  programs: 242). The three `v22_d_*` programs print the same with both compilers.

## 6. Console `INPUT` and `LINE INPUT`

- [x] 6.1 `run_legacy_tests.py`: the `<name>.stdin` sidecar (delta `testing/golden-corpus`), for check and record,
  both compilers; the same rule in `verification\run.sh`; `tests\corpus\README.md` and the tool's README say so.
  Verify: the runner's own tests (or a dry run on two throwaway programs, one with and one without the sidecar)
  pass; a full corpus run with the old compiler gives the recorded baseline (no program changes result).
  Done (2026-10-10): the sidecar is given to the program as its standard input (`press_any_key.py` takes it as a
  second argument on Windows). Dry run: a program with the sidecar read its lines, one without ran as before. A
  program with a sidecar must end with `SYSTEM`: with `END` it waits at "Press any key" until the timeout. Full
  corpus run with the old compiler: 299 pass, 5 known failures, the baseline.
- [x] 6.2 Write and run `verification\v22_e_*` with `.stdin` files (design D1, "Console input"). Verify: outputs
  recorded, the same on two runs; findings in `study\00` §5, including how `END` behaves with redirected input.
  Done (2026-10-10): `v22_e_input`, `v22_e_redo`, `v22_e_eof`, `v22_e_randomize`, `v22_e_bit` and 21 rejection
  programs (`v22_x138`–`x158`); each `v22_e_*` output is the same on two runs. `v22_e_redo` was rewritten once: its
  first version expected the program to ask again, which it never does. Findings in `study\00` §5.
- [x] 6.3 Correct design D7 and the console-input requirement where 6.2 disagrees; list each correction here; if
  the old compiler's behaviour is not reproducible under the runner, stop and put the options to the user (the
  statements stay marked meanwhile). Verify: strict validation passes.
  Done (2026-10-10). The behaviour is reproducible under the runner. Corrections (design D7, the console-input and
  `RANDOMIZE` requirements, the sidecar requirement): (1) no "Redo from start": characters that do not fit are
  dropped, missing fields are 0 or empty; (2) the compiler writes one `qbs_input` call and the runtime stores, so
  no error check between targets; (3) a sidecar program ends with `SYSTEM`; (4) when the input runs out the
  program waits until the timeout, so the scenario "Input runs out" is now a timeout failure, not a recorded
  result; (5) one `,` after the last target is accepted; (6) a `_BIT` variable is accepted and not stored;
  (7) `RANDOMIZE` without a seed is the plain call. Four rows added to `SOMEDAY.md` ("QB64pe behaviours to
  review"). Strict validation passes.
- [x] 6.4 `Op::Input` with `Source::Console`: prompt forms, targets, emission (design D5). Tests: `ir`, `cpp`,
  `check-fail` (a prompt that is no literal as the old compiler reports it, a target that is no place); call-site
  programs. Also `RANDOMIZE` without a seed, marked in task 5.4: measured with a `.stdin` file in 6.2, then
  compiled as the plain call (`sub_randomize(NULL,0)`) or left marked with the reason. Verify: `cargo test` green;
  equal.
  Done (2026-10-10): tests `console_input_ir`, `console_input_cpp`, `console_input_errors`; 22 call-site programs
  (`input_*`, `line_input_*`, `randomize_no_seed`; the whole set is 176, all equal). The call-site check found
  three type values written wrongly at first (a member carried `ISUDT`, `_OFFSET` lacked `ISOFFSET`, `_BIT`
  carried `ISINCONVENTIONALMEMORY`); all fixed. Programs with a variable index or a member are left out of the
  call-site set: the two compilers spell the index and the member address differently (the `cpp` test and `s42`
  cover them). The parser takes one `,` after the last target of the console forms, and a `;` after a target is
  a real error (it was "not supported yet"). `RANDOMIZE` without a seed is the plain call. A `_BIT` variable is
  compiled as the old compiler compiles it. Still marked: a member of an element as a target. `v22_e_input`,
  `v22_e_redo`, `v22_e_randomize` and `v22_e_bit` print the same with both compilers.
- [x] 6.5 Slice program `s42_console_input` with its `.stdin`, recorded; lists updated. Verify: tier 2 green.
  Done (2026-10-10): `s42_console_input` on `slice.list` (243). No corpus or upstream program uses console
  `INPUT`, so no other program joined a list. Tier 2: slice 243 of 243, upstream 49 of 49, differential 59 of 59.
  Tier 1, clippy, fmt, ruff and the repo check are clean.

## 7. `SHELL`, `COMMAND$`, `ENVIRON$`, and functions by demand

- [x] 7.1 Write and run `verification\v22_d_*` for `SHELL` (statement forms and function), `COMMAND$`, `ENVIRON$`
  (design D1, "The rest", second half). Verify: outputs recorded (machine-dependent text kept out of them);
  findings in `study\00` §5.
  Done (2026-10-10): `v22_d_shell`, `v22_d_environ` and `v22_x159`–`x172`, `x178`, `x179` (one rejection or one
  accepted form each); the C++ read through the call-site programs of 7.2. A bare `SHELL` (an interactive shell)
  is only compiled (`x178`, `x179`, behind `IF 0`), never run. No wrong code found. Corrections to the deltas:
  `language/builtin-statements` (a command that is no string is an error; `CALL SHELL(c$)` is accepted by the old
  compiler and left "not supported yet", as `CALL RANDOMIZE`; scenarios for the two word orders and an empty
  command), `language/builtin-functions` (scenarios for a bad `ENVIRON$` index and the `SHELL` function's exit
  code). Nothing questionable enough for `SOMEDAY.md`.
- [x] 7.2 Rows and rules: `SHELL` ×3 and the function, `COMMAND$` (bare and indexed), `ENVIRON$` (`StrOrIndex` only
  if measured as two entries). Tests: `typed`, `ir`, `check-fail`; call-site programs. Verify: `cargo test` green;
  equal.
  Done (2026-10-10). `SHELL` is `StmtRule::Plain` in its three forms, with no new code: the ported template rule
  gives `sub_shell`, `sub_shell2` and `sub_shell3` with their masks. The function and `COMMAND$` are `Rule::Plain`
  rows (`COMMAND$` bare by its one optional slot). **`ENVIRON$` is two libqb entries**, `func_environ(qbs*)` and
  `func_environ(int32_t)` (`qb64pe.bas` 20952 passes a string as it is), so it has the rule `StrOrIndex`. Tests:
  `shell_environ_typed.bas`, `shell_environ_ir.bas`, `shell_environ_errors.bas`. Call-site programs: 15 for the
  statement, 3 for the function, 5 for `COMMAND$`, 8 for `ENVIRON$`; all equal with no new rule. **Found on the
  way and fixed**: a supported function that needs an argument, named alone (`n = EOF`), was "not supported yet";
  the old compiler reports "Incorrect number of arguments" for every one tried (`v22_x180`–`x185`), and it is a
  real error now (one line of `files_errors.check-fail.snap` changed).
- [x] 7.3 Slice program `s43_shell_environ` (with `.normalize` if needed), recorded; lists updated. Verify: tier 2
  green.
  Done (2026-10-10): `s43_shell_environ` (no `.normalize` needed: it prints only comparisons and text the
  program set itself) and the corpus programs `212`, `215`, `217`, `246` on `slice.list`. The full tier 2 is run
  in task 8.2.
- [x] 7.4 Count the corpus and upstream blockers again (the marks of every program not on a pass list, as in
  `proposal.md`) and write the table here. For each plain function named there or by Civil War Strategy that an
  existing rule may fit (candidates: `_ACOS`, `_ASIN`, `_SINH`, `_COSH`, `_TANH`, `_COT`, `_CSC`, `_SEC`, `_D2R`,
  `_R2D`, `_CEIL`, `_STRCMP`, `_STRICMP`, `CSRLIN`, `POS`): measure its result type and edge values
  (`verification\v22_f_*`), add the row, a `typed` test, a call-site program and a line in a slice program
  (`s44_more_functions`); leave it marked when no rule fits, with the reason in this task. Update the list in the
  delta `language/builtin-functions`. Verify: `cargo test` green (coverage check); tier 2 green; the delta's list
  equals `SUPPORTED`.
  Done (2026-10-10). **Added, all `Rule::Plain`** (`v22_f_functions`, `v22_f_startdir`, `v22_x173`–`x177`,
  `x186`–`x188`): `_ACOS`, `_ASIN`, `_SINH`, `_COSH`, `_TANH`, `_CEIL` (the `std::` functions, held by C++
  overloading as `_ATAN2` is), `_COT`, `_CSC`, `_SEC`, `_D2R`, `_R2D` (libqb takes and returns a `double`: a held
  override beside `TIMER`'s), `_STRCMP`, `_STRICMP`, and `_STARTDIR$` (not in the candidate list; 23 upstream
  programs name it). Tests: `more_functions_typed.bas`, 49 call-site programs (all equal, no new rule),
  `s44_more_functions`, `_STARTDIR$` in `s43`. Newly on `slice.list`: `s43`, `s44` and 10 corpus programs (`77`,
  `154`–`157`, `166`, `212`, `215`, `217`, `246`): 255. Upstream `keyboard/devices_windows` on `pass.list` (50).
  **Left marked:** `CSRLIN` and `POS` (a plain rule fits, but under `$CONSOLE:ONLY` both read the real console's
  cursor, which stays at 1, 1 while output goes to a file and is unset without a console: they come with the
  screen, step 9a); `_ERRORLINE` (needs the source line recorded per statement, which the emitter does not write
  yet); `MKL$`, `CVL` (no libqb entry in the table: the old compiler writes them itself); `_DEFLATE$` (needs the
  zlib dependency in the build); `_MIN`, `_MAX`, `_CLAMP`, `_SHL`, `_SHR`, `_ROL`, `_ROR`, `_IIF` (typed by their
  arguments, no existing rule; `proposal.md`, "Not in this change").

  The count (release build, each mark once per program; a statement's first mark can hide later ones):

  | Set | Programs with an expected output on no list | Marks, most frequent first |
  |---|---|---|
  | Corpus | 26 (were 74) | `REDIM` 3, an array used without `DIM` 3, a dynamic array 2, `GET` 2, `ERASE` 2, `STRING` members 2; once each: `_CLAMP`, `EXIT SELECT`, `_SHL`, `_SHR`, `_ROL`, `_ROR`, `PUT`, `LOCK`, `UNLOCK`, `TAB`, `SPC`, `PRINT USING`, `WIDTH`, arrays as members, a whole `TYPE` assigned, `SHARED` of an array, `DEFINT`, `OPTION BASE`, `POS`, `CSRLIN`, `CHAIN`, `_IIF`, `_MIN`, `_MAX` |
  | Upstream | 194 | arrays as `TYPE` members 59, `_DEST` 52, `REDIM` 50, `$CONSOLE` 32, `_MEM` 30, `_MEMFREE` 22, `_FREEIMAGE` 18, `_MEMGET` 17, `LBOUND` of an expression 16, `_NEWIMAGE` 16, `DIM AS type` before the names 15, then images, `ERASE`, `PUT`, `MKL$`, `DEFLNG` 13 each or fewer |

  Corpus programs waiting on one mark only: `108` `_CLAMP`, `151` `EXIT SELECT`, `177` `GET`, `184` `TAB`, `185`
  `SPC`, `186` `PRINT USING`, `189` `WIDTH`, `200` a whole `TYPE` assigned, `201` `ERASE`, `235` `DEFINT`, `236`
  `OPTION BASE`, `238` `CHAIN`, `243` `_IIF`. Upstream, as `STATUS.md` says, waits on steps 10 and 11.

## 8. The verdict and integration

- [x] 8.1 The call-site verdict (design D8, spec `testing/call-site-check`): run the tool over all of
  `tests\callsite`, write the counts and the normalisation rules here, decide by the two conditions, and put the
  verdict to the user. "Kept": add the tier-2 command to `crates\README.md` and `rust.yml` (job `tier2`).
  "Dropped": delete `tools\callsite`, `tests\callsite` and the capability's delta. Either way a dated row in
  `DECISIONS.md`. Verify: the row is there; with "kept", the command in the README runs as written and is green.
  **Verdict (2026-10-10): kept.** `256 programs: 256 equal, 0 differ, 0 not compared` in 61 s; `self-test: 9
  recorded pairs, 10 distinct pairs, 0 failures`. Six normalisation rules, none naming a built-in: the statement
  frame; numbered temporaries and labels; redundant parentheses; an integer cast around a whole call argument; the
  `ll` suffix of a literal that is a whole argument; a `passed` mask written as bits joined by `|`. Both
  conditions of D8 hold. What it found: three wrong type values of console `INPUT` (task 6.4); groups 3, 5 and 7
  went in with no difference that was a bug. `python tools\callsite\callsite.py` is in the tier-2 table of
  `crates\README.md` (run as written: green) and a step of `rust.yml`'s `tier2` job. **The CI step is not seen
  running**: the job had no `qb64pe.exe` (sources and toolchain only), so the step that fetches QB64pe now also
  builds it as `setup_win.cmd` does (`mingw32-make OS=win BUILD_QB64=y`, cached; cache key `-3`). Row in
  `DECISIONS.md`, 2026-10-10.
- [x] 8.2 Full runs with the release build: tier 1, `slice.list`, the full corpus, upstream and differential
  lists (`--jobs 8 --build-cache target\build-cache`), with `QB64RUST_NO_FOLD=1` once for `slice.list`. Verify: no
  program wrong at run time; the numbers written into `STATUS.md` "Numbers at the last full runs".
  Done (2026-10-10, release build against the reference clone). Tier 1 green. `slice.list` 255 of 255, also with
  `QB64RUST_NO_FOLD=1`; upstream `pass.list` 50 of 50; differential 59 of 59; the call-site check 256 of 256.
  Full corpus (307 programs): 271 pass (the 255 of `slice.list` and 16 `.err` programs), 31 rejected (26 with an
  `.output`, the compile-only `239_lprint`, 4 `.err` programs with only "not supported yet" errors), 5 known
  failures. Full upstream suite (404): 50 pass, 309 rejected (293 with an `.output`, 16 compile-only), 44 `.err`
  programs with only marked errors, 1 known failure. Every failure is a rejection by the compiler (exit code 1);
  none builds and prints the wrong output, no compiler crash. Shrink-only lists unchanged: 0 false errors, 0 parse
  gaps, 64 only-marked rejections.
- [x] 8.3 Tier 1 on Civil War Strategy: still no real error; note here which of its marks are gone and which
  statements it needs next (input for step 11 and step 9a). Verify: `cargo test -p qb64rust-driver --test inputs`
  green.
  Done (2026-10-10): no real error; the compiler still stops at its cap of 100 errors, all "not supported yet",
  so only the marks of the first part of the file are seen. **Gone** (counted as lines of `CWSTRAT.BAS` that use
  them): `RND` 71, `INPUT` 28 (console and file), `CLOSE` 18, `OPEN` 15, `_FILEEXISTS` 10, `WRITE` 8, `DATA` 7,
  `MID$` statement and function 4, `SWAP` 3, `TIMER` 3, `NAME` 2, `RANDOMIZE`, `READ`, `RESTORE`, `EOF`, `SHELL` 1
  each. Its console `INPUT`s are compiled only under `$CONSOLE:ONLY`, which the game does not have. **Needed
  next, for step 11:** `DEFINT` (1 line, but it types every name), `COMMON` (12), arrays in procedures, a label
  with the name of a SUB (`cannon`, `ships`, `topbar`) and the label `CLS:`. **For step 9a and the screen:**
  `LINE` 658, `COLOR` 182, `LOCATE` 144, `PAINT` 86, `DRAW` 62, `PSET` 49, `CLS` 36, `PLAY` 32, `CIRCLE` 28,
  `SOUND` 28, `INKEY$` 25, `TAB` 22, graphics `PUT` 19 and `GET` 8, `VIEW` 11, `SCREEN` 9, `DEF SEG` 8, `BLOAD`
  4, `CSRLIN` 2, `POS` 1, `BEEP`, and `_TITLE`, `_ICON`, `_FULLSCREEN`, `$EXEICON`.
- [x] 8.4 Documents: `STATUS.md` (step 9 done, step 9a next, open items), `ROADMAP.md` if a milestone line
  changes, `crates\README.md` (what is supported; the "still marked" paragraph), `tests\corpus\README.md` and
  `tests\upstream\README.md` (counts), `study\00` (document map and §6 if a "keep" was added), `SOMEDAY.md` and
  `DIVERGENCES.md` for what was decided on the way, `CLAUDE.md` layout rows for `tools\callsite`, `tests\callsite`
  and `verification\v22*`. Verify: `python tools\repo_check\check_repo.py --untracked` clean; `openspec validate
  m2-builtin-statements --strict` passes; every count in the documents equals the run of 8.2.
  Done (2026-10-10). Also `tests\corpus\slice\SOURCE.md` (`s35`–`s44`), `tests\programs\README.md` and `study\00`
  §10 (the call-site check). `SOMEDAY.md`: five rows ("QB64pe behaviours to review": `SWAP` with a bad index,
  2.5 read into an `_INTEGER64`, `OPEN … FOR OUTPUT ACCESS READ`, `WRITE` and an inner quote, a quoted number in
  `DATA`). `DIVERGENCES.md`: no row; nothing was decided as a difference (`questions.md` Q3 is open). `study\00`'s
  document map: no new study. Repo check: 0 findings; strict validation passes.
- [x] 8.5 Tell the user which files to stage and suggest a commit message per group (`CLAUDE.md` human rule 1:
  Claude does not stage or commit). Verify: the list covers every path `git status --porcelain` shows.
  Done (2026-10-10), in the session's closing message.
