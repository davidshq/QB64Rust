# language/record-files Specification

## Purpose
How programs read and write files by position: `GET` and `PUT` on a file open `FOR RANDOM` or `FOR BINARY`, the
record buffer of `FIELD` with `LSET` and `RSET`, and `LOCK` and `UNLOCK`, with the forms that compile and the
run-time errors each raises, as measured with `qb64pe.exe` (`verification\v30_d`, `v30_e`, `v30_i`,
`v30_x20`–`x56`, and for `FIELD`, `LSET` and `RSET` `v30_j`–`v30_l`, `v30_x100`–`x170`).

## Requirements

### Requirement: The forms of GET and PUT on a file
`GET [#]n[, [position][, variable]]` and `PUT` with the same list SHALL compile to the runtime call the old
compiler writes, also after `CALL`. The file number SHALL be any numeric expression. A first argument that starts
with `(` SHALL be read as the graphics form. The third argument SHALL be a variable; every form the old compiler
rejects SHALL be a compile error that is not "not supported yet". A whole array `und()` of a name that is no array
SHALL be an array used without `DIM`, and SHALL read and write what the old compiler's program does; `GET` of
`und$()`, an array of strings so made, SHALL read into its first element as into a string variable, and `PUT` of
it SHALL be the old compiler's error, as for a declared array of strings.

#### Scenario: With and without a position
- **WHEN** `GET #1, 3, c`, `GET #1, , c`, `PUT #1, , c` and `PUT #k + 1, 2, c` are compiled by both compilers
- **THEN** the runtime calls are the same

#### Scenario: Not a variable
- **WHEN** a program has `PUT #1, , 5`, `PUT #1, , a + 1`, `PUT #1, , LEN(s$)` or `PUT #1, , K` with `K` a `CONST`
- **THEN** each is a compile error

#### Scenario: A variable in parentheses
- **WHEN** `PUT #1, , (c)` is compiled
- **THEN** it compiles as `PUT #1, , c` does

#### Scenario: Rejected arguments
- **WHEN** a program has `GET #1, , b` with `b` a `_BIT` variable, `GET #1, , s$()` with `s$()` an array of
  variable-length strings, `GET #1, , t` with `t` of a `TYPE` that has a `STRING` member, `GET #1, , r.v()` with
  `v` a member array, `GET #1, ,`, `GET`, `GET #, , c` or `GET "a", , c`
- **THEN** each is a compile error

#### Scenario: An array that is not declared
- **WHEN** the programs `verification\v30_x176_get_undeclared_array`, `x177` and `x178` (`GET #1, , und()`,
  `PUT #1, , und()`, `GET #1, , und$()`) and `verification\v56_e_undeclared_arrays`, which has data in the file,
  run
- **THEN** each prints what the old compiler's program printed

#### Scenario: PUT of an array of strings not declared
- **WHEN** a program has `PUT #1, 1, uns$()` and no array `uns$` (`verification\v56_e2`)
- **THEN** it is a compile error that is not "not supported yet"

### Requirement: What a variable reads and writes
`PUT` SHALL write and `GET` SHALL read the bytes of the variable: a numeric variable its type's size, a
fixed-length string its length, a `TYPE` variable or member its size, an array element its own, a whole array
`a()` the bytes of all its elements from the first. A variable-length string SHALL be written with its
characters and read for as many characters as it holds.
A whole array of a type with dynamic member arrays (`language/arrays-and-types`, "Dynamic member arrays") SHALL
be written and read in the type's declared layout: each element as the type's members one after the other with
no padding, each dynamic member array as its elements at its declared bounds, through a buffer that belongs to
the statement. `GET` SHALL unpack that buffer into the array after its read whatever the read raised, as the old
compiler does, so a read that raised an error brings back what the same statement read last; bytes the statement
never read SHALL be 0 (the old program reads memory nobody wrote, `QB64PE-DIVERGENCES.md` D-021). A dynamic member array
whose bounds or number of dimensions are not its declared ones SHALL raise error 9: `PUT` then writes nothing;
`GET` reads the file, stores the other members and elements, and stores that member's declared number of elements
into its block, its bounds unchanged, when the block holds at least that many, and leaves it as it was when it
holds fewer (the old compiler writes past the block, `QB64PE-DIVERGENCES.md` D-019). A type with a `STRING` member
anywhere in it SHALL be a compile error there ("UDT must have fixed size"), as SHALL be one value of such a type
(requirement "Dynamic member arrays"). On a file open `FOR RANDOM` the whole array SHALL be one record, as any
whole array is.

