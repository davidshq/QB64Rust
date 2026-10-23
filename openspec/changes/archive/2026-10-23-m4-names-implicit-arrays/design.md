# Design

## Context

See `proposal.md` for why. What the code has now:

- **Implicit arrays** are made by `Checker::implicit_array` (`sema\src\check\places.rs`). In the main module
  below `$DYNAMIC` the array's allocation is a `StmtKind::Allocate` with `joined: true`. The IR lowering holds it
  in `Lowerer::joined` and puts it first in the next statement's ops (`ir\src\lower.rs`, `emit_as`), or alone
  before a label (`flush_joined`). Where that would not be the old compiler's place, `Checker::unjoined` with a
  `NoJoin` reason (`check\mod.rs`) makes the use a mark:
  - `Header`: the line of a block (`check\blocks.rs`);
  - `Item`: a later item of `PRINT` or `READ`, an item of `WRITE` or `PRINT #`, a target of `INPUT`
    (`check\io.rs`);
  - `RedimBound`: a bound of `REDIM` (`check\places.rs`).
- **Errors are pending and checked at fixed points** (`ir\src\lib.rs`, crate documentation): after each item of
  `PRINT` and `WRITE`, each target of `INPUT`, not in `READ`. `PRINT`, `PRINT USING`, `WRITE`, `INPUT` and `READ`
  are each **one** op holding all their items (`Op::Print`, `Op::PrintUsing`, `Op::Write`, `Op::Input`,
  `Op::Read`), and the emitter writes them item by item as the old compiler does.
- **`STATIC a(bounds)`** is a dynamic array allocated when the program starts (`Shape::Dynamic { at_start }`,
  constants only). Its allocation goes into `maindata.txt` after the array's descriptor
  (`codegen-cpp\src\lib.rs`, `per_program_var`; `arrays.rs`, `allocate_at_start`). That fragment runs again
  whenever a handler is entered. Bounds that are not constants by their spelling are marked (`places.rs`, the
  declaration of arrays). So is a `STATIC` beside a shared main array (`static_array_free`).
- **Names the old compiler rejects are marked where the rule's edges were not measured:**
  - `implicit_array` and `not_an_array` (`places.rs`): a built-in function's name as a place, a name with one
    leading underscore, a FUNCTION's name, `x()` of a name that is no array;
  - `Checker` call statements (`check\proc.rs`, the arms after the SUB lookup): a name that is no SUB, a
    keyword or a built-in with a suffix;
  - the parser's `not_supported_statement` (`syntax\src\parser\mod.rs`): `LEN` as a statement;
  - `label_ref` and `restore` (`check\flow.rs`): a target named like a built-in;
  - `reserved_proc` (`check\decl.rs`): a FUNCTION named like a built-in with a suffix;
  - `not_an_assignment` (`check\builtins.rs`): `WIDTH (80) = 80`;
  - `parser\decl.rs` and `parser\proc.rs`, and the call in `check\proc.rs`: a declared name with blanks around
    `.`.
- **The built-in table** (`tools\builtins\builtins.json`, `crates\builtins`) holds every built-in of
  `qb64pe.bas` with its suffixes (`mayhave`, `musthave`). It also holds the functions and constants of the
  auto-included files (`auto_include`, among them `_IKW_ENCODEURL`, `_WHATISMYIP`). `implicit_array`'s comment,
  that the table "does not know every spelling" (`_CONNECTIONADDRESS$`), dates from before that: the row has
  `mayhave: "$"`.
- **The old compiler's wording** is a note after our error, paired from recorded rejections
  (`tools\diagnostics\old_wording.py`, `crates\base\src\old_wording.tsv`). A new error on a line the old compiler
  rejects gets its note when the table is regenerated.

## Goals / Non-Goals

**Goals:**

- Every program of the proposal passes the verification suite: the accepted ones run with the recorded output,
  and the rejected ones fail with an error that is not marked.
- The family `v55` (D9) settles the edges of each rule before the code. A form it does not record keeps its mark.

**Non-Goals:**

