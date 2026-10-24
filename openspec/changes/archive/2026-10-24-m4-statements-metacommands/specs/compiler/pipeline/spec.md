# Spec Delta

## MODIFIED Requirements

### Requirement: The whole language parses
Every statement and expression form the old compiler accepts SHALL parse into a node of its own kind, without a
parser diagnostic. A construct that the later stages do not compile yet SHALL be reported "not supported yet" by
`sema` at that node, not by the parser.

#### Scenario: Statement not compiled yet
- **WHEN** a program contains `DO: FILES: LOOP UNTIL a > 0`
- **THEN** the tree has a `DoBlock` holding the `FILES` statement, and the only diagnostic is one "not supported
  yet" error at `FILES`

#### Scenario: Built-in statement read by its template
- **WHEN** a program contains `LINE (0, 0)-(9, 9), , BF`
- **THEN** it parses into one `BuiltinStmt` with `BF` as a word, not as a variable

#### Scenario: Member access after an index
- **WHEN** a program contains `PRINT a(1).b`
- **THEN** it parses without a diagnostic, and the item is a member access on `a(1)`

### Requirement: Included files
A comment `'$INCLUDE:'file'` (or `REM $INCLUDE:'file'`) SHALL include the file after the line it is on;
`$INCLUDE:'file'` without a comment SHALL be an error, as in the old compiler. An absolute path SHALL be used as
written; a relative one SHALL be looked up in the including file's folder, then relative to the compiler root (see
the CLI spec), never relative to the working directory. `$INCLUDEONCE` in a file SHALL make later inclusions of that file empty. A missing file, an empty name,
or an inclusion deeper than 100 levels SHALL be an error at the include; a file that includes itself SHALL NOT be
an error by itself. Diagnostics in an included file SHALL name that file and its line and column.
A block or a SUB or FUNCTION opened in one file and closed in a file it includes, or opened in an included file
and closed in the file that includes it, SHALL compile and run as if the included text stood at the include,
as in the old compiler.

#### Scenario: Nested include not looked up in the main file's folder
- **WHEN** the main file includes `sub/a.bi`, `sub/a.bi` includes `b.bi`, and `b.bi` exists only next to the
  main file (and not under the compiler root)
- **THEN** the compile fails with "file not found" at the include in `sub/a.bi`

#### Scenario: Include relative to the compiler root
- **WHEN** a program includes `'$INCLUDE:'extra/x.bi'`, which exists only under the directory given with
  `--include-root`
- **THEN** that file is included

#### Scenario: Include without a comment
- **WHEN** a program contains `$INCLUDE:'lib.bi'` (no `'` or `REM`)
- **THEN** the compile fails with an error at that line, which is not marked "not supported yet"

#### Scenario: Error in an included file
- **WHEN** `lib.bm` has a syntax error on its line 4 and the main file includes it
- **THEN** the error is reported at `lib.bm` line 4

#### Scenario: Block closed in an included file
- **WHEN** the main file opens `FOR i = 1 TO 2`, the next line includes a file whose lines are a `PRINT` and
  `NEXT`, and the main file then prints `after`
- **THEN** the program compiles, and its output is the included `PRINT` twice, then `after`, as the old
  compiler's (`verification\v16_m7_block_across`)

#### Scenario: SUB closed in an included file
- **WHEN** the main file opens `SUB s`, and the file it includes on the next line holds the SUB's body and
  `END SUB`
- **THEN** the program compiles, and a call of `s` runs that body (`verification\v16_m7_sub_across`)

#### Scenario: Every kind of block, both ways
- **WHEN** a `FOR`, `DO`, `IF`, `SELECT CASE`, `WHILE`, SUB or FUNCTION is opened in one file and closed in the
  other, an `ELSE` stands in the included file, a `NEXT j, i` there closes two loops of the main file, or the
  closer is two levels of include down
- **THEN** each program compiles and prints what the old compiler's prints (`verification\v57_b01`–`b15`, `b17`,
  `b18`, `b20`)

#### Scenario: A block an included file leaves open
- **WHEN** an included file opens `FOR i = 1 TO 2` and nothing closes it
- **THEN** the compile fails with the error of a `FOR` without `NEXT` (`verification\v57_b16`)

#### Scenario: NEXT of another variable across files
- **WHEN** the main file opens `FOR i` and the included file closes it with `NEXT j`
- **THEN** the compile fails with an error at `NEXT j` in the included file (`verification\v57_b19`)

#### Scenario: NEXT inside a $IF of an included file
- **WHEN** the main file opens `FOR i` and the included file closes it with a `NEXT i` inside a `$IF`
- **THEN** the compile fails with one error, at the `NEXT` (the old compiler's "NEXT without FOR",
  `verification\v57_b21`)

#### Scenario: A CONST in a SUB opened by an included file
- **WHEN** an included file opens a SUB, and its part in the main file defines `CONST n = 5`
- **THEN** `n` is the SUB's: a `TYPE` in that part may size a member with it and the main module's `n` is a variable;
  a main-module `CONST n` below is "Name already in use" at the main module's use of `n`, and a main-module `TYPE`
  below that sizes a member with it is "Invalid number/constant after STRING * type" (`verification\v57_b22`–`b25`)

