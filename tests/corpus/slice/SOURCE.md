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