- `CALL fs$5(…)`, a FUNCTION called as a statement, and the other `fs$5` forms (`v21_x54`–`x57`, `proc_errors`):
  step 4, as design D6 of `m4-line-numbers-whole-arrays` placed them.
- `CALL SWAP(a, b)` (`v26_x128`), a target of `READ` or an operand of `SWAP` in parentheses (`v22_x84`,
  `v22_x103`): statement forms, step 4.
- The memory and record forms (`GET`/`PUT` of a variable that is no array or of an undeclared array, `VARPTR` of
  a variable that is no array element): step 3, though some are "a name that is no array".
- A label named like a SUB or FUNCTION (`flow.rs`, `unclear_proc_label`): not in the list; it stays as it is.
- Any other statement's name as an array (`LSET (a) = "x"` and the like, `not_an_assignment`): only `WIDTH` was
  measured to be one.

## Decisions

### D1. An allocation joins its statement at a position, not only first

`StmtKind::Allocate`'s `joined: bool` becomes `join: Join` (`sema\src\lib.rs`):

- `Join::None`: a statement of its own (today's `false`).
- `Join::First`: the first part of the next statement (today's `true`).
- `Join::Before(n)`: before part `n` of the next statement. The parts are numbered in the order they are
  evaluated: the items of a `PRINT`, `PRINT #` or `WRITE`; the items before `USING`, then the format, then the
  values of a `PRINT USING`; the targets of `READ`, `INPUT`, `INPUT #`, `LINE INPUT` and `LINE INPUT #`.

`implicit_array` gives `Before(n)` where `NoJoin::Item` marks today. The checkers of those statements
(`check\io.rs`) number their parts. The lowering (`Lowerer::joined` keeps its `Join`) does not split the next
statement's op: **the op carries its allocations**, `allocs: Vec<(usize, Allocation)>` on `Op::Print`,
`Op::PrintUsing`, `Op::Write`, `Op::Read` and `Op::Input`, and the emitter writes allocation `(n, a)` just before
the code of part `n`, as the old compiler does. With no allocation the field is empty and the C++ is unchanged.

What the old compiler writes (`qbx.cpp` fragments of `qb64pe.exe -z`, read in task 1.2; `<<ALLOC x>>` stands for
the allocation block, `if (x[2]&2) error(10); else …`):

- `PRINT " x"; dp(1); " y";` (`v24_x264` case 1): `qbs_set(tqbs," x"); if (is_error_pending()) goto skip4;
  qbs_print(tqbs,0); <<ALLOC DP>> qbs_set(tqbs, … dp(1) …); if (is_error_pending()) goto skip4; …`. The check
  after the item that names the array ends the statement, line end included.
- `READ a, dq(1), b` (case 2): `*a=func_read_float(…); <<ALLOC DQ>> tmp_long=array_check(…); if
  (!is_error_pending()) dq[tmp_long]=func_read_float(…); *b=func_read_float(…);`. No check ends the statement:
  `b` is read with the error pending, which takes no item and stores 0 (the recording: ` 4  2  0`, then the next
  `READ` gets 5), as `Op::Read` already does.
- `WRITE i, dr(1)` (case 3): `qbs_print(qbs_add(ltrim(str(i)),","),0); if (is_error_pending()) goto skip14;
  <<ALLOC DR>> qbs_print(ltrim(str(dr(1))),1); …`. The separator is part of the item before it, as our
  `write` emits it already, so `2,` is written before the error.
- `INPUT a, ia(1), b` (`v24_x265` case 1): `qbs_print("? ",0); qbs_input_variableoffsets[1]=&a; <<ALLOC IA>>
  qbs_input_variableoffsets[2]=&ia[array_check(…)]; …; qbs_input(3,1);`: after the prompt, in the phase that
  takes the targets' addresses, before the line is read (with an error pending nothing is read; `v35_x62`).
  `LINE INPUT ls$(1)` without a prompt has the allocation first; `LINE INPUT "p"; fx$(1)` after the prompt.
- `INPUT #1, a, fa(1), b` (probe, not kept): `tmp_fileno=1; …; *a=func_file_input_float(…); if
  (is_error_pending()) goto skip1; <<ALLOC FA>> tmp_long=array_check(…); if (!is_error_pending()) fa[…]=…; if
  (is_error_pending()) goto skip1; …`. `LINE INPUT #1, fl$(1)`: after the file number's check. `PRINT #1, a;
  fp(1); b` and `WRITE #1, a, fw(1), b`: as on the screen, after the file number. The file number is evaluated
  once; a split op would evaluate it twice, which is why the op carries the allocations.
- `PRINT USING "##"; a; fu(1)`: after the format and the values before it, before the value that names the
  array; `PRINT USING "##"; fv(1)` after the format.
- The first item of a console `PRINT` and the first target of `READ`: before them, which `Join::First` gives
  (`PRINT fz(1); a`: `tqbs=qbs_new(0,0); <<ALLOC FZ>> qbs_set(tqbs, …)`).

*Alternative*: split the op at each `n` and put an `Op::Allocate` between the halves. Rejected after the C++:
the halves would evaluate a file number twice, `PRINT USING` cannot be split (one format, one position in it),
and a `WRITE` half would need a separator rule of its own. *Alternative*: an expression kind that allocates and
then gives its value (`ExprKind::AllocThen`). Rejected: the old compiler writes the allocation as a C++
statement between the item's lines, not inside an expression.

### D2. A block's line carries its own allocations

The allocations made while a header is checked are collected by the header's checker (`check\blocks.rs`,
`unjoined(NoJoin::Header)` becomes a collector), not pushed. Where they go depends on the header, as measured in
`v24_x262`:

- **Before the block**, once (`Join::First` on the block's first statement): `IF`, `FOR` (its start, end and
  `STEP`), `WHILE`, `DO WHILE` / `DO UNTIL` at the top, `SELECT CASE`'s selector. `if_block`, `for_loop` and
  `select` already take a joined allocation into the header's statement (the `IF` test, the `FOR` header's
  `AssignAll`, the selector's copy). A loop tested at the top runs it once, **in the statement of its first test
  only** (case 7 of `v24_x262`: `WHILE f(1) < 2` raises no error 10). Today `loop_block` places the loop's top
  label first, and `flush_joined` makes the allocation a statement of its own. With an allocation pending,
  `loop_block` now writes the first test apart: `[allocations, Branch(leave → exit, Skip)]`, a `Jump` to the
  body, then the top label and the plain test the loop jumps back to.
- **With the header's own test, each time it runs**: `ELSEIF` (not reached: not allocated, case 12), `LOOP UNTIL`
  / `LOOP WHILE` (every round: case 6 raises error 10 on the second) and `CASE` (case 10). These are new fields
  of the block's parts: `Branch::allocs` (`sema::Branch`), `LoopTest::allocs` and `Case::allocs`. The lowering
  (`if_block`, `loop_block`, `select`) puts them first in the statement that tests that part.

An error of such an allocation is an error of the header's statement, as an error in its condition is.
`v55_a_implicit_headers` records each header run twice under `RESUME NEXT`, the second allocation raising error
10. The old C++ is `S_n:; <<ALLOC>> <test>`, all in the statement that `RESUME` re-runs from `S_n`:

- `IF a(1) = 0`, `IF s(1) <> 0`, `WHILE q(1) > 0`, `DO WHILE`, `DO UNTIL r(1) = 0`, a `CASE`: the test is `if
  (cond || is_error_pending())`, so the error takes the branch (the `IF` and the `CASE` run their body, the loop
  enters its body; cases 1, 13, 14, 15, 16: ` then`, ` less`, ` body` though the test is false). That is
  `OnError::Skip` on a branch that jumps past the body, as the lowering has it.
- `ELSEIF c(1) = 0` / `v(1) <> 0`: `}else{ <<ALLOC>> if (cond) {`: the value is used (`OnError::UseValue`), and
  the error is serviced in the first statement that runs after it, which it skips (case 19: neither ` elseif` nor
  ` else` is printed).
- `LOOP UNTIL d(1) = 0`: `<<ALLOC>> }while(!cond && !is_error_pending());`: the error leaves the loop (case 5).
- `FOR k = m(1) TO 1`: `<<ALLOC>> fornext_value=…; …; if (is_error_pending()) goto fornext_error;`: the body runs
  with `k` as it was (case 17: ` body 5`), as the lowering's header has it.
- `SELECT CASE g(1)`: `<<ALLOC>> sc=…;` and the event check right after: the selector is a statement of its own,
  and the `CASE` tests run as usual after `RESUME NEXT` (case 18).
- A `CASE` after one that matched allocates nothing (case 8: `UBOUND` raises error 9).

### D3. An array first named in a bound of `REDIM`

`NoJoin::RedimBound` stops marking. In the main module the array is dynamic wherever it stands
(`Shape::Dynamic`) and its `Allocate` takes the `REDIM`'s mode (`AllocMode::Redim`, or `Preserve` under
`_PRESERVE`) with `Join::First`. So each run makes it anew or keeps it, and raises no error 10 (`v24_x266` cases
1, 2, 4; `x268` cases 1–3). A later `REDIM rc(20)` of it works (`x268` case 2). In a procedure it is the
procedure's implicit array, which nothing allocates (`x268` case 5: error 9 at each use), as any other there.
`DIM dv(dw(1) + 3)` keeps the `Dim` mode below `$DYNAMIC` and raises error 10 on the second run (`x266` case 3,
`x268` case 4). Code: `implicit_array`, where `self.stmt.no_join == Some(NoJoin::RedimBound)`; the mode comes
from the `REDIM` being checked (a field of `Checker::stmt`).

