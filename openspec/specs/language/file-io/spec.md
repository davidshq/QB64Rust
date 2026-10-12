# language/file-io Specification

## Purpose
How programs use files by number: opening and closing, sequential writing and reading, the file functions and the
file-system statements, with the runtime errors each raises, as measured with `qb64pe.exe`.

## Requirements

### Requirement: OPEN and CLOSE
`OPEN name$ [FOR mode] [ACCESS …] [SHARED | LOCK …] AS [#]n [LEN = r]` and the old form `OPEN mode$, [#]n, name$[,
r]` SHALL open a file as the old compiler's runtime does, for every mode, access and lock word. `CLOSE` SHALL close
every open file, and `CLOSE [#]n, …` the files named; closing a number that is not open SHALL NOT be an error.

#### Scenario: Write, close, read back
- **WHEN** a program opens `t.tmp` for output as #1, prints a line to it, closes it, opens it for input, reads the
  line and prints it
- **THEN** the line printed is the line written

#### Scenario: File not found
- **WHEN** `OPEN "nope.tmp" FOR INPUT AS #1` runs under a handler that prints `ERR` and resumes next
- **THEN** the handler prints 53

#### Scenario: Closing a file that is not open
- **WHEN** `CLOSE #5: PRINT "closed"` runs with no file open under a handler that prints `ERR`
- **THEN** it prints `closed` and the handler does not run

#### Scenario: File number from an expression
- **WHEN** `f = FREEFILE: OPEN "t.tmp" FOR OUTPUT AS f: PRINT #f, "x": CLOSE f` runs
- **THEN** `t.tmp` holds the line `x`

#### Scenario: Old form
- **WHEN** `OPEN "O", #1, "t.tmp": PRINT #1, "x": CLOSE` runs
- **THEN** `t.tmp` holds the line `x`

### Requirement: PRINT to a file
`PRINT #n, items` SHALL write what `PRINT` would print (numbers with their sign blank and trailing blank, `;` and
`,` separators, the line end unless the statement ends with a separator) to file `n`, with zones and line ends as
the old compiler's runtime writes them to a file. A file number that is not open SHALL raise error 52.

#### Scenario: Numbers and strings
- **WHEN** `PRINT #1, "x"; 7; "y"` runs with file 1 open for output
- **THEN** the file gains the line `x 7 y`

#### Scenario: File not open
- **WHEN** `PRINT #3, "x"` runs with no file open under a handler that prints `ERR` and resumes next
- **THEN** the handler prints 52

#### Scenario: Raising item
- **WHEN** `PRINT #1, "a"; CHR$(-1); "b"` runs under a handler that resumes next, with file 1 open for output
- **THEN** the file gains what the old compiler's program writes for it: `a` and nothing after it, without a line
  end

#### Scenario: Zones in a file
- **WHEN** `PRINT #1, "a", "b"` runs with file 1 open for output
- **THEN** the file gains `a`, 13 blanks and `b`: zones are 14 columns wide in a file

#### Scenario: Items need a separator
- **WHEN** `PRINT #1, "a" "b"` is compiled
- **THEN** it is a compile error, as in the old compiler (only console `PRINT` puts a `;` between adjacent items)

### Requirement: WRITE
`WRITE #n, items` SHALL write the items separated by commas, strings in double quotes and numbers without blanks,
followed by a line end, and `WRITE items` SHALL write the same to the console, as the old compiler.

#### Scenario: Mixed items to a file
- **WHEN** `WRITE #1, 1, "a", 2.5` runs with file 1 open for output
- **THEN** the file gains the line `1,"a",2.5`

#### Scenario: To the console
- **WHEN** `WRITE "a", 5` runs
- **THEN** it prints `"a",5` and a line end

#### Scenario: Trailing comma
- **WHEN** `WRITE #1, 1,` runs with file 1 open for output
- **THEN** the file gains `1,` without a line end, as with the old compiler

