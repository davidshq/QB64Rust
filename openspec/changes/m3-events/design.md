# Design

## Context

Motivation, scope and what was found before the proposal: `proposal.md`; the behaviour: the delta specs. Where
the parts stand in this compiler (2026-10-17):

- **Parser**: `syntax\parser\errors.rs` reads `ON <event>[(args)] [GOSUB] target` as `OnEventStmt` (`on_event`;
  anything after the target is dropped by `recover`) and `<event>[(args)] ON|OFF|STOP|FREE` as `EventSwitchStmt`
  (`event_switch`, tried in `parser\mod.rs` 566 before the template statements), both for the six words of
  `EVENT_WORDS` (`TIMER KEY STRIG PEN COM UEVENT`). `KEY 15, x$`, `KEY LIST` reach the template statement
  (`parser\template.rs`), whose only `KEY` form is `(?){On|Off|Stop}`, hence the wrong "matches none of its forms".
- **`sema`**: `check\mod.rs` 1006–1009 marks both nodes "not supported yet". `check\flow.rs` `jump_words_in`
  already counts the label after `GOSUB` in an `OnEventStmt` as a jump target, and `label_ref(tok, body)` resolves a
  label in a body. Plain built-ins are rows of `builtins.rs` `STATEMENTS` and `FUNCTIONS`, checked by
  `check\builtins.rs` `builtin_stmt` against the table's templates (`TIMER [(?)]{On|Off|Stop|Free}`, `KEY
  (?){On|Off|Stop}`; `_FREETIMER` a function of no argument).
- **IR**: labels belong to one body (`LabelId` indexes its `labels`); `validate.rs` `own_label` holds every jump to
  its body. Code after `SYSTEM` is kept (a `GOSUB` target there works today).
- **`codegen-cpp`**: `lib.rs` `FRAGMENTS` writes `onkey.txt`, `ontimer.txt`, `ontimerj.txt`, `onkeyj.txt` (and the
  `STRIG` pair) empty. Main-module variables are allocated once behind a `NULL` test (`maindata.txt`), `ret0.txt`
  has `case 0: return;`, and every statement ends in `if(!qbevent)break;evnt(…);` or `if(qbevent){evnt(…);}`, so a
  `GOSUB` handler can re-enter `QBMAIN` as the old compiler's does.
- **The old compiler** (`qb64pe.bas` 7726–7901 `ON TIMER`, 7906–8052 `ON KEY`, 7281–7340 `KEY`'s own forms): per
  handler it writes `ontimer_setup(n, seconds, id, pass);` or `onkey_setup(k, id, pass);` where the statement stands,
  numbers the handlers of each event from 1 in source order (`id`), and appends the `case id:` of `events()`'s
  switch to `ontimer.txt`/`onkey.txt` and, for a label, `if(timer_event_id==id)goto LABEL_x;` to
  `ontimerj.txt`/`onkeyj.txt`, which `QBMAIN` runs at entry.
- **libqb/`qbx.cpp`**: `TIMERTHREAD` marks a due timer and sets `qbevent`; `evnt` calls `events()`, which runs the
  case of each marked event that is `ON`, not in progress, outside error handling. `keyboard_keydown` marks a
  trapped key; the screen-state tool presses keys through it (`tools\screen\screen_state.cpp` `press`).

## Goals / Non-Goals

**Goals:**

- The handlers, switches and `KEY` forms with the old compiler's checks and the same libqb calls and fragments, so
  that the call-site check compares them and the runtime does the rest.
- One IR operation per handler setup; the numbering of handlers and the fragments are code generation.

**Non-Goals:**

- `STRIG` (device input; user, 2026-10-17), line numbers as targets.
- Any change to libqb or `qbx.cpp`: timing, servicing and re-entry stay theirs.

## Decisions

### D1. The parser: three event words for `ON`, switches by template, `KEY`'s own forms

- `EVENT_WORDS` keeps the six words for `ON` only (`on_stmt`), so that `ON PEN GOSUB h` stays one `OnEventStmt`
  that `sema` rejects with a real error (D2). `on_event` reads an optional expression after a SUB's name (the
  argument), as a child of the node, before `recover`.
- `event_switch` and `EventSwitchStmt` go (kind, AST type, the `mod.rs` branch). `TIMER(t) ON` and `KEY(1) OFF` are
  then read by the template statement like any built-in (`template.rs`), and `STRIG(0) ON` too. `PEN ON`, `COM(1)
  ON`, `UEVENT ON` fall through to `call_stmt`, which `sema` rejects (D2).
- `KEY`'s own forms, read in `template.rs` `template_statement` before the table, as `PALETTE USING` is: `KEY ON`,
  `KEY OFF`, `KEY LIST` alone, and `KEY expr, expr` (a comma outside parentheses), as a new node `KeyStmt` (word,
  or the two expressions). `KEY ON 1` is a syntax error there ("expected the end of the statement after `KEY ON`").

*Alternative considered*: keep `EventSwitchStmt` and map it onto the table in `sema`. Rejected: the template
machinery already reads these forms, gives the old compiler's "syntax error with the form as reference" for
`KEY(1) FREE` and `TIMER(1, 2) ON`, and `KEY ON` must not be a switch anyway.

### D2. `sema`: a module `check\events.rs`

`Checker::on_event(stmt)` (called from `check\mod.rs` in place of the mark):

- **Event**: `TIMER` and `KEY`; `STRIG` is "not supported yet: `ON STRIG` (device input)"; `PEN`, `COM`, `UEVENT`
  are the error "`PEN` is not an event" (the old compiler's own "Name already in use (PEN)" comes as the note,
  `tools\diagnostics`). A `PEN`, `COM` or `UEVENT` statement (D1) gets the same error in `check\builtins.rs`
  where a statement name is looked up.
- **Operands**, converted as the old compiler's `evaluatetotyp` does: `ON TIMER(n, s)`: `n` to `LONG` (a float
  rounded as for any `LONG` argument), `s` to `DOUBLE`; `ON KEY(k)`: `k` to `DOUBLE`, as the old compiler
  writes it into `onkey_setup`'s `int32` parameter, so C++ truncates it (measured: `ON KEY(0.7)` is error 5, `ON
  KEY(31.6)` key 31, `v42_d`). An empty operand or a third one is the old compiler's error ("Expected ... )",
  "Expected )").
- **Handler**: after `GOSUB` a label, resolved in the main module (`label_ref(tok, None)`) whatever body the
  statement is in; a line number stays "not supported yet". Otherwise a name looked up in `procs_by_name` that must
  be a SUB ("Expected GOSUB/sub" for none or a FUNCTION), with at most one parameter, given an argument exactly when
  it has one, that parameter of an integer type (`_BYTE` … `_INTEGER64`, signed or not, `_OFFSET`; not `_BIT`, a
  float, a string, an array). The argument is converted to `_INTEGER64`. Measured in task 1.1 (`v42`): a `BYVAL`
  parameter is already an error at the SUB's header ("BYVAL can currently only be used with DECLARE LIBRARY",
  `x23`); a SUB defined after the statement is the normal case and works (`x53`); a `TYPE` parameter passes the
  old compiler's checks but its C++ fails to build (`x42`), so it is a compile error here (as `m3-versioninfo` did
  for a build that fails); a string operand or argument is "Illegal string-number conversion", two keys or two
  arguments "Invalid expression" (`x29`, `x33`–`x36`).
- New `StmtKind::OnEvent { event: Event, handler: Handler }` (`sema\lib.rs`): `Event::Timer { number:
  Option<Expr>, seconds: Expr }`, `Event::Key { key: Expr }`; `Handler::Label(LabelId)` (of the main module),
  `Handler::Sub { proc: ProcId, arg: Option<Expr> }`.
- `Checker::key_stmt(stmt)`: `StmtKind::Key(KeyForm::On | Off | List | Assign { n, text })`, `n` to `LONG`, `text` a
  string (an error otherwise, measured in 1.1).
- `builtins.rs`: rows `("TIMER", StmtRule::Plain)`, `("KEY", StmtRule::Plain)` in `STATEMENTS`, `("_FREETIMER",
  Rule::Plain)` in `FUNCTIONS`. `STRIG` gets no row and stays marked by `builtin_stmt`'s "not supported yet".

### D3. IR: `Op::OnEvent` and `Op::Key`

`ir\lib.rs`: `Op::OnEvent { event: EventOp, handler: HandlerOp }` with the operands as `Value`s
(`Timer { number: Option<Value>, seconds: Value }`, `Key { key: Value }`) and `HandlerOp::Label(LabelId)` (an index
into `p.main.labels`) or `HandlerOp::Sub { proc, arg: Option<Value> }`; `Op::Key(KeyOp)` for `KEY`'s own forms.
Both raise (libqb's error 5), so the pending-error rule applies as to a built-in call. `validate.rs`: the label of
`OnEvent` is checked against the main module's labels from any body (the one exception to `own_label`), the
procedure's one parameter against the argument. `lower.rs` and `dump.rs` gain the two ops.

*Alternative considered*: lowering a GOSUB handler to a `Gosub` op in a synthetic entry block. Rejected: the entry
is `qbx.cpp`'s re-entry of `QBMAIN`, not a jump of the program, and only the code generator writes it.

### D4. Code generation: a module `codegen-cpp\src\events.rs`

- The `Emitter` keeps a counter per event (`ontimer_ids`, `onkey_ids`) and two strings per event (cases,
  jumpers). Bodies are emitted main first, then procedures in definition order, which is the old compiler's source
  order, so the ids are the same.
- At the statement: `ontimer_setup(<n or 0>,<seconds>,<id>,<arg or 0>);` and `onkey_setup(<k>,<id>,<arg or 0>);`
  with the old compiler's spelling of each operand.
- Cases: for a label, the eight lines the old compiler writes (`timer_event_occurred++; timer_event_id=<id>;
  timer_event_occurred++; return_point[next_return_point++]=0; … QBMAIN(NULL); break;`) and the jumper `if(timer_
  event_id==<id>)goto LABEL_<X>;`; for a SUB, `case <id>:` and `<proc>((<ctype>*)&i64);` or `<proc>();`, `break;`.
- `lib.rs` `emit` sets `ontimer.txt`, `ontimerj.txt`, `onkey.txt`, `onkeyj.txt` from them; `Op::Key` is
  `key_on();`, `key_off();`, `key_list();`, `key_assign(<n>,<text>);`.

### D5. Tests

- **Measurements** (task 1.1): a family `verification\v42_*` recorded with the runner's `--suite verification`:
  the 16 compile checks of the proposal and what the spec needs at run time (timers, `STOP`/`OFF`/`FREE`,
  `_FREETIMER` reuse, error 5 cases, a SUB handler and its argument, a handler set up in a SUB, `ON KEY(1.7)`,
  `KEY` in `$CONSOLE:ONLY`, `BYVAL`), each with its output.
- **Front end**: `tests\frontend\events_errors.bas` (`check-fail`, every rejected form), `events_ok`, `events_ir`,
  `events_cpp`.
- **Call-site**: one program per form of `tests\callsite\` (`TIMER` switches with and without a number, `KEY(k)`
  switches, `_FREETIMER`, `KEY ON|OFF|LIST`, `KEY n, s$`, `ON TIMER` and `ON KEY` with a label and with a SUB);
  `tools\callsite\callsite.py` compares the setup call; if the fragments need a rule, it goes into its `README.md`.
- **Corpus**: a console slice program of timers (`s134_events_timer`; short intervals and a margin of at least
  0.3 s around each expected count, so load does not change the output), and a screen program `k01_on_key` whose
  keys file presses `f1`, an arrow and `a` for a user key, with `KEY LIST` and `KEY ON` on the screen.
- **Upstream**: the four `timer\` programs into `pass.list`.

## Risks / Trade-offs

- [Timing in the slice and upstream programs under `--jobs 8`] → intervals and margins as in D5; a program that
  fails once under load and passes alone is recorded as such, as `midisoundbank_file_test` is.
- [Re-entering `QBMAIN` jumps over everything in `main.txt` to the label, which a C++ compiler rejects if it
  crosses an initialised local] → a `GOTO` from the first statement to the same label already compiles; the
  `events_cpp` and slice tests put the handler label after loops, `SELECT` and error handling.
- [Keys in the screen program: the tool presses and releases at once and has no modifiers] → user keys with flags
  0 only; the flags are measured at compile level by the call-site programs.
- [The old compiler's faults we keep: `ON KEY`'s key truncated by C++, the SUB's argument passed as a pointer to an
  `_INTEGER64` cast to a narrower type] → kept its way (no memory misuse on little-endian, same values) and listed
  in `SOMEDAY.md`'s "QB64pe behaviours to review" if the measurements confirm them.
