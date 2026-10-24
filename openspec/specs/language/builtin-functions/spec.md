# language/builtin-functions Specification

## Purpose
How built-in functions are compiled in expressions: which are supported, how arguments are converted to their
slots, the type of each result, and the runtime errors they raise, as measured with `qb64pe.exe`. Built-ins not
listed here are reported "not supported yet".

## Requirements

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
(`language/conventional-memory`): `PEEK`, `VARPTR`, `VARSEG`, and the sound functions `PLAY`, `_SNDOPEN`, `_SNDCOPY`, `_SNDLEN`, `_SNDGETPOS`, `_SNDPLAYING`, `_SNDPAUSED`, `_SNDRATE`, `_SNDOPENRAW`, `_SNDRAWLEN`, `_SNDNEW`
(`language/sound`), and the choice, the least and greatest, and the shifts: `_IIF`, `_MIN`, `_MAX`, `_CLAMP`,
`_SHL`, `_SHR`, `_ROL`, `_ROR`, and the image, colour, font and mouse functions (`language/images`,
`language/fonts`, `language/mouse`): `_LOADIMAGE`, `_COPYIMAGE`, `_WIDTH`, `_HEIGHT`, `_PIXELSIZE`,
`_CLEARCOLOR`, `_BLEND`, `_PALETTECOLOR`, `_AUTODISPLAY`, `PMAP`, `_SCREENIMAGE`, `_CLIPBOARDIMAGE`, `_RGB`,
`_RGBA`, `_RED`, `_GREEN`, `_BLUE`, `_ALPHA`, `_RED32`, `_GREEN32`, `_BLUE32`, `_ALPHA32`, `_HSB32`,
`_HSBA32`, `_HUE32`, `_SATURATION32`, `_BRIGHTNESS32`, `_LOADFONT`, `_FONT`, `_FONTWIDTH`, `_FONTHEIGHT`,
`_PRINTWIDTH`, `_PRINTMODE`, `_UFONTHEIGHT`, `_UPRINTWIDTH`, `_ULINESPACING`, `_UCHARPOS`, `_MAPUNICODE`,
`_MOUSEINPUT`, `_MOUSEX`, `_MOUSEY`, `_MOUSEBUTTON`, `_MOUSEWHEEL`, `_MOUSEMOVEMENTX`, `_MOUSEMOVEMENTY`,
`_MOUSEHIDDEN`, `_MOUSEDISABLED`, and the functions of memory blocks (`language/mem-blocks`): `_MEM`,
`_MEMELEMENT`, `_MEMNEW`, `_MEMIMAGE`, `_MEMSOUND`, `_MEMEXISTS`, `_MEMGET`, `_OFFSET`, and the conversions
between a number and its bytes: `MKI$`, `MKL$`, `MKS$`, `MKD$`, `_MK$`, `CVI`, `CVL`, `CVS`, `CVD`, `_CV`, and the checksum, compression
and base64 functions (`language/data-processing`): `_ADLER32`, `_CRC32`, `_DEFLATE$`, `_INFLATE$`,
`_BASE64ENCODE$`, `_BASE64DECODE$`, and `_MD5$`, and `_CAST`, `_OS$`, `_CLIPBOARD$` and `_FILES$` (`language/file-io`),
and the keyboard functions `_KEYHIT` and `_KEYDOWN` (`language/keyboard`), the window and console functions
`_RESIZE`, `_RESIZEWIDTH`, `_RESIZEHEIGHT`, `_EXIT`, `_ACCEPTFILEDROP`, `_TOTALDROPPEDFILES`, `_DROPPEDFILE$`,
`_MESSAGEBOX`, `_OPENFILEDIALOG$`, `_SAVEFILEDIALOG$`, `_SELECTFOLDERDIALOG$` (`language/console-and-window`),
`_CONTROLCHR`, `_DEFAULTCOLOR` and `_BACKGROUNDCOLOR` (`language/screen-output`), and `_DIR$`, `_SHELLHIDE`,
`_COMMANDCOUNT`, `_COMPILEDATE$`, `_COMPILETIME$` and `_READBIT`, and the functions no program used before
`m4-builtins-remaining`: `_ASINH`, `_ACOSH`, `_ATANH`, `_D2G`, `_G2D`, `_R2G`, `_G2R`, `_SETBIT`, `_RESETBIT`,
`_TOGGLEBIT`, `MKSMBF$`, `MKDMBF$`, `CVSMBF`, `CVDMBF`, `LPOS`, `INPUT$` (a count and an optional file number, which
may carry a `#`), `_COMPILERVERSION$`, `_ENVIRONCOUNT`, `_UPTIME`,
the lock keys (`language/keyboard`), the console, window and dialog functions `_CINP`, `_CONSOLEINPUT`,
`_SCALEDWIDTH`, `_SCALEDHEIGHT`, `_SMOOTH`, `_DESKTOPREFRESHRATE`, `_COLORCHOOSERDIALOG` and `_INPUTBOX$`
(`language/console-and-window`), `_BLINK` and `_DISPLAY` (`language/screen-output`), and `INP`
(`language/conventional-memory`). Every function of the old compiler's built-in table SHALL be compiled, or be
the old compiler's error where it rejects the form. A supported function that needs an argument, named without one, SHALL be a compile error, as in the
old compiler. An argument of any other numeric type SHALL be
converted to its slot as an assignment would, except where the old compiler special-cases the argument's type
(`LEN`, `STR$`, `HEX$`, `OCT$`, `_BIN$`, `ABS`, `SGN`, `VAL` with a type, `_IIF`, `_MIN`, `_MAX`, `_CLAMP`, `_ROL`,
`_ROR`, as measured). `VAL(s$, t)` with an
unsigned integer type `t` SHALL parse as libqb's unsigned `VAL` does (not narrowed to `t`, a minus sign dropped:
`VAL("-1", _UNSIGNED LONG)` is 1), and with a `_BIT` type SHALL be a compile error, as in the old compiler. A
fixed-length string argument SHALL be a string.

