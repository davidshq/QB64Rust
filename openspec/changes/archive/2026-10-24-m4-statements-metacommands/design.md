# Design

## Context

For the motivation and the 43 programs, see `proposal.md`. For the behaviour, see the delta specs. How the parts
stand in this compiler (2026-10-23, on `m4-finish`):

- **Marks.** `self.unsupported(span, …)` in `sema` gives a "not supported yet" mark, and `self.error(span, …)` a
  real error. In the parser, `syntax_error` (`syntax` `parser\mod.rs` 265) turns a syntax error at a known
  operator or keyword into a mark (`unhandled_token`); the marks of `x = OR`, `2 ^ _NEGATE`, `IF THEN`,
  `WRITE …;`, `ON STRIG(1 GOSUB`, `DEFINT a AS` and `CALL _MEMPUT(…, 1 AS LONG)` all come from there. The note
  after an error comes from `crates\base\src\old_wording.tsv`, which `tools\diagnostics\old_wording.py` writes by
  pairing our message with the old compiler's recorded message on the same line.
- **Metacommands.** `Checker::meta` (`check\mod.rs` 1392) knows every metacommand this compiler builds; anything
  else ends in `unsupported("metacommand …")`. The old compiler knows `$CONSOLE[:ONLY]`, `$SCREENHIDE`,
  `$SCREENSHOW`, `$RESIZE:…`, `$NOPREFIX`, `$ERROR`, `$LET`, `$IF`/`$ELSEIF`/`$ELSE`/`$END IF`, `$CHECKING:…`,
  `$COLOR:…`, `$DEBUG`, `$ASSERTS[:CONSOLE]`, `$EXEICON`, `$VERSIONINFO`, `$UNSTABLE:…`, `$EMBED`,
  `$INCLUDEONCE`, `$ERRORLOCATION:…`, `$USELIBRARY` and `$MIDISOUNDFONT:`. Any other `$` line reaches its
  statement parser and is "Syntax error" (`qb64pe.bas` 1836–2092, 3520–3900).
- **Stubs.** The old compiler's table marks `FRE`, `SETMEM`, `FILEATTR`, `IOCTL`/`IOCTL$`, `TRON`, `TROFF`,
  `RESET`, `LIST`, `OPTION`, `DEF` and the statements it compiles by rules of its own (`SWAP`, `CLOSE`, `READ`,
  `MID$`, `_MEMPUT`, …) with `sub_stub`/`func_stub`. A stub that reaches its generic path is "Command not
  implemented" (`qb64pe.bas` 11567, 20237). That is how `CALL _MEMPUT` and `CALL _MEMFILL` fail: `CALL` takes the
  generic path.
- **Included files.** Each included file is parsed at once as a tree of its own (`parser\meta.rs` `include`,
  design D9 of `m2-parser-breadth`). A block left open at its end, or a closer whose block is not in its file, is
  marked (`parser\blocks.rs` `unclosed`, `stray_closer`; `parser\proc.rs` 31). `sema` walks statements by
  recursion: a block's checker collects its body with `block_body`, and an include statement is followed by the
  included tree's statements (`check_statement`, `check\mod.rs` 906).
- **The flag between two settings.** The parser notes the `$IF` lines where the old compiler's two passes read a
  flag differently (`passes_disagree`, `parser\meta.rs` `flag_condition`). `sema` marks each (`check\mod.rs`
  1398).
- **`RUN`** has a node (`RunStmt`) and a mark (`check\mod.rs` 1294). libqb has `sub_run_init`, `sub_run` and the
  global `run_from_line`; `qbx.cpp` includes the generated `runline.txt` at the start of `QBMAIN`
  (`qb64pe.bas` 9764–9836).

## Goals / Non-Goals

**Goals:**

- Each of the 43 programs passes, or its result is the old compiler's error with that error's note; none is
  marked "not supported yet".
- A rule turns into an error only the forms a recording shows the old compiler rejects. Neighbours with no
  recording stay marked.

