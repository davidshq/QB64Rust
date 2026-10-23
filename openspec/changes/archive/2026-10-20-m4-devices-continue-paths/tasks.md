# Tasks

Standing rules: CLAUDE.md. Differently here: four parts in one change (user, 2026-10-20), each built and
tier-1-tested in its own group; the `v47` recording with `qb64pe.exe` is batched in task 1.1, before the code.

## 1. Measurements and records

- [x] 1.1 The family `verification\v47_*` (design D5): `v47_a_continue`, `v47_b_devices`, `v47_c_dot_blanks`, the
  `v47_e*` relative names (each file put in the clone's root for the recording and removed after) and the
  `v47_x*` rejected forms, recorded with `--suite verification`; one line in `verification\README.md`. Where a
  measurement differs from a delta spec or the design (the `_CONTINUE` error, `_CONTINUE 1`, `ON STRIG`'s operand
  errors, what `v47_b` prints, declarations with blanks around a dot), the spec or design is changed to match in
  this task. Verify: every program has its recorded output or error; `v47_b` prints nothing that counts
  controllers; `python tools\repo_check\check_repo.py --untracked` is clean (no local path in the `v47_e`
  recordings).
- [x] 1.2 `DECISIONS.md`: a row for the change (the four parts as one, user 2026-10-20), and a row replacing
  answer (2) of 2026-10-13: relative `$EXEICON` and `$EMBED` names resolve against the compiler root (design D3).
  Verify: both rows name `openspec\changes\m4-devices-continue-paths\design.md`.
- [x] 1.3 The device-input build first (design "Risks"): `RuntimePart::DeviceInput` in `sema` and `ir`,
  `DEP_DEVICEINPUT=y` in `make_flags`, and the `_DEVICES` row, enough to build `console_only/devices`. Verify: it
  builds and prints `-1`; its make flags equal the old compiler's for the same program (`-z`, `makedeps$`); the
  licence file names `libstem_gamepad`; a unit test of `make_flags` pins the order.

## 2. _CONTINUE

- [x] 2.1 `syntax`: `ContinueStmt`, `Parser::inside_loop`, the error outside a loop (design D1); snapshots
  updated. Verify: parser tests of `_CONTINUE` in each loop kind, through `IF` and `SELECT`, outside a loop, in a
  SUB with no loop, `_continue` in lower case, `_CONTINUE 1` (a call, as the old compiler reads it);
  `cargo test -p qb64rust-syntax`.
- [x] 2.2 `sema` and `ir`: `StmtKind::Continue`, the continue label in `exits`, placed in `loop_block` and
  `for_loop` (design D1). Verify: `tests\frontend\continue_errors.bas` (`check-fail`) and `continue_ir.bas` (`ir`:
  the label before the step, the `LOOP` test, the `WEND` jump) pass; `cargo test -p qb64rust-ir`.

## 3. Blanks around a dot

- [x] 3.1 `sema`: `spelled_name`, and `field_member`/`field_place` reading a member access on a plain name as the
  dotted name, member or plain variable (design D2). Verify: `tests\frontend\dot_blanks_typed.bas` (`typed`: `a . s`
  read, assigned, an element `a . s(2)`, `ERASE a . S`, `LBOUND(a . s)`, a by-reference argument, the plain dotted
  name `a . b`, `a . b . c`) gives the tree of the same lines without blanks; the "blanks around `.`" mark is gone
  from `sema`; `cargo test -p qb64rust-sema`.

## 4. Relative names of $EXEICON and $EMBED

- [x] 4.1 `sema`: the two marks narrowed to a drive with no slash after it (design D3). `driver`: `icon_path`,
  `embedded_path` and their callers take the compiler root, `check_program` and `check_for_server` included.
  Verify: driver tests of a name under a root given by `--include-root`, a name with one leading slash, a missing
  file under the root ("not found", named as written), `C:icon.ico` ("not supported yet"); the language server
  reports the missing file too; `cargo test -p qb64rust-driver`.
- [x] 4.2 `tools\upstream\copy_upstream_tests.py`: the three files named relative to the compiler's folder
  mirrored under `tests\upstream\root\`, from a list in the script, with a test that an upstream program names
  each one; `tests\upstream\SOURCE.md` says so. Verify: `copy_upstream_tests.py --check` (or the script's own
  check) passes; the script's tests pass; the files equal the clone's byte for byte.

## 5. Device input

- [x] 5.1 `sema`: the other 12 rows (`Rule::Plain`, `StmtRule::Plain`) with their comment (design D4). Verify:
  `tests\frontend\devices_errors.bas` (`check-fail`: too many arguments, a string argument) and
  `devices_typed.bas` (`typed`: each function with and without its optional argument, the `STRIG` switch forms)
  pass, and the tier-1 coverage check of the built-in table counts them.
- [x] 5.2 `ON STRIG`: `Event::Strig` in `sema` and `ir`, the operands and errors in `check\events.rs`, the setup,
  `case` and jumper in `codegen-cpp\src\events.rs` with `onstrig.txt`/`onstrigj.txt` (design D4). Verify:
  `tests\frontend\strig_errors.bas` (`check-fail`) and `strig_cpp.bas` (`cpp`: a label handler, a SUB handler
  with and without an argument, a controller given and left out) pass, and its C++ builds (a CLI test with
  `--ignored`); the events spec's "STRIG left marked" test is replaced.
- [x] 5.3 Call-site programs for every form and argument type of the 13 rows, and `event_strig_*` (design D5).
  Verify: `python tools\callsite\callsite.py --glob` over the new names reports every one equal; a rule added to
  `callsite.py` comes with its `README.md` line and test.

## 6. Programs and lists

- [x] 6.1 Slice programs `s147_continue`, `s148_dot_blanks`, `s149_devices_console`, recorded with `qb64pe.exe`
  (`--record`) and added to `tests\corpus\slice.list`. Verify: all three pass with this compiler (`--glob`).
- [x] 6.2 The 8 upstream programs (`colors/color_with_dest`, `console_only/devices`, `deviceinput/test`,
  `arrays/04_float_family`, `arrays/08_whole_member_array_memops_numeric`, `icon/test`, `winresource/test`,
  `embed/test`; `qb64pe/element_getStringValue` keeps its other errors) onto `tests\upstream\pass.list`. Then the
  `v19_dot_blanks_*`, `v29_e02`, `e03`, `e08`, `e18`, `v44_x26` and `v47` programs from `known.list` onto
  `tests\verification\pass.list`. Verify: each passes with this compiler (`--list` of the programs moved); each
  verification program is named once across the two lists (tier 1).

## 7. Close

- [x] 7.1 The closing runs: `tools\close\close.py --check` passes (it includes the demand reports, the shrink-only
  lists, the wording table and `counts.py`). Verify: the upstream demand report no longer names `_DEVICES`, the
  `$EXEICON` name, blanks around `.` or `_CONTINUE`.
- [x] 7.2 Documents: `STATUS.md`'s "Next"; `SOMEDAY.md` (the `STRIG` row of "Left marked by `m3-events`" and the
  relative-name rows of `m3-sound` and `m3-network-embed` closed or narrowed to `C:name`; a row for declarations
  with blanks around a dot if 1.1 left them marked; the questionable behaviours kept, under "QB64pe behaviours to
  review"). Verify: `openspec validate m4-devices-continue-paths --strict` passes.
- [x] 7.3 `python tools\testtime\test_time.py`. Verify: its five files are named among those to stage.

## Workflow follow-up

- Archive the change once the user has reviewed it (`/opsx:archive`).
- Tell the user which files to stage and suggest a commit message (nothing is staged by Claude).
