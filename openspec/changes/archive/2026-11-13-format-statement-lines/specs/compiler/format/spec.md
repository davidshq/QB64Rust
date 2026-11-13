# Spec Delta

## ADDED Requirements

### Requirement: Rewrite settings
`-y` SHALL take two settings beyond QB64pe's: `OneStatementPerLine` and `BlockIf`, each a boolean read as the
other boolean layout settings, both false by default. They SHALL be given by `-f:`, by `--format-default` and
by the `[format]` table, in the order of "Order of layout settings". With both false, no line SHALL differ from
the layout without them.

#### Scenario: Off by default
- **WHEN** `x = 0: y = 0` is formatted with no setting
- **THEN** it becomes `x = 0: y = 0`

#### Scenario: On the command line
- **WHEN** `x = 0: y = 0` is formatted with `-f:OneStatementPerLine=true`
- **THEN** the output is the two lines `x = 0` and `y = 0`

#### Scenario: A bad value
- **WHEN** `-f:BlockIf=maybe` is given
- **THEN** the exit status is 1 with a message that the setting needs `true` or `false`

### Requirement: One statement per line
With `OneStatementPerLine` true, a line that holds two or more statements SHALL be written as one line for each
statement, in order, without the colons between them, each indented by its block depth. A statement after
`CASE …`, after a block's `ELSE` or after `ELSEIF … THEN` on the same line SHALL go to its own line. A line
with one statement SHALL be written as without the setting.

#### Scenario: Two assignments
- **WHEN** `x = 0: y = 0` is formatted with the setting
- **THEN** the output is `x = 0` and `y = 0`

#### Scenario: A loop on one line
- **WHEN** `for i = 1 to 3: print i: next` is formatted with the setting
- **THEN** the output is `For i = 1 To 3`, `    Print i` and `Next`

#### Scenario: SELECT CASE on one line
- **WHEN** `SELECT CASE x: CASE 1: b = 5: CASE ELSE: b = 6: END SELECT` is formatted with the setting
- **THEN** the output is `Select Case x`, `    Case 1`, `        b = 5`, `    Case Else`, `        b = 6` and
  `End Select`

#### Scenario: A statement after ELSEIF and after ELSE
- **WHEN** a block `IF` has the lines `ELSEIF b THEN x = 2: b = 3` and `ELSE x = 3` and is formatted with the
  setting
- **THEN** those lines become `ElseIf b Then`, `    x = 2`, `    b = 3`, `Else` and `    x = 3`

#### Scenario: One statement and a colon
- **WHEN** `x = 1:` is formatted with the setting
- **THEN** the line is written as without the setting

### Requirement: A single-line IF is one statement
With `OneStatementPerLine` true and `BlockIf` false, a single-line `IF` with both of its branches SHALL count as
one statement. It SHALL be written on one line, and the colons inside its branches SHALL stay.

#### Scenario: Colons inside the IF stay
- **WHEN** `IF a THEN b = 1: x = 2` is formatted with `OneStatementPerLine` alone
- **THEN** it becomes `If a Then b = 1: x = 2`

#### Scenario: A statement before the IF
- **WHEN** `x = 1: IF a THEN b = 1: x = 2` is formatted with `OneStatementPerLine` alone
- **THEN** the output is `x = 1` and `If a Then b = 1: x = 2`

### Requirement: Single-line IF to a block
With `BlockIf` true, a single-line `IF` SHALL be written as a block: the `IF … THEN` line, the statements of
the first branch one level in, then `ELSE` and the statements of the second branch if the `IF` has an `ELSE`,
then `END IF`. With `OneStatementPerLine` false, the statements of a branch SHALL stay together on one line
with their colons. A single-line `IF` inside a branch SHALL become a block inside that block.

#### Scenario: Both branches
- **WHEN** `IF a THEN x = 1 ELSE x = 2` is formatted with `BlockIf`
- **THEN** the output is `If a Then`, `    x = 1`, `Else`, `    x = 2` and `End If`