**Non-Goals:**

- The about 55 built-ins that no program uses, `$ERRORLOCATION` and `$USELIBRARY`. They are step 4b. They stay
  marked, and the metacommand rule (D3) must not make the two metacommands errors.
- The 9 programs that steps 2 and 3 left marked (`proposal.md`).
- Our messages copying the old compiler's. The note carries its wording, as for every error.

## Decisions

### D1. The measurement family `v57` comes first

`verification\v57_*` records the forms beside the 43 that no family measured. The code then follows the
recordings, not guesses. Where a recording disagrees with a delta spec, the spec is changed in the same task, as
in `m4-builtins-param-types` 1.1. The family covers:

- **Metacommands:** `$STATIC` bare, `$INCLUDEONCE x`, `$FOO:BAR`, a lower-case `$nosuch`, `$CONSOLE:OFF`, a `$`
  alone, and a `$` line in an included file.
- **Stubs:** `TRON`, `TROFF`, `RESET`, `LIST`, `IOCTL 1, "x"`, `x = IOCTL$(1)`, `SETMEM(0)`,
  `FILEATTR(1, 1)`, `FRE(-2)`, `FRE("")`, `CALL TRON`, `CALL CLOSE`, `CALL CLOSE(1)`,
  `CALL MID$(a$, 1, 1, "x")`, and `CALL _MEMGET(…)`.
- **`RUN`:** `RUN` that restarts with a counter kept in a file; `RUN label` and `RUN 100` from the main module and
  from a SUB; `RUN` of a missing file; `RUN` of a second program; `RUN` after `SCREEN 13` and `COLOR`;
  `RUN` beside `ON TIMER`.
- **`RESUME` in a procedure:** reached under a handler of the main module, of each kind (`RESUME`, `RESUME 0`,
  `RESUME NEXT`, `RESUME label`); and reached with no error pending.
- **Places in parentheses:** `SWAP a, (b)`, `SWAP (x(1)), x(2)`, `SWAP (s$), t$`, `READ (x(1))`, `READ ((a))`,
  `INPUT (a)` and `LINE INPUT (a$)`; `LBOUND((a), 1)`, `UBOUND((b))` and `LBOUND(((a)))`.
- **Omitted arguments:** the first, middle and last argument left out of `POINT`, `_RGB32`, `_RGB`, `INSTR`,
  `MID$`, `LEFT$`, `_RGBA32` and a FUNCTION of the program.
- **A FUNCTION as a statement:** `f 1`, `f(1)` and `CALL f(1)` for a program FUNCTION of each kind of name.
- **Libraries:** `h_l` and `h_l&(2)` called in the `v37_x22` pair, and the reverse pair (a library SUB and a
  program FUNCTION).
- **`DECLARE … ALIAS`:** `ALIAS` with a quoted name, with an empty name, and on a `DECLARE FUNCTION`.
- **Colours:** `_RED`, `_GREEN` and `_BLUE` of 256, 300, 2222 and 65535 in modes 0 and 13, each in a `CONST`
  twice over two builds, to see whether the value changes.
- **The flag between two settings:** a `PRINT`, a `DIM`, a `SUB` call and a `CONST` in each branch; the `$ELSE`
  branch alone; the same with `$ASSERTS:CONSOLE`.
- **Blocks across files:** `FOR`, `DO`, `IF`, `SELECT CASE` and `WHILE` opened in the main file and closed in an
  included file, and the reverse; a SUB opened in an included file and closed in the main one; two levels of
  include.
- **`_SCREENHIDE` the function:** after `$SCREENHIDE` and without it, under `$CONSOLE`.

*Alternative:* build from the 43 programs alone. Rejected: each rule reads a class of forms, and the members of
the class that no recording covers are where a false error would come from.

**What the family found** (task 1.1, 2026-10-23; the delta specs were changed to it):

