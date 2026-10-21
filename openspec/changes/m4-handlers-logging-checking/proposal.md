# Proposal

## Why

The fifth change of M4 (user, 2026-10-21: "Do 2", `STATUS.md` "Next" item 3, the upstream report's remaining
lines). Item 3 has four areas, split into two changes. This one takes the three areas that are a few lines of the
old compiler each. The old compiler's own sources (`qb64pe\*`, ten upstream programs) follow as their own change,
because they need about thirty more built-ins and name rules. They also use the handler chain, so it comes first.

The demand report names these lines (upstream, `--include-root tests\upstream\root`):

| Line | Upstream programs |
|---|---|
| metacommand `$CHECKING` (`$Checking`) | `source_ordering/checking_off`, `mem/mem_checking_off` (the only mark of each) |
| `ON ERROR GOTO _NEWHANDLER`, `ON ERROR GOTO _LASTHANDLER` | `on_error/errtest`, and the four `qb64pe/element*` programs |
| `_LOGERROR`, `_LOGWARN`, `_LOGINFO`, `_LOGTRACE` | `logging/commands` |

`tests\verification\known.list` also holds `v41_x62_checking_off` and `v33_x54_debug_checking_off` as
"not supported yet".

Found before this proposal, from `qb64pe.bas`, libqb and the upstream programs:

- **The handler chain** (`qb64pe.bas` 10044–10114). `ON ERROR GOTO _NEWHANDLER label` pushes the active
  handler's number onto `error_handler_history` and then sets the new handler. `ON ERROR GOTO _LASTHANDLER` pops
  the last handler back into `error_goto_line`; an empty history gives 0, no handler. `ON ERROR GOTO 0` already
  clears the history, as our emitter writes it. The three forms have three errors of their own: "No label allowed
  after _LASTHANDLER", "Expected: ON ERROR GOTO [_NEWHANDLER] label" and "Zero not allowed after _NEWHANDLER".
- **`_LOGTRACE`, `_LOGINFO`, `_LOGWARN`, `_LOGERROR msg$`** (`emit\logging.bas`) are each one call to a libqb
  macro: `sub__logtrace(file, func, line, msg)`. The file is the source as named on the command line, or the
  included file. The function is the procedure's C name (`SUB_X`), or "Main QB64 Code". The line is the line in
  that file. The runtime writes the entry to the log set up by `QB64PE_LOG_*`, which the runner already sets
  (handler `file`). `_LOGMINLEVEL` is a plain row of the table.
- **`$CHECKING:OFF` and `:ON`** set `CheckingOn` for the lines that follow, in source order, through SUBs and
  included files (`source_ordering/checking_off`: a FUNCTION defined between OFF and ON is unchecked). Below OFF
  the old compiler writes, at about forty places:
  - no event and error check around a statement, at a label or at a block header;
  - no `array_check` in an index;
  - the unchecked forms of `_MEMGET` (function and statement), `_MEMPUT` and `_MEMFILL`;
  - no root-data guard of a user-type array.

  The new compiler reads `$CHECKING` only in `afterlastline.bm`, and only per procedure (design D8 of
  `m3-network-embed`).

## What Changes

- **`ON ERROR GOTO _NEWHANDLER label` and `ON ERROR GOTO _LASTHANDLER`**: the handler chain, as the old compiler
  writes it, with its three errors. Inside a procedure the label is a label of the main module, as for
  `ON ERROR GOTO label`.
- **`_LOGTRACE`, `_LOGINFO`, `_LOGWARN`, `_LOGERROR msg$`** and the function **`_LOGMINLEVEL`**: the runtime's
  log, with the old compiler's file, procedure and line. A message that is not a string is the old compiler's error.
- **`$CHECKING:OFF` and `$CHECKING:ON` anywhere** in a program, its included files and its procedures. The
  statements below OFF are written as the old compiler writes them there: no event or error check, no index check,
  and the unchecked `_MEMGET`, `_MEMPUT` and `_MEMFILL`. The flag becomes a statement's, no longer a procedure's.
- **The verification suite records and compares a program's log** (`<name>.log.txt`, written when the program
  logged entries of its own scope). Only those entries are compared, without the timestamp and the stack frames'
  addresses, and with the folder masked.
- **Tests**: a measurement family `v50` in `verification\`, front-end tests, call-site programs for the log
  statements and for `_MEMGET`, `_MEMPUT` and `_MEMFILL` under OFF, and slice programs. Four upstream programs go
  onto `tests\upstream\pass.list`: `source_ordering/checking_off`, `mem/mem_checking_off`, `on_error/errtest`
  and `logging/commands`. The `qb64pe/element*` programs stay off it until the second change. `v41_x62` and
  `v33_x54` move to `tests\verification\pass.list`.

## Capabilities

### New Capabilities

- `language/run-time-checks`: `$CHECKING:OFF` and `:ON`, and what a statement below OFF leaves out.
- `language/logging`: the four log statements and `_LOGMINLEVEL`.

### Modified Capabilities

- `language/error-handling`: new requirement "The handler chain" (`_NEWHANDLER`, `_LASTHANDLER`, their errors).
- `testing/verification-suite`: new requirement "The program's log" (recorded with `qb64pe`, compared otherwise).

## Impact

- `syntax`: `ON ERROR GOTO _NEWHANDLER 0` parsed, so that its error can be given.
- `sema`: `check\flow.rs` `on_error` (the chain and its errors); the log statements' rule in `builtins.rs`;
  `check\mod.rs` (`$CHECKING` read everywhere, the flag kept per statement and per label).
- `ir`: `Op::SetHandler` gains the chain's two forms; `checked` moves from `Proc` to each statement and label;
  the log call carries its file, procedure and line.
- `codegen-cpp`: `procs.rs` `body` and `labels` per statement; `place.rs` (`array_check` left out); `mem.rs`
  (the unchecked forms); `stmt.rs` (the chain, the log calls).
- `tools\legacy_tests`: the log's recording and compare, with tests in `test_run_legacy_tests.py`.
- Lists: `tests\verification\known.list` loses `v33_x54` and `v41_x62`. No new dependency, no libqb change.
