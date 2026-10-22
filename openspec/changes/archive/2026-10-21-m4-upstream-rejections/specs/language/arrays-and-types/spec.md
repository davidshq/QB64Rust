## ADDED Requirements

### Requirement: Member array bounds that name no constant
A bound of a member array in a `TYPE` block that names no `CONST` standing above the block SHALL be the compile
error "TYPE member array bounds must be numeric constant", as the old compiler gives it (measured,
`verification\v51_x01`–`x15`). A bound that names a `CONST` above the block, a constant of the auto-included file
or a function of the constant evaluator (`_PI`, `ABS(…)`) SHALL stay "not supported yet" (requirement "Member
arrays").

#### Scenario: A variable as the bound
- **WHEN** a program does `DIM AS _BYTE a`, `a = 1` and declares `array (a) AS LONG` in a `TYPE` block (upstream
  `arrays/39_err_member_must_be_const`)
- **THEN** it is the compile error "TYPE member array bounds must be numeric constant" at the bound, not "not
  supported yet"

#### Scenario: A variable assigned above the block
- **WHEN** `n = 3` stands above a `TYPE` block with the member `v(n) AS LONG` (`verification\v24_x195`)
- **THEN** it is the same compile error

#### Scenario: A CONST below the block or in a SUB
- **WHEN** a `TYPE` block of the main module has the member `v(top) AS LONG` and `CONST top = 3` stands below it,
  or in a SUB above or below it (`verification\v51_x03`, `x04`, `x13`)
- **THEN** it is the compile error "TYPE member array bounds must be numeric constant"

#### Scenario: A call in the bound
- **WHEN** the bound is `LEN(x)`, `LEN("abc")`, an array's element `a(1)` or a call of the program's FUNCTION
  `f(1)` (`verification\v51_x05`, `x06`, `x12`, `x15`)
- **THEN** it is the same compile error

#### Scenario: A CONST as the bound
- **WHEN** `CONST top = 3` stands above a `TYPE` block with the member `v(top) AS LONG` (`verification\v24_x196`)
- **THEN** the only diagnostic is "not supported yet" at the bound

### Requirement: Undefined member types
A member of a `TYPE` block declared `AS name`, where `name` is no type word of the old compiler and no `TYPE` of
an earlier block, SHALL be the compile error "Undefined type", and before the names of an `AS type` list the old
compiler's "Expected element-name AS type, AS type element-list, or END TYPE" (measured,
`verification\v51_x20`–`x27`). A name of an earlier block that was not entered SHALL stay "not supported yet".

#### Scenario: A member array of an undefined type
- **WHEN** a `TYPE` block declares `array (10) AS notdefined` and the program has no `TYPE notdefined` (upstream
  `arrays/40_err_undefined_type`)
- **THEN** it is the compile error "Undefined type" at the member, not "not supported yet"

#### Scenario: Near types and sizes
- **WHEN** a member is declared `AS _UNSIGNED nosuch`, `AS BYTE`, `AS _Bytef`, `AS Stringfoobar * 2` or `AS
  INTEGER * 2` (`verification\v51_x22`–`x25`, `x27`)
- **THEN** it is the compile error "Undefined type"

#### Scenario: A type defined in an earlier block
- **WHEN** the member names a `TYPE` that the program defines in an earlier block
- **THEN** the member's type is checked as requirement "User types" says, and no "Undefined type" error is given

#### Scenario: A type defined in a later block
- **WHEN** the member names a `TYPE` that the program defines in a later block (`verification\v51_x21`)
- **THEN** it is the compile error "Undefined type"

### Requirement: LBOUND and UBOUND of an element
`LBOUND` and `UBOUND` whose array argument is an element of a member array with its indexes, as
`UBOUND(f(10).a(5), 1)`, SHALL be the compile error "Incorrect number of arguments", as the old compiler gives it.
Each other element form that the old compiler is measured to reject (in `verification\v51`) SHALL be its
measured error. A form that is not measured SHALL stay "not supported yet".

#### Scenario: UBOUND of a member array element
- **WHEN** `PRINT UBOUND(f(10).a(5), 1)` is compiled for `DIM f(15) AS e`, where `e` has the member `a(1 TO 50,
  2 TO 20) AS _BYTE` (upstream `arrays/42_err_solved_bug_Incorrect_nr_of_args`)
- **THEN** it is the compile error "Incorrect number of arguments", not "not supported yet"

#### Scenario: LBOUND of a member array element
- **WHEN** `PRINT LBOUND(f(10).a(5), 1)` is compiled for the same array (upstream
  `arrays/43_err_solved_bug_Incorrect_nr_of_args`)
