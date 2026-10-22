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
`_COMMANDCOUNT`, `_COMPILEDATE$`, `_COMPILETIME$` and `_READBIT`. Any
other built-in function SHALL be reported "not supported yet". A supported function that needs an argument, named without one, SHALL be a compile error, as in the
old compiler. An argument of any other numeric type SHALL be
converted to its slot as an assignment would, except where the old compiler special-cases the argument's type
(`LEN`, `STR$`, `HEX$`, `OCT$`, `_BIN$`, `ABS`, `SGN`, `VAL` with a type, `_IIF`, `_MIN`, `_MAX`, `_CLAMP`, `_ROL`,
`_ROR`, as measured). `VAL(s$, t)` with an
unsigned integer type `t` SHALL parse as libqb's unsigned `VAL` does (not narrowed to `t`, a minus sign dropped:
`VAL("-1", _UNSIGNED LONG)` is 1), and with a `_BIT` type SHALL be a compile error, as in the old compiler. A
fixed-length string argument SHALL be a string.

#### Scenario: Unsupported built-in
- **WHEN** a program uses `k = _SETBIT(a, 1)`
- **THEN** the compiler reports `_SETBIT` "not supported yet" and no other error for that expression

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


## ADDED Requirements

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
