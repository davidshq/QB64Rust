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

### Requirement: Errors the C++ build finds
When the C++ build fails and every error of the C++ compiler and the linker names the C function of a declaration
in a block that is not `DYNAMIC`, each error SHALL be reported as an error of the program, not marked "not
supported yet": a compile error at the BASIC line the C++ compiler names, with its message and its notes; an
undefined symbol at the declaration. The summary line SHALL follow and the exit status SHALL be 1. Any other
failure SHALL be reported as the toolchain's output.

#### Scenario: A function the header does not declare
- **WHEN** a block names `"v37_lib/v37h"`, declares `FUNCTION h_none& (BYVAL v AS LONG)`, which the header does
  not declare, and line 6 prints `h_none&(1)`
- **THEN** compiling it fails with an error at line 6 naming `h_none`, then `1 error`, and no executable
  (`verification\v37_x30`; the old compiler's C++ fails)

#### Scenario: The C name in another letter case
- **WHEN** a block names `"v37_lib/v37h"` and declares `FUNCTION H_L& (BYVAL v AS LONG)` where the header has
  `h_l`
- **THEN** compiling it fails with an error at the line that calls it, naming `H_L` (`verification\v37_x31`)

#### Scenario: A function of the library that the header does not declare
- **WHEN** a block names `"v37_lib/v37h", "./v37c"` and declares `c_half#`, which `v37c.a` holds and the header
  does not declare
- **THEN** compiling it fails with an error at the line that calls it, naming `c_half` (`verification\v37_x23`)

#### Scenario: An argument C++ cannot convert
- **WHEN** a header function takes a `const void*`, declared `FUNCTION h_isnull& (BYVAL p AS _OFFSET)`, and the
  program prints `h_isnull&(5)`
- **THEN** compiling it fails with an error at that line naming `h_isnull` that says no conversion from `long
  long` to `const void *` is known (`verification\v37_x17`)

#### Scenario: A function defined nowhere
- **WHEN** a `DECLARE CUSTOMTYPE LIBRARY` block with no name declares `FUNCTION f& ()`, which neither libqb nor the
  C runtime defines
- **THEN** compiling it fails with an error at the declaration naming `f` as not defined (`verification\v37_x04`)

#### Scenario: Another C++ failure
- **WHEN** the C++ build fails with an error that names no library function
- **THEN** the toolchain's output is printed as before, with no summary line, and the exit status is 1

### Requirement: A library procedure and a program procedure of one name
A library FUNCTION and a program SUB of the same name without its suffix, or a library SUB and a program
FUNCTION, SHALL compile as in the old compiler: the name as a statement SHALL call the SUB, and the name as a
value, with its suffix or without, SHALL call the FUNCTION, whichever of the two is the library's (measured,
`verification\v57_l01`–`l04`).

#### Scenario: A library FUNCTION and a program SUB
- **WHEN** a `DECLARE LIBRARY` block declares `FUNCTION h_l& (BYVAL v AS LONG)` and the program defines `SUB h_l`
- **THEN** it compiles (`verification\v37_x22`)

#### Scenario: Both called
- **WHEN** that program calls `h_l` as a statement and prints `h_l&(2)`, and another prints `h_l(2)`
- **THEN** the statement runs the program's SUB, and each value is the library FUNCTION's, 3
  (`verification\v57_l01`, `l02`)

#### Scenario: A library SUB and a program FUNCTION
- **WHEN** a `DECLARE LIBRARY` block declares `SUB h_v (BYVAL v AS LONG)`, the program defines `FUNCTION h_v&`, and
  the program runs `h_v 3` and prints `h_v&(2)`
- **THEN** the statement calls the library SUB and the value is the program FUNCTION's (`verification\v57_l03`)