- **THEN** it is the same compile error

#### Scenario: Other elements and values
- **WHEN** the argument is an element of an array of a `TYPE` (`LBOUND(f(1))`), an element of another array, a
  call of the program's FUNCTION, a literal or `a + 1` (`verification\v51_x28`–`x31`, `x33`, `x39`)
- **THEN** it is the compile error "Expected ." for the first and "Syntax error" for the others

#### Scenario: The member array itself
- **WHEN** `PRINT LBOUND(f(10).a, 2); UBOUND(f(10).a, 2)` runs for the same array
- **THEN** it prints ` 2  20 `, as requirement "LBOUND and UBOUND" says

### Requirement: Unknown type names
In an `AS` clause of `DIM`, `REDIM`, `STATIC` or `COMMON`, a name that is no type word of the old compiler (a
word without its underscore, as `BYTE`, is none) and no `TYPE` of the program SHALL be the compile error "Unknown
type", and so SHALL `* n` after a word other than `STRING` and `_BIT`; after `_UNSIGNED` each SHALL be "Type
cannot be _UNSIGNED" (measured, `verification\v51_x60`–`x88`). A name of a `TYPE` block that was not entered
SHALL stay "not supported yet".

#### Scenario: A misspelt numeric type
- **WHEN** a program does `DIM b AS _Bytef`, or `STATIC b AS _Bytef` in a SUB (upstream
  `types/dim_bad_typename_byte`, `types/static_bad_typename_byte`)
- **THEN** it is the compile error "Unknown type", not "not supported yet"

#### Scenario: A size after another word
- **WHEN** a program does `DIM b AS INTEGER * 2`, or `DIM b AS _UNSIGNED INTEGER * 2` (`verification\v51_x65`,
  `x84`)
- **THEN** it is the compile error "Unknown type", and "Type cannot be _UNSIGNED"

#### Scenario: A type that does not exist in a DIM AS list
- **WHEN** a program does `DIM AS nosuch a` and has no `TYPE nosuch` (`verification\v24_x232`)
- **THEN** it is the compile error "Unknown type"

#### Scenario: A type defined below its use
- **WHEN** `DIM v AS pt` stands above the program's `TYPE pt` block
- **THEN** the program compiles as requirement "User types" says, and no "Unknown type" error is given

### Requirement: Words glued to STRING
In an `AS` clause of `DIM`, `REDIM`, `STATIC` or `COMMON`, a word that starts with `STRING` and is not `STRING`
SHALL be the compile error "Expected STRING * number/constant", with `* n` or without and after `_UNSIGNED`
(measured, `verification\v51_x67`, `x70`, `x83`), unless a `TYPE` of the program has that name: the old compiler
looks the user types up before the `STRING` prefix, so `TYPE StringList` is a type in every `AS` clause.

#### Scenario: A word glued to STRING
- **WHEN** a program does `DIM s AS Stringfoobar * 2`, or `STATIC s AS Stringfoobar * 2` in a SUB (upstream
  `types/dim_bad_typename_string`, `types/static_bad_typename_string`)
- **THEN** it is the compile error "Expected STRING * number/constant"

#### Scenario: Without a size
- **WHEN** a program does `DIM s AS Stringfoobar` (`verification\v51_x70`)
- **THEN** it is the same compile error

#### Scenario: A TYPE whose name starts with STRING
- **WHEN** a program defines `TYPE StringList` and declares `DIM a AS StringList`, `REDIM c(1 TO 2) AS
  StringList` or a parameter `x AS StringList` (`tests\frontend\type_string_prefix_ok.bas`)
- **THEN** each is the user type, and no error is given

### Requirement: Unknown type names in SHARED, _DEFINE and parameters
A name that is no type in `SHARED` or `_DEFINE` SHALL be the compile error "Cannot convert type (…) to symbol",
and in a parameter of a SUB, a FUNCTION or a `DECLARE LIBRARY` procedure, an array parameter too, "Illegal
SUB/FUNCTION parameter" (measured, `verification\v51_x61`, `x63`, `x69`, `x74`, `x82`, `x87`, `x89`). A form
not measured in these places SHALL stay "not supported yet".

#### Scenario: A misspelt type in SHARED and in a parameter
- **WHEN** a SUB has `SHARED b AS _Bytef`, or the parameter `(b AS _Bytef)` (`verification\v51_x61`, `x63`)
- **THEN** it is the compile error "Cannot convert type (_BYTEF) to symbol", and "Illegal SUB/FUNCTION
  parameter"
