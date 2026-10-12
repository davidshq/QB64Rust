# Design

## Context

Motivation and scope: `proposal.md`. Measured facts: `study\00` §5, the paragraph on `verification\v26_*` (51
programs, recorded 2026-10-12). What the code does today:

- `sema` (`check\mod.rs`, `meta`) knows one metacommand, `$CONSOLE:ONLY`. It strips every blank before comparing,
  so `$CONSOLE : ONLY` is accepted (the old compiler: "Syntax error", `v26_x31`); it pushes the "select the
  console" statement for the first one only (the old compiler writes it at each, `v26_x25`); every other
  metacommand is "not supported yet", and so is a program without `$CONSOLE:ONLY` (one mark at the top of the
  file).
- The emitter writes `int32 console=1; int32 screen_hide_startup=0;` as fixed text (`decl.rs`). The driver passes
  `DEP_CONSOLE_ONLY=y` to `make` as a fixed argument (`build.rs`); the build key's version string stands for the
  fixed arguments.
- Generated code does not depend on the console mode. In the old compiler the mode decides the two globals, the
  make flags (`DEP_CONSOLE=y`, `DEP_CONSOLE_ONLY=y`) and the compile error of `_ECHO`, nothing else
  (`qb64pe.bas` 3590–3624, 8333, 13076–13086, 13708–13717). `PRINT`, `INPUT`, `END` and errors are the same
  calls in both modes; what differs at run time is libqb's.
- Built-ins come in as rows (`sema\builtins.rs`: `SUPPORTED`, `STATEMENTS`) checked by the built-in table, with a
  call-site program per form and argument type. The table has every built-in of this change; ten of them are
  "special-format" rows (`_DEST ?`, `_MOUSESHOW [?]`, `_MOUSEHIDE [{_Hide|_Disable}]`, `_ICON [?[,?]]`,
  `_MOUSECURSOR ?[,(?,?)]`, `_WINDOWSIZELIMIT [(?,?)][-(?,?)]`, `_FREEIMAGE [?]`, `_WINDOWHANDLE [?]`, `_NEWIMAGE
  ?,?[,?]`, `_SOURCE ?`). The parser already matches statements against their templates (`_DEST` with no argument
  is "matches none of its forms" today).
- The parser evaluates `$IF` in file order (`syntax\pp.rs`). A condition that names one of the six precompiler
  flags is taken as broken and marked "not supported yet" by `sema`, because their values come from the whole
  program.
- A name of QB64pe's auto-included files is "not supported yet" at its use (`sema\check\decl.rs`,
  `qb64rust_builtins::is_auto_include_name`; the names come from `builtins.json` `auto_include`, 310 constants of
  `beforefirstline.bi`, 287 of the `$COLOR` files, 3 functions of `afterlastline.bm`). `beforefirstline.bi` holds
  only `CONST` lines, `$INCLUDEONCE` and three `$IF` blocks (`WIN`, `32BIT`).

Constraints: nothing copied from other projects (the constants' text is read from QB64pe by the extractor, as the
table is); the reference clone is not modified (a build writes only into its git-ignored folders); no divergence
without a decision.

## Goals / Non-Goals

**Goals:**

- A program with `$CONSOLE` builds and runs as the old compiler's, with the console and the window.
- The old compiler's accept/reject line for the four metacommands, exactly.
- Every new built-in comes in the standard way: a row, call-site programs, a slice line.
- Precompiler flags and auto-included constants get one general mechanism each, not a special case for the
  upstream tests.

**Non-Goals:**

- Testing anything a window shows. A program whose output goes to the window compiles and builds with this
  change, since the generated code is the same; it is on no pass list until M3 gives it a check.
- Running a window built-in in a test. The `glut` tests end before theirs on Windows.
- `$ASSERTS`, `$DEBUG`, the network functions, `$COLOR`, `CHR$` and `EXP` in a `CONST`.

## Decisions

### D1. The console mode is a fact about the whole program, found by `sema`

`sema` walks the metacommand statements of compiled code as it does today (file order, included files where they
stand, procedures where they stand) and records:

