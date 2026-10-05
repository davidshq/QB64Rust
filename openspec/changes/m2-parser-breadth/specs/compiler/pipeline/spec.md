# Spec Delta

## MODIFIED Requirements

### Requirement: Lossless syntax tree
The parser SHALL produce one tree per file of a program (the main file and each inclusion of a file), and the
tokens of each tree, printed in order, SHALL reproduce that file byte for byte, including whitespace, comments,
line ends, inactive `$IF` branches and any text it could not parse.

#### Scenario: Round trip of the corpus
- **WHEN** every `.bas` file under `tests/corpus` is parsed
- **THEN** printing each tree gives the file's exact bytes, and no parse panics

#### Scenario: Round trip of an included file
- **WHEN** a program includes `lib.bi` with `'$INCLUDE:'lib.bi'`
- **THEN** printing the main tree gives the main file's bytes, and printing the included tree gives `lib.bi`'s
  bytes

## ADDED Requirements

### Requirement: The whole language parses
Every statement and expression form the old compiler accepts SHALL parse into a node of its own kind, without a
parser diagnostic. A construct that the later stages do not compile yet SHALL be reported "not supported yet" by
`sema` at that node, not by the parser.

#### Scenario: Statement not compiled yet
- **WHEN** a program contains `FOR i = 1 TO 3: PRINT i: NEXT`
- **THEN** the tree has a `ForBlock` holding the `PRINT` statement and a `NextStmt`, and the only diagnostic is
  one "not supported yet" error at `FOR`

#### Scenario: Built-in statement read by its template
- **WHEN** a program contains `LINE (0, 0)-(9, 9), , BF`
- **THEN** it parses into one `BuiltinStmt` with `BF` as a word, not as a variable

#### Scenario: Member access after an index
- **WHEN** a program contains `PRINT a(1).b`
- **THEN** it parses without a diagnostic, and the item is a member access on `a(1)`

### Requirement: Block structure
Block statements (`IF`, `FOR`, `DO`, `WHILE`, `SELECT CASE`, `TYPE`, `DECLARE LIBRARY`, `DEF FN`, `SUB`,
`FUNCTION`) SHALL be nodes holding their body. A missing closer SHALL be one error at the block's header; a closer
without its block SHALL be one error at the closer; parsing SHALL continue after either.

#### Scenario: Missing END IF
- **WHEN** a program opens `IF a THEN` on line 2 and never closes it
- **THEN** one error is reported at line 2, and statements after it are still parsed

#### Scenario: One NEXT closes two loops
- **WHEN** a program has `FOR i = 1 TO 2`, then `FOR j = 1 TO 2`, then `NEXT j, i`
- **THEN** both loops are closed and no error is reported

### Requirement: Comment metacommands are never ignored
A comment whose first non-blank character after `'` or `REM` is `$` SHALL be examined for `$INCLUDE`, `$STATIC`
and `$DYNAMIC` by the old compiler's rule. Each one found SHALL either take effect or be reported "not supported
yet"; it SHALL NOT be treated as a plain comment.

#### Scenario: Comment include
- **WHEN** a program contains `'$INCLUDE: 'lib.bm'`
- **THEN** `lib.bm` is included after that line

#### Scenario: Not a metacommand
- **WHEN** a program contains `'$Format:Off`
- **THEN** it is an ordinary comment

### Requirement: Preprocessor
`$IF`, `$ELSEIF`, `$ELSE`, `$END IF` and `$LET` SHALL be evaluated in file order across included files, for
Windows 64-bit. Code in an inactive branch SHALL be kept in the tree but not parsed or checked; an active `$ERROR`
SHALL be an error.

#### Scenario: Inactive branch
- **WHEN** a program contains `$IF LINUX THEN`, a line that is not valid BASIC, and `$END IF`
- **THEN** no error is reported for that line

#### Scenario: Block header in a $IF branch, closer outside
- **WHEN** a program contains `$IF WIN THEN`, `IF x THEN`, `$END IF`, a statement and `END IF`
- **THEN** the compile fails with an error at the `$END IF` (the old compiler rejects it too, reporting the
  `END IF`)

#### Scenario: SUB inside a $IF branch
- **WHEN** a program contains `$IF WIN THEN`, a whole `SUB` … `END SUB`, and `$END IF`
- **THEN** no error is reported for the nesting

#### Scenario: Guarded self-include
- **WHEN** `a.bi` contains `$IF DONE = UNDEFINED THEN`, `$LET DONE = 1`, `'$INCLUDE:'a.bi'` and `$END IF`, and the
  main file includes `a.bi`
- **THEN** no error is reported, and `a.bi` is parsed twice, the second time with the branch inactive

#### Scenario: $LET in an included file
- **WHEN** `lib.bi` contains `$LET FAST = 1` and the main file, after including it, contains `$IF FAST = 1 THEN`
- **THEN** the branch is active

### Requirement: Included files
A comment `'$INCLUDE:'file'` (or `REM $INCLUDE:'file'`) SHALL include the file after the line it is on;
`$INCLUDE:'file'` without a comment SHALL be an error, as in the old compiler. An absolute path SHALL be used as
written; a relative one SHALL be looked up in the including file's folder, then relative to the compiler root (see
the CLI spec), never relative to the working directory. `$INCLUDEONCE` in a file SHALL make later inclusions of that file empty. A missing file, an empty name,
or an inclusion deeper than 100 levels SHALL be an error at the include; a file that includes itself SHALL NOT be
an error by itself. Diagnostics in an included file SHALL name that file and its line and column.

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
- **THEN** the compile fails with an error at that line

#### Scenario: Error in an included file
- **WHEN** `lib.bm` has a syntax error on its line 4 and the main file includes it
- **THEN** the error is reported at `lib.bm` line 4

#### Scenario: Included twice
- **WHEN** a file that does not contain `$INCLUDEONCE` is included twice
- **THEN** it is parsed twice, and the names in each inclusion are distinct symbols

### Requirement: No follow-on errors after an unsupported construct
When a declaration is reported "not supported yet", the names it declares (or, for `DEFxxx` and `_DEFINE`, the
implicit variables of its letter ranges from that point on) SHALL be unknown, and no further error SHALL be
reported for an expression or statement because it uses an unknown name. Names the old compiler knows but the new
one does not yet (such as the constants of the auto-included files) SHALL be reported "not supported yet", not as
reserved names.

#### Scenario: Fixed-length string not supported
- **WHEN** a program has `DIM s AS STRING * 5` and then `s = "hello"`
- **THEN** the only error is the "not supported yet" error at the `DIM`

#### Scenario: Auto-included constant
- **WHEN** a program contains `PRINT _TRUE`
- **THEN** the error at `_TRUE` is marked "not supported yet"