#### Scenario: Unsupported built-in
- **WHEN** a program uses `VARPTR` of a `TYPE` variable that holds a `STRING`, in conventional memory
  (`verification\v56_a5`, a form step 3 of M4 left marked)
- **THEN** the compiler reports it "not supported yet" and no other error for that expression

#### Scenario: Every function of the table compiles
- **WHEN** each function of `tools\builtins\builtins.json` is called in a form its syntax gives, with arguments of
  its parameters' types
- **THEN** none is reported "not supported yet"

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

#### Scenario: An image function
- **WHEN** `PRINT _WIDTH(_NEWIMAGE(64, 48, 32)); _RED32(&HFF102030)` runs
- **THEN** it prints ` 64  16 `, as with the old compiler

#### Scenario: A number and its bytes
- **WHEN** `PRINT LEN(MKL$(1)); CVL(MKL$(-2)); CVI(MKI$(300)); CVS(MKS$(1.5)); CVD(MKD$(2.25));
  LEN(_MK$(_UNSIGNED _BYTE, 255)); _CV(_INTEGER64, _MK$(_INTEGER64, -9))` runs
- **THEN** it prints ` 4 -2  300  1.5  2.25  1 -9 `, as with the old compiler

#### Scenario: CV of a number
- **WHEN** `PRINT CVL(5)` is compiled
- **THEN** it is a compile error ("CVL requires a STRING argument"), as in the old compiler

### Requirement: Names and arity
A built-in function SHALL be recognised by its name without regard to case, written with its required suffix
(`LEFT$`, `CHR$`) or without one where it has none (`LEN`, `ABS`); a name used with another suffix SHALL NOT be the
built-in. A call with too few or too many arguments, or with an argument of the wrong kind (a string where a number
is needed or the reverse), SHALL be a compile error, except where the old compiler accepts it, as measured.

#### Scenario: Wrong number of arguments
- **WHEN** `PRINT LEFT$("abc")` is compiled
- **THEN** it is a compile error, not a "not supported yet" mark

#### Scenario: String where a number is needed
- **WHEN** `PRINT SQR("4")` is compiled
- **THEN** it is a compile error

#### Scenario: Optional argument
- **WHEN** `PRINT MID$("hello", 2); MID$("hello", 2, 3)` runs
- **THEN** it prints `elloell`