- Every unknown metacommand line, also `$`, `$CONSOLE:OFF`, `$INCLUDE: '…'` and one in an included file, is
  "Syntax error"; one in a branch not taken is nothing; `$ERRORLOCATION:ON` compiles (stays marked, 4b).
- `TRON`, `TROFF`, `LIST`, `IOCTL`, `IOCTL$`, `SETMEM`, `FILEATTR`, `FRE` of any argument, and after `CALL` `TRON`,
  `TROFF`, `MID$` and `_MEMGET`, are "Command not implemented", also when not reached; `CALL FRE(0)` is "Expected
  CALL sub-name [(...)]". **`RESET` is no
  stub**: `RESET`, `CALL RESET`, `CALL CLOSE` and `CALL CLOSE(1)` compile and close the files.
- `RUN` matches D5 in each form. In a `$CONSOLE:ONLY` program the output after `RUN label` is lost (the program
  goes on in screen 0, `sub_run_init`); `RUN` with no argument and `RUN` from a procedure go through the start of
  the main module and print again. `COMMON SHARED` and `STATIC` variables are cleared. `RUN` of a missing file is
  error 53 and the program goes on. `RUN` of a label of a SUB, of a numeric variable and of no label is "Label
  'x' not defined"; of `1 + 2` "Illegal string-number conversion".
- `RESUME` in a SUB is the main module's C++ line in the SUB's function (D7). In a FUNCTION the C++ build fails.
- Every parenthesised place measured is accepted and is the place: `SWAP`, `READ`, `INPUT`, `LINE INPUT`,
  `LBOUND`/`UBOUND` (any depth, a dimension, dynamic arrays, parameters).
- Omitted arguments: the last one left out is "Invalid expression" for `POINT`, `_RGB32`, `_RGBA32`, `MID$`,
  `LEFT$` and a program FUNCTION; any other is "Incorrect number of arguments" for `POINT`, `_RGB`, `INSTR`, `MID$`
  (also the last of `_RGB` and `INSTR`) and "Syntax error" for `_RGB32`, `_RGBA32`, `LEFT$` and a program
  FUNCTION; `INSTR(, a$, b$)` compiles (stays marked). Each error's message names the function, so its note is the
  old compiler's wording for that function; two old messages hold digits (`POINT({0|1|2|3})`, `QB64(PE)` of
  `$NOPREFIX`), which `old_wording.py` takes for text of one program, so those two errors have no note.
- A program FUNCTION as a statement: `f 1` and `f(1)` "Syntax error", `CALL f(1)` "Expected CALL sub-name [(...)]",
  for every kind of name.
- Libraries: with a library FUNCTION `h_l&` and a program SUB `h_l`, the statement `h_l` calls the SUB, `h_l&(2)`
  and `h_l(2)` the library; with a library SUB `h_v` and a program FUNCTION `h_v&`, `h_v 3` calls the library and
  `h_v&(2)` the program.
- `ALIAS` outside a library is accepted quoted, empty and on a FUNCTION; the alias names nothing.
- Colours beyond the palette in mode 0 (D10): the value changes from build to build (0 mostly; 32, 1 and 179 seen),
  and `_RED(65535, 0)` made `qb64pe.exe` crash (exit 139) in most builds. Those programs were not kept. In mode 13
  each value above 255 is "UNEXPECTED INTERNAL COMPILER ERROR!".
- The flag between two settings (D9): the model holds (below).
- Blocks across files (D8): every kind compiles in both directions, also `ELSE` in the included file, `NEXT j, i`
  there, two levels down, `EXIT FOR` there and a FUNCTION. A block an included file leaves open is "FOR without
  NEXT"; `NEXT j` closing `FOR i` across files "Incorrect variable after NEXT".
- `STRING * n` members: a `CONST` of an expression is taken; a `CONST` below the block or with a suffix is "Invalid
  number/constant after STRING * type".
