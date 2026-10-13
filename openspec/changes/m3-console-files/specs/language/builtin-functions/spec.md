# Spec Delta

## MODIFIED Requirements

### Requirement: Supported built-in functions
`sema` SHALL compile these built-in functions: `INSTR`, `CHR$`, `LEN`, `LEFT$`, `RIGHT$`, `MID$`, `ASC`, `STR$`,
`VAL`, `STRING$`, `SPACE$`, `LTRIM$`, `RTRIM$`, `_TRIM$`, `UCASE$`, `LCASE$`, `HEX$`, `OCT$`, `_BIN$`, `_TOSTR$`,
`ABS`, `SGN`, `INT`, `FIX`, `SQR`, `SIN`, `COS`, `TAN`, `ATN`, `LOG`, `EXP`, `CINT`, `CLNG`, `CSNG`, `CDBL`,
`_ROUND`, `_PI`, `_ATAN2`, `_HYPOT`, `LBOUND`, `UBOUND`, `ERR`, `ERL`, `RND`, `TIMER`, `COMMAND$`, `ENVIRON$`,
`SHELL`, `EOF`, `LOF`, `LOC`, `SEEK`, `FREEFILE`, `_FILEEXISTS`, `_DIREXISTS`, `_CWD$`, and the functions added by
demand: `_ACOS`, `_ASIN`, `_SINH`, `_COSH`, `_TANH`, `_CEIL`, `_COT`, `_CSC`, `_SEC`, `_D2R`, `_R2D`, `_STRCMP`,
`_STRICMP`, `_STARTDIR$`, the destination and window functions (`language/console-and-window`): `_DEST`,
`_SOURCE`, `_CONSOLE`, `_SCREENEXISTS`, `_SCREENICON`, `_TITLE$`, `_DESKTOPWIDTH`, `_DESKTOPHEIGHT`, `_SCREENX`,
`_SCREENY`, `_WINDOWHANDLE`, `_WINDOWHASFOCUS`, `_NEWIMAGE`, `_FULLSCREEN`, the screen functions
(`language/screen-output`): `INKEY$`, `CSRLIN`, `POS`, `POINT`, `SCREEN`, `TAB`, `SPC`, the colour functions
`_RGB32` and `_RGBA32`, `DATE$` and `TIME$`, and the functions of conventional memory
(`language/conventional-memory`): `PEEK`, `VARPTR`, `VARSEG`, and the sound function `PLAY`
(`language/sound`), and the choice, the least and greatest, and the shifts: `_IIF`, `_MIN`, `_MAX`, `_CLAMP`,
`_SHL`, `_SHR`, `_ROL`, `_ROR`. Any other built-in function SHALL be
reported "not supported yet", as SHALL an argument of
type `_MEM`. A supported function that needs an argument, named without one, SHALL be a compile error, as in the
old compiler. An argument of any other numeric type SHALL be
converted to its slot as an assignment would, except where the old compiler special-cases the argument's type
(`LEN`, `STR$`, `HEX$`, `OCT$`, `_BIN$`, `ABS`, `SGN`, `VAL` with a type, `_IIF`, `_MIN`, `_MAX`, `_CLAMP`, `_ROL`,
`_ROR`, as measured). `VAL(s$, t)` with an
unsigned integer type `t` SHALL parse as libqb's unsigned `VAL` does (not narrowed to `t`, a minus sign dropped:
`VAL("-1", _UNSIGNED LONG)` is 1), and with a `_BIT` type SHALL be a compile error, as in the old compiler. A
fixed-length string argument SHALL be a string.

#### Scenario: Unsupported built-in
- **WHEN** a program uses `k = _KEYHIT`
- **THEN** the compiler reports `_KEYHIT` "not supported yet" and no other error for that expression

#### Scenario: Supported built-in compiles
- **WHEN** `PRINT LEFT$("hello", 2); LEN("abc")` is compiled and run
- **THEN** it prints `he 3`, as with the old compiler

#### Scenario: Unsigned argument
- **WHEN** `u~& = 4294967295: PRINT HEX$(u~&); STR$(u~&); ABS(u~&)` runs
- **THEN** it prints `FFFFFFFF 4294967295 4294967295 `

#### Scenario: VAL with an unsigned type
- **WHEN** `PRINT VAL("-1", _UNSIGNED LONG); VAL("300", _UNSIGNED _BYTE)` runs
- **THEN** it prints ` 1  300 `, as with the old compiler

