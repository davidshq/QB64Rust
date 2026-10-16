# Design

## Context

See `proposal.md` for why. What shapes the approach:

- A plain built-in comes in as a row, a call-site program and a slice line (`DECISIONS.md` 2026-10-10). Of this
  change's names only `LOCK`, `UNLOCK`, `CHAIN`, `_SHL` and `_SHR` are plain. The others are special-cased by
  the old compiler, each in a way that was read in `qb64pe.bas` and measured (`study\00` §5, the `v30`
  paragraph).
- Everything these statements do when the program runs is libqb's. The check that the new compiler is right is
  therefore the call it writes (the call-site check, form by form) and one slice program per topic that shows the
  calls are reached in the right order with the right values.
- The parser already builds a node for each form: `UsingClause` in `PrintStmt` and `LprintStmt`, `FieldStmt`,
  `LsetStmt` (both `LSET` and `RSET`), `ExitStmt` with any keyword, and a `BuiltinStmt` read by the template of
  the file form for `GET #1, , x`. `sema` marks each "not supported yet" at one place: `check\io.rs` (`PRINT
  USING`), `check\mod.rs` (`LPRINT`, `FIELD`, `LSET`/`RSET`), `check\builtins.rs` ("`GET` on a file"),
  `check\blocks.rs` (`exit_loop`, any word that is no loop).
