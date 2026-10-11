# Spec Delta

## MODIFIED Requirements

### Requirement: Corpus layout
The corpus SHALL live in `tests/corpus/<group>/`. Each test SHALL be a `<name>.bas` with exactly one of
`<name>.output` (the program must compile, run, and print this), `<name>.err` (the program must fail to compile
with this compiler output) or `<name>.norun` (compile only; holds the reason it must not run), and MAY have a
`<name>.normalize` sidecar. A test with an `.output` MAY also have a `<name>.screen` (the state of the screen the
program must leave, spec `testing/screen-state`) and with it a `<name>.keys` (the keys it is given). Each group
SHALL have a `SOURCE.md` naming where its programs came from, the source commit and the licence.

#### Scenario: Program from QB64Fresh
- **WHEN** a reader opens `tests/corpus/runtime_comparison/SOURCE.md`
- **THEN** it names QB64Fresh's `tests/runtime_comparison/` at commit `85e6df8`, the MIT licence, `CLAUDE.md`
  rule 5, and the files that were not copied

#### Scenario: Programs are unchanged copies
- **WHEN** a corpus `.bas` from `runtime_comparison` is compared with the QB64Fresh file at `85e6df8`
- **THEN** the bytes are identical

#### Scenario: A screen program
- **WHEN** a reader opens `tests/corpus/screen/`
- **THEN** each program there has an `.output` and a `.screen`, and those that read the keyboard a `.keys`