- `_SCREENHIDE` as a value: -1 hidden, 0 shown and under `$CONSOLE:ONLY`.
- `PALETTE USING`: `a()` is accepted; a `SINGLE` or `_INTEGER64` array "Array must be of type INTEGER or LONG"; a
  scalar or nothing "Expected PALETTE USING array-name"; at run time a value of -1 or 64 and too few elements are
  error 5.
- Neighbours of the parser's marks: `ON KEY(1 GOSUB` and `ON TIMER(1 GOSUB` "Expected )"; `WRITE #1;` "Expected #
  ... ,", `WRITE 1;` "Invalid expression"; `DEFLNG a-c AS LONG` "_DEFINE: Expected ,", `DEFSTR s AS STRING` "_DEFINE:
  Expected - or ,", `_DEFINE AS` "_DEFINE: Expected ... AS ..."; `x = AND 1` "Expected variable/value before
  'AND'"; a block `IF THEN` "Syntax error", `ELSEIF THEN` "Expected ... THEN"; `SHARED q` in the main module
  "SHARED must be used within a SUB/FUNCTION"; `2 ^ NOT x` compiles (.25).

### D2. Errors from the parser's marks: one rule per recorded form, not a wider `unhandled_token`

`unhandled_token` stays a heuristic. Each recorded rejection gets its own error where the parser knows the
form, with its own message:

- an operator where a value is expected outside a `CONST` (`x = OR`, `2 ^ _NEGATE`): the error "expected a
  value before `OR`" at the operator;
- `IF` with `THEN` where its condition should be;
- `;` in `WRITE`: the parser reads `WRITE`'s items with a rule of their own that rejects `;`;
- `ON STRIG(b GOSUB`, `ON KEY(n GOSUB` and `ON TIMER(t GOSUB`: "expected `)`" at `GOSUB`;
- `DEFINT a AS`, and `_DEFINE AS`: in the `DEFtype` parser;
- `CALL _MEMPUT(…)` and `CALL _MEMFILL(…)`: in `sema`, before the arguments are read (D4).

*Alternative:* make `unhandled_token` return no mark wherever the old compiler's error is known. Rejected: the
same token is a mark in other places where the old compiler accepts the form, and the guard against false errors
(tier 1 over every accepted program) only sees the programs we have.

### D3. Metacommands: the old compiler's list, closed

`Checker::meta` ends with three arms in place of the mark:

1. a name of the old compiler's list that this compiler does not build (`$ERRORLOCATION`, `$USELIBRARY`):
   still marked, until 4b;
2. `$NOPREFIX`: the error with the old compiler's message as the note;
3. any other line: the error "unknown metacommand `$…`", with the note "Syntax error".

The parser's metacommand lines in a comment (`$INCLUDE`, `$STATIC`, `$DYNAMIC`, `syntax\meta.rs`) are not
touched: a bare `$INCLUDE:` or `$DYNAMIC` line is not a comment, so it reaches arm 3. An inactive `$IF` branch is
not checked, as today.

### D4. The old compiler's stubs: a table check in `sema`

`supported_builtin` misses a stub's name, and today that is a mark. A name whose only entries in the built-in
table have `callname` `sub_stub` or `func_stub`, and which no rule of `sema` compiles, becomes the error
"`NAME` is not implemented in QB64pe", with the note "Command not implemented". The rule is placed after
`sema`'s own rules for `SWAP`, `CLOSE`, `RESET`, `READ`, `MID$`, `_MEMGET`, `_MEMPUT`, `_MEMFILL`, `_CONTINUE`,
`OPTION`, `DEF`, `ASC`, `LSET`, `RSET`, `PRINT`, `WRITE`, `INPUT`, `END` and `SYSTEM`, so those keep their rules.
After `CALL`, the rules apply as the old compiler applies them (measured, `v57_s11`–`s19`): `CALL SWAP(a, b)` is
`SWAP a, b`, `CALL CLOSE`, `CALL CLOSE(n)` and `CALL RESET` are `CLOSE`, `CLOSE n` and `RESET`, while
`CALL MID$(…)`, `CALL _MEMGET(…)`, `CALL _MEMPUT(…)`, `CALL _MEMFILL(…)`, `CALL TRON` and `CALL FRE(0)` are the
error. `RESET` is built as the old compiler builds it, `CLOSE` with nothing after it (`sub_close(NULL,0);`), and
`RESET` with anything after it is its "Syntax error - RESET takes no parameters" (`qb64pe.bas` 10893–10902). Code:
`check\builtins.rs`, `check\proc.rs` (`call_stmt`).