- Graphics `GET` and `PUT` already pass the bytes of an array from an element on (`StmtArg::Block`, written as
  libqb's `byte_element` in `codegen-cpp\src\builtins.rs`). The file form passes the same structure for any
  variable.
- The IR lowers `EXIT FOR` to a jump to the exit label of the innermost open loop of that kind (`lower.rs`, the
  `exits` stack). A `SELECT` has an end label but puts nothing on that stack.
- `FIELD`, `LSET` and `RSET` are not measured. The table marks `LSet` and `RSet` as stubs (the old compiler
  writes their code itself), and the parser's nodes for them were written from the syntax alone.

## Goals / Non-Goals

**Goals:**

- The twelve corpus programs compile with no diagnostic, and those that can be compared pass.
- Every accepted form writes the old compiler's calls; every rejected form is rejected with an error that is not
  "not supported yet".
- The result type of `_IIF`, `_MIN`, `_MAX` and `_CLAMP` equals the old compiler's for every pair of types, held
  by a test that reads the measured table.

**Non-Goals:**

- Re-testing libqb: what each format character prints and what each file error is are shown once per topic in a
  slice program, not exhaustively.
- Running `CHAIN` or `LPRINT` in a test.
- `MKI$`, `CVI` and their family, although programs with `FIELD` often use them.

## Decisions

### D1. PRINT USING is a statement kind of its own

`StmtKind::PrintUsing { to, before, format, values, newline }` in `sema` and the IR, beside `Print`:

- `to`: where it prints: the console, a file number, or the printer (D8).
- `before`: the items before `USING`, as `Print` holds them; written first as a plain `PRINT` with no line end
  writes them.
- `format`: one string expression, evaluated once, before any value (the old compiler copies it into a string
  of the statement's own).
- `values`: each a string, a number in the type its entry takes (SINGLE, DOUBLE, `_FLOAT`, `_UNSIGNED
  _INTEGER64`, or `_INTEGER64` for every other integer type, converted with no cast as the old compiler writes
  it), or a `TAB`/`SPC` with its count.
- `newline`: false when the statement's last element is `;` or `,`.

`codegen-cpp\src\io.rs` writes it as measured: the temporary string and the position counter, one call per
value followed by the jump to the statement's skip label when an error is pending, the flush for a `TAB` or
`SPC`, the final print, and the block after the skip label that prints what was built and keeps the error. The
IR's pending-error rule for statements does not apply inside it: the statement is one unit whose C++ holds its
own jumps, as the old compiler's does, and `validate.rs` treats it so.

A comma between values is a separator and nothing else. `USING` twice, a value of a user type, a missing
value and the other rejected forms (`v30_x01`–`x19`) are errors in `sema` or, where the parser already rejects
them, stay parse errors; task 4.1 says for each which it is.

*Alternative considered:* `Print` with an optional format. Rejected: a `Print` item is printed by its type, a
`PrintUsing` value goes to an entry chosen by its type with other conversions, and the error path differs; one
kind with two meanings would put an `if` into every place that reads it.

### D2. GET and PUT on a file: a rule of their own

`GET` and `PUT` get a second rule, `StmtRule::File`, for the table entries whose call is `sub_get` and
`sub_put`; the graphics entries keep `StmtRule::Graphics`. `builtin_stmt` already tries each form of a name in
table order and already tells the two apart by the first part (a point or `STEP` is graphics).

The third argument decides the call:

| Third argument | Call | Passed |
|---|---|---|
| none | `field_get` / `field_put` | file, position or `NULL`, mask |
| a variable-length string (variable, element, member) | `sub_get2` / `sub_put2` | the string itself for `GET`; its characters and length as a `byte_element` for `PUT` |
| anything else that is a place | `sub_get` / `sub_put` | the place's address and size as a `byte_element` |

The place and its size: a numeric variable (its type's size), a fixed-length string (its characters and
length), a `TYPE` variable or member (its size), an array element (the element alone, with the bounds check),
a whole array (from element 0, the bytes of all elements: what `StmtArg::Block` holds today). `StmtArg` gets
one more variant for "the bytes of one place"; `codegen-cpp` writes both through the one `byte_element`
temporary it has. The mask is 1 when the position is given.

Rejected in `sema` with the reasons measured (`v30_x20`–`x48`): a third argument that is no variable; a `_BIT`
variable or array; a whole array of variable-length strings; a `TYPE` with a `STRING` member; a whole member
array; a string as file number or position. `GET #1, , "lit"` is an internal error of the old compiler
(`x24`); here it is the plain "a variable is needed" error.

*Alternative considered:* the plain rule with a new slot kind. Rejected: the choice of entry point by the
argument's type is not something a slot can say.

### D3. FIELD, LSET, RSET: measured first

Task group 1 measures them before any code (programs `verification\v30_j…` and `v30_x100` on, the C++ read
with `qb64pe -z` outside the repository): the calls of `FIELD` for one and several parts; which targets are
accepted (a variable-length string variable, an array element, a member; a fixed-length string, a number); the
file number's forms; a `FIELD` on a file that is not random; widths beyond the record; a second `FIELD` on the
same file; what an assignment with `=` to a field variable does to it; `LSET` and `RSET` with a shorter and a
longer value, on a fixed-length string, on a `TYPE` value (QuickBASIC copies records with `LSET`), on a
number; whether `GET` with a variable changes field variables.

**What was found (task 1.1, 2026-10-13; `verification\v30_j`–`v30_l`, `v30_x100`–`x170`, 67 programs; the
facts are in `study\00` §5).** The shape expected holds, and no form needs more than a statement and a call:

- The buffer, the list of a file's field strings and a string's link to its field are all libqb's (a pointer
  in every `qbs`). For the compiler a field variable is an ordinary variable-length string: no new
  representation, and nothing changes in assignment, `GET` or `CLOSE`.
- `FIELD` is `StmtKind::Field { file, parts }`, each part a (string place, width): `field_new(file);` and then
  `field_add(place, width);` per part, with nothing between the calls. File and width are LONG slots. A place
  is a variable-length string variable, element or `STRING` member; a whole array `e()` is its element 0 with
  no bounds check, as the old compiler has it (`SOMEDAY.md`, "QB64pe behaviours to review").
- `LSET` and `RSET` are `StmtKind::Lset { right: bool, target, value }`: `sub_lset(target, value);` or
  `sub_rset`. The target is a string place of either kind (a fixed-length one is written as the string value
  libqb makes of its bytes, `qbs_new_fixed`), the value any string expression.
- Order: the file number, then for each part the target's index before its width; in `LSET` the target before
  the value. The old compiler writes both into one call and its C++ compiler evaluates them in that order; the
  IR must keep it when it takes a call out of an argument.
- The spec's scenarios were corrected: error 50 for widths beyond the record (the parts before it stay
  fields), `RSET` cuts a longer value at its end as `LSET` does, a `TYPE` value is rejected as a target of
  `LSET` (QuickBASIC copies records with it: `DIVERGENCES-QB45.md`), and scenarios were added for a second
  `FIELD`, `=` to a field variable, the errors of a file that is not random and the rejected forms.
- Nothing of the shape has to stay "not supported yet". One accepted form does (found when group 6 was built):
  a whole array `e()` as the string of `FIELD`, `LSET` or `RSET`, which the old compiler takes as the array's
  first element with no bounds check (`SOMEDAY.md`).

### D4. LOCK and UNLOCK: two plain rows

Template `[#]?[,[?][{To}?]]`, call `sub_lock` or `sub_unlock` with the mask 0, 1 or 3. If the plan of the
template (`qb64rust_builtins::passing`) does not give the mask of `LOCK #1,` (0, measured) or the errors of
`TO` in a wrong place, the fix goes into the template code, for every statement, as in `m3-sound` D1.

### D5. EXIT SELECT and EXIT CASE

