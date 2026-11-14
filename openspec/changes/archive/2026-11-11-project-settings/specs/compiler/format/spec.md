# Spec Delta

## MODIFIED Requirements

### Requirement: Default settings
A layout setting that is not given SHALL take QB64pe's built-in default: `AutoIndent` true, `AutoIndentSize` 4,
`IndentSubs` true, `AutoLayout` true, keywords in CamelCase. A setting is given by `--format-default`, by the
`[format]` table of the program's `qb64rust.toml`, or by `-f:`. QB64pe's `config.ini` SHALL NOT be read.

#### Scenario: No settings
- **WHEN** a program with no `qb64rust.toml` in its folder or above is formatted with no `-f:` setting
- **THEN** the output equals `qb64pe -y` with the five defaults given explicitly

### Requirement: Program with an error
When the parser reports an error (a syntax error, or an included file that is not found), `-y` SHALL print the
diagnostics as a compile prints them, exit with status 1, and write no output file. An error of a later stage (a
SUB that is not defined, a type error) SHALL NOT stop formatting: `-y` SHALL print the
diagnostics, write the output file and exit with status 0. A warning SHALL NOT stop formatting. A
`qb64rust.toml` that cannot be used SHALL stop formatting: `-y` SHALL print the error with the file's name and
line, exit with status 1, and write no output file.

#### Scenario: Syntax error
- **WHEN** a program whose second line is `x = 1 +` is formatted
- **THEN** the exit status is 1, the error names line 2, and no output file exists

#### Scenario: A SUB that is not defined
- **WHEN** a file whose only line is `call   missing` is formatted
- **THEN** the error `CALL missing`: no SUB of this name is printed, the exit status is 0, and the output file
  reads `Call missing`

#### Scenario: A bad settings file
- **WHEN** `qb64rust.toml` beside `p.bas` holds `[format]` and `keyword-case = "uper"`, and `qb64rust -y p.bas -o
  out.bas` is run
- **THEN** the error names `qb64rust.toml` and its line 2, the exit status is 1, and `out.bas` does not exist

## ADDED Requirements

### Requirement: Format table of the settings file
`-y` SHALL read the `[format]` table of the first `qb64rust.toml` found in the folder of the file it formats or a
folder above it. The table SHALL hold `auto-indent`, `indent-subs` and `auto-layout` (each `true` or `false`),
`indent-size` (an integer from 1 to 64) and `keyword-case` (`"camel"`, `"upper"` or `"lower"`). A key that is
absent SHALL leave its setting as it was given otherwise.

#### Scenario: Capitals for the project
- **WHEN** `qb64rust.toml` beside `p.bas` holds `[format]` and `keyword-case = "upper"`, and `p.bas` is `print 1`
- **THEN** `qb64rust -y p.bas -o out.bas` writes `PRINT 1`

#### Scenario: Only one key
- **WHEN** the table holds only `indent-size = 2`
- **THEN** blocks are indented two spaces a level and keywords are in CamelCase

#### Scenario: No table
- **WHEN** `qb64rust.toml` holds only a `[fixes]` table
- **THEN** the output is the same as with no `qb64rust.toml`

### Requirement: Order of layout settings
A layout setting SHALL take its value from the highest of these that gives it: a `-f:` setting, then the
`[format]` table, then a `--format-default` option, then QB64pe's default. When `-f:KeywordCapitals` or
`-f:KeywordLowercase` is given, the keyword case SHALL come from those two alone, and the table's
`keyword-case` SHALL play no part.

#### Scenario: The command line over the file
- **WHEN** the table holds `indent-size = 2` and `-f:AutoIndentSize=8` is given
- **THEN** blocks are indented eight spaces a level

#### Scenario: CamelCase asked for on the command line
- **WHEN** the table holds `keyword-case = "upper"`, and `-f:KeywordCapitals=0 -f:KeywordLowercase=0` is given
- **THEN** keywords are written in CamelCase

#### Scenario: The file over a format default
- **WHEN** the table holds `keyword-case = "upper"` and `--format-default KeywordLowercase=true` is given
- **THEN** keywords are written in capitals

### Requirement: Format default option
`-y` SHALL take `--format-default <setting>=<value>` any number of times. The setting names and values SHALL be
those of the `-f:` layout settings, with the same errors. A name that is not a layout setting SHALL be an error
with exit status 1.

#### Scenario: No settings file
- **WHEN** `p.bas` has no `qb64rust.toml` above it and `--format-default KeywordCapitals=true` is given
- **THEN** keywords are written in capitals

#### Scenario: Not a layout setting
- **WHEN** `--format-default OptimizeCppProgram=true` is given
- **THEN** the exit status is 1 with a message that names the setting

### Requirement: A bad format table
An unknown key in `[format]`, a value of the wrong type, an `indent-size` outside 1 to 64 or a `keyword-case`
that is not one of the three words SHALL be an error that names the file, the line and, for a key, the known
keys.

#### Scenario: A misspelled key
- **WHEN** `qb64rust.toml` holds `[format]` and `indent = 2`
- **THEN** the error names `qb64rust.toml`, its line 2 and the five keys

#### Scenario: Indent size out of range
- **WHEN** the table holds `indent-size = 65`
- **THEN** the error says that the size must be in the range 1 to 64
