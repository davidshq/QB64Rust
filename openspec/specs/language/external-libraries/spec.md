# language/external-libraries Specification

## Purpose
`DECLARE LIBRARY` in its four forms (plain, `STATIC`, `DYNAMIC`, `CUSTOMTYPE`) on Windows: declaring C and C++
functions, finding the library or header that holds them, calling them, and the forms that are rejected, as the
old compiler's program behaves.

## Requirements

### Requirement: Library blocks
A `DECLARE [STATIC|DYNAMIC|CUSTOMTYPE] LIBRARY ["name"[, "name"…]]` block SHALL compile, holding `SUB` and
`FUNCTION` headers, comments and metacommands up to `END DECLARE`, in the main module, a SUB, an included file
or a `$IF` branch. Its procedures SHALL be visible to the whole program, also before the block.

#### Scenario: A block in a SUB
- **WHEN** a SUB holds a block declaring `FUNCTION h_ul~& (BYVAL v AS _UNSIGNED LONG)` from a header, and the
  main module calls `h_ul~&(1)` after calling the SUB
- **THEN** the program prints the C function's result in both places, as `verification\v37_i` records

#### Scenario: A statement inside the block
- **WHEN** a block holds `PRINT "x"` before `END DECLARE`
- **THEN** compiling it fails with an error

### Requirement: Library declarations
A declaration SHALL name its C function by `ALIAS` (a string or a name; a string may hold C++ names with `<>`,
`::` and the other characters the old compiler allows) or else by its name as written, in that letter case. A
FUNCTION without a suffix SHALL have the default type, and a parameter without a type the default type.

#### Scenario: C++ names through ALIAS
- **WHEN** a block with a header declares `FUNCTION maxl& ALIAS "h_max<int32_t>" (BYVAL a AS LONG, BYVAL b AS
  LONG)` and `FUNCTION twice& ALIAS "hns::twice" (BYVAL v AS LONG)`
- **THEN** `maxl&(3, 9)` is 9 and `twice&(21)` is 42

#### Scenario: Default type of a FUNCTION
- **WHEN** a block declares `FUNCTION h_d (BYVAL v AS DOUBLE)`, whose C function returns `v + 1` as a `double`
- **THEN** `PRINT h_d(1.5)` prints ` 2.5 `, and with `DEFLNG H` before the block `PRINT h_d(1.7)` prints ` 2 `
  (the C result is truncated, not rounded)

#### Scenario: Default type of a parameter
- **WHEN** a block declares `FUNCTION h_d# (BYVAL v)`
- **THEN** `PRINT h_d#(1.5)` prints ` 2.5 `, and with `DEFLNG V` before the block ` 3 `

### Requirement: Rejected declarations
The forms the old compiler rejects SHALL be compile errors, each with the old compiler's reason: a `STRING`
parameter with `BYVAL`, an array parameter, `STATIC` after a declaration, `AS type` after a FUNCTION, a suffix
on a SUB, a `STRING * n` parameter, a TYPE defined after the block, an `ALIAS` that is empty or holds a character
not allowed, `BYVAL` outside a library, and a name already in use.

#### Scenario: An array parameter
- **WHEN** a block declares `SUB h_pl (v() AS LONG)`
- **THEN** compiling it fails with "Arrays cannot be passed to a library"

#### Scenario: A STRING by value
- **WHEN** a block declares `FUNCTION h_len& (BYVAL s AS STRING)`
- **THEN** compiling it fails with "STRINGs cannot be passed using BYVAL"

### Requirement: Rejected uses of a library procedure
A library FUNCTION SHALL be called without parentheses when it has no parameters (`f&()` is an error), SHALL NOT
be a statement, a `CONST` value or the target of an assignment, and a library SUB SHALL NOT be a value, each with
the old compiler's reason.

#### Scenario: Empty parentheses
- **WHEN** a block declares `FUNCTION h_ticks& ()` and the program prints `h_ticks&()`
- **THEN** compiling it fails with "Expected (...)"