#### Scenario: Semicolon between items
- **WHEN** `WRITE 1; 2` is compiled
- **THEN** it is a compile error

### Requirement: INPUT and LINE INPUT from a file
`LINE INPUT #n, s$` SHALL read one line without its line end. `INPUT #n, target, …` SHALL read one field per
target as the old compiler's runtime does: fields end at a comma or a line end, a quoted field keeps its commas, a
numeric target takes the field's value in the target's type. Reading past the end of the file SHALL raise error 62.

#### Scenario: Line and field
- **WHEN** a file holds the lines `1,"a",2.5` and `x 7 y`, and a program runs `LINE INPUT #1, s$: INPUT #1, t$`
- **THEN** `s$` is `1,"a",2.5` and `t$` is `x 7 y`

#### Scenario: Fields of a WRITE line
- **WHEN** a file holds `1,"a,b",2.5` and a program runs `INPUT #1, n&, s$, d#: PRINT n&; s$; d#`
- **THEN** it prints ` 1 a,b 2.5 `

#### Scenario: Past the end
- **WHEN** `LINE INPUT #1, s$` runs at the end of file 1 under a handler that prints `ERR` and resumes next
- **THEN** the handler prints 62

#### Scenario: Value outside the target's range
- **WHEN** a file holds `300,7` and `INPUT #1, b%%, n&` runs under a handler that prints `ERR` and resumes next
- **THEN** the handler prints 6, `b%%` is 0, `n&` is unchanged and the field `7` is still unread, as with the old
  compiler

#### Scenario: Targets that are no string variable or no variable
- **WHEN** `LINE INPUT #1, n&`, `LINE INPUT #1, a$, b$` or `INPUT #1, 5` is compiled
- **THEN** each is a compile error

#### Scenario: Whole array as a target
- **WHEN** `INPUT #1, a()` is compiled
- **THEN** the diagnostic is "not supported yet" (the old compiler accepts it)

### Requirement: File functions
`EOF(n)`, `LOF(n)`, `LOC(n)`, `SEEK(n)`, `FREEFILE`, `_FILEEXISTS(name$)`, `_DIREXISTS(name$)` and `_CWD$` SHALL
return what the old compiler's runtime returns, and `SEEK [#]n, position` SHALL set the position of file `n`.

#### Scenario: EOF before and after the last line
- **WHEN** a file holds two lines and a program prints `EOF(1)` after reading the first and after reading the second
- **THEN** it prints ` 0 ` and `-1 `

#### Scenario: FREEFILE skips open numbers
- **WHEN** `OPEN "a.tmp" FOR OUTPUT AS #1: PRINT FREEFILE` runs
- **THEN** it prints ` 2 `

#### Scenario: Existence
- **WHEN** `PRINT _FILEEXISTS("nope.tmp"); _DIREXISTS(".")` runs
- **THEN** it prints ` 0 -1 `

### Requirement: File-system statements
`KILL name$`, `NAME old$ AS new$`, `MKDIR path$`, `RMDIR path$` and `CHDIR path$` SHALL act on the file system as
the old compiler's runtime does and raise its errors.

#### Scenario: KILL of a missing file
- **WHEN** `KILL "m1.tmp"` runs twice for an existing file under a handler that prints `ERR` and resumes next
- **THEN** the first removes the file and the second makes the handler print 53

#### Scenario: Rename
- **WHEN** `NAME "a.tmp" AS "b.tmp": PRINT _FILEEXISTS("a.tmp"); _FILEEXISTS("b.tmp")` runs for an existing `a.tmp`
- **THEN** it prints ` 0 -1 `

#### Scenario: Make, enter and remove a folder
- **WHEN** `MKDIR "d": PRINT _DIREXISTS("d"): RMDIR "d": PRINT _DIREXISTS("d")` runs
- **THEN** it prints `-1 ` and ` 0 `
