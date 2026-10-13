# Design

## Context

Motivation and scope: `proposal.md`. What the code does today (read 2026-10-12):

- `sema` (`check\mod.rs`, after the walk) gives a program with no console one mark, "programs without `$CONSOLE`
  or `$CONSOLE:ONLY`". Nothing else depends on it: `Settings` already has `console`, `window` and `window_hidden`
  as separate facts, `ir::validate` wants a console or a window, the emitter writes the two start-up globals from
  them, and `driver\build.rs` `make_flags` gives `DEP_CONSOLE=y` only when `console` is set. A program with
  `$CONSOLE` and a window is built and run since `m2-console-screenhide` (`s63_console_window`), so the windowed
  libqb variant exists in the clone and in CI.
- The generated code does not depend on the console mode (design of `m2-console-screenhide`, Context: `PRINT`,
  `INPUT`, `END` and errors are the same calls in both modes; what differs at run time is libqb's). Read from the
  source then, not yet measured for a program with no console at all.
- A built-in statement that is one call of the runtime is a row of `sema\builtins.rs` `STATEMENTS` with
  `StmtRule::Plain`: its slots come from the table entry and its `specialformat` template
  (`qb64rust_builtins::passing::plan`, a port of the old compiler's `seperateargs` pass rules), the parser reads
  the statement by the same template, and the emitter writes the call with the `passed` mask. Templates with
  parenthesised pairs, a `-` between them and optional choices are compiled already (`_MOUSECURSOR ?[,(?,?)]`,
  `_WINDOWSIZELIMIT [(?,?)][-(?,?)]`, `_MOUSEHIDE [{_Hide|_Disable}]`).
- The table has a row for every built-in of this change (`tools\builtins\builtins.json`):

  | Built-in | Call | Template | Table types |
  |---|---|---|---|
  | `SCREEN` | `qbg_screen` | `[?][,[?][,[?][,[?][,[{_ManualDisplay}?]]]]]` | 5 × LONG |
  | `CLS` | `sub_clsDest` | `[?][,[?][,?]]` | LONG, `_UNSIGNED LONG`, LONG |
  | `COLOR` | `qbg_sub_color` | `[?][,[?][,[?][,?]]]` | 4 × LONG |
  | `LOCATE` | `qbg_sub_locate` | `[?][,[?][,[?][,[?][,?]]]]` | 5 × LONG |
  | `WIDTH` | `qbsub_width` | `[{#\|LPrint}][?][,[?][,[?][,[?]]]]` | 5 × LONG |
  | `VIEW PRINT` | `qbg_sub_view_print` | `{Print}[?{To}?]` | LONG, LONG (+ mask) |
  | `VIEW` | `qbg_sub_view` | `[[{Screen}](?,?)-(?,?)[,[?][,?]]]` | 6 × LONG |
  | `WINDOW` | `qbg_sub_window` | `[[{Screen}](?,?)-(?,?)]` | 4 × `_FLOAT` |
  | `PSET`, `PRESET` | `sub_pset`, `sub_preset` | `[{Step}](?,?)[,?]` | `_FLOAT`, `_FLOAT`, LONG |
  | `LINE` | `sub_line` | `[[{Step}](?,?)]-[{Step}](?,?)[,[?][,[{B\|BF}][,?]]]` | 4 × `_FLOAT`, 3 × LONG |
  | `CIRCLE` | `sub_circle` | `[{Step}](?,?),?[,[?][,[?][,[?][,?]]]]` | 3 × `_FLOAT`, `_UNSIGNED LONG`, 3 × `_FLOAT` |
  | `PAINT` | `sub_paint` | `[{Step}](?,?)[,[?][,[?][,?]]]` | 2 × `_FLOAT`, 2 × `_UNSIGNED LONG`, STRING |
  | `DRAW` | `sub_draw` | one argument | STRING |
  | `PALETTE` | `qbg_palette` | `[?,?]` | 2 × LONG |
  | `PCOPY` | `sub_pcopy` | two arguments | 2 × LONG |
  | `_LIMIT`, `_DELAY` | `sub__limit`, `sub__delay` | one argument | DOUBLE |
  | `SLEEP` | `sub_sleep` | `[?]` | LONG |
  | `_DISPLAY` | `sub__display` | none | |
  | `INKEY$` | `qbs_inkey` | none | returns STRING |
  | `CSRLIN` | `func_csrlin` | none | returns LONG |
  | `POS` | `func_pos` | one argument | LONG; returns LONG |
  | `POINT` | `func_point` | `?[,?]` | 2 × SINGLE; returns DOUBLE |
  | `SCREEN` | `func_screen` | `?,?[,?]` | 3 × LONG; returns `_UNSIGNED LONG` |

  The table is the old compiler's registration, not what it writes: `m2-core-builtins` found a third of the
  functions special-cased, and `PAINT`'s third slot is a colour or a tile string by the argument's type. Task 1.1
  reads the C++ of every form before a row is added.
- `Slot::from_table` knows LONG, DOUBLE, `_FLOAT`, any-numeric and STRING; a statement slot typed `_UNSIGNED LONG`
  or SINGLE is new. `table_ty` knows no `_UNSIGNED LONG` return type.
- The corpus runner builds a program that has a `<name>.screen` with `tools\screen\screen_state.cpp`, runs it
  with the state and keys variables and compares the state, for either compiler; it takes `--list`. 29 programs
  are recorded in `tests\corpus\screen` (their `.output` is empty). No screen program is on `slice.list`.
- Tier 1 fails for a corpus or upstream program that compiles cleanly and is on no pass list or in
  `tests\known_clean_not_passing.list` (spec `testing/compiler-tests`, "Clean programs are listed"). Upstream has
  47 programs without `$CONSOLE:ONLY`; none has a recorded screen (design D8 of `m2-screen-state-check`: 27 never
  run, 10 are console tests, 10 leave an empty text screen).

Constraints: nothing copied from other projects; the reference clone is not modified; libqb is not changed, so
everything the runtime does is the old compiler's by construction and the work is the call; no divergence without
a decision; a behaviour that looks questionable is implemented QB64pe's way and listed in `SOMEDAY.md`.

## Goals / Non-Goals

**Goals:**

- A program whose output goes to the window builds and runs as the old compiler's.
- Every built-in of the proposal comes in the standard way: a row, a call-site program per form and argument
  type, a slice line; here the slice line is a screen program, checked by its state.
- The screen programs of `tests\corpus\screen` in reach of these built-ins pass: at most 27 of the 29 (`g03`
  names `PLAY`, `g06` graphics `GET` and `PUT`). An upper bound: each must leave the recorded state.
- The old compiler's accept/reject line for every form of each template.

**Non-Goals:**

- A second implementation of anything libqb draws. The compiler writes calls; pixels are libqb's.
- Checking what a window shows beyond the state file (title, size, timing, sound).
- Making Civil War Strategy build: it also needs `DEF SEG`, `BLOAD`, `TAB`, graphics `GET`/`PUT`, sound and
  labels named like a SUB (the next change of the proposed order).
- A screen state for the upstream programs without `$CONSOLE:ONLY`.

## Decisions

### D1. A program with no console is compiled; the mark goes, nothing takes its place

The mark in `check\mod.rs` is removed. `Settings` then says `console: false`, `window: true`, and the existing
emitter and driver code gives the globals and the make flags of that case. `_ECHO`'s error for a program with no
console stays where it is.

Task 1.2 measures what was only read before: for a program of `PRINT "x"`, `SYSTEM` and for one with
`$SCREENHIDE` alone, the old compiler's two globals, the libqb variant and the link flags in the clone's
`internal\temp\compilelog.txt`, as task 1.2 of `m2-console-screenhide` did for the four console cases; and that
the C++ of `PRINT`, `INPUT`, `END` and a statement after an error is the same text as under `$CONSOLE:ONLY`. If a
flag or a line differs, `make_flags` or the emitter follows it, with a unit test for the case; if the statement
code differs by mode, that is a finding that changes group 2 and is put to the user before code.

As measured (task 1.2, 2026-10-12; `qb64pe -x` of one program in three modes, outside the repository; the
clone's `git status --porcelain` empty before and after). The two rows that join the four of
`m2-console-screenhide` D3:

| Program | Make flags | libqb variant, link | `-D` that differs | Globals |
|---|---|---|---|---|
| no metacommand | none | `libqb_make_00000000`, `-mwindows`, `window-gui.o`, `main-thread-gui.o` | `DEPENDENCY_NO_ICON` | `console=0`, `screen_hide_startup=0` |
| `$SCREENHIDE` alone | none | as the row above | as the row above | `console=0`, `screen_hide_startup=1` |

The libqb variant is the one a `$CONSOLE` program with a window links, so nothing new is built; the link line
ends `-mwindows` where a program with a console has `-mconsole`, which the clone's `Makefile` chooses by
`DEP_CONSOLE`. `make_flags` gives no flag for `console: false, window: true` already: no change, and a unit test
for the two rows. **No statement's code differs by mode:** the fragments of the program (`PRINT "x"`, `PRINT 1;
"a", 2`, `INPUT l`, `INPUT "p"; t`, `LINE INPUT t`, `ERROR 5`, an assignment, `END`, `SYSTEM`) are the same text
with no metacommand, with `$SCREENHIDE` and with `$CONSOLE:ONLY`, but for `global.txt`'s two globals and the two
calls `$CONSOLE:ONLY` writes where it stands (`sub__dest(func__console()); sub__source(func__console());`). Group
2 is built as written.

*Alternative considered:* keep the mark for programs that never call a screen statement. Rejected for the reason
D1 of `m2-console-screenhide` gives: it needs flow analysis to be right, and the "clean programs are listed" rule
already makes every clean, unchecked program visible.

### D2. Each built-in is a `Plain` row unless task 1.1 shows the old compiler writes it another way

Task 1.1 compiles one program per form of each template with `qb64pe -z`, outside the repository, and notes for
each built-in: the call, the conversion of each argument (by passing an INTEGER, a LONG, an `_INTEGER64`, a
SINGLE, a DOUBLE and a literal), the `passed` mask, and for a function the type the generated code holds and the
type the old compiler believes (probed as in `m2-console-screenhide` D6: the result in `PRINT`, in `HEX$` and in
an operation). The findings replace the table above with one line per form, as D6 of that change did.

**As measured (task 1.1, 2026-10-12).** 455 one-line forms were compiled with `qb64pe -z` outside the
repository (every argument left out in turn, every choice, a variable of each of INTEGER, LONG, `_INTEGER64`,
SINGLE, DOUBLE, `_FLOAT`, `_UNSIGNED LONG`, `_BYTE` and a literal in one slot of each slot type). **Every
statement is the plain call of its table row and template**, `LINE` included, with one exception, `PAINT`'s
third argument. The accept line is the template's: a form the template does not give is "Syntax error -
Reference: …", a string in a number's slot "Nth sub argument requires a number" (and the other way round; "Number
required for sub" / "String required for sub" for a statement without a template).

How each slot type is passed (one conversion per slot type, the same in every row):

| Table type | An integer argument | A float argument | libqb's parameter | In `sema` |
|---|---|---|---|---|
| LONG | as it is (`*__INTEGER64_Q`, `4294967295ll`) | `qbr(…)` | `int32`, in some entries `uint32` (colours of `COLOR`, `LINE`, `PSET`, `PALETTE`) | `Slot::Long`, as before |
| `_UNSIGNED LONG` | as it is | `qbr(…)` | `uint32` | new `Slot::ULong`: a store into an `_UNSIGNED LONG` |
| `_FLOAT` | as it is | as it is (`*__SINGLE_S`, `1.5E+0`) | `float` (`sub_pset`, `sub_line`, `sub_paint`, `qbg_sub_window`), `double` (`sub_circle`) | `Slot::Float`, as before |
| SINGLE (`POINT`) | as it is | as it is | `float` | `Slot::Float`: passed in its own type, the table's name maps to it |
| DOUBLE (`_LIMIT`, `_DELAY`) | as it is | as it is | `double` | `Slot::Double`, as before |
| STRING | | | `qbs*` | `Slot::Str`, as before |

So the two "new slot types" are one new variant: `_UNSIGNED LONG` is written by the old compiler exactly as a
LONG slot is (C++ narrows the argument to the parameter; the low 32 bits either way), and it becomes a slot of
its own only so that the IR states the conversion to the type libqb takes. SINGLE needs no conversion at all.

One line per form (the mask is the last C argument; `N` stands for `NULL`):

| Built-in | Forms measured: call | Kind |
|---|---|---|
| `SCREEN` | `SCREEN m`: `qbg_screen(m,N,N,N,N,1)`; `, c` 2; `, , a` 4; `, , , v` 8; `, , , , _MANUALDISPLAY r` 16 (the word alone is no C argument); bare `SCREEN`, `SCREEN 12,` and `SCREEN 12, , , ,` accepted (mask 0 or 1); a fifth argument without the word is a syntax error | `Plain` |
| `CLS` | `sub_clsDest(method, colour, dest, mask)`, bits 1, 2, 4; `CLS`, `CLS ,` mask 0; four arguments a syntax error | `Plain`, slot 2 `_UNSIGNED LONG` |
| `COLOR` | `qbg_sub_color(fg, bg, border, dest, mask)`, bits 1, 2, 4, 8; `COLOR` alone and four arguments accepted | `Plain` |
| `LOCATE` | `qbg_sub_locate(row, col, cursor, start, stop, mask)`, bits 1 to 16; `LOCATE` alone accepted; six arguments and `LOCATE , , , ,` syntax errors | `Plain` |
| `WIDTH` | `qbsub_width(option, v1, v2, v3, v4, mask)`: `WIDTH 80, 50` is `(N,80,50,N,N,3)`; `WIDTH #1, 80` is `(1,1,80,N,N,3)` (the `#` is option 1 and the file number the first value); `WIDTH LPRINT 80` is `(2,80,N,N,N,1)`; `WIDTH` alone, `WIDTH #1`, `WIDTH LPRINT` accepted; five values a syntax error | `Plain`, every form |
| `VIEW PRINT` | `qbg_sub_view_print(N,N,0)`, `VIEW PRINT 2 TO 10`: `(2,10,1)`; `VIEW PRINT 2`, `2 TO`, `TO 10`, `2, 10` syntax errors | `Plain` |
| `VIEW` | `qbg_sub_view(x1,y1,x2,y2,fill,border,mask)`: the points 1, `SCREEN` 2, fill 4, border 8; `VIEW` alone 0; LONG slots, so a float coordinate goes through `qbr` | `Plain` |
| `WINDOW` | `qbg_sub_window(x1,y1,x2,y2,mask)`: the points 1, `SCREEN` 2; `WINDOW` alone 0 | `Plain` |
| `PSET`, `PRESET` | `sub_pset(x,y,colour,mask)`: `STEP` 1, colour 2; `PSET STEP (1, 1)` with a blank accepted | `Plain` |
| `LINE` | `sub_line(x1,y1,x2,y2,colour,box,style,mask)`: first point 1, second `STEP` 2, first `STEP` 4, colour 8, style 16; box `N`, 1 (`B`) or 2 (`BF`), in any letter case; `LINE -(3, 4)` is `(N,N,3,4,N,N,N,0)`; trailing commas after the second point and after the colour accepted, after the box word not; `LINE (1, 2), 3` and `LINE (1, 2)` syntax errors | `Plain` (the plan of `passing.rs` gives these bits already, its test `nested_blocks`) |
| `CIRCLE` | `sub_circle(x,y,r,colour,start,end,aspect,mask)`: `STEP` 1, colour 2, start 4, end 8, aspect 16 | `Plain`, slot 4 `_UNSIGNED LONG` |
| `PAINT` | `sub_paint(x,y,fill,border,tile,mask)`: `STEP` 1, fill 2, border 4, tile 8. **The fill is a number or a string**, and the old compiler writes a cast before it that picks libqb's overload: `(uint32)3`, `(uint32)qbr(…)` for a float, `(uint32)NULL` when left out, `(qbs*)__STRING_T` for a string. Border: a number only; tile: a string only | `Plain` with a slot kind of its own, `Slot::ColorOrTile` (a number, an integer as it is and a float rounded, or a string as it is; the emitter writes the cast) |
| `DRAW` | `sub_draw(s)` | `Plain` |
| `PALETTE` | `qbg_palette(N,N,0)`, `PALETTE 1, 2`: `(1,2,1)`; one argument a syntax error | `Plain` |
| `PCOPY` | `sub_pcopy(src,dst)` | `Plain` |
| `_LIMIT`, `_DELAY` | `sub__limit(x)`, `sub__delay(x)`, the argument as it is | `Plain` |
| `SLEEP` | `sub_sleep(N,0)`, `sub_sleep(n,1)` | `Plain` |
| `_DISPLAY` | `sub__display()`; with an argument "SUB does not require any arguments" | `Plain` |

Functions (the result: the type the C++ holds, the type the old compiler believes, as `PRINT`, `HEX$` and an
operation show it):

| Function | Call | Held | Believed | Kind |
|---|---|---|---|---|
| `INKEY$` | `qbs_inkey()`; bare only: `INKEY$()` is "Expected (...)", `INKEY$(1)` "Incorrect number of arguments" | STRING | STRING | `Rule::Plain` |
| `CSRLIN` | `func_csrlin()`; bare only, as `INKEY$` | `int32` | LONG (`qbs_str((int32)(…))`, `func_hex(…,8)`) | `Rule::Plain` |
| `POS(n)` | `func_pos(n)`, a LONG slot; bare or with two arguments "Incorrect number of arguments" | `int32` | LONG | `Rule::Plain` |
| `POINT` | `POINT(x, y)`: `func_point(x,y,0\|1)`; `POINT(n)`: `func_point(n,NULL,0)`; both arguments as they are (the SINGLE slot) | `double` | DOUBLE (`qbs_str((double)(…))`, `func_hex_float(…)`; a store into an integer rounds with `qbr`) | `Rule::Plain`: one optional slot, as `_NEWIMAGE` has |
| `SCREEN(r, c[, f])` | `func_screen(r,c,NULL,0)`, `func_screen(r,c,f,0\|1)`, LONG slots | `uint32` | `_UNSIGNED LONG` (`qbs_str((uint32)(…))`, `func_hex(…,8)`, `-SCREEN(…)` an `_INTEGER64`) | `Rule::Plain`; `table_ty` learns `_UNSIGNED LONG` |

What follows for the code:

- **No new `StmtRule`.** `LINE` is the generic call; `PAINT` differs in one slot, which is a slot kind
  (`Slot::ColorOrTile`), not a rule: its other slots and its mask are the template's.
- **Two entries of one name:** `VIEW` and `VIEW PRINT` are two table entries (told apart by the table's
  `secondargmustbe`/`secondargcantbe`); `stmt_lookup` returns every form of a name and the checker matches the
  statement against each, as for `OPEN`'s two. `SCREEN` is a statement and a function, told apart by position.
- **`WIDTH`'s `#` and `LPRINT` forms and `SCREEN`'s `_MANUALDISPLAY`** are the plain call of their template and
  are compiled with the rest; the call-site check covers them, no test runs them (a file's width and the
  printer are not part of a screen's state).
- **After `CALL`** (measured, `verification\v27_x02`): the old compiler accepts every one of these statements
  after `CALL`, with the template's form between the parentheses (`CALL LINE((1, 2)-(3, 4))`, `CALL PSET((1, 2),
  3)`, `CALL CLS`), except `VIEW PRINT` ("Expected CALL sub-name (...)"). The statements without a template
  (`DRAW`, `PCOPY`, `_LIMIT`, `_DELAY`, `_DISPLAY`) are read like a SUB call and compiled; the others stay "not
  supported yet" after `CALL`, as the window statements of `m2-console-screenhide` do (the parser leaves the
  parentheses unread there), listed in `SOMEDAY.md`.

*Alternative considered:* hand-written checking per statement, as the old compiler has for some. Rejected: the
template machinery and the call-site check exist for exactly this, and a rule per built-in is what the 2026-10-10
decision avoids.

As built (groups 3 to 5). All rows came in together with their tests, since no rule had to be written; each
group's screen programs were then run and listed.

- **The cast of `PAINT`'s fill and an expression.** The old compiler writes its cast before the first operand
  only: `PAINT (1, 2), l + 1` is `sub_paint(1,2,(uint32)*__LONG_L+ 1,…)`, so C++ casts `l` and adds in
  unsigned arithmetic. The new compiler writes the cast around the whole value (`(uint32)(*__LONG_L+1)`). The
  32 bits libqb gets are the same in every case: measured for 14 expressions, the old compiler's infix
  operators at that level are `+`, `-`, `*` and the bitwise ones (equal modulo 2^32 either way), and `\`,
  `MOD`, `/`, `^`, a comparison, `NOT`, a negation and anything with a float are one call or one parenthesised
  operand behind the cast.
- **`name (…) = …`.** Reviewing the snapshots that changed showed `cls(1) = 5` compiling as `CLS` with a
  comparison. Measured for 44 of the 45 supported statement names (all but `MID$`, which is written as an
  assignment; `verification\v27_x51`–`x57` record seven of them): the old compiler reads a statement that
  starts with a name, parentheses and `=` as an assignment, whatever follows, and rejects it (mostly "Name
  already in use (CLS)"), except `WIDTH`, which it makes an array (and `WIDTH = 80` a variable). With something between
  the `)` and the `=` (`CLS (1) + 2 = 5`) it is the statement with a comparison. `sema` now rejects the shape
  for every built-in statement (`not_an_assignment`); `SEEK (1) = 2, 3`, `RANDOMIZE (1) = 1` and `PCOPY`-like
  calls of earlier changes had the same hole.
- **The call-site check.** 301 programs for this change; all compare equal, with one generic extension of rule
  5 (the `u` suffix of a whole-argument literal, beside `ll`: the new compiler folds a literal's conversion to
  an `_UNSIGNED LONG` slot into `4u`). Seven programs written first were taken out because the check cannot
  compare them, and their cases moved into screen programs: a literal its slot's conversion changes (`COLOR
  4294967295` is `(-1)` here, `CLS , -1` is `4294967295u`), an expression as the fill of `PAINT` (above), and
  an expression stored from a function (`tools\callsite\README.md`, "What it does not compare"). No rule
  names a built-in.
- **A statement's name and a colon** (review of group 4). Civil War Strategy starts ten lines with `CLS :`,
  which the parser reads as a label and `sema` marked. Measured for 17 statements (`verification\v27_x58`–
  `x63`): to the old compiler it is the statement with no argument, never a label, and an error when the
  statement needs one. `sema` now compiles a label node that names a compiled statement as that statement with
  every slot absent (`bare_stmt`), for the statements of earlier changes as well.
- **`PALETTE USING`** (the same review) is the one hand-written form among this change's built-ins and is
  outside its scope. The parser gave it a syntax error, since no form of `PALETTE`'s template matches: a false
  error for an accepted line. It is marked now (`verification\v27_x64`).
- **The screen programs.** 42 of the 45 compile, and each left its recorded state on its first run. Of the
  first 29, 26 pass, one fewer than the goal above counted: `m32_newimage_32` names `_RGB32` and `_RGBA32`,
  image built-ins the proposal leaves out, beside `g03` (`PLAY`) and `g06` (graphics `GET` and `PUT`).
- **Not built:** nothing of D2's table. Left marked: `SOMEDAY.md`, "Left marked by `m3-window-output`".

### D3. The call-site check covers every form; no rule names a built-in

Each form of each template gets a program in `tests\callsite` (one per argument left out, per choice, and per
argument type for one representative slot of each slot type), following the existing naming
(`line_box.bas`, `locate_row_only.bas`, …). The six normalisation rules stand. If a new built-in cannot compare
equal without a rule that names it, the verdict of 2026-10-10 says to look at the check again: that is reported
to the user with the case, and the built-in is checked by its screen programs meanwhile.

The check's known limits apply (`tools\callsite\README.md`): it does not compare a number passed where libqb
takes a `double` or `long double`, so coordinates in the call-site programs are variables of the slot's own
measured type or float literals, and the mixed types are covered by screen programs (D4).

### D4. Screen programs are the slice programs of these built-ins, and join `slice.list`

`slice.list` takes `<group>/<name>`, so a screen program is listed as `screen/m01_screen_1` and the tier 2 slice
run checks it by its state with no change to the runner. The coverage rule of tier 1 ("every built-in in a
`slice.list` program") is then met by a screen program, in code that runs: these built-ins do not join
`WINDOW_BUILTINS`, the list of never-run ones.

New screen programs, recorded with `qb64pe.exe` (`--record`: two runs, equal states), for what the 29 do not use
or use too thinly; named `d…` ("drawing and text statements by form"), listed in `tests\corpus\screen\SOURCE.md`:

- `d01_print_window`: `PRINT` in its forms, `WRITE`, a long line that wraps, enough lines to scroll, `PRINT`
  with a comma (14-column zones in the window);
- one program per statement family with the forms the 29 leave out: `STEP`, every left-out argument, `B`/`BF`
  with a style, `LINE -(x, y)`, arcs and ellipses of `CIRCLE`, `PAINT` with a border colour and with a tile
  string, `PRESET`, `VIEW` with and without `SCREEN`, `WINDOW` and `WINDOW SCREEN`, `VIEW PRINT` with scrolling,
  `PALETTE` with and without arguments, `PCOPY`, `CLS` with each argument, `COLOR` in a text and a graphics mode,
  `LOCATE` with cursor shape;
- arguments of every numeric type for one statement per slot type (a float coordinate that rounds, an
  `_INTEGER64` colour), which the call-site check does not compare (D3);
- the functions: `CSRLIN`, `POS`, `POINT` in both forms, `SCREEN(r, c)` and `SCREEN(r, c, 1)`, each result
  printed and so part of the state;
- runtime errors under a handler (a mode that does not exist, drawing in `SCREEN 0`, `LOCATE` outside the
  screen): the error number printed;
- pacing: a loop with `_LIMIT`, `_DELAY 0.01`, `SLEEP` ended by a key from a `.keys` file, `_DISPLAY`. Small
  values only: nothing in a recorded state may depend on time (`tools\screen\README.md`).

How many programs is settled by task 1.4 from what 1.1 found; a statement that needed a rule of its own gets a
program of its own.

*Alternative considered:* a list of its own for screen programs, run by a separate command. Rejected: one more
command to forget, and the coverage test would need to learn a second list. If GitHub's runner cannot run them
(risk below), that is the fallback.

### D5. Functions follow the function rules

`INKEY$` and `CSRLIN` have no slot and are called bare only, like `_CWD$` and `FREEFILE`; with parentheses each is
whatever the old compiler does with it (an array used without `DIM`, or an error: measured in 1.3). `POS(n)`,
`POINT(x, y)`, `POINT(n)` and `SCREEN(r, c[, f])` are `Rule::Plain` if their C++ is the table's; `POINT` is the one
most likely to need a rule (two forms, one template `?[,?]`). Each gets a `typed` front-end test for its held and
believed type. `CSRLIN` and `POS` are also valid in a console program (the corpus README notes their recorded
values there); they are not restricted to programs with a window.

A function and a statement of one name (`SCREEN`, and `POS` has only the function) are told apart by position,
as `_DEST` is.

### D6. What newly compiles and has no check is listed, with the reason

After D1 some corpus and upstream programs compile without an error and have no recorded screen: tier 1 then
fails until each is on a pass list or in `tests\known_clean_not_passing.list`. Each such upstream program is
listed there with one of two reasons, decided per program in task 2.3 by running it with both compilers:

- it writes nothing to the window, or sends its output to the console, and its recorded output matches: it joins
  `tests\upstream\pass.list` instead;
- its result is on the window: listed with "output on the window, no recorded screen state".

Recording screen states for upstream programs is not part of this change (design D8 of `m2-screen-state-check`
found nothing worth recording among them).

### D7. Order inside the change

Group 1 measures. Group 2 lifts the mark and brings in `d01_print_window` and the programs of the 29 that need no
new statement beyond what group 2 itself gives (expected: none of the 29, so group 2's proof is `d01`, `c07` to
`c10` as far as they need no `LOCATE` or `COLOR`). Group 3 adds the text-screen rows and the functions, group 4
the drawing rows, group 5 pacing and what the remaining programs of the 29 still name. Each group ends with its
screen programs on `slice.list` and passing, with and without folding, so the change can stop after any group.

## Risks / Trade-offs

- [GitHub's Windows runner may not give a program an OpenGL window that libqb can draw into, or may give another
  state] → Seen at the first push that has a screen program on `slice.list` (task 2.5). `s63_console_window`
  already runs there with a hidden window, so a window can be made; whether the state is equal is not known. If
  it is not: screen programs move to `tests\corpus\screen.list`, run by tier 2 locally and not by the `tier2`
  job, and the coverage test reads both lists (the alternative of D4).
- [A screen program shows a window for about 0.3 s on the user's desktop during tier 2, and could take a key the
  user types] → Known from `m2-screen-state-check` (design D5; `SOMEDAY.md`: a desktop of their own). With
  `--jobs 8` several show at once. Not solved here; noted in `crates\README.md` beside the tier 2 commands.
- [A recorded state depends on the machine (font rendering, DPI, the GPU)] → The state is libqb's software
  buffers, not the window's pixels, and was equal across two runs at recording; a difference between the user's
  machine and CI would show at task 2.5 and is handled as the first risk.
- [The table's types are not what the old compiler passes (seen before: `sub_seek`, `CLNG`, `ASC`)] → Task 1.1
  reads the C++ of every form first, and the call-site check compares every form after.
- [A statement's own parser in the old compiler accepts or rejects forms its template does not show] → Task 1.3
  measures one program per doubtful form (`v27_x*`), as every change since `m2-procedures-and-errors` has; a form
  not measured stays "not supported yet", never guessed.
- [Tier 2 grows by the screen programs] → About 30 to 50 programs at roughly a second each with the build cache;
  counted in task 6.1 and written into `STATUS.md`.
- [`_LIMIT`, `_DELAY` and `SLEEP` make a test slow or timing-dependent] → Small values, no state that depends on
  how long anything took; `--record` refuses states that differ between two runs.

## Open Questions

- Answered (task 1.3, `verification\v27_a_console_functions`): `INKEY$` in a `$CONSOLE:ONLY` program with
  standard input redirected gives `""`; `POS(0)` is 1 there and `CSRLIN` does not move.
- Whether the order proposed for M3 in `proposal.md` stands (task 1.5 puts it to the user; this change does not
  depend on the answer).
