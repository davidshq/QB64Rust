# Spec Delta

## MODIFIED Requirements

### Requirement: Supported built-in functions
`sema` SHALL compile these built-in functions: `INSTR`, `CHR$`, `LEN`, `LEFT$`, `RIGHT$`, `MID$`, `ASC`, `STR$`,
`VAL`, `STRING$`, `SPACE$`, `LTRIM$`, `RTRIM$`, `_TRIM$`, `UCASE$`, `LCASE$`, `HEX$`, `OCT$`, `_BIN$`, `_TOSTR$`,
`ABS`, `SGN`, `INT`, `FIX`, `SQR`, `SIN`, `COS`, `TAN`, `ATN`, `LOG`, `EXP`, `CINT`, `CLNG`, `CSNG`, `CDBL`,
`_ROUND`, `_PI`, `_ATAN2`, `_HYPOT`, `LBOUND`, `UBOUND`, `ERR`, `ERL`, `RND`, `TIMER`, `COMMAND$`, `ENVIRON$`,
`SHELL`, `EOF`, `LOF`, `LOC`, `SEEK`, `FREEFILE`, `_FILEEXISTS`, `_DIREXISTS`, `_CWD$`, and the functions added by
demand that the list in `sema` names. Any other built-in function SHALL be
reported "not supported yet", as SHALL an argument of type `_MEM`. An argument of any other numeric type SHALL be
converted to its slot as an assignment would, except where the old compiler special-cases the argument's type
(`LEN`, `STR$`, `HEX$`, `OCT$`, `_BIN$`, `ABS`, `SGN`, `VAL` with a type, as measured). `VAL(s$, t)` with an
unsigned integer type `t` SHALL parse as libqb's unsigned `VAL` does (not narrowed to `t`, a minus sign dropped:
`VAL("-1", _UNSIGNED LONG)` is 1), and with a `_BIT` type SHALL be a compile error, as in the old compiler. A
fixed-length string argument SHALL be a string.

#### Scenario: Unsupported built-in
- **WHEN** a program uses `_IIF(x > 0, 1, 2)`
- **THEN** the compiler reports `_IIF` "not supported yet" and no other error for that expression

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

### Requirement: Built-ins without parentheses
A built-in function that takes no required argument (`_PI`, `RND`, `TIMER`, `COMMAND$`, `FREEFILE`, `_CWD$`) SHALL
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
