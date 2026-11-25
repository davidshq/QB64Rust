# $ENCODING

`'$ENCODING:UTF8` says that a source file is saved as UTF-8. The compiler then stores each character of the
file's strings and `DATA` items as its code page 437 byte, so the program draws the characters the editor shows.
It stands in a comment.

## Syntax

```text
'$ENCODING:UTF8
'$ENCODING:CP437
```

## Behavior

The line is a comment that starts with `'$ENCODING:`, alone on its line but for blanks. It may be on any line and
applies to the whole file. The value may be in any letter case.
<!-- src: verification\v74_a1_line_utf8; verification\v74_a3_line_case_blanks -->

In a file declared UTF-8, `"─"` is one character, the byte 196: `LEN("─")` is 1. The program is the one the same
text saved as code page 437 gives.
<!-- src: verification\v74_b_declared_text; verification\v74_f2_twin_utf8; D-077 -->

A string or `DATA` item may also hold the symbols the screen draws for the bytes 1 to 31 and 127, such as `♥`
for byte 3 and `⌂` for byte 127. A comment may hold any character.
<!-- src: verification\v74_f2_twin_utf8 -->

`encoding = "utf-8"` under `[project]` in `qb64rust.toml` declares every file in the settings file's folder and
below it. A file's own line goes over it, so `'$ENCODING:CP437` keeps one file as code page 437. A file outside
that folder, such as a library's, is code page 437 unless it has its own line.
<!-- src: D-077 -->

A file declared UTF-8 may start with a byte order mark (the bytes 0xEF 0xBB 0xBF), which is skipped.
<!-- src: verification\v74_d2_bom_declared; D-077 -->

The text of `$VERSIONINFO` and the file names of `$INCLUDE`, `$EMBED`, `$EXEICON` and `DECLARE LIBRARY` are
read as written.
<!-- src: D-077 -->

A file that declares nothing is read as bytes, as QB64pe reads it: a UTF-8 `"─"` is then three characters.
<!-- src: verification\v74_e_undeclared_text -->

## Errors

- A character with no code page 437 byte in a string or a `DATA` item of a UTF-8 file: "`日` (U+65E5) has no
  code page 437 byte: a string of a UTF-8 file holds code page 437 characters".
  <!-- src: D-077 -->
- Bytes that are not UTF-8 in a file declared UTF-8: "the byte 0xC4 is not UTF-8, and the file is declared UTF-8
  by its `'$ENCODING:UTF8` line".
  <!-- src: D-077 -->
- A byte order mark in a file that is not declared UTF-8: "the file starts with a byte order mark (the bytes
  0xEF 0xBB 0xBF), which only a file declared UTF-8 may hold".
  <!-- src: verification\v74_d1_bom -->
- `$ENCODING:UTF8` outside a comment: "unknown metacommand `$ENCODING`".
  <!-- src: verification\v74_c_metacommand -->
- A warning, and the program is still built: a line with another value, such as `'$ENCODING:LATIN1`, which then
  declares nothing, and a second line that names another encoding than the first.
  <!-- src: verification\v74_a4_line_unknown_value -->

## Differences

From QB64pe: QB64pe reads the line as a comment and copies the bytes of a string as they are, so a file declared
UTF-8 draws other characters there. It rejects a file that starts with a byte order mark (`DIVERGENCES-QB64PE.md`
D-077).
<!-- src: D-077 -->

## Availability

<!-- begin generated: availability -->
Not checked: no call-site program, example or probe uses it.
<!-- end generated -->

## See also

[`$INCLUDE`](meta-include.md), [`$VERSIONINFO`](meta-versioninfo.md), [`DATA`](data.md)
