# Tasks

Order: group 1 first (the last measurements), then 2 (everything else builds on it), then 3 to 6 in any order
(3 and 4 add rows; 5 and 6 stand alone), then 7. Every group closes with `cargo fmt --check`, `cargo clippy` and
`cargo test` green and with its tests, lists and `crates\README.md` lines in place, so the change can stop after
any group.

Standing rules for every task: work inline, no subagents; nothing staged or committed by Claude: each group ends
by naming the files to stage and a commit message; text with backslashes goes through the Edit and Write tools;
`python tools\repo_check\check_repo.py --untracked` is clean before a group is reported done. "Tier 2 slice" below
is `python tools\legacy_tests\run_legacy_tests.py --suite corpus --qb64 target\release\qb64rust.exe --list
tests\corpus\slice.list`, run also with `QB64RUST_NO_FOLD=1`. "The call-site check" is `python
tools\callsite\callsite.py`. A new measured fact goes into the `v26` paragraph of `study\00` §5 in the task that
measures it.

Already measured before this proposal (2026-10-12): `verification\v26_a`–`v26_h2` and `v26_x01`–`x39`, 51
programs, recorded with `qb64pe.exe`.

## 1. Measurements before code

- [x] 1.1 Measure the two line rules of design D2 for other metacommands: `verification\v26_x40…`, one program
  each, for a tab before `$IF`, `$LET`, `$INCLUDEONCE`, `$CONSOLE:ONLY` and `$SCREENHIDE`, and for each of them
  but `$SCREENHIDE` (measured, `v26_x36`) after a statement and a colon. Record with `python
  tools\legacy_tests\run_legacy_tests.py --suite verification --glob "v26_x*.bas"`. Verify: a `.compile.txt` and
  an `.out.txt` exist for each; `study\00` §5 says for which metacommands each rule holds; design D2 and its open
  question are rewritten to match.
  Found (2026-10-12; `v26_x40`–`x50`, 11 programs, include files in `v26_inc\`): both rules hold for all of
  them, and in the source each is one place that names no metacommand, so both go into `syntax` for every
  metacommand (D2 rewritten). Two programs beyond the list: `x49` (a tab before `$IF` and its `$END IF`) and
  `x50` (a tabbed `$ELSE` and `$END IF` in a branch that is not taken: skipped text; the new compiler rejects
  that program today, a false error for 2.1 to fix). Today the new compiler accepts `x40`–`x43`, `x47`–`x49`.
- [x] 1.2 Read the old compiler's make line for four programs (`$CONSOLE:ONLY`; `$CONSOLE`; both; `$CONSOLE` with
  `_ICON`): find where `qb64pe -x` leaves it (the clone's git-ignored `internal\temp`: its `compilelog.txt` holds the
  link line and the name of the libqb variant, seen for `v26_x38`; or its `-q`-less output)
  and note the `DEP_*` flags of each. Verify: the four flag sets are written into design D3, replacing "read in
  task 1.2", and D3's mapping is corrected if a flag differs from it.
  Found (2026-10-12): the make line itself is kept nowhere on Windows; the flags are read from what it does
  (the `-D` defines, the variant's name and the link flag in `compilelog.txt`) and from the source lines that
  build it. D3 holds the table. The mapping stands; one flag is added to it (`DEP_CONSOLE=y` under
  `$CONSOLE:ONLY` too, which changes no build), and `_ICON` means a third libqb variant. The four programs were
  compiled outside the repo (three to six lines each: the metacommands of the table's first column, `_DEST
  _CONSOLE` where there is a window, `PRINT "ok"`, `SYSTEM`, and `_ICON` after it; each printed `ok`); all
  three variants exist in the local clone already, and its `git status` was the same before and after.
- [x] 1.3 Read the old compiler's C++ (`qb64pe -z`) for one call of each of the 20 window built-ins and of
  `_DEST`, `_SOURCE`, `_CONSOLE`, `_ECHO`, in every form of its template. Note per built-in: the call, the
  conversion of each argument, the result type as the generated code holds it, and whether an existing rule of
  `sema\builtins.rs` fits. Verify: design D6 holds a table with one line per built-in, and its three "oddities"
  (`_SCREENEXISTS`, `_WINDOWHANDLE`, the parenthesised pairs) each have an answer.
  Found (2026-10-12): D6 holds the table (67 lines of forms and 54 of result-type probes, `qb64pe -z`, outside
  the repo; the call-site programs of 3.1 and 4.1 keep each form). No new rule: every statement is
  `StmtRule::Plain`, every function `Rule::Plain` but `_SCREENEXISTS`, which is `Rule::Fixed(LONG)` with one
  marked case (as the direct argument of `HEX$`, `OCT$`, `_BIN$`: the old compiler passes width 0, not 8).
  `table_ty` has to learn `_UNSIGNED _OFFSET` for `_WINDOWHANDLE`. The parser accepts every form today.
- [x] 1.4 Measure the flags once more where D7 assumes: `v26_x…` programs for `$IF _EXPLICIT_` between two `OPTION
  _EXPLICIT` lines, `$ASSERTS:CONSOLE` with `$IF _CONSOLE_` above and below it (compile only if the program is
  otherwise rejected), and a program that needs three rounds (a `$CONSOLE` inside `$IF _EXPLICIT_`, with `OPTION
  _EXPLICIT` on the last line). Each program that runs prints where it is recorded: under `$CONSOLE:ONLY`, after
  `_DEST _CONSOLE`, or with `_ECHO`. (D7's rule for a round that is not the last and a `$CONSOLE:ONLY` in its
  own flag's branch are measured: `v26_x39`, `x38`.) Verify: each is recorded; D7 says what each showed; a
  second corner where the old compiler's passes disagree, if found, is added to D7 and to the `compiler/pipeline`
  delta as a marked case.
  Found (2026-10-12; `v26_x51`–`x56`, six programs): each as D7's port gives it, and no second corner (D7 says
  why, from the source). One thing D7 did not have: `OPTION _EXPLICIT` is in force by its flag, not by its
  line (`x54`, `x55`); added to D7, to the `compiler/pipeline` delta as two scenarios (that one, and three
  rounds) and to task 6.2. `x56` measures the running value of `_ASSERTS_`, which D7 had read from the source.

## 2. The console mode, the metacommands and the build (design D1–D4)

- [x] 2.1 `sema`: the four metacommands matched exactly, each rejected form an error of the program (D2); the
  mode recorded (console, window, hidden; D1); "select the console" pushed at every `$CONSOLE:ONLY`; the mark
  reworded to "programs without `$CONSOLE` or `$CONSOLE:ONLY`". Tests: a `check-fail` front-end test with every
  rejected form of `v26_x09`–`x14`, `x23`, `x29`–`x32`, `x35`–`x37` (and the forms of 1.1 that hold), a
  `check-ok` test with the accepted forms of `x01`, `x04`, `x08`, `x15`. Verify: `cargo test` passes with the
  new snapshots reviewed; each rejected form's diagnostic is not marked "not supported yet".
- [x] 2.2 IR: `Program::settings` (console, window, window hidden, runtime parts), `lower`, `validate`, the dump's
  `settings` line (D3). Tests: an `ir` front-end test for `$CONSOLE` with `$SCREENHIDE`, one for `$CONSOLE:ONLY`,
  one for both metacommands, one for `$SCREENHIDE` followed by `$SCREENSHOW`. Verify: the four snapshots show the
  settings the `language/console-and-window` delta names; existing `ir` snapshots change by that one line only.
- [x] 2.3 Emitter and driver: the two globals written from the settings; the make flags from the settings in
  place of the fixed `DEP_CONSOLE_ONLY=y`; the flags hashed into the build key and its version string changed
  (D3, D4). Tests: a `cpp` front-end test for the globals in each of the four cases of 2.2; a unit test in
  `build.rs` that the key differs between two settings and that the flags of each case equal those noted in
  task 1.2. Verify: `cargo test -p qb64rust-driver` passes, with `-- --ignored` too (the tests that build an
  executable).
- [x] 2.4 Record two slice programs with `qb64pe.exe` and add them to `slice.list`. `s62_console_only_forms`, a
  program without a window: `PRINT` above `$CONSOLE:ONLY`, `$CONSOLE:ONLY` and `$CONSOLE` in one program, the
  metacommands as last lines and in a SUB (`_DEST` comes in group 3, so `$CONSOLE:ONLY` a second time after
  `_DEST 0` goes into `s64`). `s63_console_window`, the first program linked with the windowed libqb: `$CONSOLE`
  and `$SCREENHIDE`, a `PRINT` that goes to the hidden window, `SYSTEM`; its recorded output is empty and its
  exit code 0. Verify: tier 2 slice passes for both with and without folding; `git status --porcelain` in the
  reference clone is the same before and after the first build of the windowed libqb.
- [x] 2.5 Regenerate the lists and snapshots the reworded mark changes (`QB64RUST_UPDATE_LISTS=1 cargo test`, then
  review the diff: only the wording and the programs that lose a mark may change). Update `crates\README.md`
  (what the compiler supports; the build's flags), `tests\corpus\README.md` (no `PRINT` with a comma in a program
  with plain `$CONSOLE`). Verify: `cargo test` passes; the list diff holds no program that gained an error.
  Group 2 as built (2026-10-12; tasks 2.1–2.5):
  - 2.1: the two line rules are in `syntax` (`parser\meta.rs`) for every metacommand, in compiled code and in a
    branch that is not taken (two more measurements on the way: `v26_x57`, a colon in such a branch, which the
    new compiler rejected before; `x58`, a tab after a metacommand). `sema` matches the four metacommands
    exactly (`check\mod.rs`, `console_meta`) and records the mode. Front-end tests `console_meta_errors` (24
    rejected lines, 26 after the review below, none "not supported yet"), `console_meta_tab_after`, `console_meta_forms` (`check-ok`),
    `no_console_untaken`; `no_console` and `pp_if_errors` changed by the two rewordings.
  - 2.2: `Program::settings` in `sema` and in the IR, the `Settings:` line first in the dump; `validate` wants a
    console or a window, and a console where one is selected. Four `ir` tests (`settings_*_ir`); the 25
    existing `ir` snapshots differ by that line only. The runtime parts are collected from task 4.1 on: no
    built-in compiled yet has one.
  - 2.3: the two globals from the settings; `build::make_flags`; the flags in the build key (version 3). Four
    `cpp` tests (`settings_*_cpp`), two unit tests in `build.rs`; the 13 `--ignored` tests pass.
  - 2.4: `s62_console_only_forms` and `s63_console_window` recorded and on `slice.list` (289); both pass with
    and without folding; the clone's `git status` unchanged. `s63`'s executable imports `glu32.dll` as the old
    compiler's does and `s62`'s does not, so the windowed libqb is what it links. The whole slice list passes
    (289 of 289, `--jobs 8 --build-cache`, 228 s), which also covers the flag added to every console-only
    build.
  - 2.5: `QB64RUST_UPDATE_LISTS=1 cargo test` changed no list (no listed program carries the reworded mark by
    its text, and no `$CONSOLE` program lost its last mark: `_DEST` is still to come). `crates\README.md`,
    `tests\corpus\README.md` and the two counts of `CLAUDE.md`'s layout table updated.
  - Review of the group (2026-10-12), three things fixed: `$CONSOLE:ONLYX` was "not supported yet: metacommand
    `$CONSOLE`" and is now the error of a wrong `$CONSOLE` form (`v26_x60`); `$INCLUDEONCE` with a tab or a
    word after it was accepted and is now an error, in `syntax` (the file is not marked) and in `sema`
    (`v26_x59`, `x61`); the slice count of `tests\corpus\README.md` was one short (67 `.output` files). Both
    forms are lines of `console_meta_errors` and `console_meta_tab_after`. Left as it is: an unknown name such
    as `$CONSOLE_ONLY` or `$SCREENHIDE2` gets the mark every unknown metacommand gets ("Syntax error" in the
    old compiler). To watch from group 3 on: the runner's `--jobs` builds only the first program's libqb
    variant alone, so several programs with a window on a clone that lacks that variant would build it at the
    same time (CI runs one job).
- [x] 2.6 After the user pushes group 2: read the `tier2` job of `rust.yml` on GitHub. Verify: it is green, and
  `STATUS.md` records how long the job took before and after; or, if a program with a window does not start
  there, the fallback of design D9 is put to the user before group 3's windowed programs go onto a list.
  - Done 2026-10-12, read at `ab418be` (run 38298251873): `tier2` is green, and its log has `PASS
    corpus:slice/s63_console_window (output, 2.5s)` and `PASS corpus:slice/s62_console_only_forms (output,
    2.0s)`. So the GitHub Windows runner builds and starts a program with `$CONSOLE` and a window; the fallback
    of design D9 is not needed. The job took 18 min 22 s, against 17 min 25 s at `bcc8635`, the push before
    it; the slice step 9 min 55 s against 9 min 21 s (the libqb variant with a window is built once more, and
    two more programs run). Not seen there yet: group 3's programs, which run at the next push.

## 3. `_DEST`, `_SOURCE`, `_CONSOLE`, `_ECHO` (design D5)

- [x] 3.1 Rows for the statements `_DEST`, `_SOURCE`, `_ECHO` and the functions `_DEST`, `_SOURCE`, `_CONSOLE`,
  with the rules task 1.3 found; `_ECHO`'s compile error in a program with no console, checked after the walk.
  Call-site programs in `tests\callsite\`: one per form and per argument type class (`_DEST` with a literal, a
  LONG, an INTEGER, a SINGLE, a DOUBLE, an `_INTEGER64`, an expression; `_ECHO` with a literal, a variable, an
  expression). Tests: `typed` for the three functions, `ir` for the three statements, `check-fail` for `_DEST`
  with no argument, `_DEST "a"`, `_CONSOLE = 1` and `_ECHO` without a console. Verify: the call-site check
  reports every new program equal; `cargo test` passes, the built-in coverage test included once 3.2 is done.
- [x] 3.2 Record slice programs with `qb64pe.exe` and add them to `slice.list`: `s64_dest_console` (the scenarios
  of "_DEST and _SOURCE" and "_ECHO" in the delta: switching, the handles, a SUB and a FUNCTION, error 258, every
  numeric type as a handle, and in a second program of the same name with suffix `b` `$CONSOLE:ONLY` a second
  time after `_DEST 0`; no `PRINT` with a comma) and `s65_console_input` with a `.stdin` file (`INPUT` and
  `LINE INPUT` after `_DEST _CONSOLE`, with and without `_SOURCE _CONSOLE`). Verify: tier 2 slice passes for both
  with and without folding; their outputs equal `v26_b`, `v26_c`, `v26_d1`, `v26_g` where they repeat them.
- [x] 3.3 Run the two upstream programs that need nothing else (`data\test`, `keyboard\devices_windows`) with
  `--suite compile --qb64 target\release\qb64rust.exe` and add those that pass to `tests\upstream\pass.list`;
  regenerate the shrink-only lists. Update `crates\README.md` and `tests\upstream\README.md` (progress "x of
  279"). Verify: the pass list passes run by itself; the progress number in the README equals the list's length.
  Group 3 as built (2026-10-12; tasks 3.1–3.3; done before task 2.6, which waits on a push):
  - 3.1: six rows in `sema\builtins.rs` and nothing else for the calls: the checker, the IR and the emitter
    took them as they are. `_ECHO`'s check is a list of the `_ECHO` statements met (`check\mod.rs`, `echoes`),
    turned into errors after the walk when the program has no console, beside the one mark. Twelve more
    measurements on the way (`v26_x62`–`x73`, `study\00` §5): `_ECHO` above the line of `$CONSOLE` and under
    `$CONSOLE:ONLY`, the forms after `CALL`, parentheses, the functions in arithmetic, and seven rejections.
    One of them changed code: `CALL _DEST(h)` and `CALL _SOURCE(h)` are accepted by the old compiler (`x70`)
    and were "not supported yet"; a statement whose template is one argument is now read like a SUB call
    after `CALL` (`check\proc.rs`). `CALL _ECHO(t$)` worked already (its entry has no template). 28 call-site
    programs (`dest_*`, `source_*` nine each and one after `CALL`: a literal, a LONG, an INTEGER, a SINGLE, a
    DOUBLE, an `_INTEGER64`, a float literal, an expression, `_CONSOLE`; `echo_*` five; `func_dest_bare`,
    `func_source_bare`, `func_console_bare`): the check reports 284 of 284 equal. Front-end tests `dest_typed`,
    `dest_ir`, `dest_errors` (19 rejected lines, none "not supported yet", and the statement `_CONSOLE ON`/`OFF`,
    which stays marked: `proposal.md`, not in this change) and `echo_no_console`.
  - 3.2: `s64_dest_console`, `s64_dest_console_b` (the program without a window) and `s65_console_input` (with
    a `.stdin` file) recorded and on `slice.list` (292); the whole list passes with and without folding (292 of
    292, `--jobs 8 --build-cache target\build-cache`, 209 s). Their lines equal `v26_b`, `v26_c`, `v26_d1` and
    `v26_g` where they repeat them. `s64` also pins the conversion of the handle by value: a SINGLE -9.6, a
    DOUBLE -10.4, a `_FLOAT` -10.5 and the literal -9.5 are all the console (-10, rounded half to even), and so
    is the `_INTEGER64` 4294967286 (its low 32 bits).
  - 3.3: `data\test` passes on its first run and is on `pass.list` (115 of 279; the list run by itself: 115 of
    115). `keyboard\devices_windows` was on the list already. `QB64RUST_UPDATE_LISTS=1 cargo test` changed no
    list. `crates\README.md`, `tests\upstream\README.md`, `tests\corpus\README.md` and the counts of
    `CLAUDE.md`'s layout table updated.

## 4. Window built-ins, compiled and never run (design D6)

- [x] 4.1 Rows for the ten statements and ten functions of `proposal.md`, with the rules of task 1.3; the
  `WINDOW_BUILTINS` list; the template matching extended for parenthesised pairs if 1.3 showed it is needed; the
  icon runtime part collected for `_ICON` (D3). Call-site programs: one per form of each template and per
  argument type class. Tests: `typed` for each function, `ir` for each statement, a `check-fail` test for one
  wrong form of each special-format statement. Verify: the call-site check reports every new program equal;
  `cargo test` passes.
- [x] 4.2 The coverage rule (`testing/compiler-tests` delta): the built-in coverage test reads `WINDOW_BUILTINS`
  and requires, for each, a slice program that names it and a call-site program; its own negative test (a name
  added to the list with no call-site program fails and is named). Verify: `cargo test` passes, and fails with
  the built-in's name when one call-site program is moved away (tried once by hand, then restored).
- [x] 4.3 Record slice program `s66_window_builtins` with `qb64pe.exe` (the shape of the upstream `glut` tests:
  `$CONSOLE:ONLY`, a `PRINT`, `SYSTEM`, then `$CONSOLE`, `_DEST _CONSOLE` and every window built-in in every form)
  and add it to `slice.list`. Run the 19 upstream `glut` tests and add those that pass to
  `tests\upstream\pass.list`; regenerate the shrink-only lists; update the two READMEs. Verify: tier 2 slice
  passes for `s66` with and without folding; the pass list passes run by itself; each `glut` test that does not
  pass is named here with its reason.
  Group 4 as built (2026-10-12; tasks 4.1–4.3):
  - 4.1: twenty rows in `sema\builtins.rs` (ten statements, ten functions; `_SCREENICON` is both) and the list
    `WINDOW_BUILTINS` (19 names as written: `_SCREENICON` once, `_TITLE` and `_TITLE$`). The checker, the IR
    and the emitter took every form as it is: the template matching needed no change for the parenthesised
    pairs. `table_ty` learned `_UNSIGNED _OFFSET`; `_SCREENEXISTS` is `Rule::Fixed(LONG)` and is marked as the
    direct argument of `HEX$`, `OCT$` and `_BIN$`. **The runtime parts are collected by `sema`, not by
    `lower`** (design D3 said `lower`): `sema` meets every built-in call as it checks it, `lower` would need a
    walk of its own; `sema::RuntimePart` is set from the table's new `dependency` column
    (`qb64rust_builtins::Builtin::dependency`, 68 entries have one) and `lower` copies it. A built-in whose
    part no build provides is "not supported yet" there. 78 call-site programs (`screenhide_stmt`,
    `screenshow_stmt`, `screenicon_stmt`, `title_*` 5, `icon_*` 10, `mouseshow_*` 5, `mousehide_*` 3,
    `mousecursor_*` 9, `windowsizelimit_*` 10, `freeimage_*` 9, the seven bare functions, `func_newimage_*` 8,
    `func_windowhandle_*` 8): the check reports 362 of 362 equal. 40 more measurements (`v26_x74`–`x113`,
    `study\00` §5): one program of accepted forms, 39 rejected forms. They changed code in two places. A
    function without argument slots called with an argument is "Incorrect number of arguments" in the old
    compiler (`_SCREENEXISTS(1)`, and `ERR(1)`, `ERL(1)`, which were "not supported yet": `x75`–`x77`); it is
    that arity error now. And the old compiler accepts every one of these statements after `CALL` with its
    template's forms between the parentheses (`x74`: `CALL _MOUSECURSOR(1, (2, 3))`), where the new compiler
    reported a syntax error for a form that is no expression list: the parser now leaves the parentheses
    unread after `CALL` of a statement with a template (but one of a single argument), and `sema` marks the
    statement "not supported yet" (`syntax\parser\call.rs`; the same for `CALL LINE(…)`, `CALL OPEN(…)`).
    Front-end tests `window_typed`, `window_ir` (its Settings line names the icon part), `window_errors` (50
    lines: 39 errors that are not marks, 11 marked forms the old compiler accepts).
  - 4.2: `every_supported_builtin_is_covered` reads `WINDOW_BUILTINS`: each name must be a supported row and
    be used in a program of `tests\callsite` (`without_callsite`, with its own test
    `window_builtin_without_callsite_is_named`, which also tells `_TITLE` from `_TITLE$`). Tried by hand: with
    `func_screeny_bare.bas` moved away `cargo test` fails with "`_SCREENY` (a window built-in) is used in no
    program of tests/callsite"; restored.
  - 4.3: `s66_window_builtins` recorded and on `slice.list`; it passes with and without folding, and the
    clone's `git status` is the same before and after (its `_ICON` links the libqb variant with the icon
    part and no window, which the clone had already). Seven of the 19 `glut` tests pass and are on
    `pass.list` (122): `icon`, `mousehide`, `mouseshow`, `screenhide`, `screenshow`, `title`, `title_func`.
    The other twelve wait on `_TRUE`/`_FALSE`, nothing else (`desktopheight`, `desktopwidth`, `mousecursor`,
    `screenexists`, `screenhide_commands`, `screenhide_sub`, `screenicon`, `screenx`, `screeny`,
    `windowhandle`, `windowhasfocus`, `windowsizelimit`); they pass with group 5. The shrink-only lists are
    unchanged.

## 5. Constants of the auto-included file (design D8)

- [x] 5.1 `tools\builtins\extract_builtins.py`: per constant of `beforefirstline.bi`, its name with suffix and the
  text of its expression, from the Windows 64-bit branches; `builtins.json` regenerated; the generated table of
  `crates\builtins` carries the field. Tests: the extractor's output is unchanged for everything else (diff of
  `builtins.json` reviewed: only `auto_include` entries gain fields); a unit test in `crates\builtins` pins
  `_TRUE`, `_FALSE`, `_SIZE_OF_OFFSET` and one constant that names another. Verify: `cargo test -p
  qb64rust-builtins` passes.
- [x] 5.2 `sema`: a constant of the file resolves on first use through the `CONST` evaluator, in file order, cached,
  global; one the evaluator cannot compute keeps its mark with the reason; the `$COLOR` names and the three
  functions keep theirs. Tests: `typed` for `_TRUE`, a constant with a suffix and one computed from others, used
  in the main module and in a SUB; `check-fail` for `_STR_CRLF` (marked) and for `CONST _TRUE = 1` (still the
  reserved-name error). Verify: `cargo test` passes; a program that names no such constant has an unchanged
  `typed` snapshot.
- [x] 5.3 Record slice program `s67_auto_constants` with `qb64pe.exe`: it prints every constant 5.2 computes, and
  `LEN` of one per type suffix; add it to `slice.list`. Count again which corpus and upstream programs lost their
  last mark (`qb64rust --report unsupported`), run them and add those that pass to their pass lists; regenerate
  the shrink-only lists; update `crates\README.md`. Verify: tier 2 slice passes for `s67` with and without
  folding; a constant whose value differed is fixed or left marked and named here.
  Group 5 as built (2026-10-12; tasks 5.1–5.3):
  - 5.1: the extractor follows the `$IF` blocks of the auto-included files (`WIN` true, `32BIT` false; a
    `CONST` under a condition it does not know stops it) and records `suffix` and `value` for every constant;
    the diff of `builtins.json` holds nothing else (checked by a script: every other key equal, the 597
    constants each gain the two fields, no other entry changes). `crates\builtins`: `AUTO_CONSTANTS` (the 310
    of `beforefirstline.bi`, in file order) and `auto_constant`; its test pins `_TRUE`, `_FALSE`,
    `_SIZE_OF_OFFSET`, `_OFFSET_MAX`, `_STR_NAT_EOL`, `_E` and `_ONE_MB` (which names `_ONE_KB`, standing
    before it).
  - 5.2: `sema\check\auto.rs`. A constant becomes a constant of the main module at its first use (in an
    expression, in a `CONST`, as the length of a `STRING * n`), the ones its expression names first; from then
    on it is a constant like any other (a use with another suffix, in a procedure). **The text is read by a
    small reader in `sema`, not by the parser** (design D8 said "parsing and evaluating the recorded text with
    the `CONST` evaluator"): the checker reads source text through the source map by a span's file, and a text
    that comes from the table has none. The file's expressions have three shapes (one operand; two operands
    with `+`, `*` or `/`; a function call), which the reader takes apart; every value is computed by the
    evaluator's own functions (`consteval`: the literal, the operator, `reread`, `settle` by the suffix), as
    `const_stmt` computes a `CONST` line. A text of another shape is marked, so a new version of the file
    cannot be misread. 231 of the 310 are computed. Left marked, with the reason: the 69 that need `CHR$`,
    `_E` (`EXP`), and nine numbers the evaluator does not read yet: `_DOUBLE_MIN`, `_DOUBLE_MAX` ("this close
    to a rounding boundary"), `_FLOAT_MIN`, `_FLOAT_MAX`, `_FLOAT_MIN_FRAC` (beyond `DOUBLE` range),
    `_INTEGER64_MIN`, `_UINTEGER64_MAX`, `_OFFSET_MIN`, `_UOFFSET_MAX` (beyond `_INTEGER64`). The old
    compiler's values for those nine are recorded in `v26_x114`; five of them are not the limits their names
    say (`SOMEDAY.md`, "QB64pe behaviours to review"), which is one more reason not to guess them. 13 more
    measurements (`v26_x114`–`x126`, `study\00` §5). They decided the reserved-name rule: `CONST _TRUE = 1`
    and `CONST _ONE_MB = 1048576` are "Invalid name", `DIM _TRUE` "Invalid variable name", so a name of the
    file is reserved like any name with a leading underscore (`check\decl.rs`, `reserved`: the mark of an
    auto-included name is kept for the names that are no constant of this file, the three functions of
    `afterlastline.bm`). Front-end tests `auto_const_typed` and `auto_const_errors` (24 lines); a `typed`
    snapshot of a program that names no such constant is unchanged (none of the existing snapshots changed);
    `mem_follow_on` used `_TRUE` as its example of a marked name and has `_CHR_CR` now.
  - 5.3: `s67_auto_constants` recorded and on `slice.list`: the 231 values by name (generated from
    `builtins.json`), their types, and the places a constant may stand. `LEN` of a numeric constant is an
    error of the old compiler (`v26_x126`), so the types are shown by arithmetic that leaves the type's range
    (`_BYTE_MAX + 1` is 128, `_LONG_MAX + 1` -2147483648, `_SINGLE_MAX * 10` `INF`) and by uses with a suffix.
    It passed on its first run, with and without folding: no value differed. Lost their last mark: the other
    twelve `glut` tests and `logging/consts`, all passing and on `pass.list` (135); no corpus program.

## 6. Precompiler flags (design D7)

- [x] 6.1 `syntax`: `PpFlags` as an input of `parse`; the settings met reported in file order; the running value of
  `_CONSOLE_` and the marked corner; `parse_settled` with its cap; a round repeated only when a condition named a
  flag. The driver, the language server and the tests call `parse_settled`. Tests: unit tests of the state
  machine (set once; set to 2 then 1; a locked flag; the cap); front-end tests in the modes they need for every
  scenario of the "Precompiler flags" requirement (`v26_h_flag_console`, `v26_h2_flag_both`, `x26`, `x27`, `x28`,
  `x33`, `x38`, `x39`, and what 1.4 added). Verify: `cargo test` passes; the mutation test and the parse-gap list are
  unchanged; tier 1's time is within 10% of before (noted here).
- [x] 6.2 `sema`: the "precompiler flag in `$IF`" mark is gone except for the corner; the program's console
  comes from the settled `_CONSOLE_` flag as well as from the walk (D1; a front-end `ir` test with the program
  of `v26_x26` shows a console in its `settings` line and no mark); `OPTION _EXPLICIT` and `OPTION
  _EXPLICITARRAY` are in force by their settled flags as well as by their statements (D7; a `check-fail` test
  with the program of `v26_x55`, a `check-ok` test with `v26_x54`); `$ASSERTS` and `$DEBUG` stay
  marked as metacommands. Record slice program `s68_precompiler_flags` with `qb64pe.exe` (the scenarios above
  that run) and add it to `slice.list`. Run the seven upstream `precomp-flags` tests and add those that pass to
  `tests\upstream\pass.list`; regenerate the shrink-only lists; update the READMEs and the comment at the top of
  `syntax\pp.rs`. Verify: tier 2 slice passes for `s68` with and without folding; the pass list passes run by
  itself.
  Group 6 as built (2026-10-12; tasks 6.1, 6.2):
  - 6.1: `syntax\pp.rs`. The six flags are names of the preprocessor like the predefined ones, set from the
    round's values at the first line (`PpState::with_flags`), so a condition that names one is evaluated by
    the ported evaluator like any other. The old compiler's state of a feature is `FeatureStates` (wanted,
    actual, locked; five features: the two options, asserts, console, debug); `settle` runs the rounds over a
    function that parses one, and is tested without a parser. The parser reports the settings it meets in
    compiled code (`PpState::met`: the five metacommand lines as the old compiler's first pass compares them,
    and the two options where their statement ends), and three flags take their value at their metacommand's
    line within a round. A `$IF` or `$ELSEIF` that names a flag with another value there than at the first
    line is noted (`ParsedProgram::passes_disagree`); the branch is chosen by the value on the line, as the
    first pass chooses it, which is what makes the settings of a round that is not the last the old
    compiler's (`v26_x39`, `x56`). `parse_settled` replaces `parse` as the entry point (the driver, the
    language server, the tests); the settled flags are `ParsedProgram::flags`. A round is parsed again only
    when a condition named a flag; otherwise the settings are applied until they settle, without a parse.
    The cap is `MAX_ROUNDS`, 7: five features, each round but the last locks one, so six parses at most; past
    it the last parse gets an error, no panic. Unit tests (`pp.rs`): set once, set to 2 then 1 and to 1 then
    2, a locked feature, three rounds, the cap (the three-round program with two parses allowed is not
    settled), the longest chain (six parses), the running values. Front-end tests, one per scenario of the
    requirement: `flags_console_ir` (above the line), `flags_console_only_ir` (2; `OPTION _EXPLICIT`; nothing
    sets the others), `flags_only_then_console_ir`, `flags_explicitarray_ir`, `flags_explicit_between_ir`,
    `flags_between_errors` (the corner: each marked line, and no mark below the second line or on another
    flag), `flags_nothing_set_errors`, `flags_three_rounds_ir`, `flags_running_value_ir`,
    `flags_asserts_errors` (`$ASSERTS:CONSOLE`, `$DEBUG`: only the two metacommand marks), `flags_include_ir`
    (the setting and the `$IF` in an included file; measured for it: `v26_x127`). All 16 flag programs of
    `verification\` give what the old compiler recorded for them. `pp_if_errors` loses its two flag marks.
    The mutation test and the parse-gap list are unchanged. Tier 1: `cargo test` takes about 28 s, against 23
    to 26 s at the commit before (measured in a worktree), with 156 more programs and 7 more tests in the
    tree; the front end over the same inputs (corpus, upstream, the real program, the front-end tests,
    `verification\` and the old compiler's sources, release build) takes 2.3 s with the binary of before and
    with this one. On the way a lookup added by group 5 (the auto-included constants, asked for every name)
    was made to look only at names with a leading underscore.
  - 6.2: `sema` marks only the noted lines ("`_CONSOLE_` in a `$IF` between two lines that set it
    differently"); the program's console comes from the settled flag as well as from the walk
    (`flags_guarded_console_ir`: console, no mark; `flags_guarded_only_ir`: `_CONSOLE_` 2, console and
    window, no "select the console"); the two options are in force by their flags as well as by their
    statements (`flags_guarded_explicit`, `flags_guarded_explicit_errors`: the variable and the array that
    are not declared are errors). `$ASSERTS` and `$DEBUG` stay marked as metacommands. Slice programs
    `s68_precompiler_flags` (three rounds; the guarded `OPTION _EXPLICIT`; flags above their lines, in an
    `$ELSEIF`, beside a predefined name, in a SUB), `s68_precompiler_flags_b` (`$CONSOLE:ONLY` then
    `$CONSOLE`; a setting that is met in no round; `OPTION _EXPLICITARRAY` alone) and
    `s68_precompiler_flags_c` (the guarded `$CONSOLE:ONLY`: flag 2, a console and a hidden window), recorded
    and on `slice.list` (297); all three pass with and without folding. Of the nine upstream `precomp-flags`
    tests (the task said seven) four pass and are on `pass.list`: `console`, `consoleonly`, `explicit`,
    `explicitarr`; so does `auto_include/no-debug` (140). The other five are rejected with only the mark of
    a feature outside this change: `asserts` and `assertscon` (`$ASSERTS`), `debug` (`$DEBUG`), `socket`
    (`_OPENCLIENT`), `all` (all three). The shrink-only lists are unchanged.
  - After the three groups (2026-10-12, release build, `--jobs 8 --build-cache target\build-cache`):
    `cargo test` 290 pass, the 13 `--ignored` tests pass, fmt and clippy clean; the call-site check 362 of
    362; the slice list 297 of 297 with and without folding; the upstream pass list 140 of 140; the
    differential pass list 59 of 59; the runner's own tests 56 pass. Not run yet: the two full suites
    (`--suite compile`, `--suite corpus`) with the new compiler, which are task 7.1.

## 7. Integration

- [ ] 7.1 Full runs with the release build: `cargo test`; the call-site check; tier 2 slice with and without
  folding; the upstream pass list; the differential pass list; `--suite compile` and `--suite corpus` in full
  with the new compiler, to see that no program outside the lists changed its result. Verify: every run passes,
  and the numbers are written into `STATUS.md` "Numbers at the last full runs".
- [ ] 7.2 Documents: `STATUS.md` (step 11 done, what is next), `ROADMAP.md` (M2's last box ticked, and whether
  M2's exit criterion is met), `crates\README.md`, `study\00` §5 and its document map, `CLAUDE.md` (the
  `verification\` row, the corpus and slice counts of the layout table), `SOMEDAY.md` (what stayed marked:
  the constants that need `CHR$` or `EXP`, the corner of D7, window built-ins not run), `DECISIONS.md` (anything
  decided on the way). Verify: every count in the layout table equals the files on disk; `repo_check` is clean.
- [ ] 7.3 After the user pushes: read the three workflows on GitHub. Verify: all green; then the change is ready
  to archive (`/opsx:archive`), which syncs the deltas into `openspec\specs`.
