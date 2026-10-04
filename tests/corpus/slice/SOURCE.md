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