#### Scenario: Included twice
- **WHEN** a file that does not contain `$INCLUDEONCE` is included twice
- **THEN** it is parsed twice, and the names in each inclusion are distinct symbols

### Requirement: Precompiler flags
A `$IF` or `$ELSEIF` condition that names `_CONSOLE_`, `_EXPLICIT_`, `_EXPLICITARRAY_`, `_ASSERTS_`, `_DEBUG_` or
`_SOCKETS_` SHALL be evaluated with the value the old compiler gives the flag from the whole program, the same on
every line of the code that is compiled. The values SHALL be settled as the old compiler settles them, by
compiling again until they no longer change. Where the old compiler's own passes disagree about the flag's value
on that line, the `$IF` SHALL be compiled as the old compiler's recorded programs show: none of its branches is
compiled, and a `CONST` in the branch the flag's value on that line chooses makes its name "Name already in use" at
the first use below; a `CONST` in another branch, and any other statement, has no effect (measured,
`verification\v26_x27`, `x34`, `v57_g01`–`g11`). `$ASSERTS` and `$ASSERTS:CONSOLE` both give `_ASSERTS_` 1, so
a `$IF` between them is no such line (`v57_g08`).

#### Scenario: Above the line that sets it
- **WHEN** a program tests `$IF _CONSOLE_ = 1 THEN` on a line above its `$CONSOLE`
- **THEN** the branch is taken

#### Scenario: $CONSOLE:ONLY alone
- **WHEN** a program with `$CONSOLE:ONLY` and no `$CONSOLE` tests `_CONSOLE_`
- **THEN** its value is 2

#### Scenario: $CONSOLE:ONLY, then $CONSOLE
- **WHEN** a program has `$CONSOLE:ONLY` and below it `$CONSOLE`
- **THEN** `_CONSOLE_` is 1

#### Scenario: OPTION _EXPLICIT
- **WHEN** a program has `OPTION _EXPLICIT` on its last line and `$IF _EXPLICIT_ THEN` and `$IF _EXPLICITARRAY_
  THEN` above it
- **THEN** both branches are taken

#### Scenario: OPTION _EXPLICITARRAY
- **WHEN** a program has `OPTION _EXPLICITARRAY` on its last line and tests both flags above it
- **THEN** the `_EXPLICITARRAY_` branch is taken and the `_EXPLICIT_` branch is not

#### Scenario: Between two lines that set the flag differently
- **WHEN** a program has `$CONSOLE:ONLY`, then `$IF _CONSOLE_ = 2 THEN` with a `PRINT` in each of its three
  branches, then `$CONSOLE`
- **THEN** the program compiles and prints nothing from the branches, as the old compiler's
  (`verification\v26_x27`)

#### Scenario: A CONST between two lines that set the flag differently
- **WHEN** the branches of that `$IF` each define `CONST k`, and a line below prints `k`
- **THEN** the compile fails with an error at the use of `k`, with the old compiler's "Name already in use" as
  its note (`verification\v26_x34`)

#### Scenario: A CONST only in a branch the flag does not choose there
- **WHEN** only the `$ELSEIF _CONSOLE_ = 1` branch of that `$IF` defines `CONST k = 1`, and a line below prints `k`
- **THEN** the program compiles and prints `k = 0`: `k` is a variable (`verification\v57_g05`)

#### Scenario: A DIM in each branch
- **WHEN** the branches of that `$IF` each declare `v` with `DIM` as another type, and a line below sets `v = 5` and
  prints it
- **THEN** the program compiles and prints ` 5 ` (`verification\v57_g01`)

#### Scenario: A jump to a label between two lines that set the flag differently
- **WHEN** the branches of that `$IF` each hold the label `here:` or the line number `10`, and a `GOTO here`,
  `GOTO 10` or, in a SUB, `GOSUB here` names it
- **THEN** the compile fails with an error at the jump, with "Label 'here' not defined" as its note; with no jump
  the program compiles (`verification\v57_j01`–`j04`)

#### Scenario: A block that crosses the $END IF
- **WHEN** a `DO` is opened in a `$IF`'s branches and closed by a `LOOP` after its `$END IF`, whether or not the
  old compiler's passes disagree there
- **THEN** the compile fails, as the old compiler's "PROGRAM FLOW ERROR!", and an error in a later line is still
  reported (`verification\v57_j05`, `j06`)

#### Scenario: Nothing sets a flag
- **WHEN** a program with none of these features tests `$IF _CONSOLE_ THEN`, `$IF _ASSERTS_ THEN`, `$IF _DEBUG_
  THEN` and `$IF _SOCKETS_ THEN`
- **THEN** none of the branches is taken

#### Scenario: A metacommand inside the branch it changes
- **WHEN** a program's only `$CONSOLE` stands inside `$IF _CONSOLE_ = 0 THEN` … `$END IF`
- **THEN** the program has a console and `_CONSOLE_` is 1 everywhere else in it, as with the old compiler

