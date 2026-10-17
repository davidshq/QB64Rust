# Spec Delta

## ADDED Requirements

### Requirement: $VERSIONINFO
A line whose upper case starts with `$VERSIONINFO` SHALL set one field of the executable's version resource, as in
the old compiler: the key stands between the line's first `:` and the first `=` after it, blanks trimmed, in any
letter case; the value is the rest of the line, trimmed, each `"` read as `'`, one pair of enclosing `'` removed.
The last line of a key SHALL count. A line in a `$IF` branch that is not taken SHALL NOT count.

#### Scenario: Every key
- **WHEN** a program sets `CompanyName`, `FileDescription`, `FILEVERSION#=1,2,3,4`, `PRODUCTVERSION#=5,6,7,8`,
  `FileVersion`, `InternalName`, `LegalCopyright`, `LegalTrademarks`, `OriginalFilename`, `ProductName`,
  `ProductVersion`, `Comments` and `Web`, each in single quotes
- **THEN** it compiles with no diagnostic, and the version resource of its executable holds each value, as the
  old compiler's does

#### Scenario: The string version follows the numeric one
- **WHEN** a program has `$VERSIONINFO:FILEVERSION#=1,2,3,4` and no `FileVersion`, or `FileVersion='text'` before
  `FILEVERSION#=1,2,3,4`
- **THEN** the string file version is `1,2,3,4` in the first and `text` in the second; a `FileVersion` after the
  numeric key replaces the digits

#### Scenario: Forms of a line
- **WHEN** a program has `$versioninfo:companyname="double quoted"`, `$VERSIONINFO : FileDescription = 'spaced'`,
  `$VERSIONINFO:InternalName='a:b=c'` and `$VERSIONINFOX:ProductName='prefix'`
- **THEN** the values are `double quoted`, `spaced`, `a:b=c` and `prefix`, as with the old compiler

#### Scenario: A value without quotes
- **WHEN** a program compiled with `-w` has `$VERSIONINFO:ProductName=no quotes` or
  `$VERSIONINFO:Comments='a' ' trailing comment`
- **THEN** each is the old compiler's warning "Missing string bracket delimiters", and the values are `no quotes`
  and `'a' ' trailing comment`

#### Scenario: Places
- **WHEN** a `$VERSIONINFO` line stands in a `$IF` branch that is not taken, in a SUB, and in an included file
- **THEN** the first sets nothing and the other two set their fields

#### Scenario: Rejected lines
- **WHEN** a program has `$VERSIONINFO`, `$VERSIONINFO:=x`, `$VERSIONINFO:CompanyName`, `$VERSIONINFO:Author='x'`,
  `$VERSIONINFO:FILEVERSION#=1,2,3` or `$VERSIONINFO:FILEVERSION#=1, 2,3,4`
- **THEN** each is a compile error at that line, not "not supported yet", where the old compiler rejects it

#### Scenario: Numeric parts the resource compiler refuses
- **WHEN** a program has `$VERSIONINFO:FILEVERSION#=1,,2,3`, `=,1,2,3`, `=70000,0,0,0` or `=08,0,0,0`
- **THEN** each is a compile error that names the part, where the old compiler's build fails; `=65535,0,0,0` and
  `=010,0,0,0` (octal, 8) compile

#### Scenario: A manifest Windows would refuse
- **WHEN** the company name, the product name or the description holds `<`, `&` or a byte that is not UTF-8
- **THEN** it is a compile error that names the character, where the old compiler's executable does not start;
  `>`, UTF-8 and an `&` in another value compile