### D4. `STATIC` bounds whose value is known when compiling; `STATIC` beside a shared array

**The bounds.** `STATIC v(2 + 3)` is allocated at the start with the value the bound folds to (`places.rs`,
where `start_bounds` is taken). A bound that is constant by its value, as `constant(&e)` gives it after
folding, is taken. `Shape::Dynamic.at_start` keeps holding numbers, since the old compiler's start code
evaluates the expression once with nothing in it that varies. A bound that names a variable of the procedure (a
parameter or a local) is a compile error. The old compiler writes it into the main module's code and its C++
build fails (`v24_x121`): the message says so, as `cpp_line_problem` does for a line number. The old compiler
writes the bound's text into `maindata.txt` (`v24_x280`: `alloc_new_desc[5]=( 2 + 3 )-alloc_new_desc[4]+1;`).
Measured in `v55_c_static_bounds` and `v55_x73`–`x75`: a `CONST` of the main module or of the SUB, `*`, `-`, `\`,
`&H4`, a range `-1 TO &H4`, and `LEN("abc")` give their value (taken). A shared main-module `n` gives its value
when the start code runs: 0 at the start (`x74`: `v(0 TO 0)`, error 9), but its later value when a handler
enters that code again, so it is no value fixed at the start and stays marked. So does a FUNCTION in the bound
(`x75`): it runs before the console is set up, and what it prints is lost.

**Beside a shared array.** `STATIC a(5)` in a procedure below `DIM SHARED a(3)` of that name and type names the
shared array: in the procedure `a` is the shared array. Its start allocation is a second allocation **of the
shared array**, written after the shared array's own in `maindata.txt` (`v24_x272`: the shared array's
`mem_static_malloc` with `[2]=1+2`, then `if (__ARRAY_LONG_A[2]&2){ error(10); }` of the `STATIC`'s allocation).
A static shared array is allocated by then, so the program ends at the start with "Duplicate definition" on line
1, before any `ON ERROR` runs (`v24_x272`; also when the SUB is never called, `v55_d3`). The IR gains
`Program::start_allocations: Vec<(VarId, Vec<(i64, i64)>)>`, the allocations another declaration adds, written by
`codegen-cpp` after `per_program_var`. A shared array that is dynamic and not yet allocated at the start (below
`$DYNAMIC`) is allocated by it instead (`v55_d1`: `UBOUND` 5), and the main module's `DIM` then raises error 10;
a `REDIM SHARED` gives it its own bounds (`v55_d4`). A `STATIC` array of another type is an array of its own
(`v55_d2`).

*Alternative*: an error, since the program cannot run. Rejected: the old compiler compiles it, and a program
whose shared array is dynamic does run (`v55_d`).

### D5. `x()` of a name that is no array

`implicit_array` with `dims == 0` makes the implicit array of **one** dimension, as for `LBOUND(name)`, then the
whole array is its first element (design D4 of `m4-line-numbers-whole-arrays`). `LEN(x())` is its size in bytes,
`(11 - OPTION BASE) * element size`: 44 for `SINGLE`, 88 for `DOUBLE` (`v48_x04`, `x14`, `v55_b`), 40 under
`OPTION BASE 1` (`v55_b2`). Below `$DYNAMIC` it is allocated as any implicit array (a second run raises error
10, `v55_b2`). A later `x(5) = 1` uses it (`UBOUND` 10); a later `x(1, 2)` is a compile error ("Cannot change the
number of elements an array has!", `v55_x68`). In a procedure it is the procedure's implicit array, which is not
allocated: `x()` and `LEN(x())` give 0 with no error (`v55_b` cases 8–9, `x72`), as an erased array's whole value
does (`v54_d`). `xs$()` is the empty string (`x71`); `LEN(xs$())` is a compile error ("Cannot pass array of
variable-length strings", `x70`), as `LEN` of a whole string array is already.

### D6. Names that call nothing become the old compiler's errors

Each rule below is a real error (`Checker::error`), and its wording comes from the table. The follow-on rule
(a marked declaration earlier in the file) still drops them, as for every error. What `v55_x` recorded (the old
compiler's message in quotes; "passes" where our compiler already gives a real error):

- **A name with one leading underscore and parentheses** that calls no built-in function with that spelling (no
  row of the table or of `auto_include` with that name and suffix): "Invalid array name" (`_foo(1)` as a place
  and as a value, `x54`; `_TRUE(1)`, `_CRC32$(…)`, `_CAST&(…)`, `_OPENCLIENT&(…)`, `_IIF$(…)`, `x55`).
  `_CONNECTIONADDRESS$(h)` calls the function (`x56`), and `__foo(1)` is an implicit array (`x57`, passes). Code:
  `implicit_array`, the `underscore` arm. A tier-1 test of `crates\builtins` checks that every function spelling
  of `builtins.json`, `mayhave` suffixes and `auto_include` included, is found by `is_builtin_function`. That is
  the fact the mark stood for. (`SUB _foo` and `FUNCTION _bar` are "Invalid name", and `_foo = 2` "Invalid
  variable name": `x52`, `x53`, `x58`, all pass.)
- **A built-in function's name, or a FUNCTION's, as a place**: "Expected variable =" for a store (`rnd(1) = 2`,
  `TAB(5) = 1`, `sin(1) = 2`, `CHR$(1) = "a"`, `x10`, `x11`; `f(3) = 5` for a FUNCTION `f`, also in a SUB below it,
  `x20`); "Expected variable" for a target of `READ`, `INPUT`, `LINE INPUT` and an operand of `SWAP` (`READ
  ABS(a)`, `READ TAB(1)`, `READ f(1)`, `INPUT ABS(a)`, `INPUT f(1)`, `LINE INPUT CHR$(1)`, `SWAP ABS(a), b`,
  `x13`–`x18`). `LEN(1) = 2` is "String expression or variable name required in LEN statement" (`x09`). The names
  that are statements too give that statement's error, which we give already (`TIMER`, `STRIG`, `PLAY`, `SCREEN`,
  `PEN`, `KEY`, `MID$`, `ASC`; `rnd = 2`; `f&(3) = 5` beside a FUNCTION `f`, "Name already in use"; `x01`–`x08`,
  `x12`, `x19`, all pass). None was accepted. Code: `implicit_array` (the `is_builtin_function` arm and
  `procs_by_name`), `check\io.rs`, `check\builtins.rs` (`LEN` as a statement).
- **A name used as a statement that is no SUB**, no keyword and no built-in: "Syntax error", with or without
  arguments, a variable's name too, also after `THEN` (`x 1`, `UndefinedSub`, `v` after `v = 1`, `IF c THEN
  nosub`, `IF c THEN nosub 1`, `IF ASC("") = 7 THEN jumped`; `x24`–`x26`, `x33`, `x34`, `v17_b_if_then_label`).
  `CALL` of it, with or without arguments: "Expected CALL sub-name [(...)]" (`x27`, `x28`). A built-in statement
  written with a suffix it does not take: "Syntax error" (`KILL$ "a"`, `_ASSERT$ 1`, `CLS$`, `BEEP%`, `PRINT$
  "a"`; `v22_x20`, `v33_x50`, `x21`–`x23`). A built-in function as a statement: "Syntax error" (`LEN("a")`, `LEN
  "a"`, `v20_x06`, `x29`; `CHR$(65)`, `ABS(1)`, `ABS 1` pass already, `x30`–`x32`). `LEN` is a keyword to the
  parser, which marks it (`not_supported_statement` gives an error for a word that is a built-in function). Code:
  `check\proc.rs`, the last two arms of the call statement; `syntax\src\parser\mod.rs`.
- **A jump to a built-in's name**: "Invalid label!" for `GOTO` and `ON … GOTO`, "Invalid label" for `GOSUB`,
  `RESTORE` and `ON ERROR GOTO`, whether a line of that name stands in the file or not (`GOTO BEEP`, `GOTO CLS`,
  `GOTO SOUND`, `GOTO abs`, `ON 1 GOTO BEEP`, `GOSUB BEEP`, `RESTORE BEEP`, `ON ERROR GOTO BEEP`; `v29_x78`,
  `x79`, `v27_x63`, `x36`–`x38`, `x42`, `x43`, `x45`). `declare_label` already takes `BEEP:` as the statement. A
  line that is a built-in **function's** name and a colon (`abs: l = 1`, `LEN: l = 1`) is "Syntax error" on that
  line, before any jump to it (`x35`, `x39`–`x41`, `x44`). Code: `label_ref` and `restore` (`check\flow.rs`), where
  `marked_label_name` is true for a built-in; `declare_label` for the line.
- **A procedure named like a built-in with a suffix it does not take**: "Name already in use" for a FUNCTION
  (`FUNCTION exp$`, `len$`, `abs%`, `timer#`; `v52_x28`, `x47`, `x49`, `x51`). A SUB with a suffix is "Type
  symbols after a SUB name are invalid" (`SUB exp$`, `SUB cls$`, `x46`, `x50`). `FUNCTION chr&` is accepted
  (`x48`, passes): `CHR$` requires its `$`, so `chr` is free, as the requirement "Reserved names" has it. Code:
  `reserved_proc` (`check\decl.rs`), both marks.