### D5. `RUN`: the old compiler's C++

The IR gets `StmtKind::Run(RunTarget)`, where `RunTarget` is `Restart`, `Label(LabelId)` or `File(Expr)`. The
C++ is the old compiler's: `sub_run_init(); sub_clear(NULL,NULL,NULL,NULL);` and then one of three things:

- in the main module, `goto S_0;` or `goto LABEL_x;`;
- in a procedure, `run_from_line=n; QBMAIN(NULL);`, with `n` the next index of a table that codegen writes to
  `runline.txt` (`if (run_from_line==n){run_from_line=0;goto LABEL_x;}`);
- for a file, `sub_run(e);`.

`sub_clear` is the one `CLEAR` already calls. A label that is also a SUB's name is "Name already in use" at the
`RUN` (`v28_x86`). The label rules are those of `GOTO` in the main module, plus the scope rule of `qb64pe.bas`
9790 (the label must be the main module's). Code: `check\flow.rs`, `ir`, `codegen-cpp\procs.rs`.

The old compiler types the element before it takes it for a label (`qb64pe.bas` 9786: `evaluate`, then a label
only when the type is not a string). So a name is a file name when it means a string: its suffix, a string `CONST`
of the name, or the type a variable of the name has there (a `DIM … AS STRING`, `DEFSTR`), assigned or not
(review follow-up, `v57_r20`–`r24`). The name is not evaluated, so no variable is made for a label.

### D6. Places in parentheses, by unwrapping in the checker

`io.rs` `stmt_place` (`READ`'s and `SWAP`'s targets) and the array argument of `LBOUND`/`UBOUND` unwrap a
`ParenExpr` before they look for the place, as often as it is nested. `v57` shows the old compiler accepts every
form measured, `INPUT (a)` and `LINE INPUT (a$)` among them (its C++ is the plain variable's: `swap_32(&a, &b)`,
`a=func_read_float(…)`). The parentheses make no copy there (`SWAP (a), b` swaps `a`). That is a row of `SOMEDAY.md`'s "QB64pe
behaviours to review": in QB 4.5 the parentheses would be a value.

### D7. `RESUME` inside a procedure, as the old compiler writes it

The old compiler writes the main module's line for `RESUME` in a SUB, in the SUB's function (read with `-z`,
task 1.2): `if (!error_handling){error(20);}else{error_retry=1; qbevent=1; error_handling=0; error_err=0;
return;}` for `RESUME` and `RESUME 0`, the same without `error_retry=1; qbevent=1;` for `RESUME NEXT`, and `goto
LABEL_x;` in place of `return;` for `RESUME label`, with the SUB's own label. The `return` leaves the SUB and skips
its exit code (the lock of its memory and the freeing of its strings), as the old compiler's does; the recordings
(`v57_x01`–`x08`) show nothing of that, and this compiler writes the same line. So the IR's `Resume` needs nothing
new: the mark goes, and codegen writes the statement in a procedure as in the main module. In a FUNCTION the
`return;` without a value does not build in the old compiler (`v57_x09`): that is a compile error here, as other
C++ failures of the old compiler are (`m4-line-numbers-whole-arrays`, `cpp_line_problem`).

### D8. Blocks across included files: a cursor over the stream of statements

The old compiler includes text, so a block may open in one file and close in another. Two parts:

- **The parser.** When it parses an included file, it hands the included parser the open blocks of the including
  file. A closer in the included file that matches one of them closes it across the boundary. The parser records
  the pair in the tree set, `cross_closers` (the block's node in one tree, the closer's node in the other). The
  including parser learns which of its blocks were closed and ends those block nodes after the include
  statement. The reverse case works the same way: a block left open at the end of an included file stays on the
  stack the including parser continues with. A closer it then meets is paired with it.
  A closer met this way stands in its own tree as a statement node of its usual kind (`NextStmt`, `LoopStmt`,
  `BlockEnd`, `ProcEnd`; a branch as an `ElseIfBranch`, `ElseBranch` or `CaseClause` that holds only its header),
  and its key is in `cross_closers`. A block whose closer stands in another tree is in `cross_open`.
- **`sema`.** It reads the trees as one stream of events in text order (`check\walk.rs`, the cursor): a statement,
  the start of a block, a branch of it (`ELSEIF`, `ELSE`, `CASE`), and its end (its closer; `NEXT j, i` gives one
  end per name; a block that ends with its node, its closer missing or nested, gives one at the end of its node
  unless it is in `cross_open`). Each include statement is followed by its tree's events, in place. A block's
  checker takes the events after its start up to its end, wherever they stand, so a SUB opened before an include
  and closed inside it takes the included statements up to `END SUB`, and the rest of that file goes back to the
  module level, as in the old compiler. The closer's node comes with its end event, so `NEXT`'s variable and the
  end line need no lookup. Pass 2 (`check_statement`, the block checkers, `proc_def`) and the label pre-pass
  (which body a label belongs to) read the stream; the other passes walk the files in order and need no block
  structure. Review follow-up: one of them did, pass 1's collector of the `CONST` names above each `TYPE` block
  (whether a `CONST` stands in a SUB), so it reads the stream too (`v57_b22`–`b25`); the default types follow the
  text alone. The parser's closer inside a `$IF` that crosses it now also pairs with a block of another file
  (`v57_b21`: one error, at the `NEXT`). The stream replaces today's recursion in `block_body` and `check_statement`; the order of the
  statements it produces is the same for every program without a crossing. Tier 1 and the `typed` snapshots show
  that.

*Alternative:* splice the included tokens into the including tree. Rejected: a tree is one file (its spans are
offsets in that file), and every diagnostic and every line number would need a second kind of span.

### D9. The flag between two settings: follow the recordings, or keep the mark

`v26_x27` prints nothing from any branch, and `v26_x34` is "Name already in use (k)" at the use of `k`. This
points to the old compiler's first pass taking one branch (the running value, 2) and its main pass another
(`actual`, 1). The main pass then drops the code it compiles because its line count no longer matches the first
pass's. `v57` tests that model, and it holds on every recording (`v57_g01`–`g11`): the disagreeing `$IF` compiles
no branch (a `PRINT`, a `DIM`, a SUB call, an assignment, the `$ELSE` branch alone, the reverse order of the two
settings); a `CONST` in the branch the flag's value on that line chooses (the first pass's) is the error of a name
already in use at the first later use, and none when the name is never used; a `CONST` in another branch leaves
its name a variable; a `DIM` in any branch is nothing. `$ASSERTS` and `$ASSERTS:CONSOLE` both set `_ASSERTS_` to
1, so no line lies between them (`v57_g08`). So `sema` drops the branches of a line in `passes_disagree`, and
notes the `CONST` names of the chosen branch: the first use of one below the `$IF`, in any body, is the error. The
parser already parses the chosen branch (it chooses by the value on the line); `sema` checks none of its
statements but the `CONST` names.

