# Proposal

## Why

The user asked (2026-10-20) for the first four lines of the upstream demand report to be built as one change:
`_DEVICES`, a `$EXEICON` name that neither starts with `./` nor is absolute, blanks around `.` after a name, and
`_CONTINUE`. They block 2 programs each; 7 programs are held back by one of them alone, and `embed/test` is held
back by the `$EMBED` form of the same relative name, 8 in all. `qb64pe/element_getStringValue` uses `_CONTINUE`
too but has other errors as well. `ROADMAP.md` lists all four under M4. Each one is small on its own
except device input, which brings with it the family the old compiler builds into the same libqb part.

Found before this proposal, from `qb64pe.bas`, `subs_functions.bas`, libqb and the recordings already in
`verification\`:

- **`_CONTINUE`** (`qb64pe.bas` 9734–9756) stands alone on its statement. It jumps to just before the bottom of the
  innermost open `FOR`, `DO` or `WHILE` loop of the current body, whatever other blocks (`IF`, `SELECT`) lie
  between: before the step at `NEXT`, before the test of `LOOP WHILE`/`UNTIL` or the jump back of a plain `LOOP`,
  and before the jump back at `WEND` (labels `fornext_continue_`, `dl_continue_`, `ww_continue_`, 6246, 6315,
  6398, 6405). Outside a loop it is "_CONTINUE outside DO..LOOP/FOR..NEXT/WHILE..WEND block". The parser reads it
  as a call of an unknown name today, which `sema` marks "not supported yet". Upstream: `colors/color_with_dest`.
- **Blanks around `.`**: the old compiler drops them, so `a . s` is `a.s`. That holds for a member of a `TYPE`
  variable, for a plain dotted name (with no `TYPE`), and in `ERASE a . S` (`verification\v19_dot_blanks_erase`,
  `_plain`, `_type`, all recorded and on `tests\verification\known.list` as "not supported yet"). The lexer and
  parser already read `a . s` as a member access whose base is a plain name (`lexer.rs` `is_member_dot`,
  `parser\assign.rs` `dotted_target`). Only `sema` stops there (`check\places.rs` `field_member`). Upstream:
  `arrays/04_float_family`, `arrays/08_whole_member_array_memops_numeric` (both `ERASE a . S`).
- **`$EXEICON` and `$EMBED` names that neither start with `./` nor are absolute.** The old compiler looks for
  these in its working directory, which is its own folder: it changes to that folder at start (`qb64pe.bas`
  3755–3777, 3843–3873). On 2026-10-13 (`DECISIONS.md`, `m3-sound` D4) this was left "not supported yet" because
  it "has no counterpart here". Since then the compiler root has become that counterpart for included files (spec
  `compiler/cli`, "Compiler root for included files": the executable's folder, or `--include-root`). The same root
  serves for these names. The recordings `verification\v29_e02`, `e03`, `e08`, `e18` and `v44_x26` (each "File …
  not found" in the old compiler) are on `known.list` as "not supported yet". Upstream: `icon/test`,
  `winresource/test` (`$EXEICON:'tests/compile_tests/…/qb64pe.ico'`) and `embed/test`; their files lie under the
  clone's root, which `tests\upstream\root\` mirrors for included files only.
- **Device input** is libqb's `DEPENDENCY_DEVICEINPUT` part: the `Makefile`'s `DEP_DEVICEINPUT=y`, a libqb variant
  of its own, `game_controller.a`, the Windows input libraries and the licence `libstem_gamepad`. The old compiler
  puts that flag after `DEP_FONT` and before `DEP_ZLIB` (`qb64pe.bas` 13714). The table's 13 rows that need it are
  `STICK`, `STRIG()`, the statement `STRIG[(…)] ON|OFF|STOP`, `_DEVICES`, `_DEVICE$`, `_DEVICEINPUT`,
  `_LASTBUTTON`, `_LASTAXIS`, `_LASTWHEEL`, `_BUTTON`, `_BUTTONCHANGE`, `_AXIS` and `_WHEEL`. `ON STRIG(i[, c])`
  sets the same part and writes `onstrig_setup(…)` with a dispatch in `onstrig.txt` and `onstrigj.txt`
  (`qb64pe.bas` 7534–7700), as `ON KEY` does with `onkey.txt` (`m3-events`). The code generator already writes both
  files empty. On 2026-10-17 (`DECISIONS.md`, `m3-events`) the user left `STRIG` events for this device-input
  change. Upstream: `console_only/devices`, `deviceinput/test` (`PRINT _DEVICES > 1`, -1: the keyboard and the
  mouse are always devices).

## What Changes

- **`_CONTINUE`** in `FOR`, `DO` and `WHILE` loops of any body, through nested `IF` and `SELECT` blocks, with the
  old compiler's error outside a loop.
- **`a . s` read as `a.s`** wherever a member access whose base is a plain name stands: a value, an assignment's
  target, an argument by reference, `ERASE`, `LBOUND`/`UBOUND`, and a member array's element (`a . s(1)`). It is a
  member when `a` is a `TYPE` variable and a plain dotted name otherwise, exactly as `a.s` is.
- **Relative `$EXEICON` and `$EMBED` names** looked up relative to the compiler root, as an included file is when it
  is not beside the file that includes it. A name with a single leading slash goes to the root of the compiler
  root's drive (the old compiler's own drive). A name with a drive and no slash after it (`C:game.ico`) stays
  "not supported yet". The decision of 2026-10-13 (2) is replaced by a new `DECISIONS.md` row.
- **Device input**: the 13 built-ins of the table, `ON STRIG(…) GOSUB label` and `ON STRIG(…) sub [argument]`,
  with the old compiler's checks and calls, and libqb's device-input part linked when a program uses any of them.
  The `STRIG` row of `language/events` is replaced.
- **Tests**: a measurement family `v47` in `verification\` (the `_CONTINUE` forms and errors, `ON STRIG`'s forms
  and errors, the device functions' errors and what they return with no gamepad, the relative names, and
  declarations with blanks around a dot). Also: front-end tests, call-site programs for every new built-in form,
  and slice programs (`_CONTINUE`, blanks around a dot, device input in a console). The 8 upstream programs go
  onto `tests\upstream\pass.list`, and the `v19_dot_blanks_*`, `v29_e*` and `v44_x26` recordings move from
  `known.list` to `pass.list`. `tests\upstream\root\` also mirrors the three files that upstream programs name
  relative to the compiler's folder.

## Capabilities

### New Capabilities

- `language/device-input`: game controllers, the keyboard and the mouse as devices: `_DEVICES`, `_DEVICE$`,
  `_DEVICEINPUT`, `_LASTBUTTON`, `_LASTAXIS`, `_LASTWHEEL`, `_BUTTON`, `_BUTTONCHANGE`, `_AXIS`, `_WHEEL`, `STICK`
  and `STRIG()`, their errors, and the runtime part they link.

### Modified Capabilities

- `language/control-flow`: a requirement for `_CONTINUE`.
- `language/events`: `ON STRIG` handlers and the `STRIG` switch are compiled; "Events the old compiler does not
  have" no longer keeps them marked.
- `language/arrays-and-types`: "Member access and dotted names" covers blanks around the dot.
- `language/console-and-window`: "$EXEICON" and "Errors of $EXEICON": a relative name is looked up under the
  compiler root.
- `language/embedded-files`: "Where the file is": the same for `$EMBED`.
- `compiler/cli`: the compiler root also serves the names of `$EXEICON` and `$EMBED`.

## Impact

- `syntax`: `_CONTINUE` as a statement of its own, with the open-loop check (`parser\proc.rs` beside `EXIT`,
  `parser\blocks.rs`).
- `sema`: `StmtKind::Continue`; the dotted name made from a member access on a plain name
  (`check\places.rs`); the relative-name checks in `check\mod.rs` and `check\embed.rs`; `RuntimePart::DeviceInput`
  and the 13 rows in `builtins.rs`; `Event::Strig` in `check\events.rs`.
- `ir`: a continue label in each loop (`lower.rs` `loop_block`, `for_loop`); `Event::Strig`;
  `RuntimePart::DeviceInput`.
- `codegen-cpp`: `onstrig_setup` and the `onstrig.txt`/`onstrigj.txt` dispatch (`events.rs`).
- `driver`: `DEP_DEVICEINPUT=y` in `make_flags`; the icon and embed paths under the compiler root (`lib.rs`
  `icon_path`, `embedded_path`).
- `tools\upstream\copy_upstream_tests.py`, `tests\upstream\root\` and `SOURCE.md`: the three files named relative to
  the compiler's folder (two `.ico`, one `.output`).
- No new dependency. A new libqb variant (`DEP_DEVICEINPUT`) is built on first use.