#### Scenario: RND with and without an argument
- **WHEN** `RANDOMIZE 5: PRINT RND; RND(0)` runs
- **THEN** it prints the same value twice, the value the old compiler's program prints

#### Scenario: ENVIRON$ by name and by index
- **WHEN** `PRINT LEN(ENVIRON$("PATH")) > 0; LEN(ENVIRON$(1)) > 0` runs
- **THEN** it prints `-1 -1 `

#### Scenario: ENVIRON$ with a bad index
- **WHEN** `s$ = ENVIRON$(0)` runs under a handler, and then `PRINT ENVIRON$(100000) = ""`
- **THEN** the first raises error 5 and the second prints `-1`

#### Scenario: The SHELL function gives the exit code
- **WHEN** `PRINT SHELL("cmd /c exit 3"); SHELL("cmd /c exit 2") * 1000000000000` runs on Windows
- **THEN** it prints ` 3  2000000000000 ` (the result is an `_INTEGER64`)

#### Scenario: A function computed in its argument's type
- **WHEN** `s! = .5: PRINT _ACOS(s!); _ACOS(.5#); _CEIL(s!) / 3` runs
- **THEN** it prints ` 1.047197580337524  1.047197551196598  .3333333432674408 `, as with the old compiler (a SINGLE
  argument is computed as a C++ `float`)

#### Scenario: A function that raises
- **WHEN** `d# = _COT(0)` runs under a handler
- **THEN** error 5 is raised and `d#` is 0

#### Scenario: A function named without its argument
- **WHEN** a program has `n = EOF` or `PRINT SIN`
- **THEN** the compiler reports an error that is not "not supported yet"

#### Scenario: The console's handle
- **WHEN** a program with `$CONSOLE:ONLY` runs `PRINT _DEST; _CONSOLE; _SOURCE`
- **THEN** it prints `-10 -10 -10 `, as with the old compiler

#### Scenario: A screen function in a console program
- **WHEN** a program with `$CONSOLE:ONLY` runs `k$ = INKEY$: PRINT LEN(k$)`
- **THEN** it compiles and prints what the old compiler's program prints

#### Scenario: TAB and SPC as values
- **WHEN** `t$ = SPC(4): PRINT "["; t$; "]"; LEN(t$); LEN(SPC(3) + SPC(2))` runs
- **THEN** it prints `[    ] 4  5 `, as with the old compiler

#### Scenario: TAB with a string
- **WHEN** a program has `PRINT TAB("a"); 1` or `PRINT TAB(1, 2); 1`
- **THEN** the compiler reports an error that is not "not supported yet"

#### Scenario: The date and the time
- **WHEN** `PRINT LEN(DATE$); MID$(DATE$, 3, 1); LEN(TIME$); MID$(TIME$, 3, 1)` runs
- **THEN** it prints ` 10 - 8 :`, as with the old compiler

#### Scenario: A colour of 32 bits
- **WHEN** `PRINT HEX$(_RGB32(1, 2, 3)); HEX$(_RGBA32(1, 2, 3, 4)); HEX$(_RGB32(5))` runs
- **THEN** it prints what the old compiler's program prints, each value an `_UNSIGNED LONG`

## ADDED Requirements

### Requirement: _IIF evaluates one part
`_IIF(condition, truePart, falsePart)` SHALL evaluate the condition and then only the part it chooses. The
condition SHALL be a number, taken as an integer the way a number is rounded (so 0.5 and 0.4 are false). The two
parts SHALL be both strings or both numbers; two strings MAY be of different kinds, and the value is the one
chosen, with its length.

#### Scenario: Only the part chosen runs
- **WHEN** `PRINT _IIF(1, a, b)` runs, `a` and `b` FUNCTIONs that each print their name and return a number
- **THEN** only `a` prints its name

#### Scenario: A fraction as the condition
- **WHEN** `PRINT _IIF(0.5, "t", "f"); _IIF(0.6, "t", "f"); _IIF(4294967296, "t", "f")` runs
- **THEN** it prints `ftt`

#### Scenario: Strings of different kinds
- **WHEN** `DIM x AS STRING * 5: x = "ab": PRINT LEN(_IIF(0, x, "abc")); LEN(_IIF(1, x, "abc"))` runs
- **THEN** it prints ` 3  5 `