### Requirement: Argument conversion
Each argument SHALL be converted to its slot's type as the old compiler converts it, measured once per slot type:
a numeric argument to an integer slot as a store into a variable of that type (a float rounded half to even to
`_INTEGER64`, then the slot's low bits), to a DOUBLE slot exactly, and to a `_FLOAT` or "any numeric" slot not at
all (passed in its own type). A string argument SHALL be passed as it is.

#### Scenario: Float argument to an integer slot
- **WHEN** `PRINT LEFT$("abcdef", 2.5); LEFT$("abcdef", 3.5)` runs
- **THEN** it prints `ababcd`, as with the old compiler (2.5 rounds to 2, 3.5 to 4)

#### Scenario: Value beyond LONG in a LONG slot
- **WHEN** `PRINT LEFT$("abcdef", 4294967298#)` runs
- **THEN** it prints `ab`, as with the old compiler (the low 32 bits of 4294967298 are 2)

### Requirement: Result types
The result of a built-in SHALL have the type the old compiler gives it, measured: the table's return type for a
plain function; for `ABS`, `INT` and `FIX` the argument's type; for `SIN`, `COS`, `TAN`, `ATN`, `SQR`, `LOG` and
`EXP` a type chosen by the argument's type; for `CLNG` LONG; for `VAL` `_FLOAT`, or with a type argument SINGLE,
DOUBLE or `_FLOAT` as named and `_INTEGER64` for every integer type; for `LBOUND`/`UBOUND` `_INTEGER64`. Printing
and arithmetic SHALL follow from that type, including where the old compiler computes in a wider type than it
reports (`CSNG` of an integer is printed as a SINGLE but not narrowed).

#### Scenario: ABS keeps the argument's type
- **WHEN** `PRINT ABS(-7%) / 2` runs
- **THEN** it prints `3.5`, and the typed tree shows `ABS` typed INTEGER

#### Scenario: Math function typed by its argument
- **WHEN** `PRINT SQR(2%); SQR(2#)` runs
- **THEN** the first value is printed as a SINGLE and the second as a DOUBLE, as with the old compiler

#### Scenario: VAL with a type
- **WHEN** `PRINT VAL("16777217", SINGLE) = 16777216&&` runs
- **THEN** it prints `-1`

### Requirement: String results
The string built-ins SHALL return what the old compiler's runtime returns for every argument value, including
empty strings, positions past the end and zero or negative lengths (an empty string, no error); `STR$` SHALL put a
blank before a value that is not negative; `HEX$`, `OCT$` and `_BIN$` of a negative integer SHALL use the width of
the argument's type, and for a 64-bit argument that is not a variable, element or member the shortest width, as
the old compiler.

#### Scenario: Negative length
- **WHEN** `PRINT "[" + LEFT$("abc", -1) + "]"` runs under a handler that prints `ERR`
- **THEN** it prints `[]` and the handler does not run

#### Scenario: STR$ sign blank
- **WHEN** `PRINT "[" + STR$(5) + "][" + STR$(-5) + "]"` runs
- **THEN** it prints `[ 5][-5]`

#### Scenario: HEX$ of a negative INTEGER
- **WHEN** `PRINT HEX$(-1%); " "; HEX$(-1&)` runs
- **THEN** it prints `FFFF FFFFFFFF`

#### Scenario: MID$ past the end
- **WHEN** `PRINT "[" + MID$("abc", 5) + "]"` runs
- **THEN** it prints `[]`

### Requirement: Runtime errors of built-ins
A built-in SHALL raise the runtime error the old compiler raises for an argument outside its domain (such as
error 5, "Illegal function call", for `ASC("")`, `SQR(-1)`, `LOG(0)` or `CHR$(256)`, and error 6, "Overflow",
for `CINT(40000)` or `EXP(1000)`), and SHALL then yield the placeholder value under the IR's pending-error rule.

#### Scenario: ASC of an empty string
- **WHEN** `x = 5: x = ASC("")` runs under a handler that prints `ERR` and resumes next, then `x` is printed
- **THEN** the handler prints 5, and `x` holds 0

#### Scenario: CINT overflow
- **WHEN** `PRINT CINT(40000)` runs under a handler that prints `ERR` and resumes next
- **THEN** the handler prints 6

### Requirement: Built-ins without parentheses
A built-in function that takes no required argument (`_PI`, `RND`, `TIMER`, `COMMAND$`, `FREEFILE`, `_CWD$`, `_STARTDIR$`, `_DEST`, `_SOURCE`, `_CONSOLE`, `_SCREENEXISTS`, `_SCREENICON`, `_TITLE$`, `_DESKTOPWIDTH`,
`_DESKTOPHEIGHT`, `_SCREENX`, `_SCREENY`, `_WINDOWHANDLE`, `_WINDOWHASFOCUS`) SHALL
be callable without parentheses, and with its optional argument in parentheses where it has one, as the old
compiler allows.

#### Scenario: _PI bare and with an argument
- **WHEN** `PRINT _PI; _PI(2)` runs
- **THEN** it prints the same two values as the old compiler

#### Scenario: TIMER and COMMAND$ bare
- **WHEN** `t! = TIMER: c$ = COMMAND$: PRINT t! >= 0; LEN(c$)` runs with no command-line arguments
- **THEN** it prints `-1  0 `

#### Scenario: A variable of the same name
- **WHEN** a program uses a name such as `FREEFILE` or `TIMER` as a variable
- **THEN** the compiler does what the old compiler does with it, as measured (a compile error where the name is
  reserved)

#### Scenario: _WINDOWHANDLE bare and with an argument
- **WHEN** a program has `PRINT _WINDOWHANDLE <> 0` and `PRINT _WINDOWHANDLE(1) <> 0`
- **THEN** both compile, as with the old compiler

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

### Requirement: _COTH is the hyperbolic cotangent
`_COTH(x)` SHALL be supported at run time as the old compiler calls it (libqb's `func_coth`, believed `_FLOAT`,
held `double`). Its value SHALL be the hyperbolic cotangent of `x`, with no upper limit and error 5 at 0, through
this repo's copy of libqb (`DIVERGENCES.md` D-031: QB64pe's `func_coth` is `2 * EXP(x) - 1`, with error 5 above
44.014845).

