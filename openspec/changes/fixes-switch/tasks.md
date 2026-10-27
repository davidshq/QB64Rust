# Tasks

Standing rules: CLAUDE.md. What this change does differently: one batch of recordings with `qb64pe.exe` (task 1.2,
family `v62`) comes before any compiler work, so that it never waits on the clone later. With every fix off, the
generated C++ of the corpus is compared byte for byte before and after (task 2.4).

## 1. Records and measurements

- [x] 1.1 `DECISIONS.md`: a row for the dependency `toml` (design D2) and one for the fix names of this change
  (spec `compiler/fixes`), naming `openspec\changes\fixes-switch\design.md`. Verify: the rows are there.
- [x] 1.2 Family `v62` in `verification\` ("the fixes"): one program per row of design D5 that shows QB64pe's
  side, plus `study\39` §3's measurements that this change needs: row 8 (the error a block gives after `REDIM`;
  `CLEAR`, `RUN` and a SUB's local dynamic array at its end), row 10 (whether an element of a `STRING * n` array
  has a NUL after it: a read of the C++ from `qb64pe -z`, written in the family's line), row 13 (the forms of
  `RESUME` in a FUNCTION), row 14 (how many `RUN`s from a SUB QB64pe takes to run out of stack, and the C++ of a
  `$CONSOLE:ONLY` program's start, for the console selection D5 copies). Recorded in one batch with `qb64pe.exe`.
  The family's line goes in `verification\README.md`, and each program goes into `tests\verification\known.list`
  or `pass.list`. Verify: `tools\legacy_tests\run_legacy_tests.py --suite verification --qb64 <qb64pe> --glob
  "v62_*"` records every program, and tier 1's `verification_lists_are_exact` passes.
- [x] 1.3 Read whether `END` from a procedure unwinds in the new compiler's C++ (design, Open Questions). Write
  the answer in design D5, row 14, in one sentence. Verify: the sentence is there.

## 2. The switch

- [x] 2.1 `crates\base\src\fixes.rs`: `Fix`, the table of names and descriptions, `Fixes` (design D1). Unit tests
  of the table and the set. Verify: `cargo test -p qb64rust-base fixes` passes.
- [x] 2.2 `Cargo.toml`: `toml` in `[workspace.dependencies]`, used by `driver`. `crates\driver\src\fixes.rs`:
  `find_settings`, `read_settings`, `apply_cli`, `for_program` (design D1, D2), with the unit tests of design D8.
  `tools\licenses\` accepts the new crates' licences. Verify: `cargo test -p qb64rust-driver fixes` passes, and
  `python tools\licenses\check.py` passes.
- [x] 2.3 The set through the stages: `check_program` takes `Fixes`; `sema` `check_with` and `Settings`, and
  `Program.fixes`; `ir::Program.fixes`; the emitter reads it. `main.rs`: `--fixes`, `--fix`, `--list-fixes`, the
  usage text, `-w`'s "fixes from <file>" line, the settings error with exit status 1. CLI tests of spec
  `compiler/fixes` ("A misspelt name", "Off overrides the file", "One fix on", "The list") and spec `compiler/cli`.
  Verify: those CLI tests and tier 1 pass.
- [x] 2.4 `build.rs` `build_key`: version 7 and the fix bits (design D4). Unit test `build_key_follows_the_fixes`,
  and the CLI test of "Built again with a fix turned on". Before any fix's code exists, write the corpus's C++ with
  `-z` into `target\fixes-before\`. Each later group compares its own with it (task 6.1). Verify: the tests pass,
  and the folder holds one `.cpp` per corpus program.

## 3. The language server and the extension

- [x] 3.1 `check_for_server`, `format_for_server`, `index_for_server` through `fixes::for_program`. A settings
  error becomes a diagnostic on line 1 (design D3). `crates\lsp`: `workspace/didChangeWatchedFiles` drops cached
  results and rechecks the open documents. The server's tests for spec `editor/language-server` ("A fix's compile
  error shows" waits for group 5, so it is written there; "A bad settings file", "The fixes turned off" with a
  fix of group 4). Verify: `cargo test -p qb64rust-lsp` and the driver's server tests pass.
- [x] 3.2 `vscode\src\languageServer.ts`: the `qb64rust.toml` file watcher. `vscode\src\newProject.ts` and its
  contribution in `package.json` (design D7). The extension's tests of spec `editor/new-project`, with the dialog
  stubbed. `vscode\CHANGELOG.md` gets an unreleased section with the New Project command and the fixes. Verify: `npm
  test` in `vscode` passes (unit and integration).

## 4. Always-on fixes

Each task: the fix, its `QB64PE-DIVERGENCES.md` row, its `v62` program and the row's older programs moved as
design D8 says (`known.list` with `D-0nn`, or a `.divergences` sidecar), and a CLI test with the divergence trace
where the program ends up on a path QB64pe never takes. Verify for each: its `v62` and older programs pass or
`KFAIL` with the new reason, run with `--glob`, and tier 1 passes.

- [x] 4.1 Row 3, a count below zero (design D5).
- [x] 4.2 Row 4, `STATIC` beside a shared static array: `sema` `check\decl.rs`. The language server's hover
  shows the SUB's own array (a server test).
- [x] 4.3 Row 7, `whole_data` and error 9 in every whole-array use.
- [x] 4.4 Row 8, the lock helper shared by `REDIM` and `ERASE`, and the other release paths that task 1.2 found
  failing.
- [x] 4.5 Row 10, copy in and copy out for `STRING * n` members and elements, including a library FUNCTION in an
  expression. Uses the `v37` test libraries (`verification\v37_lib\build_libs.py`).
- [x] 4.6 Row 13, `RESUME` in a SUB through the exit label.
- [x] 4.7 Row 14, the unwind of `RUN` from a procedure, and the console in a `$CONSOLE:ONLY` program after `RUN`.
  The program of spec `language/control-flow` "RUN from a SUB many times" is a CLI test (100,000 runs, ended by a
  counter in the process's environment: a counter file took over 300 s here, two file opens per run).
- [x] 4.8 Row 16, the `_CAST` helpers and the folder's matching values. `v49_a_cast` and `v49_x18` are run with
  and without `QB64RUST_NO_FOLD=1`.

## 5. Switched fixes

Each task: the fix behind `fixes.is_on` (design D6), its `fixed\<name>.out.txt` sidecar or CLI test (design D8),
and its row in the new table "Fixes behind the switch" of `QB64PE-DIVERGENCES.md` (name, what QB64pe does, what
the fix does, the QB 4.5 program where one was measured). Verify for each: the program with the fixes off still
passes against its `qb64pe.exe` recording, the fixed run or CLI test passes, and tier 1 passes.

- [x] 5.1 The runner: `--fixes off` in every `qb64rust` command and in its first line, the `fixed\` sidecars run a
  second time with `--fixes on` and reported as `<name> [fixes on]`, and `verification_lists_are_exact` checks
  `fixed\`. `tools\legacy_tests\README.md` and `tests\verification\README.md` describe both. Verify: the runner's
  own tests (`tools\legacy_tests\test_*.py`) pass, and a `--glob` run over one program with a sidecar shows both
  results.
- [x] 5.2 `resume-after-block` (`ir` `lower.rs`, the statement boundary's `next`; `crates\ir\src\lib.rs`'s module
  comment). Sidecars for the `v17` block-header programs that the spec's scenarios name.
- [x] 5.3 `dim-empty-error`. Sidecar for `v24_b_dynamic`.
- [x] 5.4 `static-local-dynamic`. Sidecar for `v55_d_static_shared`.
- [x] 5.5 `whole-array-value`, `resume-in-procedure`, `run-in-procedure`: the compile errors, with CLI tests
  for each, on and off. Also the language server test "A fix's compile error shows" (task 3.1).
- [x] 5.6 `checking-on-service`. Sidecar for `v50_c_checking`.

## 6. Closing

- [x] 6.1 With every fix off, the corpus's C++ from `-z` equals `target\fixes-before\`, except for programs that
  use a form an always-on fix rewrites (rows 3, 7, 8, 10, 13, 14 and 16). Each of those is named in this task with
  its row. Verify: a script under `target\` compares the two folders, and the names it lists are only those.
  *Done* (`target\fixes_cpp.py`, `target\classify_cpp.py`; `--dump cpp` of each program, 484 with C++, 29 rejected
  as their `.err` expects): 474 equal, 39 differ, every difference one of these rows. Row 3 (the test of a count
  below zero in every dynamic allocation) and row 8 (the lock renewed by `ERASE`, `CLEAR` and the release of a
  `REDIM` that allocates nothing): `runtime_comparison\120_redim_no_preserve`, `203_redim_multiple`, `40_erase`;
  `screen\d18_get_put`; `slice\s112`–`s117`, `s119`, `s120`, `s131`–`s133`, `s151_clear`,
  `s154_qb64pe_sources_builtins`, `s17_signs`, `s20_arrays`, `s52_dim_list`, `s52_dynamic`, `s52_dynamic_memory`,
  `s53_redim`, `s54_redim_preserve`, `s55_erase`, `s55_implicit`, `s56_proc_arrays`, `s57_array_params`,
  `s58_common`, `s59_type_strings`, `s61_member_arrays`, `s73_conventional_arrays`, `s89_mem_references`,
  `s90_mem_get_put`. Row 3 alone: `screen\g06_message_box`. Rows 3, 7 and 8 (row 7: a whole array's file `GET` and
  `PUT` guarded): `slice\s118_dyn_member_files`, `s126_owner_copies`, `s83_binary_file`. Row 16 (`_CAST` from a
  float): `slice\s150_builtins_more`. No corpus program has a `RUN` or `RESUME` in a procedure or a `STRING * n`
  member passed to a library (rows 10, 13, 14).
- [x] 6.2 `openspec validate fixes-switch --strict` passes. `LAYOUT.md` gets rows for `crates\base\src\fixes.rs`,
  `crates\driver\src\fixes.rs`, `vscode\src\newProject.ts` and `tests\verification\fixed\`. `SOMEDAY.md`'s
  candidates from `study\39` §4 are unchanged. Verify: all three.
- [x] 6.4 Deterministic builds (added during the close, user, 2026-10-27; design D4): no timestamp in the program's
  objects and executable, the build's date and time only for `_COMPILEDATE$`/`_COMPILETIME$`, the key at version 8;
  `tests\corpus\slice\s52_dynamic_memory` removed (its only case is D-040 now) and `s90_mem_get_put`'s `ERASE` case
  (D-043) left out, both recorded again with `qb64pe.exe` (output and layout). Verify: CLI test
  `builds_are_deterministic` and tier 1 pass.
- [x] 6.3 The closing runs: `python tools\close\close.py --plan`, each command run by its own (the run without
  folding is included, since row 16 touches constant evaluation), then `--check` passes. Then the counts by
  `tools\legacy_tests\counts.py`, `STATUS.md`'s "Next" (item 3 is under way, and the libqb change follows), and
  `python tools\testtime\test_time.py`. Verify: `--check` passes.

## Workflow follow-up

- Archive the change once the user has reviewed it (`/opsx:archive`).
- Propose the second change, the libqb rows (2, 5, 9, 11, 12, 17, 18) and `bit-number-check`, on the switch built
  here.
- Tell the user which files to stage and suggest a commit message (nothing is staged by Claude).