#### Scenario: Several statements in a branch, both settings
- **WHEN** `IF hp <= 0 THEN PRINT "Dead": lives = lives - 1` is formatted with both settings
- **THEN** the output is `If hp <= 0 Then`, `    Print "Dead"`, `    lives = lives - 1` and `End If`

#### Scenario: Several statements in a branch, BlockIf alone
- **WHEN** the same line is formatted with `BlockIf` alone
- **THEN** the output is `If hp <= 0 Then`, `    Print "Dead": lives = lives - 1` and `End If`

#### Scenario: An IF inside a branch
- **WHEN** `IF a THEN IF b THEN x = 1 ELSE x = 2` is formatted with `BlockIf`
- **THEN** the output is `If a Then`, `    If b Then`, `        x = 1`, `    Else`, `        x = 2`,
  `    End If` and `End If`

#### Scenario: An empty first branch
- **WHEN** `IF a THEN ELSE x = 4` is formatted with `BlockIf`
- **THEN** the output is `If a Then`, `Else`, `    x = 4` and `End If`

#### Scenario: A colon right after THEN or ELSE
- **WHEN** `IF x THEN: x = 4` and `IF x THEN ELSE: x = 3` are formatted with `BlockIf`
- **THEN** the output is `If x Then`, `    x = 4`, `End If`, and `If x Then`, `Else`, `    x = 3`, `End If`,
  with no colon

#### Scenario: A statement before the IF, BlockIf alone
- **WHEN** `x = 1: IF a THEN b = 1` is formatted with `BlockIf` alone
- **THEN** the output is `x = 1: If a Then`, `    b = 1` and `End If`

### Requirement: A jump written as a line number
With `BlockIf` true, a line number that stands alone after `THEN` or `ELSE` SHALL be written as `GOTO` and the
number, on its own line in the block. `IF cond GOTO target` SHALL be written as `IF cond THEN`, `GOTO target`
and `END IF`.

#### Scenario: THEN and a line number
- **WHEN** `IF x THEN 100 ELSE 200` is formatted with `BlockIf`
- **THEN** the output is `If x Then`, `    GoTo 100`, `Else`, `    GoTo 200` and `End If`

#### Scenario: IF and GOTO without THEN
- **WHEN** `IF y GOTO 100` is formatted with `BlockIf`
- **THEN** the output is `If y Then`, `    GoTo 100` and `End If`

### Requirement: The start of a rewritten line
A line number, a label, or a SUB's name and a colon at the start of a line SHALL stay on the first output line,
with the first statement after it, written as without the settings. A built-in statement's name and a colon
(`CLS:`) is a statement: it SHALL get its own line, without the colon.

#### Scenario: A label before two statements
- **WHEN** `top: x = 9: b = 9` is formatted with `OneStatementPerLine`
- **THEN** the output is `top: x = 9` and `b = 9`

#### Scenario: A line number before a single-line IF
- **WHEN** `10 IF x THEN 20` is formatted with `BlockIf`
- **THEN** the output is `10 If x Then`, `    GoTo 20` and `End If`

#### Scenario: A SUB's name and a colon
- **WHEN** `DrawBoard: PRINT "Ready": x = 1` is formatted with `OneStatementPerLine`, `DrawBoard` a SUB that no
  jump names
- **THEN** the output is `DrawBoard: Print "Ready"` and `x = 1`

#### Scenario: A built-in statement's name and a colon
- **WHEN** `CLS: LOCATE 1, 1: PRINT "Ready"` is formatted with `OneStatementPerLine`
- **THEN** the output is `Cls`, `Locate 1, 1` and `Print "Ready"`

### Requirement: The comment of a rewritten line
When a line that ends in a comment is written as two or more lines, the comment SHALL be written first, on a
line of its own, at the indent of the first output line. Its text SHALL NOT change. A line that the settings
do not change SHALL keep its comment at its end.

