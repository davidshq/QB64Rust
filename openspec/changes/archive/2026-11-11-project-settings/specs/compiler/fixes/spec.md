# Spec Delta

## MODIFIED Requirements

### Requirement: The settings file
The compiler SHALL read the fixes from the first file named `qb64rust.toml` found in the main `.bas` file's folder
or a folder above it. Its `[fixes]` table SHALL hold `all = true|false` (the master switch) and `<name> =
true|false` for single fixes; a single fix's setting SHALL override `all`. The tables `[project]`, `[format]` and
`[lints]` are those of specs `editor/language-server`, `compiler/format` and `compiler/lints`. Any other table
SHALL be ignored.

#### Scenario: Master switch with an override
- **WHEN** `qb64rust.toml` beside `p.bas` holds `[fixes]`, `all = true` and `resume-after-block = false`
- **THEN** `p.bas` is built with every fix on except `resume-after-block`

#### Scenario: Found in a folder above
- **WHEN** `qb64rust.toml` is in `proj\` and the program is `proj\src\p.bas`, with none in `proj\src\`
- **THEN** the file in `proj\` is used

#### Scenario: The nearest file wins
- **WHEN** both `proj\qb64rust.toml` and `proj\src\qb64rust.toml` exist
- **THEN** only `proj\src\qb64rust.toml` is read for `proj\src\p.bas`

### Requirement: Command-line fixes
`--fixes on|off` SHALL set the master switch and `--fix <name>=on|off` a single fix, either one any number of
times. The command line SHALL override the file: `--fixes` replaces the file's whole fix set, then each `--fix`
overrides its fix. An unknown name or value SHALL be an error with exit status 1. The file SHALL still be read
and checked when `--fixes` is given; only `--no-settings` (spec `compiler/cli`) leaves it unread.

#### Scenario: Off overrides the file
- **WHEN** `qb64rust.toml` turns every fix on and `qb64rust --fixes off p.bas` is run
- **THEN** `p.bas` is built with every fix off

#### Scenario: One fix on
- **WHEN** `qb64rust --fixes off --fix whole-array-value=on p.bas` is run
- **THEN** only `whole-array-value` is on

#### Scenario: A bad file with --fixes
- **WHEN** `qb64rust.toml` holds `[fixes]` and `nonsense = true`, and `qb64rust --fixes off p.bas` is run
- **THEN** the compile fails with the error naming `qb64rust.toml` and exit status 1

#### Scenario: No settings file wanted
- **WHEN** the same file is there and `qb64rust --no-settings p.bas` is run
- **THEN** `p.bas` is built with every fix off, and no error names `qb64rust.toml`
