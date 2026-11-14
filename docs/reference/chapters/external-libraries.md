# Libraries

A `DECLARE LIBRARY` block makes C and C++ functions callable as SUBs and FUNCTIONs, and `$USELIBRARY` includes a
library of BASIC files installed beside the compiler.

The pages of this area: [DECLARE LIBRARY](../entries/declare-library.md),
[$USELIBRARY](../entries/meta-uselibrary.md).

## Declaring

A block names its library: a header, a static library, a DLL with `DYNAMIC`, or none for the C runtime the program
already reaches. Its procedures are known to the whole program.
<!-- src: language/external-libraries "Library blocks"; language/external-libraries "Finding a library"; tests\reference\examples\declare-library_1 -->

An argument passes its address unless the parameter is `BYVAL`; a `STRING` reaches C as a pointer to its bytes, and a
`char*` result becomes a new string.
<!-- src: language/external-libraries "Arguments of a library call"; language/external-libraries "Strings, user types and results"; tests\corpus\slice\s122_library_strings_types -->

## When it fails

A library that is not found is a compile error. Where the C++ build fails at a declaration, the error is reported at
the BASIC line; a `DYNAMIC` library that does not load raises error 259 when the program starts.
<!-- src: language/external-libraries "Rejected library names"; language/external-libraries "Errors the C++ build finds"; language/external-libraries "DLLs at the start of the program" -->

## $USELIBRARY

`$USELIBRARY:'author/library'` reads the library's descriptor under the compiler's folder and includes its files at
the top, after the main module and at the bottom.
<!-- src: language/external-libraries "$USELIBRARY"; language/external-libraries "The order of several libraries" -->