*Alternative*: keep the marks and list the programs as known. Rejected: the marks stand for "the old compiler may
accept this", and the table and `v55` settle that. A real error is also what the editor shows the user.

### D7. Declared names with blanks around `.`

`DIM a . b AS LONG` declares `a.b`, and `SUB a . b (n)` defines the SUB `a.b`. The old compiler drops the blanks
around the dot as it reads the line (`v47_x17`, `x18`), as it does at a use (`places.rs`, the dotted names of
`v19_dot_blanks_*`). The parser's declared name (`parser\decl.rs`, `parser\proc.rs`) takes `Name Dot Name …`
with blanks as one name node over those tokens. `sema` reads its text without the blanks, through the helper
that reads a dotted name at a use. The call `a . b 3` (`check\proc.rs`, the arm with `unparsed_args` starting
with `.`) looks up the joined name. `v55_x` records, all accepted: `DIM a . b(3)` (an array `a.b`), `FUNCTION a .
b` called as `a.b(3)` and `a . b(4)`, `CALL a . b(3)` of a `SUB a.b`, a list `DIM a . b AS LONG, c . d AS
INTEGER`, `DIM a . b$`, `DIM SHARED a . b` (`x59`–`x61`, `x63`–`x65`). Beside a `TYPE` variable `a`, `DIM a . b
AS LONG` is "DIM: Expected ," (`x62`): an error, as the dotted name at a use beside such a variable reads the
member.

