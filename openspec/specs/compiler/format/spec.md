# compiler/format Specification

## Purpose
The `-y` mode of `qb64rust`: lays out a QB64 source file as `qb64pe -y` lays it out (indentation, spacing, keyword
case, the spelling of names), with the same layout settings, without changing the program.

## Requirements

### Requirement: Format command
`qb64rust -y [-q] [-m] [-f:<layout setting>=<value>]... <file.bas> -o <out>` SHALL write the formatted text of
the file to `<out>` and exit with status 0. Without `-o`, it SHALL exit with status 1, print that formatting
requires an output file, and write nothing. Formatting SHALL NOT build, run or write C++. It SHALL NOT change the
input file.

#### Scenario: Runner's format line
- **WHEN** `qb64rust -y -m -f:autoindent=1 -f:autoindentsize=4 -f:indentsubs=1 -f:autolayout=1
  -f:keywordcapitals=0 -f:keywordlowercase=0 general.bas -o out` runs in QB64pe's `format_tests\general`
- **THEN** the exit status is 0 and `out` equals `full-cc.out`

#### Scenario: No output file
- **WHEN** `qb64rust -y prog.bas` is run without `-o`
- **THEN** the exit status is 1, a message says that formatting requires an output file, and no file is written

### Requirement: Layout settings
The settings `AutoIndent`, `IndentSubs`, `AutoLayout`, `KeywordCapitals` and `KeywordLowercase` (each `true`,
`on`, `yes`, `1` or `-1` for true, `false`, `off`, `no` or `0` for false, in any case) and `AutoIndentSize` (1 to
64, read as `VAL` reads it) SHALL be read as `qb64pe` reads them. Names are not case
sensitive. Keywords SHALL be in capitals when only `KeywordCapitals` is true, in lower case when only
`KeywordLowercase` is true, and in CamelCase otherwise. A value out of range or not boolean SHALL be an error
(exit 1).

#### Scenario: Both keyword settings
- **WHEN** `-f:KeywordCapitals=true -f:KeywordLowercase=true` is given
- **THEN** keywords are written in CamelCase, as `qb64pe` writes them

#### Scenario: Indent size out of range
- **WHEN** `-f:AutoIndentSize=65` is given
- **THEN** the exit status is 1 with a message that the size must be in the range 1 to 64

### Requirement: Default settings
A layout setting that is not given SHALL take QB64pe's built-in default: `AutoIndent` true, `AutoIndentSize` 4,
`IndentSubs` true, `AutoLayout` true, keywords in CamelCase. No configuration file SHALL be read.

#### Scenario: No settings
- **WHEN** a program is formatted with no `-f:` setting
- **THEN** the output equals `qb64pe -y` with the five defaults given explicitly

### Requirement: Output equal to the old compiler's
For a program the old compiler formats, the output SHALL equal `qb64pe -y`'s output with the same settings, byte for
byte after line endings are taken as LF, unless a row of `DIVERGENCES.md` says otherwise. This SHALL hold for
spacing, keyword case, indentation, metacommands, comments, labels and line numbers.

#### Scenario: Spacing and keyword case
- **WHEN** the line `if x   =   1 then` is formatted with the defaults
- **THEN** it becomes `If x = 1 Then`

#### Scenario: Indentation by control depth
- **WHEN** a `SELECT CASE` with a `CASE 1` and one statement is formatted inside a `SUB` with the defaults
- **THEN** `Select Case x` is indented one level, `Case 1` two levels, and the statement three, as in QB64pe's
  `format_tests\general\full-cc.out`

#### Scenario: Metacommand in a comment
- **WHEN** the line `        '$include:'inc.bi'` inside an `IF` block is formatted with the defaults
- **THEN** it becomes `    '$Include:'inc.bi'`

### Requirement: Names spelled as declared
Each variable, array, constant, procedure and label SHALL be written everywhere with the spelling of its
definition, as the old compiler writes it. A call before the definition SHALL be written the same. A built-in
SHALL be written with the old compiler's spelling for the keyword style.

#### Scenario: Variable and procedure spelling
- **WHEN** `DIM myVar AS LONG`, `MYVAR = 1`, `x = myfunc(2)` and later `FUNCTION myFunc (n)` are formatted with the
  defaults
- **THEN** the lines read `Dim myVar As Long`, `myVar = 1`, `x = myFunc(2)` and `Function myFunc (n)`

#### Scenario: Built-in spelling
- **WHEN** `x = len("a") + _rgb32(1, 2, 3)` is formatted with the defaults
- **THEN** it becomes `x = Len("a") + _RGB32(1, 2, 3)`

### Requirement: The old IDE's corrections
`?` used as `PRINT` SHALL be written as the `PRINT` keyword in the keyword style, and a string that has no
closing quote at the end of the line SHALL get one, as `qb64pe -y` does.

#### Scenario: Question mark
- **WHEN** `PRINT x;:?MYVAR` is formatted with the defaults, `myVar` declared so
- **THEN** it becomes `Print x;: Print myVar`

### Requirement: Lines left as written
A line joined to the next by `_`, together with the lines it joins, SHALL be written as read. The lines from
`'$FORMAT:OFF` to `'$FORMAT:ON` SHALL keep their own indentation and spacing; only the letter case of keywords and
names SHALL be applied to them, as `qb64pe -y` applies it with `AutoIndent` and `AutoLayout` off. The text of a
comment, of a string literal and of a `DATA` item SHALL never change.

#### Scenario: Continued line
- **WHEN** `print   1; _` / `2` is formatted
- **THEN** both lines are written unchanged

#### Scenario: Format off
- **WHEN** the lines between `'$FORMAT:OFF` and `'$FORMAT:ON` hold `  for i=1 to 2:next`
- **THEN** that line is written `  For i=1 To 2:Next`, its indent and spacing as read

### Requirement: Layout off
With `AutoLayout` false, each line SHALL keep its own spacing. Only the keyword case, the spelling of names and,
when `AutoIndent` is true, the indentation SHALL be applied, as `qb64pe -y` applies them.

#### Scenario: Spacing kept
- **WHEN** `if x   =   1 then` is formatted with `-f:AutoLayout=false`
- **THEN** it becomes `If x   =   1 Then`

### Requirement: Formatting keeps the program
The formatted text SHALL be the same program: compiled, it SHALL give the same typed tree as the input.
Formatting the output again with the same settings SHALL give the same text.

#### Scenario: Same typed tree
- **WHEN** a corpus program and its formatted text are each dumped with `--dump typed`
- **THEN** the two dumps are equal apart from source positions and the letter case of names

#### Scenario: Idempotent
- **WHEN** a formatted program is formatted again with the same settings
- **THEN** the text does not change

### Requirement: Program with an error
When the front end reports an error, `-y` SHALL print the diagnostics as a compile prints them, exit with status
1, and write no output file. A warning SHALL NOT stop formatting.

#### Scenario: Syntax error
- **WHEN** a program whose second line is `x = 1 +` is formatted
- **THEN** the exit status is 1, the error names line 2, and no output file exists

### Requirement: Included files
An included file SHALL be read to resolve names and SHALL NOT be formatted or written. The `$INCLUDE` line itself
SHALL be formatted. Included files SHALL be looked up as a compile looks them up (the including file's folder,
then the compiler root).

#### Scenario: Name from an include
- **WHEN** `main.bas` includes `inc.bi`, which declares `DIM SHARED total AS LONG`, and `main.bas` writes `TOTAL = 1`
- **THEN** the formatted `main.bas` reads `total = 1`, and `inc.bi` is unchanged