#### Scenario: Rejected forms
- **WHEN** a program has `_IIF("a", 1, 2)`, `_IIF(1, "a", 2)`, `_IIF(1, 2)`, `_IIF(1, 2, 3, 4)` or the line `_IIF
  1, 2, 3`
- **THEN** each is a compile error that is not "not supported yet"

### Requirement: The result type of _IIF, _MIN, _MAX and _CLAMP
The numeric result of `_IIF`, `_MIN`, `_MAX` and `_CLAMP` SHALL have the type the old compiler gives it for the
types of its value arguments: a `_FLOAT` among them gives `_FLOAT`, else a DOUBLE gives DOUBLE, else a SINGLE
gives SINGLE, whatever integer stands beside it; of two integers the wider, and of two of one width the unsigned
one. The table `verification\v30_result_types.txt` SHALL hold for every pair of numeric types.

#### Scenario: A SINGLE beside a wide integer
- **WHEN** `s! = 16777217: q&& = 16777216: PRINT _MAX(s!, q&&)` runs
- **THEN** it prints what the old compiler's program prints: the result is computed as a SINGLE

#### Scenario: Signed and unsigned of one width
- **WHEN** `b%% = -1: ub~%% = 255: PRINT _MIN(b%%, ub~%%)` runs
- **THEN** it prints ` 255 `

#### Scenario: Every pair of types
- **WHEN** the compiler's result type of `_MIN(a, b)`, `_IIF(1, a, b)` and `_CLAMP(a, b, b)` is compared with the
  table for all 256 pairs of numeric types
- **THEN** every one is equal, the two pairs that depend on the order included

### Requirement: _MIN, _MAX and _CLAMP
`_MIN(a, b)` and `_MAX(a, b)` SHALL give the lesser and the greater, `_CLAMP(value, low, high)` the value held
between the two bounds, each argument evaluated once, in order, and computed in the result's type as the old
compiler's program computes it. A string or a `TYPE` value SHALL be a compile error, as SHALL a wrong number of
arguments.

#### Scenario: Least, greatest and clamped
- **WHEN** `PRINT _MIN(3, 7); _MAX(3, 7); _CLAMP(15, 0, 10); _CLAMP(-5, 0, 10); _CLAMP(5, 0, 10)` runs
- **THEN** it prints ` 3  7  10  0  5 `

#### Scenario: Arguments in order
- **WHEN** `PRINT _CLAMP(a, b, c)` runs, each a FUNCTION that prints its name
- **THEN** the names are printed once each, in the order `a`, `b`, `c`

#### Scenario: A string
- **WHEN** a program has `_MIN("a", "b")` or `_CLAMP(1, 2)`
- **THEN** each is a compile error that is not "not supported yet"

### Requirement: Shifts and rotations
`_SHL(value, count)` and `_SHR(value, count)` SHALL shift an `_UNSIGNED _INTEGER64`, whatever the argument's
type, the count taken modulo 64. `_ROL(value, count)` and `_ROR(value, count)` SHALL turn the bits within the
width of the first argument's type (a literal by its size), the result unsigned of that width; a float or a
string as either argument SHALL be a compile error, as SHALL a `_BIT` type.

#### Scenario: A shift of a negative value
- **WHEN** `PRINT _SHR(-1, 1); _SHL(1, 65)` runs
- **THEN** it prints ` 9223372036854775807  2 `

#### Scenario: A rotation by the type's width
- **WHEN** `b~%% = 129: i% = 1: PRINT _ROL(b~%%, 1); _ROR(i%, 1); _ROR(70000, 32)` runs
- **THEN** it prints ` 3  32768  70000 `

#### Scenario: A float argument
- **WHEN** a program has `_ROL(1.5, 1)`, `_ROL(1, 1.5)` or `_ROR("a", 1)`
- **THEN** each is a compile error that is not "not supported yet"

### Requirement: Not in a CONST
`_IIF`, `_MIN`, `_MAX`, `_CLAMP`, `_SHL`, `_SHR`, `_ROL` and `_ROR` in a `CONST` expression SHALL be a compile
error, as in the old compiler.

#### Scenario: A CONST with _MIN
- **WHEN** a program has `CONST c = _MIN(1, 2)`
- **THEN** it is a compile error that is not "not supported yet"
