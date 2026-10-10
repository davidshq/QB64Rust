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
- [ ] 3.3 `OPEN` (both forms), `CLOSE`, `SEEK`: `StmtRule::Open`, `OpenOld`, `Close`; `ast.rs` accessor for
  `CloseStmt`; emission. Tests: `typed`, `ir`, `cpp` and `check-fail` front-end programs; `tests\callsite`
  programs for every mode, access and lock word and each argument type. Verify: `cargo test` green; the call-site
  tool reports them equal.
- [ ] 3.4 The file functions as rows: `EOF`, `LOF`, `LOC`, `SEEK`, `FREEFILE`, `_FILEEXISTS`, `_DIREXISTS`, `_CWD$`
  (design D4; bare names for `FREEFILE` and `_CWD$` as measured). Tests: a `typed` program; call-site programs.
  Verify: `cargo test` green apart from the coverage check, which names exactly these functions until 3.7's slice
  program exists.
- [x] 3.5 `PRINT #` and `WRITE` (file and console): `Op::Print` with `to`, `Op::Write`, `check\io.rs`, `ast.rs`
  accessors for `WriteStmt`, `codegen-cpp\src\io.rs` (design D5). Tests: `ir` and `cpp` programs for every item
  kind, separators and the empty forms (`PRINT #n, USING` stays marked). Verify:
  `cargo test` green; every existing `PRINT` snapshot unchanged.
- [x] 3.6 `INPUT #` and `LINE INPUT #`: `Op::Input` with `Source::File`, accessors for `InputStmt` and
  `LineInputStmt`, the type code per target type with a unit test against the codes read in 3.1, target places by
  their store rules. Tests: `ir`, `cpp`, `check-fail` (a target that is no place, a numeric `LINE INPUT` target).
  Verify: `cargo test` green.
- [ ] 3.7 Slice programs `s36_files` (write, append, read back, every item kind), `s37_file_errors` (52, 53, 55,
  62 and the others measured, under a handler), `s38_file_functions`, recorded with `qb64pe.exe`; corpus and
  upstream programs that now pass added to their lists. Verify: tier 2 green on the three lists; the corpus count
  in this task (programs newly on `slice.list`).

**Where group 3 stands (2026-10-10, session stopped at the user's request):** the code of 3.3 to 3.7 is written
and tier 1 is green (`cargo test`, clippy, fmt, ruff, repo check). 3.5 and 3.6 are ticked. Still open:
- 3.3, 3.4: the `tests\callsite` programs for `OPEN` (every mode, access and lock word, each argument type),
  `CLOSE`, `SEEK` and the file functions are not written, so "the tool reports them equal" is not shown. Expected
  to need one more normalisation rule: an integer literal's `ll` suffix as a whole call argument (`sub_seek(1,2ll)`
  against `sub_seek( 1 , 2 )`).
- 3.7: `s36_files`, `s37_file_errors` and `s38_file_functions` are recorded and pass with the new compiler, as do
  19 corpus programs (`05`, `52`–`54`, `59`, `60`, `86`, `95`–`97`, `174`, `176`, `178`, `180`–`183`, `213`,
  `219`); all 22 are on `slice.list` (223). The full tier 2 run over the three lists has not been repeated since,
  and `crates\README.md` does not list the file statements yet.