#### Scenario: $CONSOLE:ONLY inside the branch it changes
- **WHEN** a program's only `$CONSOLE:ONLY` stands inside `$IF _CONSOLE_ = 0 THEN` … `$END IF`
- **THEN** `_CONSOLE_` is 2, and the program has a console and a window (the line that would make it one without
  a window is not compiled), as with the old compiler

#### Scenario: OPTION _EXPLICIT inside the branch it changes
- **WHEN** a program's only `OPTION _EXPLICIT` stands inside `$IF _EXPLICIT_ = 0 THEN` … `$END IF`, and a line
  below uses a variable that is not declared
- **THEN** `_EXPLICIT_` is 1 and that use is the error of a variable that is not declared, as with the old
  compiler (the option is in force by its flag, though its line is not compiled in the last round)

#### Scenario: Three rounds
- **WHEN** a program's only `$CONSOLE` stands inside `$IF _EXPLICIT_ THEN` … `$END IF` and its only `OPTION
  _EXPLICIT` on the last line
- **THEN** the program has a console, and `_CONSOLE_` and `_EXPLICIT_` are 1 on every line

#### Scenario: A setting inside a branch a line above has switched off
- **WHEN** a program has `$CONSOLE` on its first line and below it `OPTION _EXPLICIT` inside `$IF _CONSOLE_ = 0
  THEN` … `$END IF`
- **THEN** `_EXPLICIT_` is 0: the branch is not taken in any round, as with the old compiler

## ADDED Requirements

### Requirement: Metacommands the old compiler does not know
A `$` line outside a comment that is no metacommand of the old compiler SHALL be a compile error at that line,
with the old compiler's "Syntax error" as its note, in the main file and in an included one (measured,
`verification\v57_m01`–`m08`). `$NOPREFIX` SHALL be a compile error with the old compiler's own message as its
note. A metacommand the old compiler knows and this compiler does not build yet SHALL stay "not supported yet".

#### Scenario: Other lines that are no metacommand
- **WHEN** a line is `$STATIC`, `$nosuch`, `$FOO:BAR`, `$CONSOLE:OFF`, a `$` alone, or `$INCLUDE: 'x.bi'`
- **THEN** the compile fails with an error at that line, with "Syntax error" as its note (`verification\v57_m01`–
  `m05`, `m08`)

#### Scenario: Unknown name
- **WHEN** an active line of a program is `$NOSUCHTHING`
- **THEN** the compile fails with an error at that line, not marked "not supported yet" (`verification\v16_m6`)

#### Scenario: Unknown name in an inactive branch
- **WHEN** `$NOSUCHTHING` stands inside `$IF 0 THEN` … `$END IF`
- **THEN** no diagnostic is given for it

#### Scenario: $DYNAMIC without a comment
- **WHEN** a program's line is `$DYNAMIC`
- **THEN** the compile fails with an error at that line (`verification\v24_x38`)

#### Scenario: A name close to $MIDISOUNDFONT
- **WHEN** a line is `$MIDISOUNDFONT`, `$MIDISOUNDFONT :DEFAULT`, `$MIDISOUNDFONTS:DEFAULT` or `$MIDISOUND:DEFAULT`
- **THEN** the compile fails with an error at that line (`verification\v39_x74`, `x79`–`x81`)

#### Scenario: $NOPREFIX
- **WHEN** a program has `$NOPREFIX`
- **THEN** the compile fails with an error whose note is the old compiler's message that `$NOPREFIX` is deprecated
  (`verification\v43_x32`)

#### Scenario: A metacommand not built yet
- **WHEN** a program has `$ERRORLOCATION:ON`
- **THEN** it is reported "not supported yet"

### Requirement: Syntax errors the old compiler reports
Where the parser meets an operator or a keyword it does not expect, and a recording shows that the old compiler
rejects that form, the diagnostic SHALL be an error, not marked "not supported yet", with the old compiler's
message as its note. The same form in a place no recording covers SHALL stay marked.

#### Scenario: An operator with no operand before it
- **WHEN** a program has `x = OR`
- **THEN** the compile fails with an error at `OR`, with "Expected variable/value before 'OR'" as its note
  (`verification\v51_x50`)

#### Scenario: A word after `^`
- **WHEN** a program prints `2 ^ _NEGATE x + 3`
- **THEN** the compile fails with an error at `_NEGATE` (`verification\v24_x238`)

#### Scenario: `IF` without a condition
- **WHEN** a program has `IF THEN PRINT`, or a block `IF THEN`, or `ELSEIF THEN`
- **THEN** the compile fails with an error at `THEN`, with "Syntax error" or, for `ELSEIF`, "Expected ... THEN" as its
  note (`tests\snippets\qb64fresh\error_detection__invalid_syntax`, `verification\v57_e10`, `e11`)

#### Scenario: Other operators with no operand before them
- **WHEN** a program has `x = AND 1`
- **THEN** the compile fails with an error at `AND`, with "Expected variable/value before 'AND'" as its note
  (`verification\v57_e08`)

#### Scenario: NOT after `^`
- **WHEN** a program prints `2 ^ NOT x` with `x = 1`
- **THEN** it compiles and prints ` .25 `, as the old compiler's program (`verification\v57_e09`)
