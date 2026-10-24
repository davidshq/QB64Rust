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

## ADDED Requirements

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
