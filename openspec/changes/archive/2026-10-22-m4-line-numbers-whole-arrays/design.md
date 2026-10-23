# Design

## Context

See `proposal.md` for why. What the code has now:

- **Line numbers are parsed and refused.** The parser makes an `ast::LineNumber` node for a number at the start of
  a line (`syntax\src\parser\flow.rs`, `line_number`, taken before a `SUB` header) and an `ast::ImplicitGoto` for
  `THEN 10` (`parser\blocks.rs`). `sema` marks both "line numbers" in the statement dispatch (`check\mod.rs`), and
  marks a number as a target in `check\flow.rs` (`jump_target`, `restore`, `zero` for `GOTO`/`RESUME`) and
  `check\events.rs` (an event's `GOSUB`). The language server already goes to a line number's definition
  (`lsp\src\definition.rs`).
- **Labels** are entered in a pass before the bodies (`check\flow.rs`, `declare_labels`, `declare_label`), keyed by
  body and upper-case name (`labels_by_name`), and written as `LABEL_<NAME>` in C++ (`codegen-cpp\src\names.rs`,
  `label_name`), with the old compiler's event check after each.
- **`ERL`** is a built-in row (`sema\src\builtins.rs`, `Rule::Fixed(Ty::F64)`); libqb sets the error's line from the
  global `double last_line` (`libqb\internal\c\qbx.cpp` 332, `libqb\src\error_handle.cpp` 427), which nothing of
  ours writes, so `ERL` is 0.
- **The old compiler's line numbers** (`qb64pe.bas` 3941–3985): a line starting with a digit or `.` takes its first
  element as a label in the same table as named labels (`validlabel`, "Duplicate label"), writes `LABEL_<n>:;` and
  then `last_line=<n>;` with the label's spelling (`p` back to `.`, a trailing `d` or `s` dropped).
- **Unsigned literals**: `sema\src\literal.rs`, `number`, types an unsuffixed integer above `i64::MAX` as
  `Unsupported("unsigned literals")`; the same digits with `~&&` take the suffixed path and compile. The old compiler
  adds `~&&` to such a literal (`qb64pe.bas` 23937).
- **A whole array `a()`** outside the whole-array assignment and an array argument is refused in `check\places.rs`
  (`element` for a declared array, the implicit-array path for an undeclared one) and `check\whole.rs` (suffixes on
  both sides). The old compiler takes it in the places the proposal lists and uses one element (`SOMEDAY.md` rows
  97, 144, 233, 365).

## Goals / Non-Goals

**Goals:**

- Every line-number form of `verification\v16_m3_*`, `v16_m4_*`, `v17_b_if_then_line`, `v22_c_restore`, `v22_x86`,
  `v29_x64`, `v33_d01`, `v33_d02`, `v42_x46`, `v43_a_errorline`, `v50_x13` compiles and passes, and `ERL` gives
  what the old compiler gives.
- The unsigned-literal and whole-array programs of the proposal pass; the family `v54` (D5) settles every form
  the specs leave to it before the code is written.

**Non-Goals:**

- A whole array of an array that is not declared (`LEN(x())` of a scalar `x`, `v48_x04`, `x14`: the old compiler
  makes an implicit array of eleven elements). That is step 2's (D6), with the other arrays used without `DIM`.
- `LEN` of a whole member array of strings (`v48_x30`): step 3's.
- A negative literal beyond the `_INTEGER64` range (`-9223372036854775809`, `-18446744073709551615`, which prints
  1 in `v54_x26`): the old compiler gives it `&&` and its C++ build decides the value; it stays "Overflow" (no
  program needs it, `CLAUDE.md` working rule 12).
- QuickBASIC's rule that a line number ends the reach of a single-line `IF`: the old compiler has none, and nor do
  we.

## Decisions

### D1. A line number is a label with a number for a name

A line number is entered by `declare_labels` as a `Label` whose name is the number's normalised spelling, in the
body it stands in, in the same `labels_by_name` table, so "Duplicate label" and "not defined" come from the code
that gives them for named labels, and every statement that takes a label takes a number through `label_ref`
(`jump_target`, `restore`, `on_error`, `resume`, `events.rs`, the `ON n GOTO` list). `Label` gains
`line_number: Option<String>`, the text `last_line` is given. `ImplicitGoto` becomes a `GOTO` in the dispatch.
`zero` keeps `0` as "no line" for `ON ERROR GOTO 0` and `RESUME 0`.

The normalised spelling is the old compiler's, as `v54` records it: its line formatter rewrites a plain whole
number (leading zeros dropped, `0` for all zeros) and keeps the text of one with a decimal point or a suffix
(`qb64pe.bas` 24688, 24847), and `validlabel` (27121) drops a suffix other than `!` and `#`, turns `.` into `p` and
adds `d` for `#`, `s` for `!`; more than 40 characters is "Invalid label!". So `010` and `10` are one label
(`v54_x04`: "Duplicate label"), `10&` and `10` one, `10.0`, `1.10`, `30#` each their own. The `last_line` text is
the label's text with `p` back to `.` and the `d` or `s` dropped, so `010&` writes `last_line=010;`, which C++
reads as octal (`ERL` 8, `v54_x25`); we write the same text. The code is `check\flow.rs`, a function
`line_label_name` beside `declare_label`, giving the key and the `last_line` text. Where C++ takes no such text
(`08&`, octal with an 8; a whole number above 18446744073709551615), the old compiler's C++ build fails
(`v54_x40`, `x41`), so here it is an error at the line number (`cpp_line_problem`; added by the review of the
change), not a g++ message.

*Alternative*: a kind of its own for line numbers in the IR. Rejected: every rule of a label (body, duplicate,
`RESTORE`'s scope, the event check after it) is the same, and the old compiler has one table.

### D2. `last_line` is written after a numbered label

The IR's label op carries the line number when there is one, and `codegen-cpp` writes `last_line=<n>;` after
`LABEL_<n>:;`, before the event check, as `qb64pe.bas` 3979 does. Code: `ir\src\lower.rs` (the `StmtKind::Label`
arm), `ir\src\lib.rs` (the op), `codegen-cpp\src\lib.rs` (where labels are written). The C++ name of a label
`10.5` goes through `c_ident` (`LABEL_10__046__5`); only the number in `last_line` is the old compiler's spelling,
and the C++ name is not compared by any test. `ERL` needs nothing else.

### D3. An unsuffixed literal above `i64::MAX` takes the `~&&` path

In `literal.rs`, `number`: a magnitude above `i64::MAX` and at most `u64::MAX`, without a minus, is passed to
`suffixed` as `~&&`, so it is held and printed as the `~&&` literal already is. The `LitError::Unsupported("unsigned
literals")` goes; its unit test changes to the new type.

### D4. A whole array as a value is its first element, an element with no indexes

The old compiler writes a whole array where it takes one as the array's first element in memory, with no bounds
check. `sema` gives it `Place::Element` with no indexes (`sema\src\lib.rs`), made in `check\places.rs` `element`
where `args.is_empty()` refused. `codegen-cpp` writes flat position 0, with no index check
(`codegen-cpp\src\place.rs`, `flat_index_in`), and the IR's validation takes an element of no indexes of an array
variable. `check\whole.rs` lets a statement with suffixes on both sides go on as an ordinary assignment of two such
places (`both_suffixed`).

*Changed while built* (task 4.1): the plan was a place of its own, `Place::First(ArrayRef)`. An element with no
indexes is the same thing to every reader of a place (its type, its root, its storage) and needed three small
edits, where a new variant needed an arm in every `match` on `Place` in `sema`, `ir` and `codegen-cpp` (about 40).
A member array never gets one: `t.m()` stays an error (`v54_x27`), also as `t.m&() = u.m&()` (`v54_x45`).
`VARPTR(a())` is the offset of that element (`v54_x39`; the IR's validation takes an offset with no index).

`v54` settled it (task 1.1): the element is the first in memory, the one of the lower bounds (`v54_c`: `a(5 TO 7)`
gives `a(5)`, `b(1 TO 2, 1 TO 3)` gives `b(1, 1)`); an erased array gives 0 and takes a store with no error
(`v54_d`); and the old compiler takes `a()` in **any expression** (`v54_x05`, `x06`, `x15`–`x17`, `x28`, `x33`,
`x34`: an operand, a built-in's argument, a condition, a negation, in parentheses, in an assignment's value when it
is not `a()` alone), and as the variable of `READ`, `INPUT`, `INPUT #`, `LINE INPUT #`, `SWAP`, `FIELD`, `LSET`,
`RSET`. So the element is made wherever an expression or a variable place is resolved, and the forms it rejects
keep their errors, most of them given today: the whole value of an assignment (`x = a()`, `x = (a())`, `v54_x29`,
`x37`), `SWAP` of two element types (`v54_x10`), a whole member array (`v54_x27`), a scalar parameter of a SUB or
FUNCTION (`v54_x31`, `x32`), `LEN` of variable-length strings (`v54_x35`). `LEN(a())` keeps its meaning, the size
in bytes (`v54_x30`). The erased array needs nothing of its own: our arrays, like the old compiler's, point their
data at libqb's `nothingvalue` (8 bytes of zeros) when not allocated (`codegen-cpp\src\decl.rs`), so the element at
offset 0 reads 0 and takes a store there, as in the old compiler; for an erased array of `STRING` or
`STRING * n` the two compilers' builds read and write the same memory too (`v54_x42`–`x44`, recorded with the
review of the change).

*Alternative*: `Place::Element` with the lower bounds as indexes. Rejected: that adds a bounds check and the
reading of the bounds, which the old compiler's code does not have, so an erased array would raise error 9 where
the old program does not.

### D5. The measurement family `v54`

Recorded with `qb64pe.exe` before the code (`verification\README.md` gains its line):

- `v54_a_line_numbers`: `ERL` after a numbered line, after an unnumbered one, in a SUB, after `RESUME`, for
  `10.5`, `10&` and a number beyond LONG; a line number in an included file; the same number in the main module
  and a SUB; `GOSUB`/`RETURN n` with numbers; `ON ERROR GOTO n`, `RESUME n`.
- `v54_b_unsigned_literals`: `18446744073709551615` printed, in an operation, stored into `~&&`, `&&`, `DOUBLE`;
  `18446744073709551616` (an error).
- `v54_c_whole_values`: a whole array as a value with a lower bound other than 0, two dimensions, `STRING`,
  `STRING * n`; `SWAP` of two whole arrays of `STRING`; `INPUT #` and `READ` into one; `PRINT USING`, `_IIF`.
- `v54_d_whole_erased`: a dynamic array after `ERASE` printed, read into, swapped.
- `v54_x01`–`x38`: one form each: `GOTO` and `ON ERROR GOTO` a number of another body, `RETURN n` in a SUB, the
  spellings (`10` and `10.0`, `010` and `10`, `30#`, `40!`, `.5`, `50.`, `1.10` and `1.1`, `010&`, `0`, 41
  digits), `IF c GOTO n`, `ERL` in a FUNCTION; `a()` in an operand, a built-in's argument, `WRITE`, `PRINT #`,
  `LINE INPUT #`, `RSET`, a condition, a negation, `INPUT`, parentheses, `LEN`, two dimensions with `OPTION BASE
  1`, an assignment's value alone and in an operation, a scalar parameter, `SWAP` of two types, a member array;
  `-18446744073709551615` (1, from the C++ build: left as "Overflow", Non-Goals). `x = a()` is recorded already
  in `v36`.
- `v54_x39`–`x45`, recorded with the review of the change (`VARPTR(a())`, `08&`, a number of 25 digits, erased
  arrays of `STRING` and `STRING * 20` printed and read into, `t.m&() = u.m&()`).

### D6. The steps of the rest of M4

The user chose a plan of steps, each a change proposed when it starts (user, 2026-10-22). From the demand report
over `tests\verification\known.list` (2026-10-22, 140 programs) and `ROADMAP.md`'s open items:

1. **`m4-line-numbers-whole-arrays`** (this change): line numbers, unsigned literals, a whole array as a value;
   about 44 programs.
2. **Names and implicit arrays**: an array used without `DIM` in the forms still marked (in a bound of `REDIM`,
   below `$DYNAMIC` in a later item, `STATIC` with a bound that is no constant, `STATIC` beside a shared array),
   `LEN(x())` of an undeclared array, a built-in's name as an array or a place (`ABS(...)`, `TAB(...)`,
   `rnd(...)`), with a suffix (`KILL$`, `_ASSERT$`, `_CRC32$`, `exp$`, `_CAST&`, `_OPENCLIENT&`), as a label
   (`BEEP`, `CLS`, `SOUND`) and as an array (`WIDTH`), names with blanks around `.`, the old compiler's errors
   for names that are no array or no SUB (`DECISIONS.md` 2026-10-20 left them); about 35 programs.
3. **Memory and record forms**: `GET`/`PUT` with arrays of strings, of a `TYPE` or of `_BIT`, a variable that is no
   array, an undeclared array; `SADD`, `VARPTR$`, `VARPTR`/`VARSEG` as an argument of `HEX$`; `_MEM` of a `TYPE`
   with a `STRING` member; `LEN` of a whole member array of strings; a `CONST` in a member array's bound; a
   variable as the length of `STRING * n`; `_UCHARPOS` of a variable; `ROADMAP.md`'s string-bearing types and
   nested and dynamic member arrays; about 25 programs.
4. **The statements and metacommands left**: `RUN`, `RESUME` in a SUB or FUNCTION, `PALETTE USING`, omitted
   arguments, `$INCLUDE` as a bare metacommand and a block closed in another file, `$NOPREFIX`, `$MIDISOUNDFONT`,
   `$DYNAMIC` bare, unknown metacommands, `_CONSOLE_` between two `$IF`s, the syntax forms (`;` after `WRITE`, `AS`
   where `,` is expected, `GOSUB` in an argument list, `_NEGATE` after `^`, `OR` alone), the `fs$5` FUNCTION forms,
   `ALIAS` outside `DECLARE LIBRARY`; and the shrink-only lists of tier 1 (`tests\known_*.list`); about 35
   programs.
5. **`SUB _GL`**: OpenGL user code and its runtime (the user, 2026-10-22: a change of its own).
6. **The coverage review**: the features checked only by their emitted call or by programs compiled and never run
   (`ROADMAP.md`), with runnable tests where they can be had.

Steps 2–4 are the order of the demand report and may be split or joined when proposed. M4 is declared done at the
merge into `main` after the last of them, by the user.

## Risks / Trade-offs

- [A whole array's element is the old compiler's accident; a later QB64pe may refuse it] → each place is a row of
  "QB64pe behaviours to review" (`SOMEDAY.md`), so the user can turn it into an error with a `DIVERGENCES.md` row.
- [`v54` contradicts a spec scenario written before it (the `~&&` stores, `ERL` after an unnumbered line, the
  element of a lower bound other than 0)] → task 1.1 changes the spec to the recording before any code, and
  `openspec validate --strict` runs after.
- [A program numbered on every line has as many labels as lines, each with a `last_line` store and an event
  check] → the old compiler writes the same code, so the build and the run match it; only one corpus program has
  line numbers today, so no test measures a large one (a numbered program for the corpus is step 6's question).
- [`ImplicitGoto` and `IF c GOTO n` in a single-line `IF` with `ELSE` are parsed today but never lowered] →
  `v17_b_if_then_line` and `v16_m4_*` run them; a front-end `ir` test shows the branch.
