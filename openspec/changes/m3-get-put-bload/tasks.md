# Tasks

Order: group 1 first (the rest of the measurements), then 2 to 6 in that order (design D8), then 7. Each of groups
2 to 6 ends with `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` and `cargo test`
(tier 1) green and with its tests, lists and `crates\README.md` lines in place, so the change can stop after any
group.

Standing rules for every task: work inline, no subagents; nothing staged or committed by Claude: each group ends
by naming the files to stage and a commit message; text with backslashes goes through the Edit and Write tools;
`python tools\repo_check\check_repo.py --untracked` is clean before a group is reported done. **Test cadence**
(`CLAUDE.md` working rule 9): while working, tier 1 and only the programs the work touches (`--glob`,
`--category`, or a list with `--pass-cache target\pass-cache`); tier 2, the run without folding and the full suites
run once, in group 7. CI is not a step of this change. A new measured fact goes into the `v28` paragraph of
`study\00` §5 in the task that measures it. A behaviour of the old compiler that looks questionable is implemented
its way and listed in `SOMEDAY.md` ("QB64pe behaviours to review").

## 1. Measurements before code

- [x] 1.1 Measure, before the design, what the proposal rests on: the C++ of each built-in (`qb64pe -z`, programs
  outside the repository), the label rule, accepted and rejected forms one program each. Verify: `verification\`
  holds the programs with their `.compile.txt` and `.out.txt`; `study\00` §5 has the facts.
  Found (2026-10-12): 83 programs, `v28_a_tab_spc`, `v28_b_bload`, `v28_c_labels`, `v28_x01_accepted`,
  `v28_x03`–`x81` (no `x02`). Ten built-ins are plain calls; graphics `GET` and `PUT` are plain calls with an
  argument made of an array's bytes; `VARPTR` and `VARSEG` are written by the compiler and move their variable
  into conventional memory; `name:` with a SUB's name is a call unless a jump names the word. A `PRINT` with a
  comma in a console program with redirected output never ends (known, `study\00` §5): the first recording of
  `v28_a_tab_spc` had such a line and wrote 1.8 MB of blanks; the line was taken out and the program recorded
  again.
- [x] 1.2 Measure what the design leaves open, one program each where a form is accepted or rejected
  (`verification\v28_x82…`), the C++ outside the repository:
  - every statement that names a label, with a SUB's name as its target (`RESUME one`, `RETURN one`, `ON n GOTO
    one`, `ON n GOSUB one`, `RUN one`, `ON KEY(1) GOSUB one`, `ON TIMER(1) GOSUB one`, and each other place of
    `qb64pe.bas` that makes a reference to a label): does the line `one:` become a label (design D1);
  - `VARSEG(a(f(1)))` and `VARPTR(a(f(1)))` with a FUNCTION that prints: is the index evaluated (D3); `VARPTR` and
    `VARSEG` in `HEX$`, `STR$`, an operation and a store into an INTEGER;
  - a conventional array with `ERASE`, in `COMMON`, passed to a SUB, with `LBOUND`/`UBOUND`, larger than 65,536
    bytes: the C++ of each and what the program prints (D4);
  - the C++ of `VARPTR`/`VARSEG` for each kind left marked (scalar, string, fixed-length string, dynamic array,
    local and `STATIC` array, array parameter, user-type array, member), `SADD` and `VARPTR$`, kept in the design
    under D4 for the change that takes them;
  - `GET (1, 2)-(3, 4), a(f(1))`: how often `f` runs (D5); `GET (1), , x` and `PUT (f), 2, x`: graphics or file
    form; the C++ of `GET`/`PUT` with a user-type array, a string array and a plain variable (left marked);
  - `TAB`, `SPC`, `PEEK`, `_RGB32`, `_RGBA32`, `_FULLSCREEN` each with one slot as INTEGER, LONG, `_INTEGER64`,
    SINGLE, DOUBLE and a literal, and each function in `PRINT`, `HEX$` and an operation.
  Verify: the design's D1, D3, D4 and D5 say what was found in place of "task 1.2 measures"; the spec scenarios
  name forms the old compiler does accept or reject, corrected if not; the reference clone's `git status
  --porcelain` is empty before and after.
  Found (2026-10-13): 16 programs more (`v28_d_varptr_index`, `v28_e_cmem_arrays`, `v28_x82`–`x95`), 99 in
  all. Every jump statement makes its word a label; `IF c THEN name` and `CALL name` do not; `RUN name` is an
  error. The index of `VARPTR` is evaluated once, that of `VARSEG` not at all, that of a `GET`/`PUT` array
  twice. **An array listed in `COMMON` is dynamic to the old compiler**, and the game's `mtn` is one: in
  conventional memory it starts with flag 4, and every allocation of a dynamic array tests that flag (design
  D4, which now covers dynamic arrays of the main module too). `GET (1), , x` is the graphics form and a
  syntax error. The C++ of each kind left marked is in `SOMEDAY.md` and `study\00` §5.
- [x] 1.3 Put the design's three open questions to the user (conventional memory for static numeric arrays of the
  main module only; `_RGB32`, `_RGBA32` and `_FULLSCREEN` in this change; data files as copies in
  `<name>.files\`). Verify: the answers are a row of `DECISIONS.md`, and the proposal, the design and the groups
  below follow them.
  Found: yes to all three (user, 2026-10-13); the design was written for these answers, so nothing below
  changes.

## 2. Labels with the name of a SUB (design D1)

- [x] 2.1 `sema`: collect the words named by the jumps of the whole program before the labels are entered; a line
  `name:` of a SUB without a required parameter is a call when no jump names the word and a label when one does;
  a SUB with a required parameter and a FUNCTION are labels; the two marks go. The safeguard of D1 for a
  statement the parser could not read. Tests: an `ir` front-end test (a call in the main module, a label and a
  `CALL` in a SUB, a jump in another body, a statement after the colon); a `check-fail` test with `v28_x71`,
  `x72`, `x74`; a unit test of the collection that names every jump statement of task 1.2. Verify: `cargo test`
  passes; no existing snapshot changes but those that held the marks.
  Found: `collect_jump_words`, `declare_label`, `label_stmt` (`sema\src\check\flow.rs`). Tests
  `sub_name_colon_ir` and `sub_name_colon_errors` (the latter also has each jump statement with a name that
  stands twice: a duplicate label only when the line is a label, which is the test of the collection asked
  for; no separate unit test). One older snapshot changed, `error_handling_errors`: its line `s: PRINT 2` beside
  `SUB s (k AS LONG)` is a label now.
- [x] 2.2 A console slice program `s70_sub_name_labels` (from `v28_c_labels`, `x73`, `x75`–`x81`), recorded with
  `qb64pe.exe`, on `tests\corpus\slice.list`. Verify: it passes with the new compiler, with and without folding
  (that program only).
  Found: recorded, and it passes on its first run, with and without folding.
- [x] 2.3 The game after the group: `qb64rust --report unsupported tests\programs`. Verify: no label mark is left
  (128 marks or fewer); `tests\programs\README.md` gets its dated paragraph in task 6.4, not here.
  Found: the five label marks are gone (128 marks in 10 messages).

## 3. The plain rows (design D2)

- [x] 3.1 `TAB` and `SPC`: two function rows. Tests: call-site programs (each slot type; as a `PRINT`, `PRINT #`
  and `WRITE` item; in an expression); a `typed` front-end test; the rejected forms of `v28_x03`–`x09`, `x53` in a
  `check-fail` test, each an error that is not "not supported yet"; a console slice program `s71_tab_spc` (from
  `v28_a_tab_spc`), recorded. Verify: the call-site programs compare equal (`python tools\callsite\callsite.py`
  on them); `s71` passes; `crates\README.md` no longer names `TAB` and `SPC` as unsupported.
  Found: two `Rule::Plain` rows and nothing else. `184_print_tab` and `185_print_spc` of the corpus, which
  waited on them, pass and are listed. The typed test is `m3_step2_functions_typed`, the rejected forms are in
  `m3_step2_errors`; `TAB(5) = 1` (`x53`) keeps the older mark for a built-in function's name as a place. With
  `TAB` compiled the game showed one more mark, `DATE$`, on a line `TAB` had hidden: `DATE$` and `TIME$` came
  in as two more plain rows (measured from the C++: `func_date()`, `func_time()`; call-site programs; `s71`
  prints their lengths and separators).
