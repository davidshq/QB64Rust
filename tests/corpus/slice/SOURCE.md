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
