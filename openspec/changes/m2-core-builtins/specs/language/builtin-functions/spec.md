# Spec Delta

## Purpose

How built-in functions are compiled in expressions: which are supported, how arguments are converted to their
slots, the type of each result, and the runtime errors they raise, as measured with `qb64pe.exe`. Built-ins not
listed here are reported "not supported yet".

## ADDED Requirements

### Requirement: Supported built-in functions
`sema` SHALL compile these built-in functions: `INSTR`, `CHR$`, `LEN`, `LEFT$`, `RIGHT$`, `MID$`, `ASC`, `STR$`,
`VAL`, `STRING$`, `SPACE$`, `LTRIM$`, `RTRIM$`, `_TRIM$`, `UCASE$`, `LCASE$`, `HEX$`, `OCT$`, `_BIN$`, `_TOSTR$`,
`ABS`, `SGN`, `INT`, `FIX`, `SQR`, `SIN`, `COS`, `TAN`, `ATN`, `LOG`, `EXP`, `CINT`, `CLNG`, `CSNG`, `CDBL`,
`_ROUND`, `_PI`, `_ATAN2`, `_HYPOT`, `LBOUND`, `UBOUND`, `ERR`, `ERL`. Any other built-in function SHALL be
reported "not supported yet", as SHALL an argument of a type the slice lacks (unsigned, `_BIT`, `_OFFSET`, `_MEM`).

#### Scenario: Unsupported built-in
- **WHEN** a program uses `_IIF(x > 0, 1, 2)`
- **THEN** the compiler reports `_IIF` "not supported yet" and no other error for that expression

#### Scenario: Supported built-in compiles
- **WHEN** `PRINT LEFT$("hello", 2); LEN("abc")` is compiled and run
- **THEN** it prints `he 3`, as with the old compiler

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
a numeric argument to an integer slot as a store into a variable of that type (a float rounded half to even), to a
float slot without rounding, and to an "any numeric" slot not at all. A string argument SHALL be passed as it is.

#### Scenario: Float argument to an integer slot
- **WHEN** `PRINT LEFT$("abcdef", 2.5); LEFT$("abcdef", 3.5)` runs
- **THEN** it prints `abcd`, as with the old compiler (2.5 rounds to 2, 3.5 to 4)

### Requirement: Result types
The result of a built-in SHALL have the type the old compiler gives it, measured: the table's return type for a
plain function; for `ABS`, `INT` and `FIX` the argument's type; for `SIN`, `COS`, `TAN`, `ATN`, `SQR`, `LOG` and
`EXP` a type chosen by the argument's type; for `VAL` `_FLOAT`, or the named type when a type argument is given;
for `LBOUND`/`UBOUND` `_INTEGER64`. Printing and arithmetic SHALL follow from that type.

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
empty strings, positions past the end and zero lengths; `STR$` SHALL put a blank before a value that is not
negative; `HEX$`, `OCT$` and `_BIN$` of a negative integer SHALL use the width of the argument's type.

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
error 5, "Illegal function call", for `ASC("")`, `SQR(-1)`, `LOG(0)` or `LEFT$(s, -1)`, and error 6, "Overflow",
for `CINT(40000)`), and SHALL then yield the placeholder value under the IR's pending-error rule.

#### Scenario: ASC of an empty string
- **WHEN** `x = 5: x = ASC("")` runs under a handler that prints `ERR` and resumes next, then `x` is printed
- **THEN** the handler prints 5, and `x` holds 0

#### Scenario: CINT overflow
- **WHEN** `PRINT CINT(40000)` runs under a handler that prints `ERR` and resumes next
- **THEN** the handler prints 6

### Requirement: Built-ins without parentheses
A built-in function that takes no required argument (`_PI`) SHALL be callable without parentheses, and with its
optional argument in parentheses, as the old compiler allows.

#### Scenario: _PI bare and with an argument
- **WHEN** `PRINT _PI; _PI(2)` runs
- **THEN** it prints the same two values as the old compiler
