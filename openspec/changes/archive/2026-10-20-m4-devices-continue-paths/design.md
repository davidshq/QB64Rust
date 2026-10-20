# Design

## Context

Motivation, scope and what was found before the proposal: `proposal.md`; the behaviour: the delta specs. Where
the four parts stand in this compiler (2026-10-20):

- **`_CONTINUE`**: no node. The parser reads it as a `CallStmt` of the name `_CONTINUE`, which `sema` marks "not
  supported yet". `EXIT FOR|DO|WHILE` is the model: `syntax\parser\proc.rs` `exit_stmt` checks with
  `Parser::inside(Block)` (`parser\blocks.rs`) that such a block is open in the current procedure, looking
  through other blocks; `sema\check\blocks.rs` `exit_loop` pushes `StmtKind::ExitLoop(kind)`; `ir\lower.rs` keeps
  the open loops in `exits: Vec<(LoopKind, LabelId)>` and lowers an exit to `Op::Jump` to the innermost loop's exit
  label. `loop_block` and `for_loop` emit each loop's bottom statement (the `LOOP` test or jump back, the `WEND`
  jump back, the `FOR` step) with `self.emit(end_at, end_line, …)`.
- **Blanks around `.`**: `syntax\lexer.rs` `is_member_dot` makes the dot after a plain name with blanks around it
  a `Dot`. `parser\assign.rs` `dotted_target` and the expression parser then build a `FieldExpr` whose base is a
  `NameRef`. In `sema\check\places.rs` `field_member`, that base arm returns "blanks around `.` after a name".
  A name written without blanks (`a.s`) is one `Ident`, which goes through `split_name` (upper case, suffix) and
  `dotted_any(t, name, suffix)`: member access when `a` is a scalar `TYPE` variable, an error for a `TYPE` array,
  else `None`, after which the caller takes it as a plain variable of that name. `dotted_any` takes the name as a
  string apart from the token, which is used only for spans and the text shown.
- **Relative names**: `sema\check\mod.rs` (the `$EXEICON` metacommand, about line 1370) and `check\embed.rs`
  `embed_meta` mark a name "not supported yet" when it neither starts with `./`/`.\` nor is absolute. The driver
  resolves the rest (`driver\src\lib.rs` `icon_path`, `embedded_path`, `beside`): a `./` name beside the main
  file, any other one as written, which until now meant absolute. `check_exe_icon` and `check_embedded_files`
  make the file checks `sema` cannot make. `check_program` (`driver\src\lib.rs`) runs them for the CLI and the
  language server alike (`check_for_server`). The compiler root is `driver\src\main.rs` `include_root` (the
  executable's folder or `--include-root`); for the server it is the client's setting or the executable's folder
  (`main.rs` about line 186). It is handed to `FileLoader` for included files.
- **Device input**: the 13 table rows (`tools\builtins\builtins.json`, dependency `DEPENDENCY_DEVICEINPUT`) are
  in no list of `sema\builtins.rs`, so they are "not supported yet". `RuntimePart::of_dependency`
  (`sema\src\lib.rs`) maps a row's dependency to a runtime part; `driver\src\build.rs` `make_flags` turns the parts
  into the `Makefile`'s flags in the old compiler's order. `check\events.rs` `on_event` returns "`ON STRIG` (device
  input)" before reading operands. `codegen-cpp\src\events.rs` `on_event` writes the setup, the `case` and the
  jumper for `timer` and `key`; `lib.rs` already writes `onstrig.txt` and `onstrigj.txt` empty. `qbx.cpp`
  has `onstrig_setup(int32 i, int32 controller, int32 controller_passed, uint32 id, int64 pass)` (line 1137), and
  `libqb.cpp` has the device code under `#ifdef DEPENDENCY_DEVICEINPUT`.

## Goals / Non-Goals

**Goals:**

- The four lines of the demand report gone, with the old compiler's checks and calls, so that the call-site check
  and the runner judge them.
- One reading of a name with blanks around a dot: the same function as the name without them.

**Non-Goals:**