- At the end of group 3, not before (review of 2026-10-10; the counts change again when 3.7 closes):
  - the counts, after the tier 2 rerun: `crates\README.md` (43 built-in functions, now with the eight file
    functions; the statement list stops at `ENVIRON`), `tests\corpus\README.md` (34 `slice\` programs and 198 on
    `slice.list`; now 38 and 223), the numbers table of `STATUS.md` (198);
  - the wording of task 3.3, which still names `StmtRule::Open` and `OpenOld` (3.2: they do not exist);
  - `CLAUDE.md` "Layout": rows for `tests\callsite\` and `tools\callsite\` and `v22*` in the `verification\` row,
    once the call-site check has its verdict (no rows if it is dropped).
- For the call-site check's verdict: the tool drops every `qbs_cleanup` line as frame, so a statement that
  forgets its cleanup compares `equal` (to be listed in `tools\callsite\README.md`, "What it does not compare");
  it has found no bug in 23 programs with four normalisation rules, and a fifth is expected. `questions.md` Q1
  and Q2 are answered (`DECISIONS.md` 2026-10-10): rule 4 stays, elements stay out of the programs.

## 4. `DATA`, `READ`, `RESTORE`

- [ ] 4.1 Write and run `verification\v22_c_*` (design D1, "Data"), with include files under `verification\v22_inc\`
  where needed. Verify: outputs recorded; findings in `study\00` §5; the unmeasured remark on bare `DATA` in
  `ast.rs` replaced by the fact.
- [ ] 4.2 Correct design D6 and the delta `language/data-read` where 4.1 disagrees; list each correction here.
  Verify: `openspec validate m2-builtin-statements --strict` passes.
- [ ] 4.3 `sema`: collect the items in the measured order into the program's data; `READ` targets; `RESTORE` with
  and without a label (a line-number target stays marked); the three marks removed. Tests: `typed` programs (data
  across a procedure and an included file under `tests\frontend\inc\`), `check-fail` (unknown label, a target that
  is no place, text after a closing quote). Verify: `cargo test` green.
- [ ] 4.4 `ir` and `codegen-cpp`: `Program::data`, `Op::Read`, `Op::Restore`, `validate`, the data fragment and the
  per-type read calls. Tests: `ir` and `cpp` programs; call-site programs for `READ` into each type and both
  `RESTORE` forms. Verify: `cargo test` green; the tool reports them equal.
- [ ] 4.5 Slice program `s39_data` (every measured item form, out of data under a handler, `RESTORE` to labels),
  recorded; lists updated. Verify: tier 2 green; the corpus count in this task.

## 5. `SWAP`, the `MID$` statement, `RANDOMIZE`, `RND`, `TIMER`

- [ ] 5.1 Write and run `verification\v22_d_*` for these (design D1, "The rest", first half). Verify: outputs
  recorded; findings in `study\00` §5.
- [ ] 5.2 Correct the delta `language/builtin-statements` (SWAP, MID$, RANDOMIZE) and `language/builtin-functions`
  where 5.1 disagrees; list each correction here. Verify: strict validation passes.
- [ ] 5.3 `SWAP` (`StmtRule::Swap`, accessor for `SwapStmt`, the swap entry per type) and the `MID$` statement
  (`StmtRule::MidAssign`; the parser's node for `MID$(…) = …` checked first: `tests\known_clean_not_passing.list`
  and the mark `MID$(...)` show how it parses today). Tests: `ir`, `cpp`, `check-fail` (different types, a literal
  operand, a target that is no string place); call-site programs per type. Verify: `cargo test` green; equal.
- [ ] 5.4 `RANDOMIZE` (row, with `USING`; the seedless form as measured or marked), `RND` and `TIMER` (rows, bare
  and with an argument; the bare-name property of design D4 replacing the `_PI` special case, `_PI`'s snapshots
  unchanged). Tests: `typed`, `ir`, `check-fail`; call-site programs. Verify: `cargo test` green; equal.
- [ ] 5.5 Slice programs `s40_swap_mid` and `s41_random` (seeded sequences; `TIMER` only in comparisons), recorded;
  lists updated. Verify: tier 2 green; the corpus count in this task.

## 6. Console `INPUT` and `LINE INPUT`

- [ ] 6.1 `run_legacy_tests.py`: the `<name>.stdin` sidecar (delta `testing/golden-corpus`), for check and record,
  both compilers; the same rule in `verification\run.sh`; `tests\corpus\README.md` and the tool's README say so.
  Verify: the runner's own tests (or a dry run on two throwaway programs, one with and one without the sidecar)
  pass; a full corpus run with the old compiler gives the recorded baseline (no program changes result).
- [ ] 6.2 Write and run `verification\v22_e_*` with `.stdin` files (design D1, "Console input"). Verify: outputs
  recorded, the same on two runs; findings in `study\00` §5, including how `END` behaves with redirected input.
- [ ] 6.3 Correct design D7 and the console-input requirement where 6.2 disagrees; list each correction here; if
  the old compiler's behaviour is not reproducible under the runner, stop and put the options to the user (the
  statements stay marked meanwhile). Verify: strict validation passes.
- [ ] 6.4 `Op::Input` with `Source::Console`: prompt forms, targets, emission (design D5). Tests: `ir`, `cpp`,
  `check-fail` (a prompt that is no literal as the old compiler reports it, a target that is no place); call-site
  programs. Verify: `cargo test` green; equal.
- [ ] 6.5 Slice program `s42_console_input` with its `.stdin`, recorded; lists updated. Verify: tier 2 green.

## 7. `SHELL`, `COMMAND$`, `ENVIRON$`, and functions by demand

- [ ] 7.1 Write and run `verification\v22_d_*` for `SHELL` (statement forms and function), `COMMAND$`, `ENVIRON$`
  (design D1, "The rest", second half). Verify: outputs recorded (machine-dependent text kept out of them);
  findings in `study\00` §5.
- [ ] 7.2 Rows and rules: `SHELL` ×3 and the function, `COMMAND$` (bare and indexed), `ENVIRON$` (`StrOrIndex` only
  if measured as two entries). Tests: `typed`, `ir`, `check-fail`; call-site programs. Verify: `cargo test` green;
  equal.
- [ ] 7.3 Slice program `s43_shell_environ` (with `.normalize` if needed), recorded; lists updated. Verify: tier 2
  green.
- [ ] 7.4 Count the corpus and upstream blockers again (the marks of every program not on a pass list, as in
  `proposal.md`) and write the table here. For each plain function named there or by Civil War Strategy that an
  existing rule may fit (candidates: `_ACOS`, `_ASIN`, `_SINH`, `_COSH`, `_TANH`, `_COT`, `_CSC`, `_SEC`, `_D2R`,
  `_R2D`, `_CEIL`, `_STRCMP`, `_STRICMP`, `CSRLIN`, `POS`): measure its result type and edge values
  (`verification\v22_f_*`), add the row, a `typed` test, a call-site program and a line in a slice program
  (`s44_more_functions`); leave it marked when no rule fits, with the reason in this task. Update the list in the
  delta `language/builtin-functions`. Verify: `cargo test` green (coverage check); tier 2 green; the delta's list
  equals `SUPPORTED`.

## 8. The verdict and integration

- [ ] 8.1 The call-site verdict (design D8, spec `testing/call-site-check`): run the tool over all of
  `tests\callsite`, write the counts and the normalisation rules here, decide by the two conditions, and put the
  verdict to the user. "Kept": add the tier-2 command to `crates\README.md` and `rust.yml` (job `tier2`).
  "Dropped": delete `tools\callsite`, `tests\callsite` and the capability's delta. Either way a dated row in
  `DECISIONS.md`. Verify: the row is there; with "kept", the command in the README runs as written and is green.
- [ ] 8.2 Full runs with the release build: tier 1, `slice.list`, the full corpus, upstream and differential
  lists (`--jobs 8 --build-cache target\build-cache`), with `QB64RUST_NO_FOLD=1` once for `slice.list`. Verify: no
  program wrong at run time; the numbers written into `STATUS.md` "Numbers at the last full runs".
- [ ] 8.3 Tier 1 on Civil War Strategy: still no real error; note here which of its marks are gone and which
  statements it needs next (input for step 11 and step 9a). Verify: `cargo test -p qb64rust-driver --test inputs`
  green.
- [ ] 8.4 Documents: `STATUS.md` (step 9 done, step 9a next, open items), `ROADMAP.md` if a milestone line
  changes, `crates\README.md` (what is supported; the "still marked" paragraph), `tests\corpus\README.md` and
  `tests\upstream\README.md` (counts), `study\00` (document map and §6 if a "keep" was added), `SOMEDAY.md` and
  `DIVERGENCES.md` for what was decided on the way, `CLAUDE.md` layout rows for `tools\callsite`, `tests\callsite`
  and `verification\v22*`. Verify: `python tools\repo_check\check_repo.py --untracked` clean; `openspec validate
  m2-builtin-statements --strict` passes; every count in the documents equals the run of 8.2.
- [ ] 8.5 Tell the user which files to stage and suggest a commit message per group (`CLAUDE.md` human rule 1:
  Claude does not stage or commit). Verify: the list covers every path `git status --porcelain` shows.
