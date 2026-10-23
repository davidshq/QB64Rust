# Spec Delta

## MODIFIED Requirements

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
