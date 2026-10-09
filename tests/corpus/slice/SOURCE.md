# Source of these programs

Written for QB64Rust by the OpenSpec change `m2-workspace-and-slice` (2026-10-03) to pin the numeric rules of
`openspec\specs\language\numeric-semantics` and the PRINT forms the first end-to-end slice of the new compiler
supports. Expected output recorded with `qb64pe.exe` 4.7.0 (`16f629784e`, default build) using the corpus
runner's `--record`.

| Program | Covers |
|---|---|
| `s01_literals.bas` | Every numeric literal form (suffixes, exponent letters, digit counts, `&H`/`&O`/`&B`); typing shows in the printed format; a SINGLE literal stored in a DOUBLE |
| `s02_integer_wrap.bas` | 32-bit computation of INTEGER and LONG arithmetic, 64-bit with `_INTEGER64`; wrap on overflow, also in literal-only expressions (`DIVERGENCES.md` D-001, D-002) |
| `s03_float_to_int.bas` | Float to integer stores: half to even, SINGLE narrowing for INTEGER targets only, truncation to the target width; float to float stores |
| `s04_division.bas` | `/` on integers (computed in `_FLOAT`, printed with 16 digits) and on SINGLE/DOUBLE operands |
| `s05_instr.bas` | `INSTR` with and without the optional start; a start of 0 as the last statement (does it raise?) |
| `s06_print_items.bas` | PRINT items: `;`, trailing `;`, auto-semicolon next to string literals, empty `PRINT`, mixed types, unary minus. No `,` items (they hang under a redirected `$CONSOLE`, `study\10` §2.2) |
| `s07_cp437_bytes.bas` | Bytes 0x80–0xFF in string literals and comments, CRLF line ends (kept by `.gitattributes`) |

`s08`–`s12` were written by the OpenSpec change `m2-procedures-and-errors` (2026-10-04) to pin
`openspec\specs\language\procedures` and `language\error-handling`, recorded the same way. Like the others they use
no `PRINT` comma and no control flow.