- **console**: a `$CONSOLE` or a `$CONSOLE:ONLY` was seen, **or the settled flag `_CONSOLE_` is not 0** (D7);
- **window**: no `$CONSOLE:ONLY` was seen;
- **hidden**: the last of `$SCREENHIDE` and `$SCREENSHOW` was `$SCREENHIDE`.

The three do not have the same source in the old compiler. Its `console=1`, its `DEP_CONSOLE=y` and `_ECHO`'s
check read the flag (`qb64pe.bas` 13076, 13717, 8333), which stays set when the metacommand that set it is in a
`$IF` branch the last round no longer takes (`v26_x26`: the only `$CONSOLE` stands inside `$IF _CONSOLE_ = 0`).
"No window" comes from the lines its main pass compiles (3597), so a `$CONSOLE:ONLY` in such a branch leaves a
program with a console and a window. Its recording shows that `_CONSOLE_` is 2, `_DEST` is -2 and a `PRINT`
with no `_DEST` is not seen (`v26_x38`); the window is shown by its build, read 2026-10-12 from the clone's
`internal\temp` after compiling it: `compilelog.txt` links `libqb_make_00000000`, the libqb with the window,
with `-mconsole`, and `global.txt` has `console=1`. "Hidden" is set the same way in the source (3617); for a
`$SCREENHIDE` in such a branch that is read, not measured (standard output does not show whether a window was
shown). Until group 6 a program of that kind is rejected whatever the walk finds, because its `$IF` is marked;
from group 6 on, `parse_settled` hands the settled flags to `sema`, which takes the console from them.

Each `$CONSOLE:ONLY` pushes the "select the console" statement where it stands (today only the first; `v26_x25`).
`$CONSOLE`, `$SCREENHIDE` and `$SCREENSHOW` push nothing. After the walk, a program with no console gets the one
mark at the top of the file, reworded: "programs without `$CONSOLE` or `$CONSOLE:ONLY`".

*Why in `sema`, not the parser:* it is where the mark and `_ECHO`'s error are reported, and where
`$CONSOLE:ONLY` already lives. *Alternative considered:* keep rejecting a program whose output can reach the
window (a `PRINT` not dominated by `_DEST _CONSOLE`). Rejected: it needs flow analysis to be right, the generated
code is the same either way, and tier 1's "clean programs are listed" rule already makes every clean program that
is on no pass list visible with a reason.

### D2. The four metacommands are matched exactly

The token's text (from `$` to the line end) is compared, in upper case, after trimming blanks at both ends only,
with `$CONSOLE`, `$CONSOLE:ONLY`, `$SCREENHIDE`, `$SCREENSHOW`. A text that starts with one of the four names
followed by anything else (`:OFF`, ` junk`, ` ' comment`, `: x = 1`, ` : ONLY`) is an error of the program,
"`$CONSOLE` must stand alone on its line" in our wording (the old compiler says "Syntax error"). Two more
rejections are about the line, not the text:

- **after a statement and a colon** (`x = 1: $CONSOLE`, "Unexpected character on line", `v26_x13`; the same for
  `$SCREENHIDE`, `v26_x36`);
- **a tab before the metacommand** (`v26_x23`; blanks are fine, `v26_x15`).

Both hold for every metacommand (task 1.1, `v26_x40`–`x50`: measured for `$IF`, `$LET`, `$INCLUDEONCE`,
`$CONSOLE:ONLY` and `$SCREENHIDE` too; in the old compiler's source each is one place that names no metacommand,
`study\00` §5). So neither is a check of the four metacommands in `sema`; both live in `syntax`, where every
metacommand line is met (`parser\meta.rs`, `metacommand`), and task 2.1 puts them there:

- **after a colon:** the parser has the error today for the preprocessor lines only (`$IF`, `$ELSE`, `$ELSEIF`,
  `$END IF`, `$LET`; "a preprocessor metacommand must stand at the start of a line"). It is widened to every
  metacommand and reworded without "preprocessor". Today `x = 1: $INCLUDEONCE` and `x = 1: $CONSOLE:ONLY` are
  accepted. `$ERROR` keeps its own error, as now.
