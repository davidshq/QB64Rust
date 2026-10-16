# language/print-using Specification

## Purpose
How `PRINT USING` and `LPRINT USING` are compiled: the forms accepted and rejected, the runtime entry each value
goes to, and what is printed when a value or the format raises an error, as measured with `qb64pe.exe`
(`verification\v30_a`–`v30_c`, `v30_x01`–`x19`).

## Requirements

### Requirement: The form of PRINT USING
`PRINT [#n,] [items] USING format; value {;|,} value … [;|,]` SHALL compile. The format SHALL be any string
expression and SHALL end at the first `;` outside parentheses; at least one value SHALL follow, and two values
SHALL have a `;` or `,` between them, also next to a string literal. An item before `USING` MAY stand with no
separator after it. `?` SHALL work as `PRINT`. Every other form SHALL be a compile error that is not "not
supported yet", as in the old compiler.

#### Scenario: A format that is an expression
- **WHEN** `f$ = "##": PRINT USING f$ + f$; 12` and `PRINT USING LEFT$("###xx", 3); 7` are compiled and run
- **THEN** both compile, and the second prints `  7`; the first prints what the old compiler's program prints

#### Scenario: Rejected forms
- **WHEN** a program has `PRINT USING "##", 5`, `PRINT USING "##"`, `PRINT USING "##";`, `PRINT USING 5; 5`,
  `PRINT USING #1, "##"; 5`, `PRINT USING "##"; 1 2`, `PRINT USING "&&"; "a" "b"`, `PRINT USING "##"; ; 5` or
  `PRINT USING "##"; 1; USING "##"; 2`
- **THEN** each is a compile error that is not "not supported yet"

#### Scenario: An item before USING with no separator
- **WHEN** `PRINT "a" USING "##"; 5` runs
- **THEN** it prints `a 5`

#### Scenario: A value of a user type
- **WHEN** `PRINT USING "##"; t` is compiled, `t` a `TYPE` variable
- **THEN** it is a compile error

#### Scenario: A question mark
- **WHEN** `? USING "##"; 5` runs
- **THEN** it prints ` 5`

### Requirement: Values by type
Each value SHALL go to the runtime entry of its type, as the old compiler writes it: a string, a SINGLE, a
DOUBLE, a `_FLOAT` and an `_UNSIGNED _INTEGER64` each to its own, every other integer type to the 64-bit signed
one. The format SHALL be evaluated once, before any value. A comma between two values SHALL do what a semicolon
does.

#### Scenario: Exponent letters
- **WHEN** `PRINT USING "#.##^^^^"; 1234!`, then with `1234#`, then with a `_FLOAT` of 1234 run
- **THEN** the exponents are written with `E`, `D` and `F`, as with the old compiler

#### Scenario: An unsigned 64-bit value
- **WHEN** `u~&& = 0: u~&& = u~&& - 1: PRINT USING "####################"; u~&&` runs
- **THEN** it prints `18446744073709551615`

#### Scenario: A comma between values
- **WHEN** `PRINT USING "##x"; 1, 2, 3` runs
- **THEN** it prints ` 1x 2x 3x`, with no print zone between the values

#### Scenario: The format read once
- **WHEN** the format is a FUNCTION call that prints a line, and three values follow
- **THEN** the line is printed once, before the values

### Requirement: Formats, as the runtime prints them
What a format prints SHALL be what the old compiler's program prints for the same format and values: number
fields, string fields, text between fields, and a format that starts again when values are left.

#### Scenario: Number fields
- **WHEN** `PRINT USING "#.#"; 0.25`, `PRINT USING "##"; 2.5`, `PRINT USING "###"; 1234`, `PRINT USING "+###"; 5`
  and `PRINT USING "**$###.##"; 12.5` run
- **THEN** they print `0.3`, ` 3`, `%1234`, `+  5` and `***$12.50`

#### Scenario: String fields
- **WHEN** `PRINT USING "!"; "hello"`, `PRINT USING "&"; "hello"` and `PRINT USING "\  \"; "hello"` run
- **THEN** they print `h`, `hello` and `hell`

#### Scenario: The format starts again
- **WHEN** `PRINT USING "## ## ##"; 1; 2` runs
- **THEN** it prints ` 1  2 `

### Requirement: TAB, SPC and items before USING
`TAB(n)` and `SPC(n)` among the values SHALL print what is built so far and then their blanks, and the place in
the format SHALL be kept. Items before `USING` SHALL be printed first, as a plain `PRINT` prints them.

#### Scenario: TAB among the values
- **WHEN** `PRINT USING "## ##"; 1; TAB(10); 2` runs
- **THEN** it prints what the old compiler's program prints: the first field, blanks up to column 10, then the
  second field of the same format

#### Scenario: Items before USING
- **WHEN** `PRINT "a"; USING "##"; 5` and `PRINT 1; 2; USING "##"; 3` run
- **THEN** the first prints `a 5`, and the second the two numbers as `PRINT 1; 2;` prints them and then ` 3`

### Requirement: The end of the line
`PRINT USING` SHALL end the line unless the statement's last element is `;` or `,`.

#### Scenario: A trailing semicolon
- **WHEN** `PRINT USING "##"; 1;` and then `PRINT USING "##"; 2` run
- **THEN** the output is ` 1 2` on one line

### Requirement: Errors when it runs
A run-time error in `PRINT USING` SHALL leave printed what was built before it, with no line end, and SHALL
skip the values after it: error 5 for a format with no field and for a value that is not finite, error 13 for a
string given to a number field or a number to a string field.

#### Scenario: A string for a number field
- **WHEN** `PRINT USING "ab ### cd &"; 5; 6` runs under a handler that prints `ERR` and resumes next
- **THEN** `ab   5 cd ` is printed with no line end and the handler prints 13

#### Scenario: A format with no field
- **WHEN** `PRINT USING "abc"; 1` runs under a handler that prints `ERR` and resumes next
- **THEN** `abc` is printed and the handler prints 5

### Requirement: PRINT USING to a file
`PRINT #n, USING format; values` SHALL write to file `n` the text the console form prints. A file that is not
open SHALL raise error 52 and a file open for input error 54; an error in a value SHALL come before the file's.

#### Scenario: Written and read back
- **WHEN** `PRINT #1, USING "##.##"; 3.14159` runs with file 1 open for output, and the file is read back
- **THEN** its line is ` 3.14`

#### Scenario: File not open
- **WHEN** `PRINT #3, USING "##"; 1` runs with no file open under a handler that prints `ERR`
- **THEN** the handler prints 52

### Requirement: LPRINT USING
`LPRINT USING format; values` SHALL compile in every form `PRINT USING` has without a file number, to the calls
the old compiler writes for the printer.

#### Scenario: The printer's calls
- **WHEN** `LPRINT USING "##"; 5` is compiled by both compilers
- **THEN** the runtime calls are the same