- [x] 3.2 `DEF SEG`, `BLOAD`, `BSAVE`, `POKE` as statement rows and `PEEK` as a function row. Tests: call-site
  programs per form and per slot type; an `ir` test; the rejected forms of `v28_x26`–`x35`, `x42`–`x44`, `x61`,
  `x67` in a `check-fail` test; `DEF SEG = 1: l = 2` (`x62`). Verify: the call-site programs compare equal;
  `cargo test` passes (the coverage test once task 4.3 has listed the slice program).
  Found: five plain rows; the parser read `DEF SEG` by its template already. `ir` test
  `conventional_memory_ir`. `CALL POKE(…)` and `CALL BSAVE(…)` compile (no template); `CALL BLOAD(…)` is
  marked and `CALL DEF SEG` (`x67`, an error in the old compiler) keeps the parser's mark.
- [x] 3.3 `_RGB32`, `_RGBA32` (function rows, one to four arguments for the first) and `_FULLSCREEN` (statement
  and function; in `WINDOW_BUILTINS`). Tests: call-site programs per form and slot type; a `typed` test; the
  rejected forms of `v28_x45`–`x52`; a line with `_RGB32` and `_RGBA32` in a console slice program; `_FULLSCREEN`
  as code that is not reached in the slice program of the window built-ins. Verify: the call-site programs
  compare equal; `m32_newimage_32` compiles, leaves the recorded state with and without folding, and is on
  `slice.list`.
  Found: `_RGB32` needed a rule of its own, `Rule::GivenArgs(1)` (one to four arguments, the call written with
  those given: libqb has an entry for each number); the others are plain. `_FULLSCREEN` stands as code that is
  not reached in `s71_tab_spc`, not in `s66_window_builtins`, whose recording was left alone. `m32` passes.