#### Scenario: Numbers of each size in a binary file
- **WHEN** a program writes an INTEGER, a LONG, a DOUBLE and a `_BYTE` with `PUT #1, , v` to a file open `FOR
  BINARY`, and prints `LOF(1)`
- **THEN** it prints 15, and reading them back with `GET` gives the values written

#### Scenario: A whole array
- **WHEN** `DIM a(1 TO 4) AS LONG`, filled, is written with `PUT #1, , a()` and read into another array with `GET
  #1, 1, b()`
- **THEN** the file is 16 bytes long and `b` holds the four values

#### Scenario: A TYPE variable
- **WHEN** a variable of a `TYPE` with a LONG and a `STRING * 6` member is written with `PUT` and read back into
  another with `GET`
- **THEN** the file is 10 bytes long and both members come back

#### Scenario: An empty string reads nothing
- **WHEN** `s$ = "": GET #1, 1, s$` runs on a binary file that holds `abcdef`, then `s$ = SPACE$(3): GET #1, 1,
  s$`
- **THEN** `s$` is empty after the first and `abc` after the second

#### Scenario: A whole array with dynamic members
- **WHEN** `REDIM p(0 TO 1)` of a type with `a AS INTEGER`, `v(1) _DYNAMIC AS LONG`, `b AS INTEGER`, each member
  at its declared bounds and set, is written with `PUT #1, 1, p()`
- **THEN** the file is 24 bytes, each element's `a`, two LONGs and `b`; `GET #1, 1, q()` into another such array
  gives back every value

#### Scenario: A member with other bounds
- **WHEN** `p(0).v` has bounds 0 to 2 and `PUT #1, 1, p()` runs under a handler, and later `q(0).v` has bounds 5
  to 5 and `GET #1, 1, q()` runs
- **THEN** the `PUT` raises error 9 and the file stays empty; the `GET` raises error 9, `q(0).a`, `q(0).b` and
  `q(1)` are read, and `q(0).v` keeps its bounds and element

#### Scenario: A member block that holds the declared elements
- **WHEN** `p(1).v` of `v(1) _DYNAMIC AS LONG` is given the bounds `0 TO 1, 0 TO 0`, or `5 TO 6`, or `0 TO 2`, and
  `GET #1, 1, p()` reads a file written with the declared bounds, under a handler
- **THEN** error 9 is raised, the member keeps its bounds, and its first two elements are the two LONGs of the file
  (`verification\v38_b`)

#### Scenario: A failed read
- **WHEN** `GET #3, 1, p()` reads a file in one pass of a loop and, after `CLOSE #3`, runs again in the next pass
  under a handler, after the program set every element of `p` to -1
- **THEN** the second raises error 52 and `p` holds the values the first read (`verification\v38_a`)

#### Scenario: A failed read of a statement that never read
- **WHEN** `GET #2, 1, p()` runs once with file 2 not open, under a handler
- **THEN** error 52 is raised and every member of every element of `p` is 0

#### Scenario: Strings
- **WHEN** `PUT #1, 1, p()` is compiled for a type with `v(1) _DYNAMIC AS STRING`
- **THEN** it is a compile error

### Requirement: The position in a binary file
In a file open `FOR BINARY` the position SHALL be a byte from 1, rounded as a number is; left out, it SHALL be
the byte after the last one read or written. A `GET` at or past the end SHALL set the bytes it cannot read to 0
and set `EOF` without growing the file; a `PUT` past the end SHALL fill the gap with 0.

#### Scenario: Past the end
- **WHEN** a program opens a binary file of 4 bytes and runs `GET #1, 3, l&` and then prints `EOF(1)` and
  `LOF(1)`
- **THEN** the upper two bytes of `l&` are 0, `EOF(1)` is -1 and `LOF(1)` is 4

#### Scenario: A gap
- **WHEN** `PUT #1, 10, b%%` runs on an empty binary file
- **THEN** the file is 10 bytes long and its first nine bytes are 0

### Requirement: Records of a random file
In a file open `FOR RANDOM` the position SHALL be a record number from 1, of the length `LEN =` gives (128 when
left out); left out, it SHALL be the record after the last one. `PUT` SHALL write only the variable's bytes at
the record's start. A variable-length string SHALL be written as two bytes of length and its characters, and read
back by that length.