#### Scenario: Values
- **WHEN** `x# = .2: y# = -.5: z# = 45: PRINT CLNG(_COTH(x#) * 1000); CLNG(_COTH(y#) * 1000); _COTH(z#)` runs
- **THEN** it prints ` 5066 -2164  1 `

#### Scenario: At 0
- **WHEN** `PRINT _COTH(0)` runs under `ON ERROR` with a handler that prints `ERR`
- **THEN** the handler prints 5

#### Scenario: The same call as the old compiler
- **WHEN** the call-site check compares `d = _COTH(e)`, `_COTH(.5)`, `_COTH(n)` and `_COTH(sh)`
- **THEN** each libqb call equals the old compiler's

### Requirement: _CAST
`_CAST(type, value)` SHALL convert a numeric value to the numeric type named (an integer type of 8 to 64 bits,
signed or `_UNSIGNED`, `_OFFSET`, `SINGLE`, `DOUBLE` or `_FLOAT`) as the old compiler's C++ cast does, and its
result SHALL have that type. Any other type named, and a string or a whole user type as the value, SHALL be the old
compiler's errors.

#### Scenario: Narrowed to a byte
- **WHEN** `f! = 3.14159: i%& = 255: PRINT _CAST(_BYTE, i%&); _CAST(_UNSIGNED _BYTE, i%& * 2); _CAST(INTEGER, f! * 2)` runs
- **THEN** it prints `-1  254  6 `, as with the old compiler

#### Scenario: A float type
- **WHEN** `PRINT USING "#####.###"; _CAST(SINGLE, 3.141592653589793#)` runs
- **THEN** it prints `    3.142`

#### Scenario: Toward zero
- **WHEN** `PRINT _CAST(INTEGER, 2.5); _CAST(INTEGER, -1.5); _CAST(LONG, 0.9999)` runs
- **THEN** it prints ` 2 -1  0 `, as with the old compiler (a float is truncated, not rounded)

#### Scenario: The type the result is seen as
- **WHEN** `PRINT HEX$(_CAST(_BYTE, -1)); " "; HEX$(_CAST(INTEGER, -1)); _CAST(SINGLE, 1 / 3)` runs
- **THEN** it prints `FF FFFF .3333333`, as with the old compiler

#### Scenario: A type _CAST does not take
- **WHEN** a program has `x = _CAST(STRING, 1)` or `x = _CAST(_BIT, 1)`
- **THEN** it is a compile error, as in the old compiler ("_CAST TYPE unsupported")