`sema` keeps a stack of open `SELECT` blocks, each with whether a `CASE` is open and whether it is
`EVERYCASE`; a procedure starts with an empty one. `EXIT SELECT` and `EXIT CASE` with no `CASE` open are
errors (between `SELECT` and the first `CASE` the parser's error, if it gives one today, stays). The statement
is `StmtKind::ExitSelect { case: bool }`.

The IR lowers it to a jump: to the block's end label, or, for `EXIT CASE` in `SELECT EVERYCASE`, to the label
after the body of the `CASE` it stands in, where the next test begins. `select` in `lower.rs` puts both labels
on a stack of its own beside `exits`; a loop inside the `CASE` does not hide them, and a `SELECT` inside a
loop does not hide the loop from `EXIT FOR`.

`EXIT CASE` in a plain `SELECT` is accepted by the old compiler and leaves the block (`v30_x62`); it is
compiled that way and listed in `SOMEDAY.md` under "QB64pe behaviours to review" only if the wiki says
otherwise.

### D6. _IIF, _MIN, _MAX, _CLAMP

- **One function for the result type**, `ops::widest(a, b)`, written from the measured rule (float kinds
  first, then width, then unsigned; `_BIT` below `_BYTE`; `_OFFSET` above every integer but `_UNSIGNED
  _INTEGER64`; the two pairs that depend on the order). It is not `ops::held`, the rule of the operators. A
  unit test in `sema` reads `verification\v30_result_types.txt` and compares all 794 forms; the table says for
  each form the C++ type and the type the result is printed as, and `widest` gives both (the held type and the
  believed type, which differ for two `_BIT` types and for `_OFFSET`).
- **`_IIF` is an expression kind of its own**, `ExprKind::Choice { condition, then, else_ }`, because only one
  part may be evaluated: neither part may be hoisted in front of the expression by the passes that take calls
  and raising operations out of an expression. `validate.rs` holds that rule. The condition is converted as an
  integer slot is (`qbr` for a float). Two string parts give a string; the C++ is the old compiler's `?:`.
- **`_MIN`, `_MAX`, `_CLAMP`** are `Rule::Widest`: every argument converted to the held type, the call written
  with that C++ type as its template argument (`std::min<T>`, `std::max<T>`, `func_clamp<T>`), which the
  table's row cannot say (its call name is empty). The call-site check compares the type argument.

All four, and the shifts of D7, are rejected in a `CONST` expression by name (`check\constants.rs`), before
the constant evaluator sees them.

*Alternative considered:* `_IIF` as a plain function with three slots. Rejected: both parts would be
evaluated, and `_IIF(n > 0, a(n), 0)` would raise the subscript error the program avoids.

### D7. Shifts and rotations

`_SHL` and `_SHR`: two plain rows (an `_UNSIGNED _INTEGER64` and a LONG slot; the result an `_UNSIGNED
_INTEGER64`). `_ROL` and `_ROR`: `Rule::Rotate`: the first argument's held integer type gives the width
(`_BYTE` 8, INTEGER 16, LONG 32, `_INTEGER64` and `_OFFSET` 64), the call is written with the unsigned C++ type
of that width as its template argument, the result is that unsigned type; a float or a string as either
argument and a `_BIT` type are errors. A literal has the type its size gives it, which is what `sema` already
holds for a literal.

### D8. LPRINT and CHAIN

- **`LPRINT`** is `Print` and `PrintUsing` with the printer as the destination (`to` becomes an enum: console,
  file, printer). `codegen-cpp` writes the old compiler's two other entry names and the flag around the
  statement (`tab_LPRINT`). No new statement kind.
- **`CHAIN`** is a plain row.
- Both join `NEVER_RUN` (`sema\src\builtins.rs`), each under a comment that says why. `LPRINT` is no row of
  `STATEMENTS`, so the coverage test of tier 1 (`driver\tests\inputs.rs`) gets it by name: the list becomes
  the names of built-ins and of statements with a node of their own.
- The two corpus programs: `239_lprint` is `.norun` and is compiled only. `238_chain` runs `CHAIN` on a name
  that does not exist and expects, after normalisation, the text of the failed `cmd` call; both compilers'
  programs call the same `sub_chain`. Task 7.2 runs it once: if the output is equal it joins `slice.list`; if
  not (the text names the program beside which the compiler is looked for), it goes on
  `tests\known_clean_not_passing.list` with that reason. Either way the call-site program is what checks
  `CHAIN`.
- `driver\tests\cli.rs` uses `LPRINT 1` as "a statement not compiled yet" in the test of `--report
  unsupported`; it takes `STOP`.

### D9. What is checked, and where

| What | Check | Where |
|---|---|---|
| The call of every plain form (`LOCK`, `UNLOCK`, `CHAIN`, `_SHL`, `_SHR`) and of the forms with a rule (`GET`, `PUT`, `_MIN`, `_MAX`, `_CLAMP`, `_ROL`, `_ROR`, `_IIF`) | equal to the old compiler's | `tests\callsite\`, one program per form and argument type |
| `PRINT USING`, `LPRINT`, `FIELD`, `LSET`, `RSET` | the calls of one statement, compared as the tool compares any line (its rules are extended if a statement of several calls needs it, with a change to its tests) | `tests\callsite\` |
| Rejected forms | an error that is not "not supported yet" | `check-fail` front-end tests, from `v30_x*` |
| Slots, places, the choice, the statement kinds | pinned | `ir` and `typed` front-end tests |
| The result type for every pair | equal to the table | a unit test in `sema` |
| What runs | output equal to the old compiler's | slice programs: formats; binary files; random files and `FIELD`; locks; `EXIT SELECT`; the eight functions |
| The corpus programs | pass | `slice.list` |

The slice programs are written fresh in the slice group's style from the verification programs and recorded
with `qb64pe.exe`. Each writes its files into the folder it runs in and removes them. The lock program does
not open a file twice from two processes; the second `OPEN` of `v30_i_lock` is inside the one program.

### D10. Order

1. `FIELD`, `LSET`, `RSET` measured; the decisions recorded (group 1).
2. The eight functions (group 2).
3. `EXIT SELECT` (group 3).
4. `PRINT USING` and `LPRINT` (group 4).
5. `GET`, `PUT`, `LOCK`, `UNLOCK` (group 5).
6. `FIELD`, `LSET`, `RSET` (group 6).
7. `CHAIN`, the corpus programs and the lists (group 7).
8. The closing runs and documents (group 8).

Groups 2 to 5 do not depend on one another; 6 needs 5 (`GET` and `PUT` with no variable).

## As built

Where the build differs from the decisions above (each found and recorded in `tasks.md` under its task):

- **D1, D6: no conversion in the tree.** The numbers of `PRINT USING`, the arguments of `_MIN`, `_MAX` and
  `_CLAMP` and the two parts of `_IIF` stay as they are; C++ converts them (to the entry's parameter, to the
  template argument, to the common type of `?:`), as in the old compiler's code. A cast written out fails the
  call-site check. The IR states the three operations that convert by themselves (`crates\ir\src\lib.rs`).
- **D5: `sema` keeps no stack of open `SELECT`s.** The parser rejects every misplaced `EXIT SELECT` and `EXIT
  CASE`; the IR's lowering has the stack (`selects` in `lower.rs`). `EXIT CASE` goes to the end of the next
  `CASE` that closes, the old compiler's count of `CASE`s (`verification\v30_x66`; `SOMEDAY.md`, "QB64pe
  behaviours to review").
- **D6: no hoisting rule in `validate.rs`.** Nothing in this compiler takes a part out of an expression, so
  there is no hoisted part to reject; the validation checks the shape of a choice (an integer condition, parts
  both strings or both numbers, as the node is).
- **By the review of the change (2026-10-13):** a FUNCTION's name with empty parentheses as the variable of
  `GET` or `PUT` or as the string of `FIELD`, `LSET` or `RSET` is an error, not "not supported yet"
  (`verification\v30_x171`–`x175`); an array that is not declared stays marked for `GET` and `PUT`, which the
  old compiler accepts (`x176`–`x178`; `SOMEDAY.md`).

## Risks / Trade-offs

- [`FIELD` needs more than a statement and a call] → measured first (D3); a form that does stays marked, and
  the user is told before group 6.
- [A hoisting pass moves a part of `_IIF` in front of it] → the expression kind and a rule in `validate.rs`; a
  slice line whose unchosen part would raise an error.
- [The call-site tool compares one call per program, and `PRINT USING` writes several] → its normalisation
  rules are read in task 4.2 before the programs are written; a change to a rule comes with a change to
  `tools\callsite`'s tests.
- [A slice program with locks or files fails under eight jobs because two programs use one file name] → each
  program's file names start with its own name.
- [`238_chain` prints a text that differs between the two compilers' programs] → D8: listed with its reason,
  not forced.
- [The run-time behaviour of `_MIN` with a value that is not a number depends on the argument order] → kept
  as the old compiler's (`std::min`), one slice line, a line in `SOMEDAY.md` under "QB64pe behaviours to
  review".

## Open Questions

None that change the plan. The two decisions the proposal names are made (`DECISIONS.md` 2026-10-13). What
group 1 finds about `FIELD` is put to the user only if a form has to stay marked.