#### Scenario: A later record leaves a gap
- **WHEN** a program opens a new file `FOR RANDOM` with `LEN = 10` and runs `PUT #1, 3, l&`, then prints `LOF(1)`
- **THEN** it prints 24, as with the old compiler

#### Scenario: A string in a record
- **WHEN** `s$ = "abc": PUT #1, 1, s$` runs on a random file, and `GET #1, 1, t$` after it
- **THEN** the file starts with the two bytes of the length 3 and `abc`, and `t$` is `abc`

#### Scenario: A record past the end
- **WHEN** `GET #1, 9, l&` runs on a random file of two records
- **THEN** `l&` is 0 and `EOF(1)` is -1

### Requirement: Run-time errors of GET and PUT
`GET` and `PUT` SHALL raise the runtime's errors: 52 for a file that is not open; 54 for a file open for input,
output or append, and for the form with no variable on a binary file; 63 for a position of 0 or below; 59 for a
value longer than the record of a random file; 75 for a `PUT` to a file open with `ACCESS READ` and a `GET` from
one with `ACCESS WRITE`.

#### Scenario: Position zero
- **WHEN** `PUT #1, 0, c` runs under a handler that prints `ERR` and resumes next
- **THEN** the handler prints 63 and the file does not change

#### Scenario: A value longer than the record
- **WHEN** a random file has `LEN = 4` and `PUT #1, 1, d#` runs under a handler that prints `ERR`
- **THEN** the handler prints 59 and nothing is written

#### Scenario: A sequential file
- **WHEN** `GET #1, , c` runs with file 1 open `FOR OUTPUT` under a handler that prints `ERR`
- **THEN** the handler prints 54

### Requirement: GET and PUT with no variable
`GET [#]n[, position]` SHALL read one record of a random file into the file's record buffer and `PUT [#]n[,
position]` SHALL write the whole buffer as one record; each SHALL move on by one record.

#### Scenario: A whole record written
- **WHEN** a random file has `LEN = 14` and `PUT #1, 9` runs on it while it is empty
- **THEN** `LOF(1)` is 126

### Requirement: FIELD
`FIELD [#]n, width AS string, …` SHALL name parts of file `n`'s record buffer by variable-length strings, in
order from the buffer's start, as the old compiler's program does: after a `GET` with no variable each holds its
part of the record, and what `LSET` or `RSET` puts into one is what a `PUT` with no variable writes.

#### Scenario: A record through fields
- **WHEN** a program opens a random file with `LEN = 8`, runs `FIELD #1, 3 AS a$, 5 AS b$`, `LSET a$ = "xy"`,
  `LSET b$ = "hello"`, `PUT #1, 1`, then sets both with `LSET` to blanks, runs `GET #1, 1` and prints `a$` and
  `b$` between brackets
- **THEN** it prints `[xy ]` and `[hello]`, and the file holds `xy hello`

### Requirement: The forms of FIELD
The file number and each width of `FIELD` SHALL be any numeric expression, taken as a LONG. A target SHALL be a
variable-length string variable, an array element or a `STRING` member. The file number SHALL be evaluated
first, then for each part the target's index before its width. The forms the old compiler rejects SHALL be
compile errors that are not "not supported yet"; a whole array `e()` as a target MAY be "not supported yet".

#### Scenario: The calls
- **WHEN** `FIELD #k + 1, 3 AS a$, w! AS e$(k), 2 AS r.v` is compiled by both compilers
- **THEN** the runtime calls are the same: one for the file, then one for each part in order

#### Scenario: A target that is no variable-length string
- **WHEN** a program has `FIELD #1, 4 AS n%`, `FIELD #1, 4 AS x` with `x` a `STRING * 4`, `FIELD #1, 4 AS a$ +
  b$`, `FIELD #1, 4 AS "lit"`, `FIELD #1, 4 a$`, `FIELD #1, 4 AS a$,`, `FIELD #1`, `FIELD "a", 4 AS a$` or `CALL
  FIELD(#1, 4 AS a$)`
- **THEN** each is a compile error that is not "not supported yet"

### Requirement: FIELD when it runs
A `FIELD` SHALL start at the buffer's first byte, and the strings of an earlier `FIELD` on the file SHALL stay
fields. Widths that go beyond the record SHALL raise error 50, a file that is not open error 52 and one that is
not open `FOR RANDOM` error 54. An assignment with `=` SHALL change the string and not the buffer.