- [x] 3.4 A screen program `d17_tab_window` (design D7), written and recorded with `qb64pe.exe` (`--record
  --category screen --glob "d17*.bas"`), its state looked at as a picture before it is trusted. Verify: it passes
  with the new compiler and is on `slice.list`.
  Found: recorded, read as text rows (a text screen), passes on its first run.

## 4. `VARPTR`, `VARSEG` and conventional memory (design D3, D4)

- [x] 4.1 `ir`: the flag on an array, the two expression kinds, the validator's rules for them; `sema`: `VARPTR`
  and `VARSEG` take a place, set the flag for a static numeric array of the main module, and mark every other
  kind with its own message; `SADD` and `VARPTR$` marked. Tests: an `ir` front-end test (the flag set by a use
  after the `DIM`, in a procedure, through `COMMON`); a `typed` test of the believed type; a `check-fail` test
  with `v28_x36`–`x41`, `x54`–`x57`; a front-end test with one line per marked kind. Verify: `cargo test`
  passes.
  Found: as designed, with dynamic arrays of the main module supported too (task 1.2). The marked kinds are in
  `m3_step2_marked` (33 marks, no error). Two false errors showed in tier 1 on the way and were fixed:
  `VARSEG(DesignArray(0))` of a `TYPE` array, in two of QB64pe's `qbasic_testcases`, got "a whole `TYPE` value
  cannot be used here"; an element of a user type is now a place there and gets the mark. `lookup` finds a
  function by its suffix too (`VARPTR` and `VARPTR$` are two table entries).
- [x] 4.2 `codegen-cpp`: the allocation of a conventional array and what task 1.2 found for `ERASE` and `COMMON`;
  the two expressions. Tests: a `cpp` front-end test (allocation and both expressions); call-site programs for
  `VARPTR` and `VARSEG` as arguments (one and two dimensions, an index that is an expression). Verify: the
  call-site programs compare equal; a program with a conventional array and one without differ in the C++ only at
  that array.
  Found: `cpp` test `conventional_memory_cpp`. The flag test is written at every allocation and release of a
  dynamic array of numbers, so five `cpp` snapshots of earlier changes gained those lines (and only those).
  `VARSEG` has call-site programs; `VARPTR` has none, because the check cannot compare an element's index
  (`tools\callsite\README.md`): its text is in the `cpp` test and its values in `s72`.
