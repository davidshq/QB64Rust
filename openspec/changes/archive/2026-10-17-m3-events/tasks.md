# Tasks

Standing rules: CLAUDE.md. Differently here: the 16 compile checks of the proposal were run in a scratch folder,
not recorded; task 1.1 records them with the run-time measurements as a family of `verification\`.

## 1. Measurements and records

- [x] 1.1 The family `verification\v42_*` (design D5): the 16 compile checks of the proposal and the run-time
  programs (a timer in `$CONSOLE:ONLY` firing; `STOP`, `OFF`, `ON` after `STOP`; `_FREETIMER` and its reuse after
  `FREE`; error 5 for `TIMER FREE`, an unallocated timer, `ON KEY(40)`; a SUB handler and its argument; a handler set
  up in a SUB; `ON KEY(1.7)`; `KEY ON`, `KEY LIST`, `KEY n, s$` in `$CONSOLE:ONLY`; a `BYVAL` parameter; a SUB
  declared after the statement; `KEY n, 5`), recorded with `--suite verification`. Then the facts into `study\00`
  §5 (a `v42` paragraph), the family into `verification\README.md`, a row in `DECISIONS.md` (the step; `STRIG` left
  for device input, user 2026-10-17). Where a measurement differs from design D2 (the `ON KEY` truncation,
  `BYVAL`, a later SUB), the design is changed to match in this task. Verify: every program has its recorded
  output or error; `python tools\repo_check\check_repo.py --untracked` is clean.
  - Done: 66 programs (`v42_a`–`f`, `x01`–`x60`). `ON KEY(0.7)` is error 5 and `ON KEY(31.6)` key 31: cut, as
    D2 expected. Design D2 changed: `BYVAL` is already an error at the SUB's header; a `TYPE` parameter passes the
    old checks but its C++ fails, so it is an error here.
  - Found: an event that comes during an error handler is not serviced there, and after `RESUME` it waits through
    plain statements until the next `TIMER ON` or `_DELAY` (`v42_e` 4.–6., `v42_f`). libqb's own behaviour: kept,
    and goes to `SOMEDAY.md` in task 4.2.

## 2. Front end

- [x] 2.1 `syntax`: `on_event` reads the SUB's argument; `event_switch` and `EventSwitchStmt` go; `KeyStmt` for
  `KEY ON|OFF|LIST` and `KEY a, b` in `template.rs` (design D1); snapshots updated. Verify: parser tests of `ON
  TIMER(1) s 5`, `TIMER(t) FREE`, `KEY(0) ON`, `KEY ON`, `KEY 15, x$`, `KEY ON 1` (a syntax error), `KEY(1) FREE`
  (the reference error); `cargo test -p qb64rust-syntax`.
- [x] 2.2 `sema`: `check\events.rs` with `on_event` and `key_stmt` (design D2), `StmtKind::OnEvent` and
  `StmtKind::Key`, the rows of `TIMER`, `KEY`, `_FREETIMER`; the two marks of `check\mod.rs` go; `PEN`, `COM`,
  `UEVENT` errors, `STRIG` marked. Verify: `tests\frontend\events_errors.bas` (`check-fail`: every rejected form of
  `v42`, `ON PEN`, `PEN ON`, `UEVENT ON`) and `events_ok.bas` (`check-ok`: every accepted form, a handler set up
  in a SUB) pass; `ON STRIG(0) GOSUB h` and `STRIG(0) ON` are "not supported yet".
  - Done: also `events_typed.bas` (`typed`), which the coverage check of tier 1 asks for `_FREETIMER`. `PEN ON`,
    `COM(1) ON`, `UEVENT ON` reached the parser's "not supported yet: statement `PEN`" once `EventSwitchStmt` was
    gone; they are a plain error there (`parser\mod.rs` `not_supported_statement`). The 42 rejected `v42`
    programs are each rejected with a real error (two of them, `x56` and `x59`, with a mark beside it).
- [x] 2.3 `ir`: `Op::OnEvent` and `Op::Key`, lowering, validation (the main module's label from any body; unit tests
  of a label index out of range and of an argument for a SUB without a parameter) and dump (design D3). Verify:
  `tests\frontend\events_ir.bas` (`ir`) passes; `cargo test -p qb64rust-ir`.

## 3. Code generation and runs

- [x] 3.1 `codegen-cpp`: `events.rs`, the setup calls, the ids, the cases and jumpers, the four fragments,
  `Op::Key` (design D4). Verify: `tests\frontend\events_cpp.bas` (`cpp`: a label handler, a SUB handler with and
  without an argument, a handler set up in a SUB, the label after a loop, a `SELECT` and an `ON ERROR` handler)
  passes, and its C++ builds (a CLI test with `--ignored`); `cargo test`, fmt and clippy green.
- [x] 3.2 Call-site programs for every form (design D5). Verify: `python tools\callsite\callsite.py --glob
  "event*"` (and the `key_*`, `timer_*` names chosen) reports every one equal; a rule added to `callsite.py`
  comes with its `README.md` line and test.
  - Done: 26 programs `event_*`, all equal; no rule added. The seconds and the key go to a `double` of libqb, so
    the programs pass them as a float literal or a `DOUBLE` variable, as the README asks for such slots (an
    integer there is written with its cast by this compiler only); the conversions are covered by `v42_b`,
    `v42_d` and the slice program.
- [x] 3.3 Slice program `s134_events_timer` and screen program `k01_on_key` with its keys file, recorded with
  `qb64pe.exe` (`--record`, two equal runs for the screen program) and added to `tests\corpus\slice.list`.
  Verify: both pass with this compiler (`--glob`), the timer program also under `--jobs 8` beside other programs.
- [x] 3.4 `verification\v42_*` built and run with this compiler; the four upstream `timer\` programs added to
  `tests\upstream\pass.list`. Verify: every accepted `v42` program prints what the old compiler's does, every
  rejected one is rejected with one error; the four upstream programs pass (`--glob "timer/*"`).
  - Done: the 24 accepted programs recorded with `qb64pe.exe` copied into a scratch corpus root and run with this
    compiler: 21 pass; `x11` (`STRIG`) and `x46` (a line number) stay marked as designed; `v42_e` differs at
    its 1. only, where the corpus runner ends `SLEEP` at once with either compiler's build (`study\00` §5; by hand
    both builds print the recorded lines). The 42 rejected programs are each rejected with a real error (task
    2.2). The four upstream programs pass (`--category timer`).
  - Found: `v42_f`'s last line was a timing edge in both builds (2, 2, 1 in three runs of each); the program was
    changed to stay clear of it and recorded again.

## 4. Close

- [x] 4.1 The closing runs (working rule 9), the shrink-only lists, the demand reports and
  `tools\diagnostics\old_wording.py` regenerated, the counts by `tools\legacy_tests\counts.py`. Verify: every run
  passes or fails only by known failures; the demand report no longer names the event switches and handlers.
  - Done: corpus 465 pass (slice list 446 of 446 by `--check-list`), upstream 336 (pass list 336 of 336), without
    folding 445 of 446 (`u02_mouse_text` failed once under load, passed alone), differential 59 of 59, call-site
    1864 of 1864, 37 CLI tests. A first full corpus run was stopped at 183 of 471: `old_wording.py` changed the
    build after it started, which `--check-list` refuses; the table now comes before the full runs (`study\32`
    §4, item 5). The user asked about the time the closing runs take: `study\32`.
- [x] 4.2 Documents: `STATUS.md` (also its line that still calls `m3-arraycopy` not committed: it is `7439ea8`),
  `ROADMAP.md`, `SOMEDAY.md` (the `STRIG` events under device input; the questionable behaviours kept, design
  "Risks"). Verify: `openspec validate m3-events --strict` passes.
- [x] 4.3 `python tools\testtime\test_time.py`. Verify: its five files are named among those to stage.

## Workflow follow-up

- Archive the change once the user has reviewed it (`/opsx:archive`).
- Tell the user which files to stage and suggest a commit message (nothing is staged by Claude).
