# Spec Delta

## MODIFIED Requirements

### Requirement: Fixes options
A compile, `-z` and `--dump` SHALL take `--fixes on|off` and `--fix <name>=on|off` as spec `compiler/fixes` says,
and SHALL read `qb64rust.toml` unless `--no-settings` is given. `qb64rust --list-fixes` SHALL be the only
argument, as `--version` is. The usage text SHALL name the three options.

#### Scenario: A dump with a fix
- **WHEN** `qb64rust --fix whole-array-value=on --dump typed p.bas` is run on a program that prints `a()`
- **THEN** it fails with the compile error of `whole-array-value` and exit status 1

#### Scenario: List with another argument
- **WHEN** `qb64rust --list-fixes p.bas` is run
- **THEN** it is a usage error with exit status 1

## ADDED Requirements

### Requirement: Lints and format options
A compile, `-y`, `-z` and `--dump` SHALL take `--lints on|off` and `--lint <code>=on|off` as spec
`compiler/lints` says. `-y` SHALL take `--format-default <setting>=<value>` as spec `compiler/format` says; in
any other mode it SHALL be a usage error. The usage text SHALL name the three options.

#### Scenario: A lint off in a format run
- **WHEN** `qb64rust -y -w --lint unused-label=off p.bas -o out.bas` is run on a program with an unused label
- **THEN** no `unused-label` warning is printed and `out.bas` is written

#### Scenario: A format default in a build
- **WHEN** `qb64rust --format-default KeywordCapitals=true p.bas` is run
- **THEN** it is a usage error with exit status 1

### Requirement: No settings file
With `--no-settings`, no `qb64rust.toml` SHALL be looked for or read, in any mode that would read it: the fixes,
the lints and the layout settings SHALL come from the command line and the defaults alone. The usage text SHALL
name the option.

#### Scenario: A file that would change the layout
- **WHEN** `qb64rust.toml` beside `p.bas` holds `[format]` and `keyword-case = "upper"`, and `qb64rust -y
  --no-settings p.bas -o out.bas` is run on `print 1`
- **THEN** `out.bas` reads `Print 1`

#### Scenario: A file with an error
- **WHEN** `qb64rust.toml` beside `p.bas` is not valid TOML and `qb64rust --no-settings p.bas` is run
- **THEN** `p.bas` is built, and no error names `qb64rust.toml`
