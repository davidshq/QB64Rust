# language/embedded-files Specification

## Purpose
Files built into the executable: `$EMBED` names a file and a handle at compile time, and `_EMBEDDED$` returns the
file's bytes at run time, as programs built with the old compiler do.

## Requirements
### Requirement: The $EMBED line
A line whose upper-case text starts with `$EMBED` SHALL name a file and a handle: the text between its first two
`'` and between the next two, without blanks at their ends, the rest of the line dropped. A `$EMBED` in a comment
SHALL be a comment, and one in a `$IF` branch not taken SHALL name nothing.

#### Scenario: Letter case, a longer name and blanks
- **WHEN** a program has `$embed:'./f.txt','a'`, `$EMBEDX:'./g.txt','b'` and `$EMBED : ' ./h.txt ' , ' c ' words`
- **THEN** it compiles, and `_EMBEDDED$("a")`, `_EMBEDDED$("b")` and `_EMBEDDED$("c")` return the three files

#### Scenario: No file or no handle
- **WHEN** a program has `$EMBED`, `$EMBED:'./f.txt'`, `$EMBED:'','h'` or `$EMBED:'./f.txt',''`
- **THEN** it is a compile error, "expected `$EMBED:'filename','handle'`" as the old compiler's

#### Scenario: In a comment
- **WHEN** a program has `'$EMBED:'./f.txt','h'` and calls `_EMBEDDED$("h")`
- **THEN** the call is a compile error: no `$EMBED` names `h`

### Requirement: Where the file is
A file name that starts with `./` or `.\` SHALL name a file in the folder of the main source file, also when the
`$EMBED` stands in an included file; an absolute name SHALL be taken as written. Any other relative name SHALL name
a file relative to the compiler root, as the old compiler looks in its own folder; a name with a single leading
slash, on the compiler root's drive. A name with a drive and no slash after it SHALL be reported "not supported
yet". A name that is not a file SHALL be a compile error.

#### Scenario: From an included file
- **WHEN** `inc/e.bi`, included by `main.bas`, has `$EMBED:'./data/f.txt','h'`
- **THEN** the file is `data/f.txt` beside `main.bas`

#### Scenario: Relative to the compiler's folder
- **WHEN** a program has `$EMBED:'tests/compile_tests/embed/test.output','test'` and is compiled with
  `--include-root tests/upstream/root`, under which that file lies
- **THEN** `_EMBEDDED$("test")` returns that file's bytes (`embed/test`)

#### Scenario: Relative, and not under the compiler root
- **WHEN** a program has `$EMBED:'v44_files/text.txt','h'` and the compiler root holds no such file
- **THEN** it is a compile error at the line, as the old compiler's (`verification\v44_x26`)

#### Scenario: Missing file or a folder
- **WHEN** the named file is not there, or is a folder
- **THEN** it is a compile error at the line that names the file as the program writes it

### Requirement: Handles
A handle SHALL be a letter followed by letters and digits, and letter case SHALL count. A file or a handle named
by two `$EMBED` lines SHALL be a compile error that names the first one's line; two files are the same when
their names are the same text after the folder is put in front.

#### Scenario: Bad characters
- **WHEN** a handle is `1abc`, `my_h` or holds a letter outside ASCII
- **THEN** it is a compile error, as the old compiler's

#### Scenario: Case counts
- **WHEN** a program embeds two files as `h` and `H`
- **THEN** it compiles and each handle returns its own file

#### Scenario: Twice
- **WHEN** the same file is embedded under two handles, or one handle names two files
- **THEN** it is a compile error at the second line, naming the line of the first

#### Scenario: The same file beside the source, written with ./ and .\
- **WHEN** one line embeds `./f.txt` and a later one `.\f.txt`
- **THEN** it is a compile error at the second line, "already embedded", as the old compiler's

### Requirement: _EMBEDDED$
`_EMBEDDED$("handle")` SHALL return a new string holding every byte of the file, whatever their values and
however many there are, at each call. Its argument SHALL be one non-empty string literal holding a valid handle
named by a `$EMBED` line above the call in file order; anything else SHALL be a compile error. A call in a `$IF`
branch not taken checks nothing.

#### Scenario: Every byte value
- **WHEN** a program embeds a file of the 256 byte values, an empty file and one of 300,000 bytes
- **THEN** `_EMBEDDED$` returns each byte for byte, as the old compiler's program does

#### Scenario: A changed copy
- **WHEN** a program changes the string one call returned and calls `_EMBEDDED$` again
- **THEN** the second call returns the file unchanged

#### Scenario: Not a literal
- **WHEN** the argument is a variable, `"a" + "b"` or `CHR$(104)`
- **THEN** it is a compile error, "Embed-Handle must be a single literal string in quotes, not a variable"

#### Scenario: Above the call
- **WHEN** a call stands above the `$EMBED` line of its handle, or the handle is named by no line
- **THEN** it is a compile error: the handle is undefined

#### Scenario: Forms
- **WHEN** `_EMBEDDED$` has no argument or two, stands without its `$`, or in a `CONST`
- **THEN** it is a compile error, as the old compiler's

### Requirement: Only used files are built in
A file SHALL be built into the executable only when a compiled `_EMBEDDED$` call names its handle, and the program
SHALL link the embed part of the runtime only then.

#### Scenario: An unused file
- **WHEN** a program embeds a file and never calls `_EMBEDDED$` for it
- **THEN** it builds without the embed part, as with no `$EMBED`
