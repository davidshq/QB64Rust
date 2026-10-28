# Spec Delta

## ADDED Requirements

### Requirement: Fixes options
A compile, `-z` and `--dump` SHALL take `--fixes on|off` and `--fix <name>=on|off` as spec `compiler/fixes` says,
and SHALL read `qb64rust.toml` when neither replaces it. `qb64rust --list-fixes` SHALL be the only argument, as
`--version` is. The usage text SHALL name the three options.

#### Scenario: A dump with a fix
- **WHEN** `qb64rust --fix whole-array-value=on --dump typed p.bas` is run on a program that prints `a()`
- **THEN** it fails with the compile error of `whole-array-value` and exit status 1

#### Scenario: List with another argument
- **WHEN** `qb64rust --list-fixes p.bas` is run
- **THEN** it is a usage error with exit status 1
