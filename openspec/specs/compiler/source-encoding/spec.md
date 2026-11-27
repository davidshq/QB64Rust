# compiler/source-encoding Specification

## Purpose
A source file may be declared UTF-8. The compiler then stores the characters of its string literals and `DATA`
items as code page 437 bytes, so the program is the one the same text saved as code page 437 gives. A file that
is not declared is read as bytes, as before.

## Requirements

### Requirement: A file's encoding
Each source file of a program SHALL have one encoding, UTF-8 or code page 437, found in this order:

1. the file's own `'$ENCODING` line;
2. the `encoding` key of the `[project]` table of the program's `qb64rust.toml`, when the file is in the
   settings file's folder or a folder below it;
3. code page 437.

A file's encoding SHALL NOT pass to the files it includes: each included file is decided by the same three
steps.

#### Scenario: Nothing declared
- **WHEN** `p.bas` has no `'$ENCODING` line and no `qb64rust.toml` is found
- **THEN** `p.bas` is read as code page 437, and its C++ is the same as before this change

#### Scenario: The project's key
- **WHEN** `qb64rust.toml` holds `[project]` and `encoding = "utf-8"`, beside `main.bas`, which includes
  `ui\menu.bm`
- **THEN** both files are UTF-8

#### Scenario: A file's line wins
- **WHEN** the project's key is `"utf-8"` and `old.bi` holds the line `'$ENCODING:CP437`
- **THEN** `old.bi` is read as code page 437 and the project's other files as UTF-8

#### Scenario: A file outside the project's folder
- **WHEN** the project's key is `"utf-8"` and `main.bas` includes a library's file from the compiler's folder
- **THEN** that file is read as code page 437 unless it has its own line

### Requirement: The $ENCODING line
A file SHALL declare its encoding with a comment that starts with `'$ENCODING:` followed by `UTF8` or `CP437`,
in any letter case, with nothing but blanks after it. The comment SHALL be the first thing on its line except
blanks. It MAY be on any line of the file and SHALL apply to the whole file. Another value SHALL be a warning
at the line, and the line SHALL then declare nothing. When two lines of one file name different encodings, the
first SHALL decide and the second SHALL be a warning at its line. These warnings SHALL carry no code, and the
program SHALL still be built, as QB64pe builds it (`verification\v74_a4_line_unknown_value`). `$ENCODING`
written outside a comment SHALL stay the error it is today.

#### Scenario: Declared UTF-8
- **WHEN** the first line of a file is `'$ENCODING:UTF8`
- **THEN** the file is UTF-8

#### Scenario: An unknown value
- **WHEN** line 1 is `'$ENCODING:LATIN1`
- **THEN** the program is built, with a warning at line 1 that names `UTF8` and `CP437`, and the file's
  encoding is found as if the line were not there

#### Scenario: Two lines that disagree
- **WHEN** line 1 is `'$ENCODING:UTF8` and line 5 is `'$ENCODING:CP437`
- **THEN** the file is UTF-8, and line 5 has a warning that names line 1

#### Scenario: A comment that only mentions it
- **WHEN** a line is `PRINT 1 ' see $ENCODING:UTF8`
- **THEN** the line declares nothing

### Requirement: The project's encoding key
The `[project]` table of `qb64rust.toml` SHALL accept `encoding = "utf-8"` or `encoding = "cp437"`. Another
value SHALL be an error that names the file, the line and the two values, and the program SHALL NOT be built.

#### Scenario: A bad value
- **WHEN** `qb64rust.toml` holds `[project]` and, on line 2, `encoding = "latin1"`
- **THEN** the compile fails with an error naming `qb64rust.toml`, line 2, `"utf-8"` and `"cp437"`

### Requirement: Text of a UTF-8 file
In a UTF-8 file, each character of a string literal and of a `DATA` item SHALL be stored as its code page 437
byte. A character below 128 is its own byte. The program SHALL be the one that the same text saved as code page
437 gives: the C++ written SHALL be the same.

#### Scenario: A box-drawing character
- **WHEN** a UTF-8 file holds `PRINT LEN("─"); ASC("─")`
- **THEN** the program prints ` 1  196`

#### Scenario: The same C++ as the code page 437 file
- **WHEN** one program is saved once as code page 437 with the first line `'$ENCODING:CP437` and once as UTF-8
  with the first line `'$ENCODING:UTF8`, and both hold `╔══╗`, `é` and `√` in string literals and `DATA` items
- **THEN** the two builds write the same C++

#### Scenario: A DATA item without quotes
- **WHEN** a UTF-8 file holds `DATA café, "naïve"` and reads both items into strings
- **THEN** the strings are 4 and 5 bytes long

### Requirement: The characters of bytes 1 to 31 and 127
In a UTF-8 file, a string literal or `DATA` item SHALL also accept the 32 symbols that code page 437 draws for
the bytes 1 to 31 and 127 (`☺` to `▼`, and `⌂`), each stored as its byte. A control character written as
itself (such as a tab) SHALL be stored as its own byte.

