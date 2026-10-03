# editor/language-support Specification

## Purpose
Makes VS Code recognise QB64 source files as one language, highlight them, and open them with the byte encoding
QB64 programs rely on, without any compiler installed.

## Requirements

### Requirement: Language registration
The extension SHALL register the language id `qb64rust` with display name "QB64" for files ending in `.bas`, `.bi`
and `.bm`. It SHALL NOT register the aliases `BASIC` or `QBasic`.

#### Scenario: Opening a .bas file
- **WHEN** the user opens `hello.bas` with no other BASIC extension installed
- **THEN** the editor's language mode is `qb64rust`

#### Scenario: Include file
- **WHEN** the user opens `lib.bi` or `lib.bm`
- **THEN** the editor's language mode is `qb64rust`

### Requirement: Syntax highlighting
The extension SHALL provide a TextMate grammar with scope name `source.qb64rust` that distinguishes comments,
string literals, numeric literals, keywords, metacommands, type suffixes and line labels/numbers.

#### Scenario: Comment forms
- **WHEN** a line contains `' text` or starts with `REM text` (any case)
- **THEN** the text from the comment marker to the end of the line is scoped as a comment

#### Scenario: Metacommand in a comment
- **WHEN** a line is `'$INCLUDE:'lib.bi'` or `$CONSOLE`
- **THEN** the metacommand is scoped differently from an ordinary comment

#### Scenario: Strings are not split by apostrophes
- **WHEN** a line is `PRINT "it's": x = 1`
- **THEN** `"it's"` is scoped as one string and `x = 1` is not scoped as a comment

#### Scenario: Case-insensitive keywords
- **WHEN** a keyword is written as `print`, `Print` or `PRINT`
- **THEN** all three receive the same keyword scope

### Requirement: Language configuration
The extension SHALL configure `'` as the line comment, `()` as brackets, `"` as an auto-closing pair, and folding
for `SUB`…`END SUB`, `FUNCTION`…`END FUNCTION` and `TYPE`…`END TYPE` blocks.

#### Scenario: Toggle comment
- **WHEN** the user runs Toggle Line Comment on `PRINT 1`
- **THEN** the line becomes `' PRINT 1`, or `'PRINT 1` per the editor's comment style

#### Scenario: Fold a SUB
- **WHEN** a file contains `SUB Foo` … `END SUB`
- **THEN** the editor offers a folding range covering the block

### Requirement: CP437 default encoding
The extension SHALL set `files.encoding` to `cp437` as the default for the `qb64rust` language. A user or workspace
setting for that language SHALL take precedence.

#### Scenario: Byte round trip
- **WHEN** a `.bas` file containing bytes 128–255 inside a string literal is opened and saved without edits
- **THEN** the file's bytes are unchanged

#### Scenario: User override
- **WHEN** the user sets `"[qb64rust]": { "files.encoding": "utf8" }`
- **THEN** `.bas` files open as UTF-8