### D8. `WIDTH` as an array

`WIDTH (80) = 80` is an element of an implicit array named `WIDTH`. `not_an_assignment` (`check\builtins.rs`)
lets the line through to the assignment instead of marking it, and `PRINT WIDTH(1)` reads the array, since
`WIDTH` is no function (`v27_x51`). Only `WIDTH`: the other statements' names stay "name already in use".

### D9. The measurement family `v55`

Recorded with `qb64pe.exe` before the code (task 1.1); `verification\README.md` gains its line:

- `v55_a_implicit_headers`: below `$DYNAMIC`, an array first named in the line of an `IF` (block and single
  line), `ELSEIF` (reached and not), `LOOP UNTIL`, `LOOP WHILE`, a later `CASE`, `SELECT CASE`, `FOR`, `WHILE`,
  `DO WHILE`, `DO UNTIL`, each run twice under `ON ERROR` and `RESUME NEXT`, also where the test is false or no
  `CASE` matches, printing what runs after the error; a `CASE` after one that matched, then `UBOUND` of its array.
- `v55_b_implicit_whole`: `PRINT x()` and `LEN(x())` of a scalar's name and of a name never used, of `DOUBLE` and
  `LONG`, `xs$()`, then an element of it, in a SUB called twice. `v55_b2_implicit_whole_base1`: under `OPTION
  BASE 1`, and below `$DYNAMIC` run twice.