- [x] 4.3 A console slice program `s72_bload_peek_poke` (from `v28_b_bload`, no write past an array's end),
  recorded. Verify: it passes with and without folding; the game's report has no `DEF`, `BLOAD` mark left.
  Found: two programs, `s72_bload_peek_poke` and `s73_conventional_arrays` (the dynamic cases, from
  `v28_e_cmem_arrays`); both pass on their first run.
- [x] 4.4 Review of groups 2 to 4, inline, by the panel's roles (the extra role a runtime engineer: libqb's
  prototypes of every new call, and `cmem` in `libqb.cpp`). Verify: each finding is fixed or listed in
  `SOMEDAY.md`; `crates\README.md` says what the compiler supports now.
  Found, by role (done once for groups 2 to 5, after the code of all of them stood):
  - *Runtime:* the prototypes of every new call were read in `qbx.cpp` and libqb's headers: `sub_defseg(int32,
    int32)`, `func_peek(int32)`, `sub_poke(int32, int32)`, `sub_bload(qbs*, int32, int32)`, `sub_bsave(qbs*,
    int32, int32)`, `func_tab(int32)`, `func_spc(int32)`, four `func__rgb32` with one to four `int32_t`,
    `sub_graphics_get(float ×4, void*, uint32, int32)`, `sub_graphics_put(float ×2, void*, int32, uint32,
    int32)`, the three-argument `byte_element`, `cmem_dynamic_malloc(uint32)`. Each argument written is what
    the entry takes. A conventional array larger than a segment is measured and compared
    (`verification\v28_f_cmem_too_large`).
  - *Compiler and languages:* a `_BIT` array was accepted by `VARPTR` in the first version (it counts as an
    integer type); marked now, as the emitter never puts one into conventional memory. The flag is set by a
    use anywhere, also in code that is not reached, as the old compiler's second pass does.
  - *QB64:* `PUT (1, 2), k` with a `CONST` and `GET (1), , x` got a mark where the old compiler has an error;
    both are errors now. `TAB(5) = 1` and `CALL DEF SEG` keep older marks (listed in `SOMEDAY.md`).
  - *Rust:* two wildcard match arms of the first version went (clippy's rule here); `Slot`, `StmtArg`,
    `StmtRule`, `Rule` and `ExprKind` are matched exhaustively in the checker, the validator and the emitter.
  - *Test:* tier 1 found the two false errors of task 4.1 through QB64pe's own test programs, not through a
    test written for the change; the marked forms now have a line each in `m3_step2_marked`.
  - *Pragmatic:* the dynamic case and `DATE$`/`TIME$` are beyond what was agreed; both are in `DECISIONS.md`
    for the user.

## 5. Graphics `GET` and `PUT` (design D5)

- [x] 5.1 `sema`: the slot kind of an array's bytes; rows for `GET` and `PUT` that stand for the graphics entries
  only, the file form keeping its mark; the marks for a user-type array, a string array and a plain variable.
  `codegen-cpp`: the argument and its temporary, declared with the body's variables. Tests: call-site programs per
  form (each `STEP`, each action, `_CLIP`, a colour, an argument left out), per array kind (static, dynamic, two
  dimensions, from an element, a local array, a `STATIC` array, a parameter) and per coordinate type; rule 2 of
  the call-site check renumbers `byte_element_N`, with a self-test pair; an `ir` test; a `check-fail` test with
  `v28_x10`, `x11`, `x14`–`x19`, `x21`, `x22`, `x24`, `x25`, `x64`; a front-end test with the marked forms.
  Verify: the call-site programs compare equal; `cargo test` passes.
  Found: `StmtRule::Graphics`, `Slot::Block`, `StmtArg::Block`; `ir` test `get_put_ir`. An index that calls a
  FUNCTION is marked (the old compiler runs it twice). `PUT (1, 2), k` with a `CONST` is an error; `GET (1), ,
  x` too. The call-site programs have whole arrays only (the check cannot compare an element's index); the
  element forms are in `d18_get_put` and the `cpp` test.
- [x] 5.2 A screen program `d18_get_put` (design D7), written and recorded, its state looked at as a picture.
  Verify: it and `g06_message_box` pass with the new compiler, with and without folding, and are on `slice.list`.
  Found: both pass on their first run. The picture of `d18` shows each action, the colour forms, both clipped
  edges, and three errors 5.
- [x] 5.3 The game after the group. Verify: `qb64rust --report unsupported tests\programs` names only `SOUND`,
  `PLAY`, `BEEP` and `$EXEICON`.
  Found: exactly those four: `SOUND` 33, `PLAY` 31, `BEEP` 1, `$EXEICON` 1 (66 marks in 4 messages; 135 in 15
  before the change).

## 6. Data files and more programs cut from the game (design D6, D7)

- [x] 6.1 The corpus runner: `<name>.files\` copied into the run folder before the compile, for a run and for
  `--record`; its files in `pass_key`; the misuse an error in the test. `.gitattributes`: `*.files/**` `-text`.
  Tests in `tools\legacy_tests\test_run_legacy_tests.py`: the copy, a changed file seen by the pass cache, a
  `.files` folder beside an `.err`. Verify: `python -m unittest discover -s tools/legacy_tests -p "test_*.py"`
  passes; `tools\legacy_tests\README.md` and `tests\corpus\README.md` describe the folder.
  Found: 68 tests pass (64 before). The new tests found a fault of the first version: the runner empties the
  run folder before each run, which took the data files with it; they are now copied again before each run.
- [x] 6.2 `tools\screen\cut_game.py`: the new programs of design D7 (the mountains, the icon and capital marker,
  a face and a fort, the army animation, a report with `TAB`), each with the pictures it loads copied into its
  `.files\`; which procedures can be cut whole is decided here and said in `tests\corpus\screen\SOURCE.md`.
  Recorded with `qb64pe.exe`; each state looked at as a picture. Verify: the old compiler passes all of
  `screen\`; `python tools\screen\cut_game.py` run twice changes nothing.
  Found: five programs, `g07_mountains`, `g08_icon`, `g09_faces_forts`, `g10_snapshot`, `g11_city_report`. The
  map (`SUB usa`) and the reports read the whole state of a game, so four of the five hold ranges of the
  game's lines among written ones, a kind of cut the script gained; only `snapshot` is a whole procedure. The
  army animation was not cut (`SUB animate` needs the armies and cities); `g10` and `g06` cover a `GET` and
  `PUT` pair in a SUB. All five pictures were looked at.
- [x] 6.3 Run the new programs with the new compiler; add those that pass to `slice.list`. Verify: each passes
  with and without folding; a program that compiles and leaves another state is fixed before the group closes;
  one that still waits (on sound) is named in `tests\corpus\README.md` with what it waits on.
  Found: all five pass on their first run, with and without folding. 51 of the 52 screen programs are listed;
  `g03_capitol` waits on `PLAY`.
- [x] 6.4 `tests\programs\README.md`: a dated "Measured" paragraph with what the game still needs.
  `SOMEDAY.md`: "Left marked by `m3-get-put-bload`", one row per mark the new code can give, with the reason and
  the measurement; "QB64pe behaviours to review" gains what the measurements found; the row "Data files beside a
  corpus program" goes. Verify: every "not supported yet" message of the new code has a row.
- [x] 6.5 Review of groups 5 and 6, as 4.4. Verify: findings fixed or listed.
  Found (group 5 is in 4.4):
  - *Test:* the runner's new tests caught the data files being removed before the run (task 6.1). The
    call-site check reported 12 of the 159 programs written first as different, none a different call (an
    element's index, operands of mixed width, a parenthesised operation); they were taken out
    (`tools\callsite\README.md`). 810 of 810 equal.
  - *QB64:* every recorded state of the seven new screen programs was looked at as a picture or as text rows
    before it was trusted; the game's pictures are the game's (mountains, faces, forts, the icon).
  - *Pragmatic:* `g07`, `g08`, `g09` and `g11` are ranges of the game's lines, not whole procedures; the
    script stops when a range's first line has changed, so a new copy of the game cannot shift one unseen.
  - *Compiler, Rust, runtime:* nothing beyond 4.4.

## 7. Closing runs and documents

- [x] 7.1 The closing runs, each once, release build, `--jobs 8 --build-cache target\build-cache`, without
  `--pass-cache`: tier 1; the runner's own tests; the call-site check; `slice.list`, also with
  `QB64RUST_NO_FOLD=1`; the upstream and differential pass lists; the full corpus and the full upstream suite with
  the new compiler; the shrink-only lists regenerated (`QB64RUST_UPDATE_LISTS=1 cargo test -p qb64rust-driver
  --test inputs`) and every program that newly compiles cleanly settled (a pass list, or
  `tests\known_clean_not_passing.list` with the reason). Verify: none wrong at run time; the numbers are in
  `STATUS.md` "Numbers at the last full runs". An edit after these runs is rerun only for what it can affect, and
  the task says which runs were not repeated and why.
  Found (2026-10-13, each run once): tier 1 292 tests; the runner's tests 68; call-site 810 of 810 equal
  (55 s with its cache); slice 357 of 357 with folding (326 s) and without (260 s); upstream pass list 140;
  differential 59; full corpus 376 pass of 395, 13 rejected, 1 `.err` only marked, 5 known failures, none
  wrong; full upstream as before (140 pass, 263 rejected or only marked, 1 known failure). The shrink-only
  lists did not change (`known_lists_are_exact` passes without a regeneration). The four programs that newly
  compiled cleanly are on `slice.list`. After the runs: one verification program was recorded and compared
  (`v28_f_cmem_too_large`: both compilers end with "Out of memory" at the start), a line was taken out of
  `tests\frontend\files_errors.bas` with its snapshot (tier 1's front-end test run again), and documents were
  written; no compiler source changed after the runs, so none was repeated.
- [x] 7.2 Documents: `STATUS.md` (where we are, next: step 3, sound), `ROADMAP.md`, `crates\README.md`,
  `study\00` (§5 complete for `v28`, the M3 row), `CLAUDE.md`'s layout table (`verification\`, `tests\corpus\`),
  `DECISIONS.md` for anything decided on the way. Verify: `python tools\repo_check\check_repo.py --untracked` is
  clean; `openspec validate m3-get-put-bload --strict` passes.
