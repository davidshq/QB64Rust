# Tasks

Standing rules: CLAUDE.md. What this change does differently: the keys file's `hold` and `lift` come first
(group 1), because the screen programs recorded in task 2.1 need them; every recording with `qb64pe.exe` is
batched in task 2.1, before the code.

## 1. hold and lift in the keys file

- [x] 1.1 `tools\screen\screen_state.cpp`: `Step::Hold` and `Step::Lift`, the modifier names, and the wait on the
  `INKEY$` buffer for a key that writes to it (design D4). Two rows in the keys-file table of
  `tools\screen\README.md`. Verify: a scratch screen program built with `qb64pe.exe` loops on `_KEYDOWN(100304)`
  and prints `-1  0 ` with `prompt`, `hold lshift`, `wait 200`, `lift lshift`; `hold bogus` exits 125 naming the
  line; the screen programs of `slice.list` still pass with `--pass-cache` (`--glob "screen/*"`).

## 2. Measurements and records

- [x] 2.1 The family `verification\v53_*` and the screen programs of design D6, recorded with `qb64pe.exe` (the
  verification suite and the corpus runner), with one line in `verification\README.md` and the programs' rows in
  `tests\corpus\README.md`. Two recordings of the resize screen program decide D2's fallback. Where a recording
  differs from a delta spec or the design (the `$RESIZE` forms, `_RESIZE`'s values, the `_CONSOLE OFF` output,
  `_EXIT`, the drop built-ins with nothing dropped, the wrong forms), the spec or design is changed here. Verify:
  every program has its recorded output, error or state; `tools\repo_check\check_repo.py --untracked` finds no
  local path in them (design D5); `openspec validate m4-qb64pe-builtins --strict` passes after any edit.
- [x] 2.2 `DECISIONS.md`: a row for the change. It records the scope (the 26 built-ins and `$RESIZE` that
  `qb64pe.bas` needs, user 2026-10-22), what is never run (design D2), and the keys file's `hold` and `lift`.
  Verify: the row names `openspec\changes\m4-qb64pe-builtins\design.md`.

## 3. Keyboard

- [x] 3.1 `sema`: rows of `_KEYHIT`, `_KEYDOWN` and `_KEYCLEAR` (design D1). Verify:
  `tests\frontend\qb64pe_builtins_typed.bas` (`typed`: each form) and `qb64pe_builtins_errors.bas` (`check-fail`:
  the `v53_x` forms of these three); `cargo test -p qb64rust-sema`.
- [x] 3.2 Call-site programs for each form of 3.1; the three `k…` screen programs on `slice.list`. Verify: the
  coverage test of tier 1; the call-site check of the new programs; the screen programs pass.

## 4. Window and console

- [x] 4.1 `sema`, `ir`, `codegen-cpp`: `$RESIZE` read into `ProgramSettings`, carried to `ir::Settings` and
  written to `dyninfo.txt` (design D3). Verify: `tests\frontend\resize_ir.bas` (`ir`: the settings of each form,
  the last one counting) and `resize_cpp.bas` (`cpp`: the lines of `dyninfo.txt`); the `v53_x` `$RESIZE` forms
  pass; `cargo test -p qb64rust-sema -p qb64rust-ir -p qb64rust-codegen-cpp`.
- [x] 4.2 `sema`: rows of `_RESIZE` (statement and function), `_RESIZEWIDTH`, `_RESIZEHEIGHT`, the statement
  `_CONSOLE`, `_EXIT`, and the drop built-ins (design D1, D2). Verify: `qb64pe_builtins_typed.bas` and
  `_errors.bas` gain them; `v53_a_window` passes; the resize screen program passes (or the functions are on
  `NEVER_RUN`, by task 2.1).