#### Scenario: A name that is no numeric type
- **WHEN** a program has `x = _CAST(pt, 1)` for a user type `pt`, `_CAST(foo, 1)`, `_CAST(_MEM, 1)`,
  `_CAST(STRING * 4, 1)` or `_CAST(1, LONG)`
- **THEN** it is a compile error, as in the old compiler ("Cannot convert type (…) to symbol")

#### Scenario: A string value
- **WHEN** a program has `x = _CAST(LONG, "1")` or `_CAST(LONG, p)` for a whole user-type value `p`
- **THEN** it is the compile error "Expected numeric argument", as in the old compiler

#### Scenario: Not one type and one value
- **WHEN** a program has `_CAST(LONG)`, `_CAST(LONG, 1, 2)`, `x = _CAST` or `CONST c = _CAST(LONG, 2.7)`
- **THEN** it is a compile error, as in the old compiler

### Requirement: The system's name and the clipboard
`_OS$` SHALL give the old compiler's runtime's description of the system the program runs on. `_CLIPBOARD$` SHALL
give the text on the system's clipboard, and `_CLIPBOARD$ = s$` SHALL put `s$` there up to its first NUL, through
the old compiler's runtime calls. Their wrong forms SHALL be the old compiler's errors.

#### Scenario: The system's name
- **WHEN** `PRINT INSTR(_OS$, "[WINDOWS]") > 0` runs on Windows
- **THEN** it prints `-1 `

#### Scenario: Through the clipboard
- **WHEN** `_CLIPBOARD$ = "abc": PRINT _CLIPBOARD$ = "abc"` runs
- **THEN** it prints `-1 `

#### Scenario: A NUL ends the text
- **WHEN** `_CLIPBOARD$ = "a" + CHR$(0) + "b": PRINT LEN(_CLIPBOARD$)` runs
- **THEN** it prints ` 1 `, as with the old compiler

#### Scenario: Wrong forms
- **WHEN** a program has `_OS$ = "a"`, `PRINT _OS$(1)`, `PRINT _CLIPBOARD$("a")`, `_CLIPBOARD$ = 5`
  or `CALL _CLIPBOARD$("abc")`
- **THEN** each is a compile error, as in the old compiler

### Requirement: Searching from the end with _INSTRREV
`sema` SHALL compile `_INSTRREV(s$, find$)` and `_INSTRREV(start, s$, find$)`. Each SHALL return what the old
compiler's program returns: the position of the last `find$` in `s$` that starts at or before `start` (the whole
string without `start`), and 0 when there is none. Edge cases (an empty `find$`, `start` of 0, below 0, or beyond
the length) SHALL give the old compiler's results as `verification\v52` records them. The result SHALL be a LONG.

#### Scenario: The last match
- **WHEN** `PRINT _INSTRREV("a.b.c", ".")` runs
- **THEN** it prints ` 4 `

#### Scenario: A start position
- **WHEN** `PRINT _INSTRREV(3, "a.b.c", ".")` runs
- **THEN** it prints ` 2 `

#### Scenario: No match
- **WHEN** `PRINT _INSTRREV("abc", "x")` runs
- **THEN** it prints ` 0 `

#### Scenario: Edge cases as recorded
- **WHEN** a program of `verification\v52` calls `_INSTRREV` with an empty string, an empty `find$`, or a `start` of
  0, below 0 or beyond the length
- **THEN** it prints what its recorded output holds

### Requirement: The reciprocal and hyperbolic functions at run time
`sema` SHALL compile `_ARCSEC`, `_ARCCSC`, `_ARCCOT`, `_SECH` and `_CSCH` of one numeric argument, outside a
`CONST` as well as inside it. As `_SEC` and `_COT` are, they SHALL be believed `_FLOAT` and computed by the old
compiler's runtime entries, so that a value prints as the old compiler's program prints it. An argument outside a
function's domain SHALL raise error 5 and return 0, as the old compiler's program does (`verification\v52_b_math`).

#### Scenario: Values
- **WHEN** `x# = 1.2: PRINT _ARCSEC(x#); _ARCCSC(x#); _ARCCOT(x#); _SECH(x#); _CSCH(x#)` runs
- **THEN** it prints what the old compiler's program prints

#### Scenario: Outside the domain
- **WHEN** `PRINT _ARCSEC(0.5)` runs (outside the domain of `_ARCSEC`)
- **THEN** it raises error 5, and after `RESUME NEXT` the value is 0