### Requirement: $USELIBRARY
`$USELIBRARY:'author/library'` SHALL read the descriptor `libraries\descriptors\author\library.ini` under the
compiler root and include the files its `[LIBRARY INCLUDES]` section names (under
`libraries\includes\author\library\`) where the old compiler does: `IncAtTop` before the first line,
`IncAfterMain` after the main module's implicit `END`, `IncAtBottom` after the last line (measured,
`verification\v58_u00`–`u26`).

#### Scenario: A library with three files
- **WHEN** a program has `$USELIBRARY:'test/three'`, whose descriptor names a file of each kind, and calls a SUB
  of the bottom file and reads a `CONST` of the top file
- **THEN** it compiles and prints what the old compiler's program prints with the same library

#### Scenario: A descriptor with only some files
- **WHEN** the descriptor names only `IncAtBottom`, or names `IncAtTop=` with no value
- **THEN** only the bottom file is included (`verification\v58_u02`, `u18`)

#### Scenario: A descriptor's spelling
- **WHEN** the descriptor has a comment line, another section before `[library includes]` in lower case,
  `  incattop = "q_top.bi" ; a comment` and `IncAtBottom=q_bottom.bm ' a comment`
- **THEN** the two files are included (`verification\v58_u07`, `u23`)

#### Scenario: Nothing to include
- **WHEN** the descriptor has no `[LIBRARY INCLUDES]` section
- **THEN** the program compiles with nothing included (`verification\v58_u24`)

#### Scenario: The line's forms
- **WHEN** a program has `$uselibrary:'test/a' ' uses a`, `$USELIBRARY 'test/a'`, `$USELIBRARYX:'test/a'`,
  `$USELIBRARY:'  test/a  '` or `$USELIBRARY:'test/a'` followed by a tab and a word
- **THEN** each includes the library (`verification\v58_u09`–`u11`, `u21`, `u25`)

#### Scenario: In an included file and in a branch not taken
- **WHEN** the line stands in a file the program includes, or inside `$IF 0 THEN … $END IF`
- **THEN** the library is included from the included file, and not from the branch (`verification\v58_u08`,
  `u22`)

### Requirement: The order of several libraries
Libraries SHALL be included in the old compiler's order: the `IncAtTop` files from the last registered library to
the first, the `IncAfterMain` and `IncAtBottom` files from the first to the last. A library SHALL be registered
once per line that names it, so one named twice has its files included twice; a library's file may itself use
`$USELIBRARY` (measured, `verification\v58_u03`–`u06`).

#### Scenario: Two libraries
- **WHEN** a program uses `test/a` and then `test/b`, each with a top file that notes its name in a string and a
  bottom file with a SUB
- **THEN** it prints `main b;a;` then `sub a` and `sub b`, as the old compiler's program does
  (`verification\v58_u03`)

#### Scenario: A library named twice
- **WHEN** a program has `$USELIBRARY:'test/a'` on two lines, or uses `test/a` and `test/c` whose top file uses
  `test/a`
- **THEN** the compile fails with an error at the second definition of the library's SUB, whose note is "Name
  already in use (ASub) …" (`verification\v58_u04`, `u05`)

#### Scenario: A library that needs another
- **WHEN** the top file of `test/c` has `$USELIBRARY:'test/a'`
- **THEN** it compiles and prints `main a;c;`, then the SUB of `c` calls the SUB of `a` (`verification\v58_u06`)

### Requirement: Errors of $USELIBRARY
A line with no quoted name, or an empty one, SHALL be the error "Expected $USELIBRARY:'author/library'"; a missing
descriptor "Library descriptor for '…' not found"; a file the descriptor names that is missing "Library source
for '…' not found (" with the key's name and ")", each with the old compiler's message as the note (measured,
`verification\v58`).

#### Scenario: Without its quotes
- **WHEN** a program has `$USELIBRARY:test/a`, `$USELIBRARY:"test/a"` or `$USELIBRARY:''`
- **THEN** the compile fails with an error whose note is "Expected $USELIBRARY:'author/library'"
  (`verification\v58_u12`–`u14`)

#### Scenario: No descriptor
- **WHEN** a program has `$USELIBRARY:'test/none'` and the compiler root holds no such descriptor
- **THEN** the compile fails with an error whose note is "Library descriptor for 'test/none' not found"
  (`verification\v58`, recorded also with the release's own folder, which has no `libraries\`)

#### Scenario: A file missing
- **WHEN** the descriptor of `test/broken` names an `IncAfterMain` file that does not exist
- **THEN** the compile fails with an error whose note is "Library source for 'test/broken' not found
  (IncAfterMain)"

### Requirement: Fixed-length strings to a char pointer
A `STRING * n` member of a value, or element of an array, passed to a library parameter `AS STRING` SHALL reach C
as a copy of its n bytes followed by a NUL, and the n bytes SHALL be copied back into the place after the call, its
indexes evaluated once, whatever the fixes (`study\39` row 10; a `QB64PE-DIVERGENCES.md` row). A `STRING * n`
variable of its own SHALL still pass its own bytes.

#### Scenario: A member read by strlen
- **WHEN** a TYPE holds `a AS STRING * 4` then `b AS STRING * 4`, `t.a = "abcd": t.b = "efgh"`, and a library
  function returning `strlen` of its `char*` is called with `t.a`
- **THEN** it returns 4, not 8 (`verification\v37_x66`, now a divergence)

#### Scenario: A write comes back
- **WHEN** a library SUB that upper-cases its `char*` in place is called with an element `e(i)` of a `STRING * 3`
  array, where `i` is read from a FUNCTION that counts its calls
- **THEN** `e(i)` is upper-cased after the call, and the FUNCTION was called once