- [x] 4.3 `sema`: rows of `_SCREENMOVE`, `_MESSAGEBOX` (statement and function), `_OPENFILEDIALOG$`,
  `_SAVEFILEDIALOG$` and `_SELECTFOLDERDIALOG$`, each name on `NEVER_RUN` (design D2). Verify: the `typed` and
  `check-fail` tests gain them; a call-site program for each form (`tests\callsite\`); the coverage test of tier 1.
- [x] 4.4 Call-site programs for each form of 4.1 and 4.2; a `$CONSOLE` slice program for `_CONSOLE ON|OFF`,
  `_EXIT` and the drop built-ins, recorded with `qb64pe.exe`, on `slice.list`. Verify: the call-site check of the
  new programs; the slice program passes.
  Done: 23 call-site programs of 4.2, all equal. `$RESIZE` (4.1) has none: it writes no call, only two lines of
  `dyninfo.txt`, held by `tests\frontend\resize_cpp.bas`. `s155_console_on_off` also holds `_SCREENMOVE` and the
  dialogs of 4.3 as code not reached (recorded again with `qb64pe.exe` after that edit); `c15_resize` is on
  `slice.list` too. The function template rule of an argument left out now takes a leading `[?][,` slot
  (`check\builtins.rs`, `may_be_left_out`; `v53_x31`).

## 5. Screen colours and control characters

- [x] 5.1 `sema`: rows of `_CONTROLCHR` (statement and function), `_DEFAULTCOLOR` and `_BACKGROUNDCOLOR` (design
  D1). Verify: the `typed` and `check-fail` tests gain them; `v53_b_colours` passes; the two `c…` screen programs
  pass and go on `slice.list`; a call-site program for each form.

## 6. The system

- [x] 6.1 `sema`: rows of `_DIR$`, `_SHELLHIDE`, `_COMMANDCOUNT`, `_COMPILEDATE$`, `_COMPILETIME$` and `_READBIT`
  (design D1, D5). Verify: the `typed` and `check-fail` tests gain them; `v53_c_system` passes; a slice program
  (no path printed) recorded with `qb64pe.exe`, on `slice.list`; a call-site program for each form.

## 7. Programs and lists

- [x] 7.1 Onto `tests\verification\pass.list` the `v53` programs that pass (any that cannot goes on `known.list`
  with its reason). The scenario of an unsupported built-in in `language/builtin-functions` is now `_SETBIT`.
  Check whether a front-end test or a slice program used `_KEYHIT` or another name of this change as its
  unsupported example, and give it a name that stays unsupported. Verify: each program passes with this compiler
  (`--list` of the programs moved); each verification program is named once across the two lists; tier 1 passes.
  Done: all 80 `v53` programs pass and are on `pass.list`, moved group by group with tasks 3 to 6. `v53_c_system`
  wrote `_UNSIGNED _INTEGER64`'s largest value as a decimal literal, which this compiler does not take yet
  ("unsigned literals"); it now writes `&HFFFFFFFFFFFFFFFF~&&`, recorded again (the same output).
  `tests\frontend\unsupported.bas` used `_KEYDOWN` and `_KEYHIT` as its unsupported examples: now `_SETBIT` and
  `_TOGGLEBIT`; `dest_errors.bas` lost its two `_CONSOLE ON|OFF` lines, which compile now. Two snippets of
  `tests\snippets` that use `_SHELLHIDE` as a statement left `known_unsupported_rejections.list` (a real error now).
- [x] 7.2 `qb64rust -z ..\QB64pe\source\qb64pe.bas` and the demand report on it (design D7), each sent to a log.
  Verify: no "not supported yet"; any other diagnostic is written below this task and in `STATUS.md`'s "Next",
  and fixed here only if it is a slip of this change; one try at `-x`, its result written below this task.
  Done (2026-10-22): `-z` reports no diagnostic at all; the demand report: "1 program: 1 without errors". `-x` built
  an executable (`-o target\qb64pe-build\qb64pe.exe`, 11.7 MB, 12 s with libqb's objects built before); it was
  not run. A bare `-z` writes its output beside the source, inside the clone, so `-o` names a folder of
  `target\`.

## 8. Close

- [x] 8.1 The closing runs: `tools\close\close.py --check` passes. It includes the coverage check of Civil War
  Strategy's scripts, because `screen_state.cpp` changed. Verify: the upstream and corpus demand reports name none
  of the change's built-ins.
- [x] 8.2 Documents. `STATUS.md`'s "Next" (item 2 done or what task 7.2 left). `SOMEDAY.md`: a row for each form
  task 2.1 left marked, `_SETBIT`, `_RESETBIT` and `_TOGGLEBIT` under deferred features, and the questionable
  behaviours kept QB64pe's way under "QB64pe behaviours to review". Verify: `openspec validate
  m4-qb64pe-builtins --strict` passes.
- [x] 8.3 `python tools\testtime\test_time.py`. Verify: its five files are named among those to stage.

## 9. Review follow-up (2026-10-22)

- [x] 9.1 The empty first slot of the dialogs: the rule of `may_be_left_out` also lets `_OPENFILEDIALOG$`,
  `_SAVEFILEDIALOG$` and `_SELECTFOLDERDIALOG$` leave their first slot empty, which only `_MESSAGEBOX` had shown.
  Verify: a call-site program for each (`func_*dialog_skip`). Done: all three equal (the old compiler takes them,
  with the same `passed` mask); the rule stays, its comment names them.
- [x] 9.2 `_COMPILEDATE$` and `_COMPILETIME$` from the build cache: their value is the time of the C++ build, in no
  fragment, so a cached executable gave an earlier one. The checker marks a program that calls either
  (`compile_stamp`, "compile time" in the IR dump's settings), and the driver neither reads nor fills the cache for
  it. Verify: `qb64pe_builtins_ir` calls `_COMPILEDATE$`; tier 1.
- [x] 9.3 `$RESIZE`'s two fields (`resize`, `resize_scale`) are one enum `Resize` (`Off`, `On`, `Stretch`, `Smooth`)
  in `sema` and the IR. Verify: no snapshot changes but 9.2's.
- [x] 9.4 `hold` and `lift` of a keys file (`tools\screen`): a `hold` of a key already held, a `lift` of one not
  held and the `end` of a `repeat` block that leaves other keys held than it found are not steps (exit code 125).
  Verify: `screen_state_mistake_in_a_held_key` (CLI, `--ignored`); the two keys files that hold keys (`k02`, `k03`)
  pass.
- [x] 9.5 `$RESIZE` with a tab before the colon and with `=` for it, recorded with `qb64pe.exe`: `v53_x82` "Syntax
  error", `v53_x83` "Invalid expression"; the second has a message of its own, so the wording table keeps a pair
  for each. On `pass.list`.
- [x] 9.6 `v53_c_system` was suspected of depending on the machine's folders: it does not on Windows (libqb asks
  for each folder with `CSIDL_FLAG_CREATE` and falls back to the profile), written in `verification\README.md`;
  nothing re-recorded.
- [x] 9.7 The closing runs again: `tools\close\close.py --check` passes (slice 485, upstream 401, verification
  3041, call-site 2151, coverage, CLI 47); `python tools\testtime\test_time.py`.

## Workflow follow-up

- Archive the change once the user has reviewed it (`/opsx:archive`).
- Tell the user which files to stage and suggest a commit message (nothing is staged by Claude).