Review follow-up (`v57_j01`–`j06`): the label pass drops the same branches, so a jump to a label or line number in
one is "Label not defined", as recorded; before, the label was entered and never placed. One step function
(`pp_drop_event`) serves both walks. A block opened in the dropped part is still skipped whole, but the
metacommands in it are counted, so a block that crosses the `$END IF` (a parse error, also in the old compiler)
ends the drop at its own end and the lines after it are checked.

### D10. `_RED` beyond the palette in a `CONST`: 0, with a divergence row

The old compiler evaluates the `CONST` with libqb inside `qb64pe.exe` and reads past a palette of 256 entries
(`const_eval.bas` 977). Its value comes from that run's memory: `v57` built two equal programs three times each
and saw 0 mostly, and 32, 1 and 179 in some builds; `_RED(65535, 0)` made `qb64pe.exe` crash (exit 139) in most.
So those programs are not kept in the family (their recording would change), and by working rule 12 the value
here is 0, the value most builds give, with a `DIVERGENCES.md` row that lists the measured values. In screen mode
13 every value above 255 is the old compiler's "UNEXPECTED INTERNAL COMPILER ERROR!" (`v57_c05`–`c08`), which this
compiler already rejects. Code: `colors.rs`.

### D11. The rest, small

- **`PALETTE USING`:** `sema` reads `array(index)`, `array` or `array()` after `USING`, an `INTEGER` or `LONG`
  array, and passes the element's address, the bytes from it to the array's end, and its bits (16 or 32) to
  `sub_paletteusing` through a `byte_element`, as `qb64pe.bas` 7239–7270 does (read with `-z`, task 1.2:
  `sub_paletteusing(byte_element((uint64)(&elem),(2*(a[2]&1)*a[5])-(2*index),byte_element_n),16)`). The parser's
  mark at `template.rs` 46 goes. A screen program records the palette it sets.