| Program | Covers |
|---|---|
| `s08_byref.bas` | By reference (plain variable of the parameter's type) and by copy (parentheses, expressions, other types, rounded and wrapped as for an assignment); a parameter passed on; string parameters with a variable, a literal and an expression |
| `s09_functions.bas` | Results of every slice type printed with the function's own type; a call before the definition; the result assigned without its suffix; `EXIT FUNCTION` before any assignment; zero-argument functions; a function changing its argument inside a `PRINT` |
| `s10_scopes.bas` | Locals (`DIM` and implicit) new on each call; `STATIC` numbers and strings; `SHARED`; `DIM SHARED` not seen by a SUB before it in the file; `DECLARE` lines; a parameter named like a main variable |
| `s11_on_error.bas` | `RESUME NEXT`, `RESUME label`, retry reaching a second handler, `ERR` and `ERL`, `ERROR` with 0, -1, 2.5, 3.5, 70000, errors inside a SUB and a FUNCTION, a handler set inside a SUB, `ON ERROR GOTO 0`, critical error 11 ending the program |
| `s12_error_in_print.bas` | `CHR$` raising inside a `PRINT`: retry re-runs the whole `PRINT`, `RESUME NEXT` skips the rest and the line end; `ERROR` in a FUNCTION called from a `PRINT`; an untrapped error (run with `QB64PE_NOPROMPT=continue`, `s12_error_in_print.noprompt`) |

`s13`–`s19` were written by the OpenSpec change `m2-control-flow-slice` (2026-10-06, design D11) to pin
`openspec\specs\language\control-flow`, `language\constants` and the operator and header-error parts of
`language\numeric-semantics` and `language\error-handling`, recorded the same way. No `PRINT` comma; every loop
and every error handler is capped. Errors are raised only with `CHR$` (also inside `INSTR`), the built-ins the new
compiler has; the `ASC`/`LEN` forms of the same rules are in `verification\v17_*`.

| Program | Covers |
|---|---|
| `s13_if.bas` | Block `IF` with `ELSEIF` chains and `ELSE`, nesting; conditions as numbers; single-line `IF` with and without `ELSE` (the statements after `ELSE` belong to it); a single-line `IF` inside another (the `ELSE` binds to the inner one); `IF … GOTO`, `IF … THEN GOTO`, `IF … THEN GOTO … ELSE` |
| `s14_loops.bas` | `WHILE`, `DO WHILE`, `DO UNTIL` (tested first), `LOOP WHILE`, `LOOP UNTIL` (tested after a pass), `DO … LOOP` left by `EXIT DO`; `EXIT WHILE` from a block `IF`; `EXIT DO` leaving only the innermost `DO`, and from inside a `WHILE`; string comparisons as conditions; loops in a SUB |
| `s15_for.bas` | `FOR` with plain, negative, float and zero steps; start, limit and step rounded to the wider type; limits evaluated once; the body changing the variable; loops that do not run; INTEGER and LONG passing their range (the loop ends), `_INTEGER64` wrapping (capped); SINGLE, DOUBLE and `_FLOAT` variables; `NEXT j, i`; `NEXT` with a suffix; `EXIT FOR` from an `IF` and from a nested `FOR`; `FOR` in a SUB |
| `s16_goto_gosub.bas` | `GOTO` into an `IF` branch, a `FOR` that never ran (its hidden limit and step are 0; capped), a `WHILE` and a `DO` body, and out of a `FOR`; a backward `GOTO`; `GOSUB` nested, from a `FOR` body and from a handler; `RETURN label` and a `GOSUB` after it; `GOSUB` and labels inside SUBs (a label inside a block, the same names as in main); `RETURN` with nothing pending in main and in a SUB (error 3); a SUB's `RETURN` while a main `GOSUB` is pending (error 3, the entry is consumed; ends the program) |
| `s17_operators.bas` | Comparisons (-1/0, LONG; float operands at the narrower type; `_INTEGER64` near its limits; strings, also bytes above 127); `NOT`, `AND`, `OR`, `XOR`, `EQV`, `IMP` in 32 and 64 bits, float operands rounded half to even; `_ANDALSO`, `_ORELSE` (short circuit, rounded operands), `_NEGATE`; `\` and `MOD` (signs, rounding, `_INTEGER64`, LONG width); `^` (associativity, result types, a negative base with a fractional exponent: error 5, placeholder 0); precedence and chained comparisons; `MOD` by 0 (fatal, last) |
| `s18_const.bas` | `CONST` seen in a later SUB; a SUB constant shadowing a main constant and reusing a main variable's name; constants in a single-line `IF`, a `FOR` body and a skipped `IF` block; the same `CONST` twice; a plain constant with a numeric suffix; typing (integer results and integer-valued floats `_INTEGER64`, other floats DOUBLE); suffixes rounding half to even; `^` right-associative; the evaluator's comparison, logic, `\`, `MOD`, string `+`, hex, precedence; constants from constants; a plain constant stored in an INTEGER |
| `s19_header_errors.bas` | An error in each header kind (single-line and block `IF`, `ELSEIF` true and false by its placeholder, `IF … GOTO`, `WHILE`, `DO WHILE`, `DO UNTIL`, `LOOP WHILE`, `LOOP UNTIL`, `FOR` start, limit and step) with `RESUME NEXT`, then with `RESUME`, then untrapped (`QB64PE_NOPROMPT=continue`, `s19_header_errors.noprompt`); stores made with a placeholder value; a SUB called with a raising argument (not entered) |

`s20`–`s22` were written by the OpenSpec change `m2-arrays-and-types` (2026-10-07, design D9) to pin
`openspec\specs\language\arrays-and-types` and the store rules of `compiler\pipeline`, recorded the same way. No
`PRINT` comma; errors are raised by a bad index (9) and by `CHR$(-1)` inside `INSTR` (5). `DIVERGENCES.md` D-004
(a member store into an element with a bad index) cannot be a slice program, since the old compiler writes element 0
there: `s22` never prints that element, and D-004 is pinned by a CLI test.

| Program | Covers |
|---|---|
| `s20_arrays.bas` | `DIM` with one and several dimensions, `lower TO upper`, negative, float (rounded half to even), `CONST` and constant-expression bounds; every slice type and `STRING` (initial values, stores converting as for a variable); float indexes; an index from an element; an array beside a scalar of the same name; a suffix naming the same array; a `DIM` run twice; `LBOUND`/`UBOUND` with and without a dimension, typed `_INTEGER64`, the dimension rounded; elements by reference and by copy (parentheses, another type), in a FUNCTION, a string element; `DIM SHARED` in a SUB and a FUNCTION |
| `s21_types.bas` | A `TYPE` used above its block; members of every numeric type and nested types, stores converting and wrapping; members with their own suffix; dotted plain names, also before `DIM` of the name; a variable named like its type; arrays of a `TYPE` with one and two dimensions, `LBOUND`/`UBOUND`; members by reference (also of elements and nested) and by copy; local (zero on each call), `STATIC`, `SHARED`, `DIM SHARED` and FUNCTION-local `TYPE` variables |
| `s22_store_rules.bas` | Element stores (raising value, index out of range above and below, both, a raising index), string elements; reads with a bad index (element 0's value), also inside an index and a `PRINT`; member stores with a raising value; a member store into an element with a bad index (elements other than 0 unchanged) and the value evaluated before the index; several indexes left to right, each dimension checked; a bad index passed by reference (SUB not entered); arguments left to right, first error wins; `LBOUND`/`UBOUND` with a dimension out of range |

`s23` was written by the OpenSpec change `m2-parser-breadth` (2026-10-07, task 7.1) to pin the preprocessor (design D8),
`s24` for the fix of the known bug "a SUB named like a built-in function" (`STATUS.md`, measured by
`verification\v19_proc_names`), both recorded the same way.

| Program | Covers |
|---|---|
| `s23_preprocessor.bas` | The predefined names on Windows 64-bit; `$LET`; `$IF` with `$ELSEIF`, `$ELSE IF`, `$ELSE`, `$END IF` and `$ENDIF`; nested `$IF`s (a true one inside a skipped branch stays skipped); `=`, `<>`, `>`, `>=`, `<` with `VERSION`; `AND`, `OR`, `XOR`; `DEFINED`/`UNDEFINED`; `$LET WIN = 0` not overriding `WIN`; a `$IF` in a `FOR` body and in a SUB; a whole SUB inside an active `$IF`; skipped branches holding an unclosed `FOR`, a `SUB`, garbage and an unknown metacommand |
| `s24_proc_names.bas` | SUBs named like built-in functions (`LOC`, `ABS` with an argument, `FRE` with two), called bare and with `CALL`; FUNCTIONs named like built-in statements (`BEEP`, `WIDTH&`, `CLOSE`) in an expression; a SUB and a FUNCTION with the bare name of a built-in written with `$` (`LEFT`, `CHR`) |

`s25`–`s29` were written by the OpenSpec change `m2-core-builtins` (2026-10-07, design D9) to pin
`openspec\specs\language\builtin-functions` and the `SELECT CASE` and `ON … GOTO/GOSUB` requirements of
`language\control-flow`, from the measurements in `verification\v20_*`; recorded the same way. No `PRINT` comma;
errors are trapped by a handler that prints `ERR` and resumes next. `DIVERGENCES-QB45.md` Q-001 (`ON 258 GOTO`) is
pinned by `s29`, Q-002 (recursion across a `SELECT`) by `s28`. Nothing depends on undefined behaviour of the old
compiler (no `STRING$(n, "")`, no DOUBLE `ON n` beyond LONG).

| Program | Covers |
|---|---|
| `s25_string_builtins.bas` | `LEFT$`, `RIGHT$`, `MID$` (with and without a length) with zero, negative and past-the-end lengths and starts; float and beyond-LONG arguments to LONG slots; `ASC` with one and two arguments and its errors; `CHR$` (rounding, error 5); `STRING$` with a string and a code (low 8 bits), `SPACE$`; `STR$` and `_TOSTR$` of every slice type, with digits, the -1 error; `LTRIM$`, `RTRIM$`, `_TRIM$` (spaces only); `UCASE$`, `LCASE$` (ASCII only); `INSTR` with a start; nested calls |
| `s26_val_len_radix.bas` | `VAL` of edge texts (`&H`, `&O`, `&B`, blanks, exponents, junk) and with each type argument (integer types `_INTEGER64`, not narrowed); `LEN` of string expressions, of variables of each type, `TYPE` variables, members, elements and implicit variables; `HEX$`, `OCT$`, `_BIN$` of each type, of places and expressions (the width of a negative value), of literals, calls and floats, the overflow error |
| `s27_math_builtins.bas` | `ABS`, `INT`, `FIX` typed by the argument; `SGN`; `SQR`, `SIN`, `COS`, `TAN`, `ATN`, `LOG`, `EXP` for each argument type (printed digits), in arithmetic; `CINT`, `CLNG`, `CSNG` (not narrowed), `CDBL`, `_ROUND` with halves; `_PI` bare and with an argument; `_ATAN2`, `_HYPOT`; every measured error (5 and 6) with the placeholder it leaves |
| `s28_select_case.bas` | `SELECT CASE` with a FUNCTION selector (evaluated once), a plain variable (read at each test), an element and a member (copied); items converted to the selector's type; an `_INTEGER64` and an expression selector; string selectors with `TO`, `IS`, lists; `IS` with each operator; `EVERYCASE` (also in a SUB); errors in the selector and in items of each kind; a bad index in the selector; `GOTO` out of and into a `CASE` body, `EXIT SUB` and `EXIT FOR` from a `CASE`; an empty `SELECT`; a recursive FUNCTION across a `SELECT` (Q-002) |
| `s29_on_goto.bas` | `ON n GOTO` for n = 0, 1, the count, count + 1, 255, 256, 258 (Q-001), 65537, negative values (error 5), floats (half to even), an `_INTEGER64` beyond LONG, an INTEGER, a DOUBLE; an error in `n` (the jump uses the placeholder); `ON n GOSUB` returning, out of range, negative, in a loop and in a SUB |

`s30`–`s34` were written by the OpenSpec change `m2-numeric-types` (2026-10-09, tasks 5.3, 5.4, 6.1, 7.2 and 8.5) to
pin the new numeric types' declarations, literals and stores (`openspec/specs/language/numeric-semantics` as
changed), the operator rules the differential programs corrected (`study/00` §5), fixed-length strings
(`language/fixed-length-strings`), and the new types in arrays, members, procedures, `FOR`, `SELECT CASE`, `CONST`
and built-ins (the deltas of `arrays-and-types`, `procedures`, `control-flow`, `constants`, `builtin-functions`);
recorded the same way. Each `_BIT * n` wider than 32 bits is declared first or
after a `_BIT * 32` pad, so the old compiler's overlap (`DIVERGENCES.md` D-009) hits nothing printed. NUL bytes are
shown with `ASC`, never printed. No `PRINT` comma.

| Program | Covers |
|---|---|
| `s30_new_types.bas` | Every `AS` spelling of the new types and `LEN` of each; stores that wrap; every suffix on one name; the numeric-semantics scenarios the old compiler agrees with: unsigned and narrow arithmetic, `_BIT` stores (mask, sign extension, a `_BIT * 64` printed through `int64`), literals held as written and believed the suffix's type (`300~%%`, `-1~&`, `40000%`, `` 9`3 ``, `&HFF%%`, `&H1FF~%%`), stores from floats by target width, the logical operators, `\` and `MOD`, `_OFFSET` division |
| `s31_unsigned_ops.bas` | C's conversions across signedness (below 32 bits, at 32, beside 64) in arithmetic and comparisons; the unsigned belief needing a 64-bit operand; unary operators' belief; `EQV`/`IMP` complementing the left operand in its own width; `NOT` of narrow and wide unsigned values; `_ANDALSO`/`_ORELSE` of unsigned 64-bit values; `_BIT` values in arithmetic (storage type, belief by width); `ll`/`ull` `_BIT` literals; `_OFFSET` with floats (`qbr` around `*` and `/`, rounded operands elsewhere, `o < 7.4` false) and the `_OFFSET` belief |
| `s32_bit.bas` | Stores into `_BIT` and `_BIT * n` scalars at widths 1, 3, 7, 16, 17, 20, 31, 32, 33, 40, 63 and 64, signed and unsigned (the mask, the sign extension from bit n-1), from wider and narrower integers, other `_BIT`s and floats (rounded half to even from `_FLOAT` at every width); `_BIT` values in arithmetic, comparisons, `IF`, `NOT`, `AND` and `STR$`; `DIM SHARED`, `STATIC` and local `_BIT`s; a `_BIT` passed to an INTEGER, LONG, `_UNSIGNED LONG` or `_INTEGER64` parameter (a copy) |
| `s33_fixed_strings.bas` | `STRING * n` with a number, a constant, a length read in 32 bits (4294967297 is 1) and 100,000 bytes; `_UNSIGNED STRING * n`; `name$n` in `DIM` and implicit, beside `name$`; NUL bytes at the start (also a local on each call, `DIVERGENCES-QB45.md` Q-005); stores that pad, cut, store `""` and copy between fixed strings of other lengths; comparisons and `SELECT CASE` with the padding; built-ins on them; members (the layout, `LEN` of the record), elements (also a bad index), members of elements; `DIM SHARED`, `STATIC`, local; passed to a `STRING` parameter as variable, member and element, plain and in parentheses (cut and padded on the way back) |
| `s34_types_procs.bas` | Static arrays of the new types and of `STRING * n` (stores that wrap, arithmetic on elements, suffixed array names, a bad index); `TYPE` members of the new types (`LEN` 24) and members of elements; an unsigned FUNCTION, FUNCTIONs named `f$n` (called as `f$n`, `f$` and `f`, never assigned), a `STRING * n` parameter (`LEN` n, not cut); variables, elements and members passed by reference to a parameter of the same width and the other signedness, `_INTEGER64` and `_OFFSET` both ways, a copy in parentheses, a `_BIT` always a copy; `FOR` with `_BYTE`, `_UNSIGNED _BYTE`, `_UNSIGNED INTEGER`, `_UNSIGNED LONG`, `_OFFSET`, `_UNSIGNED _OFFSET` and `_UNSIGNED _INTEGER64` variables (past the range, down through 0, a limit above 2^63); `SELECT CASE` with unsigned and `_BYTE` selectors, read or copied, with negative and float items; `CONST` with the new suffixes, in range and not, `_BIT` suffixes, constants used with another suffix; `HEX$`, `OCT$`, `_BIN$`, `ABS`, `SGN`, `INT`, `FIX`, `CINT`, `CLNG`, `_ROUND`, `CSNG`, `CDBL`, `SQR`, `EXP`, `VAL` with a type, and LONG slots with arguments of the new types |
