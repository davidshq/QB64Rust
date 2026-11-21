# Coming from QuickBASIC 4.5

QB64Rust follows QB64pe, so some QuickBASIC 4.5 behavior differs. These are the main things to watch for.
The [QB45 divergence register](../DIVERGENCES-QB45.md) has the full list and examples.
See also [Coming from QB64pe](coming-from-qb64pe.md) for QB64Rust's own changes.

| Area | What differs in QB64Rust |
|---|---|
| Integer overflow | `INTEGER` arithmetic uses 32 bits. Integer results wrap instead of raising QuickBASIC's overflow error. |
| Floating-point arithmetic | Precision and rounding can change comparisons, conversion to integers, and numbers written to files. |
| Constants | Powers are allowed in `CONST`; chained powers group from the right there, but from the left at run time. |
| Array declarations | Declare arrays explicitly. Undeclared arrays in procedures or below `$DYNAMIC` may raise errors instead of working as in QuickBASIC. There is no "Array not dimensioned" warning. |
| `STATIC` arrays | `STATIC a(5)` gives an upper bound of 5, rather than declaring five dimensions. Mixing a `STATIC` procedure header with `STATIC` statements can change which locals retain values. |
| Other array syntax | A literal upper bound of 0 has special behavior under `OPTION BASE 1`. Empty parentheses and repeated declarations can be accepted with a different meaning. |
| `GOSUB` in procedures | Return from every pending `GOSUB` before leaving a procedure; otherwise a later `RETURN` can raise error 3. |
| Recursive `SELECT CASE` | A recursive call can overwrite the value an outer `SELECT CASE` is testing. |
| Evaluation order | Numeric assignments to array-element members can read the value before evaluating the index. `LEN` can skip evaluating an array index. |
| Graphics | `DRAW`, `LINE`, `CIRCLE` and the fonts in `SCREEN 9` and `10` can produce different pixels. Check `PAINT` boundaries in particular. |
| Console output | Comma zones differ in console programs: 10 columns on Windows, one space on Linux and macOS. Window and file output keep 14-column zones. |
| Strings | Negative lengths in `LEFT$` and `RIGHT$` give an empty string. The string size limit is much larger; an over-limit concatenation clears its target after raising error 5. |
| Records and files | Record-to-record `LSET` is unsupported. Assignment to a `FIELD` string does not permanently detach it. Embedded quotes can change how `INPUT #` splits fields. |
| Error recovery | A failed numeric `READ` sets its target to 0. Some other failed assignments also change their targets. |
| Music | `PLAY` accepts a wider tempo and octave range, but background-music events (`ON PLAY`, `PLAY ON/OFF/STOP`) are unsupported. |
| Other syntax | `ON n GOTO/GOSUB` with `n > 255` continues instead of raising error 5. Unary `+` before a variable and some doubled signs are rejected. |

If you are coming from BASIC PDS 7.1, also check `AND`/`OR` conditions: both sides are evaluated here,
which can consume extra random numbers or keypresses. PDS-only forms such as `CURDIR$`, `DIR$`, `CHDRIVE`,
`ON LOCAL ERROR` and named `COMMON` blocks need replacements.
See [Porting a BASIC PDS 7.1 program](../PORTING-PDS.md) for those edits.

For features still missing, see [what is not supported yet](appendix-unsupported.md).