- **`DECLARE SUB … ALIAS` outside a library:** the parser accepts `ALIAS name` there (quoted, bare or empty), and
  the line stays ignored, as every `DECLARE` line is.
- **A library procedure beside a program procedure:** as `v57_l01`–`l04` record, the name as a statement is the
  SUB and the name as a value (with or without its suffix) the FUNCTION, whichever is the library's. The C++ is
  the old compiler's: `SUB_H_L();` and `h_l(2)`, `h_v(3);` and `FUNC_H_V(…)`.
- **`STRING * n` member with a `CONST`:** `member` typing reads the constant by the rule of `DIM`'s
  (`check\decl.rs` 380), a constant without a suffix defined above the block, of any expression. The constants are
  known in pass 1 since `m4-memory-records` (its D-design for member bounds). A constant below the block or with a
  suffix is the old compiler's error.
- **`_SCREENHIDE` the function:** a plain row of the built-in table (`func__screenhide`, `int32`).
- **`SHARED` in the main module:** the error with the old compiler's note, declared name or not.
- **A program FUNCTION as a statement:** an error, "Syntax error" without `CALL` and "Expected CALL sub-name" with
  it, for every kind of name.
- **Omitted arguments:** the error where `v57` records one for that function, with a message per class (the last
  argument; another of a function with optional arguments; another of any other), the mark otherwise.

## Risks / Trade-offs

- [The cursor walk of D8 touches every program's checking] → It is built first in its task group. It changes no
  output for a program without a crossing: tier 1, the `typed`/`ir` snapshots and the slice list run before the
  crossing itself is built.
- [An error where the old compiler accepts a neighbour] → Each error rule names its recorded form. Tier 1's guard
  (`known_false_errors.list` empty) runs over every accepted program.
- [`RUN` in a procedure re-enters `QBMAIN`, and its stack grows with each run] → That is what the old compiler
  does too. `v57` runs it a bounded number of times.
- [D9's model may not hold] → It held on every `v57` recording. A form outside them (a `$IF` nested in the
  branch, a branch with a SUB) follows the same rule; the risk is a form whose first pass does more than define a
  `CONST`.
- [`RUN` of a second program in the runner] → `v57_r09` runs `sort` with no input, which prints nothing, so the
  test reads only that nothing follows the `RUN`; the output of a second program depends on the console (`cmd`
  printed its banner with the Windows version) and is not recorded.
- [`RESUME` in a SUB returns without the SUB's exit code] → It is the old compiler's C++, kept as it is; a row of
  `SOMEDAY.md`'s "QB64pe behaviours to review".
