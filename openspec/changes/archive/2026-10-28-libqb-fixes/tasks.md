# Tasks

Standing rules: CLAUDE.md. What this change does differently: the always-on fixes are edits of libqb's copy
(`tools\libqb\edits\`, then `copy_libqb.py`), never of the emitter, and one batch of recordings with `qb64pe.exe`
(task 1.2, family `v63`) comes before any edit.

## 1. Records and measurements

- [x] 1.1 `DECISIONS.md`: a row for the fix name `bit-number-check` and for `END` printing its line without
  waiting when the input is not a console (design D2), naming `openspec\changes\libqb-fixes\design.md`. Verify:
  the rows are there.
- [x] 1.2 Family `v63` in `verification\` ("the libqb fixes"), the programs of design D8, with `.stdin` sidecars
  where a program reads input (one empty, one with a last line without a newline). Recorded in one batch with
  `qb64pe.exe`. The family's line in `verification\README.md` gives what each shows, among them whether `INPUT$`
  reads redirected input (design, Open Questions), and each program goes into `tests\verification\known.list` or
  `pass.list`. Verify: `tools\legacy_tests\run_legacy_tests.py --suite verification --qb64 <qb64pe> --glob
  "v63_*"` records every program, and tier 1's `verification_lists_are_exact` passes.

## 2. The libqb edits

Each task: its edit files under `tools\libqb\edits\` (design D1), `libqb\SOURCE.md`'s list, `copy_libqb.py` run,
its `QB64PE-DIVERGENCES.md` row (D-048 to D-053), its `UPSTREAM.md` section, and the programs that change moved as
design D8 says. Verify for each: `python tools\libqb\copy_libqb.py --check` passes, its `v63` programs and the
row's older programs pass or `KFAIL` with the new reason (run with `--glob`), and tier 1 passes.

- [x] 2.1 Row 9, `func__memnew`'s 0-byte lock (design D4). It is the smallest edit, so it comes first and proves
  the rebuild of libqb's objects and the build cache's miss.
- [x] 2.2 Row 11, `func__statusCode`'s test (design D4).
- [x] 2.3 Rows 17 and 18, `bitops.h`'s mask (design D6). `v58_f09_bits` and `v30_h_shifts` still pass.
- [x] 2.4 Row 5, `BLOAD` and `BSAVE` (design D3). The scenarios of spec `language/conventional-memory` are `v63`
  programs; Civil War Strategy's scripts, which `BLOAD` its pictures, still pass (`--glob "g12_game*.bas"`).
- [x] 2.5 Row 12, error 11 trappable (design D5): four edits (the fourth, the error's text, found here). The ten verification programs and three corpus
  programs of design D8 run, and are moved by what they now print; `DIVERGENCES-QB45.md` Q-003 says the two now
  agree. The scenario "A division by zero on a joined line" is checked against `v58_e23`'s new output.
- [x] 2.6 Row 2, console input and `END` (design D2). `v22_e_eof`, the four programs that time out at `END`, and
  the corpus programs with a `.stdin` file (`slice\s42_console_input`, `s65_console_input`,
  `s117_dyn_member_kinds`) run. A CLI test runs a `$CONSOLE:ONLY` program with `END` from the null device and
  checks that it ends at once.

## 3. The switched fix

- [x] 3.1 `bit-number-check` (design D7): `Fix::BitNumberCheck` in `crates\base\src\fixes.rs`, appended; the
  wrapper in `codegen-cpp` `builtins.rs` and `qbr_bit_number` in the prelude; its row in `QB64PE-DIVERGENCES.md`'s
  table "Fixes behind the switch"; the sidecar `tests\verification\fixed\v63_<name>.out.txt`; the CLI test of
  "The list" (eight lines). Verify: the program passes with the fixes off and with them on, `cargo test -p
  qb64rust-base` and `-p qb64rust-codegen-cpp` pass, and tier 1 passes.

## 4. Closing

- [x] 4.1 With every fix off, the corpus's C++ from `-z` equals that of the commit before this change (only
  libqb changed). Verify: a script under `target\` compares the two, and no program differs.
- [x] 4.2 `openspec validate libqb-fixes --strict` passes. `LAYOUT.md`'s row for `tools\libqb\` names the edits
  by row. `vscode\CHANGELOG.md`'s unreleased section has a line on the fixes. Verify: all three.
- [x] 4.3 The closing runs: `python tools\close\close.py --plan`, each command run by its own, then `--check`
  passes. Then the counts by `tools\legacy_tests\counts.py`, `STATUS.md`'s "Next" (item 3 done, `study\39` built),
  and `python tools\testtime\test_time.py`. Verify: `--check` passes.

## Workflow follow-up

- Archive the change once the user has reviewed it (`/opsx:archive`).
- Tell the user which files to stage and suggest a commit message (nothing is staged by Claude).
