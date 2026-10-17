# Proposal

## Why

The next step of M3 is the user's choice (`STATUS.md`; user, 2026-10-17: "Lets do event switches and event
handlers now"). They are the first two lines of the upstream demand report (four programs each, the same four:
`timer/sleep_ends_early`, `timer_delayed_run`, `timer_first_run`, `timer_stop`, each using a switch and a handler),
and `ROADMAP.md` lists them under M3. The parser already reads both (`OnEventStmt`, `EventSwitchStmt`); `sema`
reports them "not supported yet". The runtime is QB64pe's: `qbx.cpp` (the timers, `events()`, the jump into
`QBMAIN` for a `GOSUB` handler) and libqb's `key-events.cpp` and `keyboard.cpp`, which this compiler already
builds and links. The code generator already writes the six fragments `qbx.cpp` includes for events, empty, and
already services events after every statement (`if(qbevent){evnt(…);}`), as error handling does.

Found before this proposal, from `qb64pe.bas` 7281–7340 and 7534–8052, `subs_functions.bas` and 16 short
programs compiled with the old compiler (the runner's copy, `-z`; recorded as a family of `verification\` in task
1.1, with the run-time measurements):

- **The old compiler has three events: `TIMER`, `KEY` and `STRIG`.** `ON PEN`, `ON COM(1)` and `ON UEVENT` are
  "Name already in use (PEN)" and so on; `PEN ON`, `COM(1) ON` and `UEVENT ON` are "Syntax error"; `PLAY` is no
  event (already measured, `v29_x61`–`x64`). Our parser takes all six words as events and `sema` marks them
  "not supported yet", which is wrong for the three that are errors.
- **A handler is `ON TIMER([n,] seconds)`, `ON KEY(k)` or `ON STRIG(…)`, then `GOSUB label` or a SUB's name and
  an optional argument.** The label is one of the main module, also from a statement inside a SUB (a label only in
  the SUB is "Label 'h' not defined"). The SUB takes no parameter, or one of an integer type (`_UNSIGNED` and
  `_OFFSET` included), and the argument is given exactly when it has one; otherwise "Expected argument after SUB",
  "SUB has no arguments", "Only SUB arguments of integer-type allowed" (also an array), "Expected GOSUB/sub" (no
  such SUB, or a FUNCTION).
- **The switches are plain built-in statements of the table**: `TIMER[(n)] {ON|OFF|STOP|FREE}`, `KEY(k)
  {ON|OFF|STOP}`, `STRIG[(…)] {ON|OFF|STOP}` (`KEY(1) FREE` and `TIMER(1, 2) ON` are syntax errors with the
  form as reference), and `_FREETIMER` gives the number of a new timer.
- **`KEY` has four forms of its own, read before the template**: `KEY ON`, `KEY OFF`, `KEY LIST` (the soft-key
  line) and `KEY n, text$` (a soft key, or with `CHR$(flags) + CHR$(scancode)` a user key 15–25 for `ON KEY`).
  Today `KEY ON` is read as an event switch and the other three are "matches none of its forms", a wrong error.
- **`STRIG` needs libqb's device-input part** (`DEPENDENCY_DEVICEINPUT`), which no build of ours has yet, nor its
  other names (`STICK`, `STRIG()`, `_DEVICES`, …). The user chose (2026-10-17) to leave it for a device-input
  change: `ON STRIG` and the `STRIG` switch stay "not supported yet".

## What Changes

- **`ON TIMER` and `ON KEY` handlers** in both forms, from any body, with the old compiler's checks, the libqb
  setup call and the generated dispatch: for a label, the case that re-enters `QBMAIN` and the jump to the label
  there; for a SUB, the case that calls it with the argument evaluated when the handler was set up.
- **The switches `TIMER[(n)] ON|OFF|STOP|FREE` and `KEY(k) ON|OFF|STOP`**, and the function `_FREETIMER`, as
  plain built-ins of the table.
- **`KEY ON`, `KEY OFF`, `KEY LIST` and `KEY n, text$`.**
- **Real errors for `PEN`, `COM` and `UEVENT`** used as events, in place of "not supported yet"; `STRIG` stays
  marked, now naming device input.
- **Tests**: a measurement family in `verification\`; front-end tests of every rejected form and of the IR and C++;
  call-site programs; slice programs for timers (console) and a screen program for `ON KEY` driven by the keys
  file; the four upstream `timer\` programs added to `tests\upstream\pass.list`.

## Capabilities

### New Capabilities

- `language/events`: event trapping: `ON TIMER` and `ON KEY` handlers, the switches, numbered timers, when an event
  is serviced, the soft keys and user keys of `KEY`, the forms that are errors and those left marked.

### Modified Capabilities

- `compiler/pipeline`: the IR holds the setting up of an event handler, whose label may be one of the main module
  while the statement stands in a procedure.

## Impact

- Changed: `crates\syntax` (the optional argument after a SUB's name in `ON … `; the switches read by their
  template; `KEY`'s own forms; only `TIMER`, `KEY` and `STRIG` are event words), `crates\sema` (a module
  `check\events.rs`; `builtins.rs`: `TIMER`, `KEY`, `_FREETIMER`; `lib.rs`: the statement kinds),
  `crates\ir` (an `Op`, its lowering, validation and dump), `crates\codegen-cpp` (the setup call and the fragments
  `ontimer.txt`, `onkey.txt`, `ontimerj.txt`, `onkeyj.txt`).
- New tests: `tests\frontend\events_*.bas`, `tests\callsite\` programs for each new form, slice programs, a screen
  program with its keys file; `tests\upstream\pass.list`; the demand report and the shrink-only lists regenerated.
- No change to libqb, the Makefile or the reference clone.
- Not in this change: `STRIG` events (device input, user 2026-10-17), line numbers as handler targets (`SOMEDAY.md`
  already holds them).
- `STATUS.md`, `ROADMAP.md`, `study\00` §5, `SOMEDAY.md` (the questionable behaviours kept), `verification\README.md`
  (the new family) and `DECISIONS.md` (the user's choice about `STRIG`) updated.