#### Scenario: The other domains
- **WHEN** a program calls `_ARCCSC(-0.5)`, `_SECH(88.1)`, `_CSCH(88.1)` or `_CSCH(0)`
- **THEN** each raises error 5 and returns 0

### Requirement: The system's folders, commands and build
`_DIR$(name$)`, `_SHELLHIDE(cmd$)`, `_COMMANDCOUNT`, `_COMPILEDATE$`, `_COMPILETIME$` and `_READBIT(a, b)` SHALL
compile to the old compiler's runtime calls, with its result types (`_SHELLHIDE` an `_INTEGER64`, `_READBIT` an
`_INTEGER64`), and SHALL give at run time what the old compiler's program gives. Their wrong forms SHALL be the old
compiler's errors.

#### Scenario: A folder of the system
- **WHEN** `d$ = _DIR$("fonts"): PRINT RIGHT$(d$, 1) = "\"; _DIREXISTS(d$)` runs on Windows
- **THEN** it prints `-1 -1 `, as with the old compiler

#### Scenario: A command run hidden
- **WHEN** `PRINT _SHELLHIDE("cmd /c exit 3")` runs on Windows
- **THEN** it prints ` 3 `

#### Scenario: No arguments on the command line
- **WHEN** a program started with no arguments prints `_COMMANDCOUNT`
- **THEN** it prints ` 0 `

#### Scenario: The date and time of the build
- **WHEN** `PRINT LEN(_COMPILEDATE$); LEN(_COMPILETIME$); MID$(_COMPILETIME$, 3, 1)` runs
- **THEN** it prints ` 11  8 :`, the C++ compiler's `__DATE__` and `__TIME__`, as with the old compiler

#### Scenario: A bit read
- **WHEN** `PRINT _READBIT(5, 0); _READBIT(5, 1); _READBIT(5, 2)` runs
- **THEN** it prints what the old compiler's program prints

### Requirement: Names that call no built-in
A name with one leading underscore and parentheses that calls no built-in function with that spelling, a
built-in function's name used as a variable, and a built-in function used as a statement SHALL be compile errors
that are not "not supported yet", as the old compiler's are. A form `verification\v55` does not record SHALL
stay "not supported yet".

#### Scenario: A name with one leading underscore
- **WHEN** `_foo(1) = 2`, `n = _TRUE(1)`, `PRINT _CRC32$("x")`, `x = _CAST&(LONG, 1)`, `PRINT _OPENCLIENT&("x")`
  or `s = _IIF$(-1, "yes", "no")` is compiled
- **THEN** each is a compile error that is not "not supported yet" (`verification\v24_x112`, `v26_x120`,
  `v33_x29`, `v49_x16`, `v44_x46`, `tests\snippets\qb64fresh\inline_conditional_functions__iif_string`)

#### Scenario: A built-in function's name as a variable
- **WHEN** `rnd(1) = 2`, `TAB(5) = 1`, `READ ABS(a)`, `INPUT ABS(a)`, `LINE INPUT CHR$(1)` or `SWAP ABS(a), b` is
  compiled
- **THEN** each is a compile error that is not "not supported yet" (the old compiler: "Expected variable =" for a
  store, "Expected variable" for the others; `verification\v24_x119`, `v28_x53`, `v22_x79`, `v55_x13`–`x15`)

#### Scenario: A built-in function as a statement
- **WHEN** `LEN("a")`, `LEN "a"` or `ABS(1)` stands alone on a line
- **THEN** each is a compile error that is not "not supported yet" (the old compiler: "Syntax error";
  `verification\v20_x06_function_as_statement`, `v55_x29`, `x31`)

#### Scenario: Every spelling of the table calls its function
- **WHEN** each function of the built-in table is called by each spelling the table gives it (its name with each
  suffix it may or must have), the functions of the files QB64pe includes by itself among them
- **THEN** none is "Invalid array name"