#### Scenario: A heart
- **WHEN** a UTF-8 file holds `PRINT ASC("♥")`
- **THEN** the program prints ` 3`

### Requirement: A character outside code page 437
In a UTF-8 file, a character in a string literal or a `DATA` item that has no code page 437 byte SHALL be a
compile error at the character. The message SHALL show the character and its code point, and say that a string
holds code page 437 characters. The program SHALL NOT be built.

#### Scenario: A Japanese character
- **WHEN** line 3 of a UTF-8 file is `PRINT "日本"`
- **THEN** the compile fails at line 3, column 8, and the message names `日` and `U+65E5`

### Requirement: Comments of a UTF-8 file
The compiler SHALL accept any character in a comment of a UTF-8 file. Nothing of a comment SHALL reach the
program.

#### Scenario: A comment in Japanese
- **WHEN** a UTF-8 file holds `' 日本語のコメント` and `PRINT 1`
- **THEN** it compiles with no error and no warning

### Requirement: A UTF-8 file must be valid
A file whose encoding is UTF-8 and whose bytes are not valid UTF-8 SHALL be a compile error at the first byte
that is not. The message SHALL name the byte and say how the file was declared: by its line, or by
`qb64rust.toml`. The program SHALL NOT be built.

#### Scenario: A code page 437 file in a UTF-8 project
- **WHEN** the project's key is `"utf-8"` and line 12 of `map.bm` holds the single byte 0xC4 in a string
- **THEN** the compile fails at line 12 of `map.bm`, and the message names the byte `0xC4` and `qb64rust.toml`

### Requirement: A byte order mark
A UTF-8 file MAY start with the bytes 0xEF 0xBB 0xBF, which SHALL be skipped. In a file that is not UTF-8,
those bytes SHALL be read as they are today. QB64pe rejects every file that starts with them, as "Unexpected
character on line" (`verification\v74_d1_bom`, `v74_d2_bom_declared`), so a UTF-8 file with the mark builds only
here.

#### Scenario: A file with the mark
- **WHEN** a file starts with 0xEF 0xBB 0xBF and its first line is `'$ENCODING:UTF8`
- **THEN** it compiles as the same file without the three bytes

### Requirement: Text that stays as written
In a UTF-8 file, the text of `$VERSIONINFO` SHALL reach the executable's manifest as its UTF-8 bytes. A file
name in `$INCLUDE`, `$EMBED`, `$EXEICON` and `DECLARE LIBRARY` SHALL be read as it is today.

#### Scenario: An accent in the product name
- **WHEN** a UTF-8 file holds `$VERSIONINFO:ProductName='café'`
- **THEN** the program builds, and the manifest holds `café` in UTF-8

### Requirement: Positions in a UTF-8 file
A position in a UTF-8 file SHALL be a byte offset, and a column in a diagnostic a 1-based byte column, as in
every other file (spec `compiler/pipeline`, "Source is bytes").

#### Scenario: A column after an accent
- **WHEN** line 1 of a UTF-8 file is `PRINT "é": PRINT x(`
- **THEN** the syntax error's column is one higher than in the same file saved as code page 437

### Requirement: Formatting a UTF-8 file
`qb64rust -y` and the editor's formatting SHALL write a UTF-8 file's string literals, `DATA` items and comments
with the bytes they had. A file SHALL be formatted when a string holds a character outside code page 437, as it
is with any error that is not a syntax error.

#### Scenario: Layout only
- **WHEN** a UTF-8 file holds `print "╔══╗" ' été` and is formatted
- **THEN** `print` gets the keyword case of the settings (`Print` by default), and the bytes of the string and
  the comment are unchanged

### Requirement: The convert command
`qb64rust convert --to utf8 <files>` SHALL rewrite each named file from code page 437 to UTF-8, and
`--to cp437` from UTF-8 to code page 437, in place. It SHALL NOT add, remove or change a `'$ENCODING` line. It
SHALL change no file when any named file cannot be converted, and SHALL then name each such file, its line and
the reason, with exit status 1:

- with `--to utf8`: a file that holds a byte above 127 and is already valid UTF-8;
- with `--to cp437`: a file that is not valid UTF-8, or that holds a character with no code page 437 byte.

A file of bytes below 128 only SHALL be left as it is.

#### Scenario: A game's file
- **WHEN** `qb64rust convert --to utf8 battle.bm` is run on a code page 437 file that holds the byte 0xC4
- **THEN** `battle.bm` holds 0xE2 0x94 0x80 in its place, and every other byte below 128 is unchanged

#### Scenario: There and back
- **WHEN** a code page 437 file is converted with `--to utf8` and then with `--to cp437`
- **THEN** its bytes are those it started with

#### Scenario: Already UTF-8
- **WHEN** `--to utf8 a.bas b.bas` is run and `b.bas` is valid UTF-8 with `é` in it
- **THEN** neither file is changed, the message names `b.bas`, and the exit status is 1