#### Scenario: A second FIELD on the file
- **WHEN** after `FIELD #1, 3 AS a$, 5 AS b$` a program runs `FIELD #1, 8 AS w$` and `LSET w$ = "12345678"`
- **THEN** `a$` is `123` and `b$` is `45678`: both statements name the buffer from its first byte

#### Scenario: Widths beyond the record
- **WHEN** `FIELD #1, 5 AS a$, 4 AS b$, 1 AS c$` runs on a random file with `LEN = 8` under a handler that
  prints `ERR` and resumes next
- **THEN** the handler prints 50, `a$` is five characters long, and `b$` and `c$` are as they were

#### Scenario: A file that is not random
- **WHEN** `FIELD #1, 2 AS c$` runs with file 1 open `FOR BINARY`, and `FIELD #3, 2 AS c$` with no file 3, each
  under a handler that prints `ERR`
- **THEN** the handler prints 54 and 52

#### Scenario: Assignment to a field variable
- **WHEN** with `FIELD #1, 4 AS a$, 4 AS b$` and a buffer that holds `abcdefgh` a program runs `a$ = "QQ"`,
  prints `LEN(a$)`, runs `LSET b$ = "x"` and prints `a$`
- **THEN** it prints 2 and then `abcd`: the assignment does not change the buffer, and the string is still a
  field

### Requirement: LSET and RSET
`LSET s = value` SHALL put the value at the left of string `s` and `RSET s = value` at its right, without
changing the length of `s`: a shorter value padded with blanks, a longer one cut at its end by both. The target
SHALL be a string variable, array element or member, of variable or fixed length, evaluated before the value.
A target that is no string place, among them a `TYPE` value, SHALL be a compile error.

#### Scenario: Left and right
- **WHEN** `a$ = SPACE$(5): LSET a$ = "ab": PRINT "["; a$; "]": RSET a$ = "ab": PRINT "["; a$; "]"` runs
- **THEN** it prints `[ab   ]` and `[   ab]`

#### Scenario: A value that is too long
- **WHEN** `a$ = SPACE$(3): LSET a$ = "abcdef": PRINT a$: RSET a$ = "abcdef": PRINT a$` runs
- **THEN** it prints `abc` twice

#### Scenario: A fixed-length string and a member
- **WHEN** `DIM x AS STRING * 5: RSET x = "ab": PRINT "["; x; "]"` runs, and `RSET r.s = "ab"` with `s` a
  `STRING * 6` member
- **THEN** it prints `[   ab]`, and `r.s` is `    ab`

#### Scenario: An empty target
- **WHEN** `a$ = "": LSET a$ = "abc": PRINT LEN(a$)` runs
- **THEN** it prints 0

#### Scenario: Rejected forms
- **WHEN** a program has `LSET n% = "a"`, `LSET r = r2` with two `TYPE` variables, `LSET "abc" = "a"`, `LSET
  MID$(a$, 2, 2) = "x"`, `LSET a$ = 5`, `LSET a$`, `LSET` or `CALL LSET(a$, "x")`
- **THEN** each is a compile error that is not "not supported yet"

### Requirement: LOCK and UNLOCK
`LOCK [#]n[, [first][ TO last]]` and `UNLOCK` with the same list SHALL compile to the runtime call the old
compiler writes, also after `CALL`, and SHALL raise the runtime's errors: 52 for a file that is not open, 5 for a
record of 0 or below, 75 for a range the wrong way round, 70 for an `UNLOCK` that matches no `LOCK` and for a
second `LOCK` of the same bytes.

#### Scenario: The three forms
- **WHEN** `LOCK #1`, `LOCK #1, 2`, `LOCK #1, 2 TO 4` and the same three with `UNLOCK` are compiled by both
  compilers
- **THEN** the runtime calls are the same

#### Scenario: TO without a first record
- **WHEN** a program has `LOCK #1, TO 4` or `LOCK #1 2 TO 4`
- **THEN** it is a compile error

#### Scenario: Locked twice
- **WHEN** `LOCK #1, 2` runs twice under a handler that prints `ERR` and resumes next, then `UNLOCK #1, 2` twice
- **THEN** the handler prints 70 once for the second `LOCK` and once for the second `UNLOCK`

#### Scenario: The program reads what it locked
- **WHEN** a program locks record 1 of a random file, reads and writes it, and closes the file without `UNLOCK`
- **THEN** no error is raised