- Declarations with blanks around a dot (`DIM a . b AS LONG`, `SUB a . b`): measured in `v47`, both accepted by
  the old compiler (`v47_x17`, `x18`). The parser does not read them (a parse error), so they stay as they are,
  on `tests\verification\known.list` and with a `SOMEDAY.md` row. A plain dotted array element written with blanks
  whose array is not declared (`p . q(1)` used without `DIM`) is "not supported yet" (task 3.1).
- A drive with no slash after it (`C:icon.ico`) for `$EXEICON` and `$EMBED`: still "not supported yet".
- Runtime tests that need a game controller. Tests run with none attached; the call-site check holds the calls.

## Decisions

### D1. `_CONTINUE` is a statement of its own, checked by the parser like `EXIT`

The parser reads `_CONTINUE` at the start of a statement, when nothing but the statement's end follows, as a new
node `ContinueStmt` (`parser\proc.rs` beside `exit_stmt`, dispatched in `parser\mod.rs` beside `EXIT`). A new
`Parser::inside_loop()` (`parser\blocks.rs`) looks back through the open blocks, as `inside` does, for any of
`Block::For`, `Block::Do`, `Block::While`, stopping at `Block::Proc`. With none open, the error is "`_CONTINUE`
outside a `DO`, `FOR` or `WHILE` loop" (the old compiler's wording follows as a note, from the recorded output,
`tools\diagnostics\old_wording.py`). `_CONTINUE` with anything after it stays a call of an unknown name, which is
what the old compiler does (it handles the word only when it is the whole statement, `n = 1`). `v47` measures the
error.

`sema\check\blocks.rs` gets `continue_loop`, which pushes `StmtKind::Continue`. Unlike `ExitLoop` it carries no
kind: the innermost loop of any kind is the target, which only the lowering knows. In `ir\lower.rs`, `exits`
becomes `Vec<(LoopKind, LabelId, LabelId)>`, with the continue label made beside the exit label. `loop_block`
places that label just before the bottom statement, and `for_loop` just before the step statement, the old
compiler's places (`dl_continue_`, `ww_continue_`, `fornext_continue_`). `StmtKind::Continue` lowers to
`Op::Jump` to the innermost entry's continue label. It is the same op an exit uses, with the same behaviour while
an error is pending.

*Alternative*: lowering `_CONTINUE` in `sema` to a jump to a made-up label. Rejected: labels of the typed tree are
the program's own, and `exits` already holds the open loops.

### D2. A member access on a plain name is the dotted name it spells

In `field_member`, the `NameRef` arm no longer returns "not supported yet". A new
`Checker::spelled_name(f: ast::FieldExpr) -> R<Option<(String, Option<Ty>, Span)>>` (`check\places.rs`) walks the
chain of `FieldExpr`s down to its `NameRef` base and joins the parts with `.`: the base's name and each member's
name, upper case, the blanks dropped. It takes the suffix from the last member, as `split_name` does for one
token, so `a . s` gives `("A.S", None)` exactly as `a.s` does. It returns `None` when the chain's base is not a
`NameRef`: `a(1) . s` keeps the path it has today.

Each place that reads a dotted `Ident` then gets the same reading for such a `FieldExpr`:

- `field_member` and `field_place` call `dotted_any` with the spelled name. When that gives `None`, the name is
  a plain variable named `A.S`, and they read it through the plain-variable path, as a `NameRef` of that name is
  read. The variable lookup takes the name as a string and the span for messages.
- `member_array` (`ERASE`, `LBOUND`/`UBOUND`, array arguments) gets that through `field_member`.
- An element `a . s(1)` is a `FieldExpr` with an argument list, so `member_element` handles it as it does for
  `a(1).s(2)`.

The span of the whole `a . s` is the one recorded for the name (`stmt.names`), so the language server's symbol is
the text as written.

*Alternative*: joining in the lexer, one `Ident` token with the blanks inside. Rejected: the token's text would
hold blanks that every reader of a name (`split_name`, `name_end`, the formatter, the language server) would have
to drop, and the lexer cannot tell `PRINT . x` (a keyword) from a name.

### D3. Relative names resolve against the compiler root