#### Scenario: A comment after two statements
- **WHEN** `x = 0: y = 0 ' reset the cursor` is formatted with `OneStatementPerLine`
- **THEN** the output is `' reset the cursor`, `x = 0` and `y = 0`

#### Scenario: A REM comment after a single-line IF
- **WHEN** `IF a THEN x = 1: REM first turn` is formatted with `BlockIf`
- **THEN** the output is `Rem first turn`, `If a Then`, `    x = 1` and `End If`

#### Scenario: One statement keeps its comment
- **WHEN** `x = 0 ' reset` is formatted with both settings
- **THEN** it becomes `x = 0 ' reset`

### Requirement: Lines the rewrites leave alone
The two settings SHALL NOT change a line joined to the next by `_` or a line it joins, a line between
`'$FORMAT:OFF` and `'$FORMAT:ON`, a line of a `$IF` branch that is not taken, a line that holds a metacommand
as a statement or in its comment, or a line where `AutoIndent` or `AutoLayout` is false. Such a line SHALL be
written as without the settings.

#### Scenario: A continued line
- **WHEN** `x = 0: y = _` and `1` are formatted with both settings
- **THEN** both lines are written as read

#### Scenario: Format off
- **WHEN** `x = 0: y = 0` stands between `'$FORMAT:OFF` and `'$FORMAT:ON` and is formatted with both settings
- **THEN** it stays one line

#### Scenario: A metacommand in the comment
- **WHEN** `DIM a(5): x = 1 '$DYNAMIC` is formatted with `OneStatementPerLine`
- **THEN** it stays one line, `Dim a(5): x = 1 '$Dynamic`

#### Scenario: AutoLayout off
- **WHEN** `x = 0:  y = 0` is formatted with `-f:AutoLayout=false -f:OneStatementPerLine=true`
- **THEN** it stays one line with its own spacing

### Requirement: A rewrite setting without effect
When `OneStatementPerLine` or `BlockIf` is true and `AutoIndent` or `AutoLayout` is false, taken after every
source of the settings, `-y` SHALL print one warning that names the rewrite setting and the setting that is
off. It SHALL still write the output file and exit with status 0. `'$FORMAT:OFF` in the file SHALL NOT give a
warning.

#### Scenario: Layout off in the settings file
- **WHEN** `qb64rust.toml` holds `[format]`, `one-statement-per-line = true` and `auto-layout = false`, and
  `p.bas` is formatted
- **THEN** a warning says that `OneStatementPerLine` has no effect while `AutoLayout` is off, the output file
  is written, and the exit status is 0

#### Scenario: Both rewrites, one warning
- **WHEN** `-f:OneStatementPerLine=true -f:BlockIf=true -f:AutoIndent=false` is given
- **THEN** one warning names both rewrite settings and `AutoIndent`

#### Scenario: Format off in the file
- **WHEN** the settings are on and the file holds `'$FORMAT:OFF`
- **THEN** no warning is printed

