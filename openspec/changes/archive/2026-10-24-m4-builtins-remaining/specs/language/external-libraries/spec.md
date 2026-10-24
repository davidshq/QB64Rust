# Spec Delta

## ADDED Requirements

### Requirement: $USELIBRARY
`$USELIBRARY:'author/library'` SHALL read the descriptor `libraries\descriptors\author\library.ini` under the
compiler root and include the files its `[LIBRARY INCLUDES]` section names (under
`libraries\includes\author\library\`) where the old compiler does: `IncAtTop` before the first line,
`IncAfterMain` after the main module's implicit `END`, `IncAtBottom` after the last line (measured,
`verification\v58_u00`–`u26`).

#### Scenario: A library with three files
- **WHEN** a program has `$USELIBRARY:'test/three'`, whose descriptor names a file of each kind, and calls a SUB
  of the bottom file and reads a `CONST` of the top file
- **THEN** it compiles and prints what the old compiler's program prints with the same library

#### Scenario: A descriptor with only some files
- **WHEN** the descriptor names only `IncAtBottom`, or names `IncAtTop=` with no value
- **THEN** only the bottom file is included (`verification\v58_u02`, `u18`)

#### Scenario: A descriptor's spelling
- **WHEN** the descriptor has a comment line, another section before `[library includes]` in lower case,
  `  incattop = "q_top.bi" ; a comment` and `IncAtBottom=q_bottom.bm ' a comment`
- **THEN** the two files are included (`verification\v58_u07`, `u23`)

#### Scenario: Nothing to include
- **WHEN** the descriptor has no `[LIBRARY INCLUDES]` section
- **THEN** the program compiles with nothing included (`verification\v58_u24`)

#### Scenario: The line's forms
- **WHEN** a program has `$uselibrary:'test/a' ' uses a`, `$USELIBRARY 'test/a'`, `$USELIBRARYX:'test/a'`,
  `$USELIBRARY:'  test/a  '` or `$USELIBRARY:'test/a'` followed by a tab and a word
- **THEN** each includes the library (`verification\v58_u09`–`u11`, `u21`, `u25`)

#### Scenario: In an included file and in a branch not taken
- **WHEN** the line stands in a file the program includes, or inside `$IF 0 THEN … $END IF`
- **THEN** the library is included from the included file, and not from the branch (`verification\v58_u08`,
  `u22`)

### Requirement: The order of several libraries
Libraries SHALL be included in the old compiler's order: the `IncAtTop` files from the last registered library to
the first, the `IncAfterMain` and `IncAtBottom` files from the first to the last. A library SHALL be registered
once per line that names it, so one named twice has its files included twice; a library's file may itself use
`$USELIBRARY` (measured, `verification\v58_u03`–`u06`).

#### Scenario: Two libraries
- **WHEN** a program uses `test/a` and then `test/b`, each with a top file that notes its name in a string and a
  bottom file with a SUB
- **THEN** it prints `main b;a;` then `sub a` and `sub b`, as the old compiler's program does
  (`verification\v58_u03`)

#### Scenario: A library named twice
- **WHEN** a program has `$USELIBRARY:'test/a'` on two lines, or uses `test/a` and `test/c` whose top file uses
  `test/a`
- **THEN** the compile fails with an error at the second definition of the library's SUB, whose note is "Name
  already in use (ASub) …" (`verification\v58_u04`, `u05`)

#### Scenario: A library that needs another
- **WHEN** the top file of `test/c` has `$USELIBRARY:'test/a'`
- **THEN** it compiles and prints `main a;c;`, then the SUB of `c` calls the SUB of `a` (`verification\v58_u06`)

### Requirement: Errors of $USELIBRARY
A line with no quoted name, or an empty one, SHALL be the error "Expected $USELIBRARY:'author/library'"; a missing
descriptor "Library descriptor for '…' not found"; a file the descriptor names that is missing "Library source
for '…' not found (" with the key's name and ")", each with the old compiler's message as the note (measured,
`verification\v58`).

#### Scenario: Without its quotes
- **WHEN** a program has `$USELIBRARY:test/a`, `$USELIBRARY:"test/a"` or `$USELIBRARY:''`
- **THEN** the compile fails with an error whose note is "Expected $USELIBRARY:'author/library'"
  (`verification\v58_u12`–`u14`)

#### Scenario: No descriptor
- **WHEN** a program has `$USELIBRARY:'test/none'` and the compiler root holds no such descriptor
- **THEN** the compile fails with an error whose note is "Library descriptor for 'test/none' not found"
  (`verification\v58`, recorded also with the release's own folder, which has no `libraries\`)

#### Scenario: A file missing
- **WHEN** the descriptor of `test/broken` names an `IncAfterMain` file that does not exist
- **THEN** the compile fails with an error whose note is "Library source for 'test/broken' not found
  (IncAfterMain)"
