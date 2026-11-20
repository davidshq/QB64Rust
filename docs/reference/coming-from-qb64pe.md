# Coming from QB64pe

QB64Rust follows QB64pe 4.7.0. These are the main intentional differences to watch for when moving a program over.
The [divergence register](../DIVERGENCES-QB64PE.md) has the full list and details of each change.

| Area | What differs in QB64Rust |
|---|---|
| Integer overflow | Integer addition, subtraction and multiplication wrap consistently, including optimized builds. |
| Constants | Fixes incorrect results for large numbers, integer powers and some math functions. Division by zero and floating-point overflow in a `CONST` are compile errors. |
| Math and logic | Fixes chained `IMP` expressions and `_COTH`. Some math constants and out-of-range conversions have consistent results where QB64pe's depend on the machine or build. |
| Evaluation order | Uses QB64pe's Windows evaluation order on Linux too. Certain array indexes that QB64pe evaluates twice are evaluated once. |
| Arrays | `_ARRAYCOPY` fixes copies between `_FLOAT` array variables. `REDIM` without `AS` resizes an existing `TYPE` array. `DIM` of an array declared by `COMMON ... AS` requires `AS`. |
| Local arrays | A procedure's `STATIC` array stays separate from a shared static array of the same name. |
| Strings | Fixes overwritten fixed-length function results and crashes with large strings. A string longer than 2,147,483,647 bytes raises error 5. |
| Errors and memory | Many cases that crash, hang or corrupt memory in QB64pe instead raise an error or complete safely. Continuing after an error can therefore behave differently. |
| Exit status | An unhandled runtime error that ends the program returns exit code 1 instead of 0. |
| Console input and output | Redirected input handles end-of-file; comma-separated output avoids QB64pe's Windows redirection hang. |
| Includes | On Windows, `$INCLUDEONCE` treats names differing only in letter case as the same file. |
| Debugging | `$DEBUG` does not connect to the QB64pe IDE debugger. |
| Accepted syntax | Allows a label before `CONST` on the same line and `_BIT` procedure parameters. |
| Formatter | `-y` avoids changes that alter a program's meaning and can format include files and programs with non-syntax errors. |
| Command line | `-c` is an alias for `-x`; `-s` is unsupported; `-p` cleans build files. Warning formatting and some error messages differ. |
| Other fixes | Corrects scaled `_SCREENCLICK` coordinates on Windows, non-ASCII paths in logs, and an intermittent audio crash. |
| Experimental wasm target | Floating-point precision, some math results and out-of-range conversions differ. See [the wasm target](chapters/wasm-target.md) for its limitations. |

Additional behavior changes are available through [optional fixes](chapters/fixes.md).
They are off by default for programs without a `qb64rust.toml`.
For features still missing, see [what is not supported yet](appendix-unsupported.md).