- `v55_c_static_bounds`: `STATIC` bounds of a main `CONST`, a SUB's `CONST`, operators, `&H4`, a range; `STATIC
  v(2 + 3)` erased and then allocated again by a handled error.
- `v55_d1`–`d4`: `STATIC a(5)` beside a shared array below `$DYNAMIC`, beside one of another type, in a SUB never
  called, beside one of `REDIM SHARED`.
- `v55_x01`–`x75`: one form each, as D4–D7 list them: built-in names as a place and as a target, statements with
  a suffix, names that are no SUB as a statement and after `CALL` and `THEN`, functions as statements, jumps to
  built-in names and lines named like built-in functions, procedures named like built-ins with suffixes, names
  with one or two leading underscores, the dotted declarations, `WIDTH` as an array, the whole array `x()` (`x68`–
  `x72`), `STATIC` bounds of `LEN("abc")`, a shared variable and a FUNCTION (`x73`–`x75`).

Also recorded in the batch: `tests\corpus\slice\s157_width_array` (task 3.2).

## Risks / Trade-offs

- [The error 10 of an allocation in the middle of a statement, after its first items, is the old compiler's
  accident] → it is kept, as the spec has it (requirement "Dynamic arrays"). It gets a row in `SOMEDAY.md`'s "QB64pe
  behaviours to review", as does the start allocation of a shared array (D4).
- [A rule of D6 turns into a false error on a program the old compiler accepts] → each rule covers only what the
  table or `v55` settles. Tier 1's walks over every corpus, upstream and snippet program, and the shrink-only list
  `tests\known_false_errors.list` (empty, may not grow), catch a false error on any program we hold. A spelling the
  family does not settle keeps its mark.
- [The `allocs` fields touch the ops every `PRINT`, `PRINT USING`, `WRITE`, `READ` and `INPUT` uses, and
  `loop_block` the loops tested at the top] → with no allocation `allocs` is empty and the loop is lowered as
  before, so the emitted C++ of every existing program stays the same. The call-site check and the slice list
  show it (`tests\corpus\slice\` has the `INPUT` programs).
- [`v55` contradicts a decision] → task 1.1 changes the delta specs and this design before any code, and
  `openspec validate --strict` runs after.
