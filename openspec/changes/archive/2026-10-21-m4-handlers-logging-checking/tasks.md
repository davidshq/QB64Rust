# Tasks

Standing rules: CLAUDE.md. What this change does differently: the runner's log recording (task 1.1) comes before
the `v50` recording with `qb64pe.exe` (task 1.2), which is batched before the code.

## 1. The log in the runner, measurements and records

- [x] 1.1 `tools\legacy_tests\run_legacy_tests.py`: the log's cut, mask, recording and compare in the verification
  suite, with the pass cache's key (design D6). Verify: new tests in `test_run_legacy_tests.py` (an equal log at
  other times and addresses, a line that differs, nothing recorded and nothing logged, a stale `.log.txt` removed
  on recording, the folder masked) pass with the rest of the runner's tests; recording the existing `verification\`
  programs writes no `.log.txt` (none logs), checked with `--record --glob "v49*"`.
- [x] 1.2 The family `verification\v50_*` (design D7), recorded with `--suite verification`, with one line in
  `verification\README.md`. Where a recording differs from a delta spec or the design, the spec or design is
  changed to match in this task: the message for `_LOGINFO 5`, the included file's name in the log, the forms of
  `$CHECKING` the old compiler rejects, and when an error raised below OFF is serviced. Verify: every program
  has its recorded output or error; `v50_b_log.log.txt` exists and holds no local path (`tools\repo_check\
  check_repo.py --untracked`); `openspec validate m4-handlers-logging-checking --strict` passes after any edit of
  the specs.
- [x] 1.3 `DECISIONS.md`: a row for the change. It records the scope (the handler chain, the log statements,
  `$CHECKING` anywhere; the `qb64pe\*` programs as the next change; user 2026-10-21) and the log's compare (design
  D6). Verify: the row names `openspec\changes\m4-handlers-logging-checking\design.md`.

## 2. The handler chain

- [x] 2.1 `syntax` and `sema`: `_NEWHANDLER 0` and `_LASTHANDLER label` parsed; `StmtKind::OnError` with the
  chain's two forms and their three errors (design D1). Verify: `tests\frontend\handler_chain_typed.bas` (`typed`)
  and `handler_chain_errors.bas` (`check-fail`: the `v50_x` chain forms); `cargo test -p qb64rust-syntax -p
  qb64rust-sema`.
- [x] 2.2 `ir` and `codegen-cpp`: `Op::NewHandler` and `Op::LastHandler`, validated, lowered and written as the old
  compiler's lines (design D1). Verify: `tests\frontend\handler_chain_cpp.bas` (`cpp`) pins the lines;
  `v50_a_chain` and `on_error/errtest` pass; call-site programs `tests\callsite\on_error_newhandler.bas` and
  `on_error_lasthandler.bas` equal.

## 3. CHECKING anywhere

- [x] 3.1 `sema`: `$CHECKING:OFF` and `:ON` read in any file, with the errors of task 1.2; `checked` on each
  statement and label, `Proc::checked` removed (design D3). Verify: `tests\frontend\checking_typed.bas` (`typed`:
  the dump shows the flag of statements, a block header and the statements inside, a label, a SUB in source order,
  an included file) and `checking_errors.bas` (`check-fail`: the rejected forms); `cargo test -p qb64rust-sema`;
  the `afterlastline.bm` programs of the corpus still pass (`--glob` of the slice programs that use `afterlastline`).
- [x] 3.2 `ir` and `codegen-cpp`: `checked` on `ir::Stmt` and `ir::Label`, copied by the lowering; `procs.rs`
  `body` and `labels` per statement; `array_check` left out in `place.rs` below OFF (design D3, D4). Verify:
  `tests\frontend\checking_cpp.bas` (`cpp`) pins a statement, a label and an index below OFF and beside ON;
  `v50_c_checking`, `v50_e_index`, `v41_x62_checking_off` and `v33_x54_debug_checking_off` pass;
  `cargo test -p qb64rust-ir -p qb64rust-codegen-cpp`.
- [x] 3.3 `codegen-cpp`: the unchecked `_MEMGET` (function and statement), `_MEMPUT` and `_MEMFILL` (`mem.rs`,
  design D4), and the `{…}` block of D5 where a statement declares inline. Verify: `checking_cpp.bas` gains each
  memory form; `v50_d_mem`, `source_ordering/checking_off` and `mem/mem_checking_off` pass; every `v50` program
  builds; call-site programs `tests\callsite\checking_off_memget.bas`, `_memget_fn`, `_memput`, `_memput_as`,
  `_memfill`, `_memfill_as` equal.

## 4. The log statements

- [x] 4.1 `sema`: `StmtRule::Log` for the four names, `StmtKind::Log`, and the `_LOGMINLEVEL` row (design D2).
  Verify: `tests\frontend\logging_typed.bas` (`typed`: each level, `_LOGMINLEVEL`) and `logging_errors.bas`
  (`check-fail`: the `v50_x` log forms); `cargo test -p qb64rust-sema`.
- [x] 4.2 `ir` and `codegen-cpp`: `Op::Log` written as `sub__log<level>(file, func, line, msg);` with the file,
  procedure and line of D2. Verify: `tests\frontend\logging_cpp.bas` (`cpp`) pins the call in the main module, a
  SUB, a FUNCTION and an included file; `v50_b_log` passes with its log; `logging/commands` passes; call-site
  programs for the four statements and `_LOGMINLEVEL` equal; a slice program `tests\corpus\slice\` with
  `_LOGMINLEVEL` and one log statement on `slice.list`.

## 5. Programs and lists

- [x] 5.1 Onto `tests\upstream\pass.list`: `source_ordering/checking_off`, `mem/mem_checking_off`,
  `on_error/errtest`, `logging/commands`. Onto `tests\verification\pass.list`, off `known.list`:
  `v33_x54_debug_checking_off`, `v41_x62_checking_off`, and the `v50` programs (any that cannot pass goes on
  `known.list` with its reason). `SOMEDAY.md`: the rows of `$CHECKING:OFF` (the `m3-mem` row, the `_ARRAYCOPY`
  row, design D8 of `m3-network-embed`) and of `on_error/errtest` leave it. Verify: each passes with this compiler
  (`--list` of the programs moved); each verification program is named once across the two lists; the shrink-only
  lists of tier 1 regenerate with no new entry.

## 6. Close

- [x] 6.1 The closing runs: `tools\close\close.py --check` passes. Verify: the upstream demand report no longer
  names `$CHECKING`, `_NEWHANDLER`, `_LASTHANDLER` or the four log statements outside the `qb64pe\*` programs.
- [x] 6.2 Documents. `STATUS.md`'s "Next" (this change done; the `qb64pe\*` programs next; its stale item 1
  corrected). `SOMEDAY.md`: a row for each form task 1.2 left marked, and the questionable behaviours kept under
  "QB64pe behaviours to review" (a `RESUME` below OFF retrying the checked statement, if measured so). Counts
  written by `tools\legacy_tests\counts.py`. Verify: `openspec validate m4-handlers-logging-checking --strict`
  passes.
- [x] 6.3 `python tools\testtime\test_time.py`. Verify: its five files are named among those to stage.

## Workflow follow-up

- Archive the change once the user has reviewed it (`/opsx:archive`).
- Tell the user which files to stage and suggest a commit message (nothing is staged by Claude).