### Requirement: An omitted argument of a built-in function
An argument left out of a call of `POINT`, `_RGB32`, `_RGB`, `_RGBA32`, `INSTR`, `MID$` or `LEFT$` SHALL be the
old compiler's error where a recording shows that it rejects the form, with its message as the note (measured,
`verification\v27_x50`, `v28_x49`, `v57_o01`–`o17`, `o21`). `INSTR` with its first argument left out SHALL
compile as the old compiler compiles it, the search starting where the old compiler's program starts it
(`v57_o07`, `v58`). A form the old compiler accepts that no recording covers, and a form of any other function,
SHALL stay "not supported yet".

#### Scenario: The first argument of POINT
- **WHEN** a program has `d = POINT(, 2)`
- **THEN** the compile fails with an error, not marked "not supported yet" (`verification\v27_x50`)

#### Scenario: The middle argument of _RGB32
- **WHEN** a program has `u = _RGB32(1, , 3)`
- **THEN** the compile fails with an error, not marked "not supported yet" (`verification\v28_x49`)

#### Scenario: The last argument
- **WHEN** a program has `u$ = MID$("abc", 1, )`, or leaves out the last argument of `POINT`, `_RGB32`, `_RGBA32`,
  `MID$` or `LEFT$`
- **THEN** the compile fails with an error whose note is "Invalid expression" (`verification\v57_o12`)

#### Scenario: Another argument
- **WHEN** another argument is left out, or any argument of `_RGB` or `INSTR` but `INSTR`'s first
- **THEN** the old compiler's message is "Incorrect number of arguments" for `POINT`, `_RGB`, `INSTR` and `MID$`, and
  "Syntax error" for `_RGB32`, `_RGBA32` and `LEFT$`

#### Scenario: The first argument of INSTR
- **WHEN** a program prints `INSTR(, "ab", "b")` and `INSTR(, "abab", "b")`, which the old compiler compiles
  (`verification\v57_o07`)
- **THEN** it compiles and prints what the old compiler's program prints (`v58`)

### Requirement: The old compiler's stubs
A name the old compiler's table lists as a stub and compiles by no rule of its own SHALL be a compile error,
with its "Command not implemented" as the note, also where the line is never reached (measured,
`verification\v57_s01`–`s20`). A stub with a rule of its own SHALL follow that rule.

#### Scenario: FRE of a number
- **WHEN** a program has `f0 = FRE(0)`
- **THEN** the compile fails with an error at `FRE`, not marked "not supported yet"
  (`tests\corpus\runtime_comparison\218_fre`)

#### Scenario: The stubs
- **WHEN** a program has `TRON`, `TROFF`, `LIST`, `IOCTL 1, "x"`, `IOCTL$(1)`, `SETMEM(0)`, `FILEATTR(1, 1)` or
  `FRE` of any argument
- **THEN** the compile fails with an error, with "Command not implemented" as its note (`verification\v57_s01`–
  `s10`)

#### Scenario: Stubs after CALL
- **WHEN** a program has `CALL TRON`, `CALL TROFF`, `CALL MID$(a$, 1, 1, "x")` or `CALL _MEMGET(…)`
- **THEN** the compile fails with an error, with "Command not implemented" as its note (`verification\v57_s11`,
  `s14`, `s15`, `s17`); `CALL FRE(0)` is "Expected CALL sub-name [(...)]", as `CALL` of any function (`s19`)

#### Scenario: RESET
- **WHEN** a program has `RESET` or `CALL RESET`
- **THEN** it compiles and closes every open file, as `CLOSE` does (`verification\v57_s03`, `s18`)

### Requirement: Hyperbolic and gradian functions
`_ASINH`, `_ACOSH`, `_ATANH`, `_D2G`, `_G2D`, `_R2G` and `_G2R` SHALL compile in any expression with the old
compiler's call, `_FLOAT` slot and result, giving its values (no error outside a domain). In a `CONST` the three
hyperbolic functions SHALL be its error, as `_SETBIT`, `MKSMBF$`, `_COMPILERVERSION$` and `_ENVIRONCOUNT` are, and
the evaluator SHALL keep its own values of the angle conversions (measured, `verification\v58`).

#### Scenario: Values
- **WHEN** a program prints `_ASINH(1); _ACOSH(2); _ATANH(.5); _D2G(90); _G2D(100); _R2G(_PI); _G2R(200)`
- **THEN** it prints what the old compiler's program prints

