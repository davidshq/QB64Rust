# DECLARE LIBRARY

A `DECLARE LIBRARY` block declares C or C++ functions that a program calls as its own SUBs and FUNCTIONs: from a
header, a static library, a DLL, or the C runtime the program already links.

## Syntax

```text
DECLARE [STATIC | DYNAMIC | CUSTOMTYPE] LIBRARY ["name"[, "name"]...]
    SUB name [ALIAS cname] [([BYVAL] parameter [AS type], ...)]
    FUNCTION name[suffix] [ALIAS cname] [([BYVAL] parameter [AS type], ...)]
END DECLARE
```

## Arguments

- `name`: the library, without its extension: a header `name.h` or `name.hpp`, a `name.lib`, `.a` or `.o`, or with
  `DYNAMIC` a `name.dll`. With no name, the functions come from what the program's C++ already reaches, such as the
  C runtime.
- `ALIAS cname`: the C function's name, as a name or a string. A string may hold C++ names such as `"h_max<int32_t>"`
  or `"hns::twice"`. Without `ALIAS` the name is the BASIC name as written, in that letter case.
- `BYVAL`: pass the argument's value. Without it a parameter is passed by its address.

## Behavior

A block may stand in the main module, a SUB, an included file or a `$IF` branch, and its procedures are known to the
whole program, also above the block.
<!-- src: language/external-libraries "Library blocks" -->

`DECLARE LIBRARY` with no name reaches the C runtime: `toupper`, `strlen`, `atoi`, and `abs` through `ALIAS`.
<!-- src: language/external-libraries "Library blocks"; tests\corpus\slice\s125_library_runtime_dll; tests\reference\examples\declare-library_1 -->

A `BYVAL` argument is converted as in an assignment. Without `BYVAL`, a variable, element or member of the parameter's
type passes its address, so C may change it; anything else passes a copy.
<!-- src: language/external-libraries "Arguments of a library call"; tests\corpus\slice\s121_library_numbers -->

A `BYVAL` argument written `0`, `(0)`, `0&` or as a `CONST` of 0 is C's `NULL`.
<!-- src: language/external-libraries "Arguments of a library call"; tests\corpus\slice\s121_library_numbers -->

A `STRING` argument reaches C as a pointer to its bytes, which C may change in place. A `TYPE` argument reaches C as a
pointer to the value. A `STRING` FUNCTION's `char*` result is copied into a new string, and `NULL` gives "".
<!-- src: language/external-libraries "Strings, user types and results"; tests\corpus\slice\s122_library_strings_types -->

A `DYNAMIC` library is loaded when the program starts, and its names found then.
<!-- src: language/external-libraries "DLLs at the start of the program"; tests\corpus\slice\s125_library_runtime_dll -->

A `CUSTOMTYPE` declaration calls the C function through the declared types, which may differ from the function's own.
<!-- src: language/external-libraries "CUSTOMTYPE libraries"; tests\corpus\slice\s124_library_customtype -->

The library is looked for as QB64pe looks for it: by the path written from the compiler's folder, `./` meaning the
source's folder, then for a header.
<!-- src: language/external-libraries "Finding a library"; tests\corpus\slice\s123_library_alias -->

A header is included at each block that names it, and then no prototype is written for the block's declarations.
<!-- src: language/external-libraries "Headers and libraries in the C++" -->

A library FUNCTION and a program SUB of the same name may both exist: the name as a statement calls the SUB, as a
value the FUNCTION.
<!-- src: language/external-libraries "A library procedure and a program procedure of one name" -->

## Errors

- A library that is not found, a `DYNAMIC` or `CUSTOMTYPE` block with no name, a name without quotes, and two names in
  a `DYNAMIC` block are compile errors.
  <!-- src: language/external-libraries "Rejected library names" -->
- A `STRING` parameter with `BYVAL`, an array parameter, a `STRING * n` parameter, `AS type` after a FUNCTION, a
  suffix on a SUB and `BYVAL` outside a library are compile errors with QB64pe's reasons.
  <!-- src: language/external-libraries "Rejected declarations" -->
- A FUNCTION with no parameters called with `()`, a FUNCTION used as a statement, a `CONST` value or an assignment's
  target, and a SUB used as a value are compile errors.
  <!-- src: language/external-libraries "Rejected uses of a library procedure" -->
- Where the C++ build fails at a declaration's C function, the error is reported at the BASIC line, with the C++
  compiler's message.
  <!-- src: language/external-libraries "Errors the C++ build finds" -->
- A `DYNAMIC` library that does not load raises error 259, and a name it does not export error 260, when the program
  starts. `ON ERROR` does not catch either.
  <!-- src: language/external-libraries "DLLs at the start of the program" -->

## Differences

A `STRING * n` member or element passed to a parameter `AS STRING` reaches C as a copy of its bytes with a NUL after
them, copied back after the call. QB64pe passed the bytes with no NUL, so C read on into the next member.
<!-- src: language/external-libraries "Fixed-length strings to a char pointer"; D-044 -->

A constant expression of 0, such as `1 - 1`, passed `BYVAL` to a parameter a header declares as a pointer, compiles;
QB64pe's C++ build failed there.
<!-- src: language/external-libraries "Arguments of a library call"; D-032 -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:declare-library_1 -->
```qb64
' Functions of the C runtime, which every program's C++ already reaches, so the block names no library.
DECLARE LIBRARY
    FUNCTION toupper& (BYVAL c AS LONG)
    FUNCTION labs& ALIAS abs (BYVAL v AS LONG)
    FUNCTION strlen~%& (s AS STRING)
    FUNCTION atoi& (s AS STRING)
END DECLARE
PRINT "toupper: "; CHR$(toupper&(ASC("q")))
PRINT "abs, by its ALIAS:"; labs&(-17)
PRINT "strlen:"; strlen~%&("four")
PRINT "atoi:"; atoi&("123abc")
END
```

Output:

```text
toupper: Q
abs, by its ALIAS: 17 
strlen: 4 
atoi: 123 

Press any key to continue
```
<!-- end generated -->

## See also

[`DECLARE`](declare.md), [`$USELIBRARY`](meta-uselibrary.md), [`_OFFSET`](_offset.md).
