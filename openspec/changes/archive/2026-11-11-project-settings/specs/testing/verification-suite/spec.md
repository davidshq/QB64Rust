# Spec Delta

## MODIFIED Requirements

### Requirement: Suites build with the fixes off
The test runner SHALL build and lay out every program of every suite with `--fixes off` and `--no-settings`, and
SHALL say so in its first line, so that a `qb64rust.toml` in the tree or above it changes no result. A run with
`qb64pe.exe` SHALL pass no such option.

#### Scenario: A settings file in the tree
- **WHEN** a `qb64rust.toml` turning every fix on stands in the repository's root and the verification suite runs
  with the new compiler
- **THEN** every program is compiled with `--fixes off`, and the results are those of a run without the file

#### Scenario: A format table in the tree
- **WHEN** a `qb64rust.toml` with `[format]` and `keyword-case = "upper"` stands in the repository's root and the
  `format-corpus` suite runs with the new compiler
- **THEN** every program is laid out with `--no-settings`, and the results are those of a run without the file