### Requirement: Finding a library
The compiler SHALL look for each name as the old compiler does on Windows, after dropping a version after `.` or
`:`: a `.lib`, `.a`, `.o` (or, `DYNAMIC`, a `.dll`, also in System32) by the written path from the old compiler's
folder, `./` alone meaning the source's folder; then a header `.h` or `.hpp`, also from the source's or the
included file's folder.

#### Scenario: A library beside the program
- **WHEN** `v37c.a` lies beside the program and the block names `"./v37c"`
- **THEN** the program links it; with `"v37c"` compiling fails with "LIBRARY not found"

#### Scenario: A header from an included file's folder
- **WHEN** an included file in `v37_inc\` names `"../v37_lib/v37h"`
- **THEN** the header `v37_lib\v37h.h` is found and its function is called

#### Scenario: A DLL of the system
- **WHEN** a block names `DECLARE DYNAMIC LIBRARY "kernel32"` and declares `FUNCTION lstrlenA& (s AS STRING)`
- **THEN** `lstrlenA&("seven!!")` is 7

### Requirement: Rejected library names
A library that is not found, a `DYNAMIC` or `CUSTOMTYPE` block without a name, a name without quotes, two names
without a comma, and two names in a `DYNAMIC` block SHALL be compile errors with the old compiler's reasons. A
declaration without a header SHALL be an error when the library does not hold its name, even if it is never
called.

#### Scenario: A name the library does not hold
- **WHEN** a block names `"./v37c"` and declares `FUNCTION c_none& (BYVAL v AS LONG)`, never called
- **THEN** compiling it fails with "Could not find sub/function 'c_none' in" the library

#### Scenario: A DLL that is nowhere
- **WHEN** a block names `DECLARE DYNAMIC LIBRARY "v37_nowhere"`
- **THEN** compiling it fails with "DYNAMIC LIBRARY not found"

### Requirement: Headers and libraries in the C++
A header SHALL be included at each block, once per name that finds it, as the old compiler includes it, and a
block that names a header SHALL write no prototype for its declarations. Without a header, each declaration
SHALL be reachable with the linkage the library gives it (C or C++), looked for in the block's last library only.
Where the old compiler's C++ fails for these rules, the program SHALL be rejected before the C++ with an error
that says why: a header without an include guard that defines something and that two names find, two `DYNAMIC`
blocks whose DLL handles have one name, and a block without a header whose last name finds no library file.

#### Scenario: A header without a guard named twice
- **WHEN** two blocks name a header that has no include guard and defines its functions
- **THEN** compiling it fails with an error naming the header and the missing guard (the old compiler's C++
  fails, `verification\v37_x16`); with `#pragma once` in the header the program compiles

#### Scenario: A header without a guard that only declares
- **WHEN** two blocks name a header that has no include guard and only declares (`extern "C" int32_t
  c_add(int32_t a, int32_t b);`), and a third block links `"./v37c"`
- **THEN** the program compiles and prints what the old compiler's prints, ` 3  2.5  0`
  (`verification\v37_x68`)

#### Scenario: A last name that finds no file
- **WHEN** a block without a header names `"./v37c", "v37_lib/"` (the last a folder only) and declares `c_add&`
- **THEN** compiling it fails at the block with an error naming `v37_lib/` (the old compiler writes no prototype
  and its C++ fails, `verification\v37_x69`)