#### Scenario: Out of the domain
- **WHEN** a program prints `_ACOSH(0)`, `_ACOSH(.5)`, `_ATANH(1)`, `_ATANH(-1)` and `_ATANH(2)`
- **THEN** it prints `nan`, `nan`, `inf`, `-inf` and `nan`, as the old compiler's program does, and no error is
  raised (`verification\v58_f01`)

#### Scenario: In a CONST
- **WHEN** a program has `CONST c = _ASINH(1.5)`, or names `_SETBIT`, `MKSMBF$`, `_COMPILERVERSION$` or
  `_ENVIRONCOUNT` in a `CONST`
- **THEN** the compile fails with an error whose note is "Unexpected element '…'" (`verification\v58_x107`–`x111`)

#### Scenario: The CONST evaluator's own value
- **WHEN** a program prints `CONST c = _D2G(90)` beside `_D2G(f##)` with `f## = 90`
- **THEN** it prints ` 99.99999999900001  100 `, as the old compiler's program does (`verification\v58_f05`)

### Requirement: Bit functions
`_SETBIT(n, b)`, `_RESETBIT(n, b)` and `_TOGGLEBIT(n, b)` SHALL compile in any expression with the result type the
old compiler gives for `n`'s type, and SHALL give its values, also for a bit beyond `n`'s width and a negative bit
(measured, `verification\v58`), as `_READBIT` does.

#### Scenario: Values
- **WHEN** a program prints `_SETBIT(5, 1); _RESETBIT(5, 0); _TOGGLEBIT(5, 2)`
- **THEN** it prints ` 7  4  1 `, as with the old compiler

#### Scenario: Types and widths
- **WHEN** a program prints `_SETBIT(b%%, 7)`, `_SETBIT(u~%%, 7)`, `_TOGGLEBIT(x&&, 63)` and `_SETBIT(5, 40)`
- **THEN** it prints what the old compiler's program prints

### Requirement: Microsoft Binary Format conversions
`MKSMBF$` and `MKDMBF$` SHALL give the 4 and 8 bytes of a number in Microsoft Binary Format as libqb gives them,
and `CVSMBF` and `CVDMBF` SHALL read them back, with the old compiler's argument rules and errors (measured,
`verification\v58`).

#### Scenario: Bytes and back
- **WHEN** a program prints the bytes of `MKSMBF$(1.5)` and `MKDMBF$(-2.25#)` in hex, and `CVSMBF(MKSMBF$(1.5))`
  and `CVDMBF(MKDMBF$(-2.25#))`
- **THEN** it prints what the old compiler's program prints

#### Scenario: A string too short
- **WHEN** a program evaluates `CVSMBF("ab")` under a handler that prints `ERR`
- **THEN** it prints what the old compiler's program prints

### Requirement: System information functions
`_COMPILERVERSION$`, `_ENVIRONCOUNT`, `_UPTIME` and `LPOS(n)` SHALL compile in any expression with the old
compiler's libqb call and result type. `_COMPILERVERSION$` SHALL give the version string the old compiler's
program gives, the version of the QB64pe release this compiler targets (measured, `verification\v58`).

#### Scenario: Values
- **WHEN** a program prints `_COMPILERVERSION$`, `_ENVIRONCOUNT > 0`, `_UPTIME > 0` and `LPOS(0)`
- **THEN** it prints what the old compiler's program prints

#### Scenario: The number of variables
- **WHEN** a program counts `ENVIRON$(i)` from 1 until it is ""
- **THEN** the count equals `_ENVIRONCOUNT`

### Requirement: INPUT$ on a file
`INPUT$(n, file)` SHALL give the next `n` characters of a file open for input or binary, the file number written
with or without `#` (the one argument the old compiler takes a `#` before), and SHALL raise the old compiler's
errors past the end of the file, for a negative count, a file not open or not readable; `INPUT$(n)` SHALL compile
(measured, `verification\v58_f16`, `x112`–`x116`; found by the test over the whole table).

#### Scenario: Characters of a file
- **WHEN** a program writes `abcdef` and `ghi` to a file and prints `INPUT$(3, #1); INPUT$(2, 1); LEN(INPUT$(0, #1))`
- **THEN** it prints `abcde 0 `, as the old compiler's program does, and reading past the end raises error 62

#### Scenario: A # with nothing after it
- **WHEN** a program has `s$ = INPUT$(1, #)`
- **THEN** the compile fails with an error whose note is "Invalid expression" (`verification\v58_x115`)