- **a tab before:** new. Today the new compiler accepts a tab before `$IF`, `$LET`, `$INCLUDEONCE` and
  `$CONSOLE:ONLY`, which the old compiler rejects. A metacommand whose `$` has a tab anywhere between the line
  start and itself is an error of the program, "a metacommand must not have a tab before it" in our wording,
  and the line then counts as no metacommand: a `$IF` opens no level and a `$CONSOLE` gives no console, as in
  the old compiler, whose message there depends on what follows (`x40`, `x41`, `x49`). One error for the line
  is enough; the errors that follow from it (`$END IF` without `$IF`) are the old compiler's too. Inside a
  branch that is not taken such a line is skipped text with no error: a tabbed `$ELSE` or `$END IF` there does
  not end the branch (`x50`, which the old compiler accepts and the new one rejects today with "`$END IF`
  without `$IF`": a false error, fixed by the same rule).

A metacommand in a comment (`'$INCLUDE`, `'$DYNAMIC`) is found another way and keeps today's rules.

*Alternative considered:* leave the forms as "not supported yet". Rejected: the spec `compiler/pipeline` forbids
that mark for an error the old compiler also reports, and each is one comparison.

### D3. The IR carries program settings; the emitter and the driver read them

`ir::Program` gains `settings`: `console: bool`, `window: bool`, `window_hidden: bool`, and `runtime_parts`, a
sorted set of an enum with one member today, `Icon`. `lower` copies the first three from `sema` and collects the
parts from the built-ins the program uses (the table's `dependency` column: `_ICON` has `DEPENDENCY_ICON`). The
names are neutral, as `compiler/pipeline` "ABI-neutral IR" asks; the dump prints one `settings` line.

- The emitter writes `console` and `screen_hide_startup` from them (`decl.rs`).
- The driver turns them into make flags: `DEP_CONSOLE=y` when `console`; `DEP_CONSOLE_ONLY=y` when not `window`;
  `DEP_ICON=y` for `Icon`. They replace the fixed `DEP_CONSOLE_ONLY=y`.
- The build key hashes the flags (they are not in the fragments), and its version string changes.

The old compiler's flags for each case (task 1.2, 2026-10-12). On Windows it keeps no copy of its make line
(it writes one into a script on Linux and macOS only, `qb64pe.bas` 14099), so they are read from what the line
does, in `internal\temp\compilelog.txt` and `global.txt` after `qb64pe -x` of four programs, and from the lines
that build it (`qb64pe.bas` 13706–13721):

| Program | Make flags | libqb variant, link | `-D` that differs | Globals |
|---|---|---|---|---|
| `$CONSOLE:ONLY` | `DEP_CONSOLE_ONLY=y DEP_CONSOLE=y` | `libqb_make_00100000`, `-mconsole`, `window-console.o`, `main-thread-console.o` | `DEPENDENCY_CONSOLE_ONLY`, `DEPENDENCY_NO_ICON` | `console=1`, `screen_hide_startup=0` |
| `$CONSOLE`, `$SCREENHIDE` | `DEP_CONSOLE=y` | `libqb_make_00000000`, `-mconsole`, `window-gui.o`, `main-thread-gui.o` | `DEPENDENCY_NO_ICON` | `console=1`, `screen_hide_startup=1` |
| `$CONSOLE`, then `$CONSOLE:ONLY` | as the first row | as the first row | as the first row | as the first row |
| `$CONSOLE`, `$SCREENHIDE`, `_ICON` | `DEP_ICON=y DEP_CONSOLE=y` | `libqb_make_00000100`, `-mconsole`, the two `gui` objects | `DEPENDENCY_ICON` | as the second row |

The mapping above is right as written. Two things the table adds. A program with `$CONSOLE:ONLY` gets
`DEP_CONSOLE=y` too (the flag is the console state, which `:ONLY` sets to 2); the driver passes only
`DEP_CONSOLE_ONLY=y` today. The clone's `Makefile` reads `DEP_CONSOLE` in one place, to choose `-mconsole`,
which `DEP_CONSOLE_ONLY` chooses as well, and the variant's name does not depend on it: adding it changes no
build, and it is added so the line equals the old compiler's. And the icon part is a third libqb variant
(`…00000100`), built once like the second (D4). A driver test compares our flags with this table for the four
cases, so a missing flag is found by a test and not by a link error.

*Why collect dependencies from the table now, for one member:* `_ICON` cannot be linked right without it, and the
same column names the parts M3's built-ins need (`DEPENDENCY_SCREENIMAGE`, `DEPENDENCY_LOADFONT`, …). A program
whose built-in names a part the driver does not know is "not supported yet" at that built-in; none of this
change's other built-ins has one.

### D4. A second libqb variant, built by the clone's Makefile

A program with a window links libqb compiled without `DEPENDENCY_CONSOLE_ONLY`. The clone's `Makefile` names the
objects by the flags (`QBLIB_NAME`), so both variants live side by side in its git-ignored folders and the first
build of the new one takes minutes. `qb64pe.exe` built the same variant there when `verification\v26_*` was
recorded, so locally it exists already. In the `tier2` job of `rust.yml` it is built once per run.

No change to how the driver calls `make`, apart from the flags.

### D5. `_DEST`, `_SOURCE`, `_CONSOLE`, `_ECHO` are rows; `_ECHO` has one extra check

- Statements `_DEST`, `_SOURCE`, `_ECHO`: `StmtRule::Plain`. Functions `_DEST`, `_SOURCE`, `_CONSOLE`:
  `Rule::Plain` (LONG, no arguments). A name that is both (`_DEST`) is told apart as `SHELL` is.
- `_ECHO` in a program with no console is the old compiler's compile error (`v26_x17`). `sema` knows the mode
  only after the whole walk (D1), so the check runs after it, over the recorded positions of `_ECHO` statements.
- After `CALL` the three statements are accepted too (`v26_x69`, `x70`, measured in group 3). `_ECHO` has no
  template and is read like a SUB call already; `_DEST` and `_SOURCE` have the template `?`, and a statement
  whose template is one argument is read like a SUB call after `CALL` (`check\proc.rs`).
- `_DEST -99` (error 258), `_DEST 1.6` and the handle values are libqb's; nothing to do but pass the argument
  converted as the table's LONG slot says, which the call-site programs check for each numeric type (`v26_x22`).

Console `INPUT` and `LINE INPUT` need no change: they were reachable only under `$CONSOLE:ONLY` because every
other program was rejected.

### D6. Window built-ins are rows, compiled and never run

Each of the ten statements and ten functions (`proposal.md`) is a row with its rule and a call-site program for
every form of its template and every argument type class the existing programs use. What the old compiler
writes for each (task 1.3, 2026-10-12: `qb64pe -z` over one program with every form below and one with probes
of the result types, both outside the repo; `l`, `i`, `s`, `d`, `q` are a LONG, an INTEGER, a SINGLE, a DOUBLE
and an `_INTEGER64`, `t` a STRING). Every numeric slot is a LONG slot, an `int32_t` in libqb's prototype: an
integer of any width is passed as it is, a float through `qbr(…)`, as `Slot::Long` does today.

| Built-in | Forms and the call | Result | Rule |
|---|---|---|---|
| `_DEST` statement | `_DEST l` → `sub__dest(*__LONG_L)`; `i`, `q` as they are; `s`, `d` in `qbr(…)`; `_DEST _CONSOLE` → `sub__dest(func__console())` | | `StmtRule::Plain` |
| `_SOURCE` statement | as `_DEST`, `sub__source(…)` | | `StmtRule::Plain` |
| `_ECHO` | `sub__echo(…)`, one string | | `StmtRule::Plain`, and D5's check |
| `_DEST`, `_SOURCE`, `_CONSOLE` functions | `func__dest()`, `func__source()`, `func__console()` | LONG, `int32_t` | `Rule::Plain` |
| `_SCREENHIDE`, `_SCREENSHOW`, `_SCREENICON` statements | `sub__screenhide()`, `sub__screenshow()`, `sub_screenicon()` | | `StmtRule::Plain` |
| `_TITLE` | `sub__title(…)`, one string | | `StmtRule::Plain` |
| `_ICON` | none → `sub__icon(NULL,NULL,0)`; `l` → `(l,NULL,1)`; `l, l` → `(l,l,3)` | | `StmtRule::Plain`; the icon part (D3) |
| `_MOUSESHOW` | none → `sub__mouseshow(NULL,0)`; a string → `(…,1)` | | `StmtRule::Plain` |
| `_MOUSEHIDE` | none → `sub__mousehide(NULL)`; `_HIDE` → `( 1 )`; `_DISABLE` → `( 2 )`; no mask | | `StmtRule::Plain` (a choice, as `OPEN` has) |
| `_MOUSECURSOR` | `l` → `sub__mousecursor(l,NULL,NULL,0)`; `l, (l, l)` → `(l,l,l,1)` | | `StmtRule::Plain` |
| `_WINDOWSIZELIMIT` | none → `sub__windowsizelimit(NULL,NULL,NULL,NULL,0)`; `(l, l)` → `(l,l,NULL,NULL,1)`; `-(l, l)` → `(NULL,NULL,l,l,2)`; both → `(l,l,l,l,3)` | | `StmtRule::Plain` |
| `_FREEIMAGE` | none → `sub__freeimage(NULL,0)` (accepted, though the syntax line names a handle); `l` → `(l,1)` | | `StmtRule::Plain` |
| `_SCREENICON`, `_DESKTOPWIDTH`, `_DESKTOPHEIGHT`, `_SCREENX`, `_SCREENY`, `_WINDOWHASFOCUS` | `func_screenicon()`, `func__desktopwidth()`, `func__desktopheight()`, `func__screenx()`, `func__screeny()`, `func__hasfocus()` | LONG, `int32_t` | `Rule::Plain` |
| `_TITLE$` | `func__title()` | STRING | `Rule::Plain` |
| `_NEWIMAGE` | `(l, l)` → `func__newimage(l,l,NULL,0)`; `(l, l, l)` → `(l,l,l,0\|1)` | LONG, `int32_t` | `Rule::Plain` (a mask as `RND` has) |
| `_WINDOWHANDLE` | bare → `func__windowhandle(NULL,0)`; `(l)` → `(l,0\|1)` | `_UNSIGNED _OFFSET`, `uintptr_t` | `Rule::Plain`; `table_ty` learns the type |
| `_SCREENEXISTS` | `func_windowexists()` | none in the table; `int32_t` | `Rule::Fixed(LONG)`, see below |

No new rule is needed. A function with empty parentheses (`_DEST()`, `_SCREENX()`, `_WINDOWHANDLE()`,
`_TITLE$()`, each of the others tried) is "Expected (...)" in the old compiler. The three oddities:

- **`_SCREENEXISTS` has no return type in the table.** The old compiler then treats the value as a 32-bit
  integer wherever one was tried against `_SCREENX`, which is a LONG: a store into each numeric type, `+`, `/`,
  `\`, `^`, `AND`, `NOT`, a minus, `=`, `IF`, `SELECT CASE` (its temporary is an `int32`), an argument of a
  LONG and of a DOUBLE parameter (the temporaries are `int32` and `double`). It differs in two spellings.
  `PRINT` and `STR$` write `qbs_str(func_windowexists())` without the `(int32)` cast a LONG gets: the same
  overload, since the C++ type is `int32_t`, so the same text. `HEX$`, `OCT$` and `_BIN$` write the width 0 (`func_hex(…,0)`,
  `func_oct(…,0)`, `func__bin(…,0)`) where `_SCREENX` gets 8, 32 and 32: another text for a negative value, and
  the value is -1 whenever the window exists (under `$CONSOLE:ONLY`, `HEX$(_SCREENEXISTS)` is the empty
  string). Inside a larger argument there is no difference (`HEX$(_SCREENEXISTS + 0)` and `HEX$(_SCREENX - 1)`
  both get 0, as the new compiler writes for any LONG expression). So the row is `Rule::Fixed(Ty::I32)`, and
  `_SCREENEXISTS` as the direct argument of `HEX$`, `OCT$` or `_BIN$` is "not supported yet" (no corpus or
  upstream program has it; M3, where the function runs, can give it the width 0).
- **`_WINDOWHANDLE` returns `_UNSIGNED _OFFSET`.** It behaves as a variable of that type, which the compiler
  has: `PRINT` and `STR$` write `qbs_str((uint64)(…))`, `<> 0` compares it as it is, a store into a LONG, an
  `_INTEGER64` or a DOUBLE is plain, and `/` rounds the quotient with `qbr(…)` (the old compiler's rule for an
  offset operand, which the new compiler already writes for a variable: both were compared for `d = o / 3`,
  `PRINT o`, `PRINT o <> 0`, `l = o`, `STR$(o)`). The optional argument is `RND`'s shape.
- **Parenthesised pairs.** The parser accepts every form of the table above today: the new compiler reports
  each line of that program "not supported yet" and nothing else. The masks are the ones
  `qb64rust_builtins::passing` computes for `LINE`'s template, which has the same shape; the call-site
  programs of task 4.1 compare each.

They are listed in a constant of `sema\builtins.rs` (`WINDOW_BUILTINS`) which the coverage test reads: for these,
"used in a slice program" means a slice program in the shape of the upstream `glut` tests (`$CONSOLE:ONLY`, a
`PRINT`, `SYSTEM`, then the built-ins as code that is not reached), and the test also requires a call-site
program per built-in (`testing/compiler-tests`, delta). The `typed` and `ir` front-end tests are required as for
any built-in.

*Alternative considered:* run them under `$CONSOLE:ONLY`, where libqb has console versions of most. Rejected for
now: their results are the machine's (`_DESKTOPWIDTH`, `_WINDOWHANDLE`, `_TITLE$`) and M3 gives them a real check.

### D7. Precompiler flags: the old compiler's state machine, ported

Per flag the old compiler keeps `wanted`, `actual` and `locked` (`utilities\statevars.bas`). A feature sets
`wanted` where it is met; at the end of a round, `actual <> wanted` with the flag not locked asks for another
round; at the start of a round `actual = wanted` unless locked, and a flag that is not 0 is locked from then on.
In its main pass `$IF` sees `actual`; in its first pass, which is the one that meets the settings, it sees a
running value for three of the flags (below). The port:

- `syntax` gets `PpFlags` (the six `actual` values) as an input of `parse`, and reports the **settings met**, in
  file order, as it parses compiled code: `$CONSOLE` (console 1), `$CONSOLE:ONLY` (console 2), `$ASSERTS`
  (asserts 1), `$ASSERTS:CONSOLE` (asserts 1, console 1), `$DEBUG` (debug 1), `OPTION _EXPLICIT` (explicit 1),
  `OPTION _EXPLICITARRAY` (explicit-array 1). `_EXPLICITARRAY_` is explicit or explicit-array, as in
  `setPrecompFlags`. `_SOCKETS_` is always 0: the network functions are "not supported yet", so a program that
  would set it is rejected anyway.
- A function `parse_settled` runs the rounds: parse with the current `actual` values as the start, apply the
  settings met to `wanted`, stop when no unlocked flag changed. At most seven rounds can happen (each round but
  the last locks a flag); the loop has that cap and a panic-free error past it.
- **A round is repeated only when it can matter:** when the parse met no `$IF`/`$ELSEIF` that names a flag, the
  tree cannot depend on the flags and the first parse is final. Almost every program takes this path, so tier 1's
  time does not double.
- **The corner the old compiler gets wrong** (`v26_x27`, `x34`): its first pass also sets `_CONSOLE_` at each
  metacommand's line, so between `$CONSOLE:ONLY` and a later `$CONSOLE` the first pass sees 2 and the second 1.
  The parser keeps that running value beside `actual`; a condition that names `_CONSOLE_` while the two differ is
  left as today: branch taken as broken, "not supported yet" from `sema`. In the last round only `_CONSOLE_` can
  have two values that are not 0, so only it needs this there.
- **A round that is not the last follows the old compiler's first pass.** The old compiler's rounds are driven
  by its first pass, and there a `$IF` sees the running value, not `actual`: `$CONSOLE`, `$CONSOLE:ONLY`,
  `$ASSERTS`, `$ASSERTS:CONSOLE` and `$DEBUG` set their flag's `$IF` value at their line (`qb64pe.bas`
  1980–2006); `OPTION _EXPLICIT` and `OPTION _EXPLICITARRAY` do not (2099). So with `$CONSOLE` on line 1 and
  below it `$IF _CONSOLE_ = 0 THEN`, `OPTION _EXPLICIT`, `$END IF`, the branch is never taken and `_EXPLICIT_`
  stays 0, though `actual` is 0 in the first round (`v26_x39`). The port keeps a running value for `_CONSOLE_`,
  `_ASSERTS_` and `_DEBUG_` beside `actual`.
- **One parse serves both passes.** The parser does not know whether a round is the last, so every parse does
  two things at a `$IF` that names a flag: it chooses the branch by the running values, and the settings met are
  those of the branches chosen so; and it notes the condition as broken when a flag it names has a running value
  other than its `actual`. When the round asks for another, its tree and notes are thrown away. When it is the
  last, every noted condition is the corner above: in a last round `_ASSERTS_` and `_DEBUG_` are locked at the
  value their metacommand sets, so only `_CONSOLE_` can differ.

- **`OPTION _EXPLICIT` is in force by its flag, not by its line** (task 1.4, `v26_x54`, `x55`). With the only
  `OPTION _EXPLICIT` inside `$IF _EXPLICIT_ = 0 THEN`, the flag is 1 and a variable that is not declared is
  the old compiler's "not defined" error, though the last round does not compile the line. It is the console's
  case again (D1). `sema` finds the two options by looking for their statements in the tree
  (`check\mod.rs`); from group 6 on it also takes them from the settled flags: explicit when `_EXPLICIT_` is
  not 0, explicit-array when `_EXPLICITARRAY_` is not 0.

Measured in task 1.4, each as the port above gives it, and no second corner found:

- `$IF _EXPLICIT_` between two `OPTION _EXPLICIT` lines is 1 (`x51`): the option has no running value, and the
  second round starts with 1.
- `$ASSERTS:CONSOLE` gives the program a console and `_CONSOLE_` is 1 above and below its line, `_ASSERTS_` 1
  (`x52`, a program with no other console metacommand, its output written with `_ECHO`).
- Three rounds (`x53`): the only `$CONSOLE` inside `$IF _EXPLICIT_`, the only `OPTION _EXPLICIT` on the last
  line. The program has a console, `_CONSOLE_` and `_EXPLICIT_` are 1.
- The running value of `_ASSERTS_` (`x56`, as `x39` for the console): `OPTION _EXPLICIT` inside `$IF _ASSERTS_
  = 0` below the line of `$ASSERTS` is never met, `_EXPLICIT_` stays 0.
- Why no second corner: in the source, the flags' `$IF` values are set from `actual` at the first line of each
  pass and at the end of the source, before the files included at the bottom (`setPrecompFlags`, `qb64pe.bas`
  1816, 3373), and changed in between only by the five metacommand lines of the first pass (1978–2006). Only
  `_CONSOLE_` has two values that are not 0, so only it can differ from `actual` in a last round. One detail of
  `statevars.bas` the port keeps: another round is asked for at the moment a setting differs from `actual`
  with the flag not locked, not by a comparison at the round's end. With today's settings the two are the
  same (no setting writes 0).

`$ASSERTS` and `$DEBUG` stay "not supported yet" as metacommands; they set their flags so that the rest of such a
program is read as the old compiler reads it and no second, misleading mark appears.

*Alternative considered:* compute the flags in a pre-scan of the text. Rejected: a metacommand in a `$IF` branch
that is not taken does not count (`v26_x16`), and one inside a branch it changes itself does (`v26_x26`); only the
old compiler's own rounds give both.

### D8. Auto-included constants come from the table, evaluated on first use

- `tools\builtins\extract_builtins.py` already reads `beforefirstline.bi` for the names. It also records, per
  constant, the **text of its expression** and its name with the type suffix, taking the `$IF` branches for
  Windows 64-bit (`WIN` true, `32BIT` false). `builtins.json` and the generated table in `crates\builtins` gain
  the field; a test of the crate pins a few (`_TRUE` is `-1`, `_SIZE_OF_OFFSET` is `8`).
- In `sema`, a name of that file that is a constant resolves, at its first use, by parsing and evaluating the
  recorded text with the `CONST` evaluator, in the order of the file (a constant may name earlier ones:
  `_ONE_MB = _ONE_KB * _ONE_KB`). The result is cached and is a global constant: visible in every body, never
  shadowed (a user cannot declare a name with a leading underscore).
- When the evaluator reports "not supported yet" (`CHR$`, `EXP`), the use keeps today's mark, with the reason.
- The `$COLOR` constants and the three functions keep today's mark.

Checked against `qb64pe.exe` by a slice program that prints every constant the evaluator computes (task 5.3);
one whose printed value differs is a bug to fix or a constant to leave marked, never a silent difference.

*Alternative considered:* include the real file from the QB64pe root as line zero, as the old compiler does.
Rejected: the front end runs without the clone today (tier 1 in CI, the language server, `--report unsupported`
with only an include root), and that would end. *Alternative considered:* only `_TRUE` and `_FALSE`, hard-coded.
Rejected: two special cases cost as much as the general table field, and corpus and upstream programs name ten
other constants of the file (`_LOG_TRACE`, `_ONE_GB`, `_SECS_IN_DAY`, `_E`, and the `_MIN`/`_MAX` pairs of
SINGLE, DOUBLE and `_FLOAT`; `--report unsupported` over both, 2026-10-12).

### D9. Tests and lists

- **Measurements first** (group 1): the open points named in D2, D3 and D6.
- **Slice programs** `s62`–`s68`, each recorded with `qb64pe.exe`: the forms that make a program without a
  window; the first program with a hidden window (its output is empty: `_DEST` comes a group later); `_DEST`,
  `_SOURCE`, `_CONSOLE` and `_ECHO`; console input with a `.stdin` file; the `glut` shape with every window
  built-in; the auto-included constants; the precompiler flags.
- **No slice program with plain `$CONSOLE` uses `PRINT` with a comma** (blanks without end under a redirected
  output, `v26_a`); `tests\corpus\README.md` says so.
- **Front-end tests**: `check-fail` for every rejected form of D2, D5 and D7's corner; `ir` for the settings line
  and the new statements; `typed` for the new functions; `cpp` for the two globals.
- **Lists**: `slice.list`, `tests\upstream\pass.list`, the shrink-only lists, `known_clean_not_passing.list`
  (a clean program whose output goes to the window is listed there with that reason).
- **The GitHub runner** (task 2.6): the first push after group 2 shows whether a program with `$CONSOLE` and
  `$SCREENHIDE` starts there. QB64pe's own CI runs its test suite on `windows-2022` (`build-process.yml` in the
  clone), and `data\test.bas`, which has that header, is in it, so it is expected to. If it does not, the programs that need a window move to a list of their own that the `tier2` job
  skips and local runs include, and the user is told; nothing else in this design depends on the answer.

## Risks / Trade-offs

- [The windowed libqb does not build or start on the GitHub runner] → Seen at the first push after group 2, before
  the other groups build on it; the fallback list is described in D9.
- [`tier2` takes longer in CI: one more libqb variant] → Measured at that push and written into `STATUS.md`; the
  job has no build cache by design (`build.rs`, `CACHE_ENV`).
- [A program whose output goes to the window now compiles cleanly and nobody checks it] → It cannot get onto a
  pass list (its recorded standard output would be empty where the old compiler's is, but its screen is
  unchecked); tier 1 forces it into `known_clean_not_passing.list` with the reason.
- [The parser's template matching does not cover parenthesised pairs] → Found by the first call-site program of
  `_WINDOWSIZELIMIT`; extending it is inside group 4. If it turns out to need a new statement shape in the tree,
  that built-in stays marked and the two upstream programs that need it stay off the pass list.
- [An auto-included constant evaluates to another type than the old compiler gives it] → The slice program prints
  all of them and is compared; types that `PRINT` does not show are compared with `LEN` in the same program.
- [Parsing in rounds changes positions or diagnostics between rounds] → Only the last round's tree and
  diagnostics are kept; a front-end test compiles a program that needs two rounds and one that needs three.
- [The flags' port accepts a program the old compiler mis-compiles] → The one known corner is marked (D7); the
  measurements of group 1 look for a second with `_EXPLICIT_`.

## Migration Plan

No migration: nothing compiled today changes its output. The reworded mark ("programs without `$CONSOLE` or
`$CONSOLE:ONLY`") changes snapshots and `known_unsupported_rejections.list`, which are regenerated in group 2.
Each group leaves fmt, clippy and `cargo test` green, so the change can stop after any group.

## Open Questions

- Answered by task 1.1: the two line rules of D2 (tab before, statement before) hold for every metacommand, so
  both checks live in `syntax` (D2). The specs of this change are unchanged by it.