#### Scenario: Two DYNAMIC blocks of one DLL
- **WHEN** two blocks name `DECLARE DYNAMIC LIBRARY "kernel32"`
- **THEN** compiling it fails at the second block with an error naming the handle `DLL_kernel32` (the old
  compiler's C++ fails, `verification\v37_x67`)

#### Scenario: A C++ library without a header
- **WHEN** a block names `"./v37cpp"` (a C++ `.a`) and declares `FUNCTION cpp_add& (BYVAL a AS LONG, BYVAL b AS
  LONG)`
- **THEN** `cpp_add&(20, 22)` is 42

#### Scenario: A name in the first of two libraries
- **WHEN** a block names `"./v37c", "./v37cpp"` and declares `c_add&`, which only `v37c.a` holds
- **THEN** compiling it fails with "Could not find sub/function 'c_add' in" `v37cpp.a`, as the old compiler's
  (`verification\v37_x24`)

### Requirement: Arguments of a library call
An argument SHALL be passed as to a program's SUB: a variable, element or member of the parameter's type by its
address, so the C function may change it, anything else as a copy. A `BYVAL` argument SHALL be converted as for
an assignment and keep its BASIC type in C++; a `BYVAL` argument written `0`, `(0)`, `0&` or as a `CONST` 0
SHALL be `NULL`, an expression such as `1 - 1` SHALL NOT.

#### Scenario: Which 0 is NULL
- **WHEN** a header function takes a `const void*`, declared `FUNCTION h_isnull& (BYVAL p AS _OFFSET)`, and the
  program prints `h_isnull&((0))`, or `h_isnull&(1 - 1)` in another program
- **THEN** the first prints ` 1 `, and the second fails to compile, as the old compiler's

#### Scenario: By reference and by copy
- **WHEN** `SUB h_pl (v AS LONG)` adds one to `*v` and the program calls `h_pl x`, `h_pl (x)`, `h_pl a(2)` and
  `h_pl k` for an `INTEGER k`
- **THEN** `x` and `a(2)` grow by one for the first and third call, and neither `x` nor `k` changes for the
  second and fourth

#### Scenario: A BYVAL conversion
- **WHEN** `FUNCTION h_l& (BYVAL v AS LONG)` returns `v + 1` and the program prints `h_l&(2.6); h_b%%(300)` with
  `h_b%%` declared `BYVAL v AS _BYTE`
- **THEN** it prints ` 4  45 `

#### Scenario: An overload chosen by the argument
- **WHEN** a header has `h_over(int32_t)` returning 1 and `h_over(double)` returning 2, and a declaration
  `FUNCTION overd& ALIAS "h_over" (BYVAL v AS DOUBLE)` is called with `1`, `1.5` and a `LONG` variable
- **THEN** the results are 1, 2 and 1, as the old compiler's

### Requirement: Strings, user types and results
A `STRING` argument SHALL reach C as a pointer to its bytes, which C may change in place; a TYPE argument SHALL
reach C as a pointer to the value. A `STRING` FUNCTION's `char*` result SHALL be copied into a new string,
`NULL` giving "".

#### Scenario: A string changed in place
- **WHEN** `SUB h_upper (s AS STRING)` upper-cases its argument, `b$ = "xyz": c$ = b$: h_upper b$`
- **THEN** `b$` is `XYZ` and `c$` is `xyz`

#### Scenario: A NULL string result
- **WHEN** `FUNCTION h_nullstr$` returns `NULL`
- **THEN** `PRINT "["; h_nullstr$; "]"` prints `[]`

### Requirement: DLLs at the start of the program
A `DYNAMIC` library SHALL be loaded, and each of its names found, when the program starts, before its first
statement: a library that does not load SHALL be run-time error 259 and a name the DLL does not export error
260, neither of which `ON ERROR` catches.

#### Scenario: A name the DLL does not export
- **WHEN** a program sets `ON ERROR GOTO handler` and declares `d_none&` from `"./v37dll"`, which does not export
  it
- **THEN** it stops before its first `PRINT` with "Sub/Function does not exist in dynamic library"

### Requirement: CUSTOMTYPE libraries
A `CUSTOMTYPE` declaration SHALL call its C function through the declared parameter and result types, which may
differ from the function's own.

#### Scenario: Another result type
- **WHEN** `DECLARE CUSTOMTYPE LIBRARY "v37_lib/v37h"` declares `FUNCTION addll&& ALIAS h_add (BYVAL a AS LONG,
  BYVAL b AS LONG)` and `h_add` returns an `int32_t`
- **THEN** `addll&&(3, 4)` is 7