`sema` stops marking these names: the `$EXEICON` check in `check\mod.rs` and `embed_meta` mark only a drive with
no slash after it (`[letter, ':', c, ..]` with `c` not a slash), with the message "a `$EXEICON` name with a drive
and no slash after it" (or `$EMBED`'s). Everything else is accepted, and `sema` still reads no file.

The driver's `icon_path` and `embedded_path` get the compiler root as a parameter: a `./` name stays beside the
main file; an absolute name stays as written; any other name is `root.join(name)`. On Windows, joining a name with
one leading slash onto the root gives the root of the root's drive, which is the old compiler's reading (its
working directory is its folder). `check_exe_icon`, `check_embedded_files`, `used_embedded_files` and
`exe_icon_path` take the root, and so do `check_program` and `check_for_server`. Their callers (`main.rs` for a
compile and for the server, `report.rs`, the driver's tests) pass the root they already give `FileLoader`. The messages are
unchanged ("the file `…` of `$EXEICON` is not found", naming the file as written; D-015). The old compiler names
the file part alone ("File 'icon.ico' not found"); `old_wording.py` pairs the note from the recordings.

`tests\upstream\root\` is "where the old compiler's own folder is" for the upstream tests. `copy_upstream_tests.py`
also mirrors there the three files that upstream programs name relative to the compiler's folder:
`tests/compile_tests/icon/qb64pe.ico`, `tests/compile_tests/winresource/qb64pe.ico` and
`tests/compile_tests/embed/test.output`. It takes them from a list in the script, with a test that each one is
named by an upstream program. `SOURCE.md` names them; the two `.ico` files are the only binary files in the copy,
from the same MIT tree. A new `DECISIONS.md` row replaces answer (2) of 2026-10-13.

*Alternative*: give the runner `--include-root <clone>` for upstream tests. Rejected: `tests\upstream\root\`
is the root everything else uses (the demand report, tier 1), and the clone's whole tree as root would let
included files resolve that do not resolve from the copy.

### D4. Device input: a runtime part and plain built-ins

`sema\src\lib.rs` gets `RuntimePart::DeviceInput`, mapped from `DEPENDENCY_DEVICEINPUT`; `ir` mirrors it
(`ir\src\lib.rs`, `lower.rs`, `dump.rs` "deviceinput"). `make_flags` pushes `DEP_DEVICEINPUT=y` after `DEP_FONT` and
before `DEP_ZLIB` (`qb64pe.bas` 13714). The `Makefile` links `game_controller.a`, the Windows libraries and the
licence by itself.

`sema\builtins.rs` gets the 13 rows as `Rule::Plain` (the functions) and `StmtRule::Plain` (`STRIG` the statement),
in a block with a comment naming this design and `v47`. That is how `_MOUSEX` and the `TIMER` switch are done: the
table's templates (`?[,?]`, `[?]`, `[(?[,?])]{On|Off|Stop}`) give the forms, the "passed" flags and the LONG
conversion. Then nothing is special to device input except the runtime part, which a used row sets through its
dependency, as it does for every other part.

`ON STRIG` (`check\events.rs` `on_event`): the `"STRIG"` arm reads one or two operands as `ON KEY` reads one,
each converted to LONG (`evaluatetotyp(e$, 32&)`, `qb64pe.bas` 7580–7590), into a new `Event::Strig { button:
Expr, controller: Option<Expr> }` (`sema\src\lib.rs`, mirrored in `ir`). Errors follow the old compiler's
order (`v47` measures them): "Expected (", "Expected ... ,", "Expected )", "Expected ... )". The handler is read as
for the other events. The statement sets `RuntimePart::DeviceInput` (the old compiler's `SetDependency`, 7535).

`codegen-cpp\src\events.rs` `on_event` gets a third word, `strig`, with its own `EventCode` (`on_strig`) written
to `onstrig.txt` and `onstrigj.txt` (`lib.rs`), and the setup `onstrig_setup(i,controller,passed,id,pass);`, where
`controller` is `0` and `passed` is `0` when the controller is left out (7566–7593). The label case writes
`strig_event_occurred++` twice, as the old compiler does and as the timer and key cases already do.

### D5. Tests

- **`v47`** (`verification\`, recorded with `qb64pe.exe` before the code; one line in `verification\README.md`):
  `v47_a_continue` (each loop kind, through `IF` and `SELECT`, in a SUB, `LOOP WHILE`/`UNTIL` with the test still
  run), `v47_x*` for each rejected form (`_CONTINUE` outside a loop, in a SUB around whose call a loop stands,
  `_CONTINUE 1`, `ON STRIG` without operands, with three, the SUB handler errors), `v47_b_devices` (with no
  game controller: `_DEVICES > 1`, `_DEVICE$(1)`, `_DEVICE$(2)`, `_LASTBUTTON(1)`, `_LASTAXIS(2)`, `STICK(0)`,
  `STRIG(1)`, the error 5 cases), `v47_c_dot_blanks` (`a . s(2)`, `LBOUND(a . s)`, `a . b . c`, a by-reference
  argument, `DIM a . b AS LONG`), and `v47_e*` for relative icon and embed names under the old compiler's folder.
  Each `v47_e*` program names a file the runner puts in the clone's root for the recording and removes after: a
  sidecar `<name>.root` lists it, and the compare puts it under a folder of its own given as `--include-root`
  (`tools\legacy_tests\README.md`; found while recording, task 1.1). Measured there too: an odd button number of
  `ON STRIG` or the `STRIG` switch is error 5 at run time in libqb (`v47_x13`, `x27`), `_CONTINUE 1` is "Command
  not implemented", `s$ . x` is rejected, and `DIM a . b AS LONG` and `SUB a . b` are accepted (`x17`, `x18`).
  Rule 12 holds for `v47_b`: it prints only what does not depend on the machine (the keyboard and the mouse,
  never a count of controllers).
- **Front-end tests** (`tests\frontend\`): `continue_*` (errors, IR with the continue labels), `strig_*` (errors,
  C++ with the setup and dispatch), `dot_blanks_*` (typed tree, the same as without blanks).
- **Call-site programs** (`tests\callsite\`): one per form and argument type of the 13 rows, and `event_strig_*`
  for `ON STRIG`.
- **Slice programs** (`tests\corpus\slice\`): `s147_continue`, `s148_dot_blanks`, `s149_devices_console`
  (`$CONSOLE:ONLY`, what `v47_b` prints), recorded with the old compiler.
- **Upstream**: the 8 programs onto `tests\upstream\pass.list`. `qb64pe/element_getStringValue` keeps its other
  errors.
- **Verification**: `v19_dot_blanks_*`, `v29_e02`, `e03`, `e08`, `e18`, `v44_x26` and the new `v47` programs onto
  `tests\verification\pass.list`, off `known.list`.
- **The make flags**: a unit test in `driver\src\build.rs` beside the others, `DEP_DEVICEINPUT=y` between
  `DEP_FONT=y` and `DEP_ZLIB=y`, checked once against the old compiler's build of `console_only/devices`
  (`-z`, its `makedeps$`).

## Risks / Trade-offs

- [The device-input libqb variant fails to build or link with the clone's toolchain (`-lxinput -ldinput8 …`)] →
  the old compiler builds `deviceinput/test` with the same `Makefile` and toolchain, so a failure is ours. It is
  found in task 1 by building `console_only/devices`, before any other work on it.
- [`_DEVICE$` and `_LASTAXIS` of the keyboard and mouse differ between machines] → `v47_b` records them on this
  machine and `s149` prints only those. If a CI runner differs, the screen-free CI step shows it, and the program
  then prints only `_DEVICES > 1` (rule 12).
- [Joining names in `sema` (D2) gives a variable whose name spans several tokens] → the span is the whole
  expression, which the language server's rename and hover already treat as one symbol occurrence. Front-end tests
  pin the typed tree.
- [A relative name that resolved nowhere before now resolves against the executable's folder for a user who
  installs `qb64rust` beside other files] → this is what the old compiler does with its folder, and the spec says
  so.

## Open Questions

None.