### Requirement: Rewrites need a program without errors
The two settings SHALL take effect only in a program that is not incomplete ("Names in an incomplete
program"). In an incomplete program every line SHALL be written as without the settings. An include file
formatted as a file of its program, the program without errors, SHALL be rewritten.

#### Scenario: A SUB that is not defined
- **WHEN** a file holds `call missing: x = 1` and is formatted with `OneStatementPerLine`
- **THEN** the output file reads `Call missing: x = 1`

#### Scenario: An include file alone
- **WHEN** `lib.bm` holds `SUB a`, `x = 0: y = 0`, `END SUB` and `qb64rust -y lib.bm -o out.bm` is run with
  `OneStatementPerLine`
- **THEN** the second line is written `x = 0: y = 0`

### Requirement: A rewritten program is the same program
A program formatted with either setting or both SHALL compile to the same typed tree and the same C++ as the
input, apart from source line numbers and the letter case of names. It SHALL hold the same comments, in the
same order. Formatting the output again with the same settings SHALL give the same text. The number of the line a statement is on changes: `_ERRORLINE` and an error
message that names a line give the new number.

#### Scenario: Same C++
- **WHEN** a corpus program and its text formatted with both settings are each dumped with `--dump cpp`
- **THEN** the two dumps are equal once the line numbers in `#line` and in the event checks are left out

#### Scenario: No comment lost
- **WHEN** a corpus program is formatted with both settings
- **THEN** the texts of its comments, in order, are those of the input

#### Scenario: Idempotent
- **WHEN** a program formatted with both settings is formatted again with both settings
- **THEN** the text does not change

#### Scenario: Nothing left to rewrite
- **WHEN** a program formatted with both settings is read
- **THEN** it has no single-line `IF` and no line with two statements, other than on the lines of "Lines the
  rewrites leave alone" and after a line's label or line number

## MODIFIED Requirements

### Requirement: Output equal to the old compiler's
For a program the old compiler formats, the output SHALL equal `qb64pe -y`'s output with the same settings, byte for
byte after line endings are taken as LF, unless a row of `DIVERGENCES-QB64PE.md` says otherwise. This SHALL hold for
spacing, keyword case, indentation, metacommands, comments, labels and line numbers. It SHALL hold with
`OneStatementPerLine` and `BlockIf` both false; `qb64pe -y` has no such settings.

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

### Requirement: Format table of the settings file
`-y` SHALL read the `[format]` table of the first `qb64rust.toml` found in the folder of the file it formats or a
folder above it. The table SHALL hold `auto-indent`, `indent-subs`, `auto-layout`, `one-statement-per-line` and
`block-if` (each `true` or `false`), `indent-size` (an integer from 1 to 64) and `keyword-case` (`"camel"`,
`"upper"` or `"lower"`). A key that is absent SHALL leave its setting as it was given otherwise.

#### Scenario: Capitals for the project
- **WHEN** `qb64rust.toml` beside `p.bas` holds `[format]` and `keyword-case = "upper"`, and `p.bas` is `print 1`
- **THEN** `qb64rust -y p.bas -o out.bas` writes `PRINT 1`

#### Scenario: Only one key
- **WHEN** the table holds only `indent-size = 2`
- **THEN** blocks are indented two spaces a level and keywords are in CamelCase

#### Scenario: No table
- **WHEN** `qb64rust.toml` holds only a `[fixes]` table
- **THEN** the output is the same as with no `qb64rust.toml`

#### Scenario: Rewrites for the project
- **WHEN** the table holds `one-statement-per-line = true` and `block-if = true`, and `p.bas` is
  `IF a THEN x = 1: y = 2`
- **THEN** `qb64rust -y p.bas -o out.bas` writes `If a Then`, `    x = 1`, `    y = 2` and `End If`

#### Scenario: The command line turns a rewrite off
- **WHEN** the table holds `one-statement-per-line = true` and `-f:OneStatementPerLine=false` is given
- **THEN** `x = 0: y = 0` stays one line

### Requirement: A bad format table
An unknown key in `[format]`, a value of the wrong type, an `indent-size` outside 1 to 64 or a `keyword-case`
that is not one of the three words SHALL be an error that names the file, the line and, for a key, the known
keys.

#### Scenario: A misspelled key
- **WHEN** `qb64rust.toml` holds `[format]` and `indent = 2`
- **THEN** the error names `qb64rust.toml`, its line 2 and the seven keys

#### Scenario: Indent size out of range
- **WHEN** the table holds `indent-size = 65`
- **THEN** the error says that the size must be in the range 1 to 64

#### Scenario: A rewrite key that is not a boolean
- **WHEN** the table holds `block-if = "yes"`
- **THEN** the error says that the value of `block-if` must be true or false
