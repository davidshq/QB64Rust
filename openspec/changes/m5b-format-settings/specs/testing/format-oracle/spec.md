# Spec Delta

## ADDED Requirements

### Requirement: Recordings under the other layout settings
For each program named in `tests\format\settings.list`, `tests\format\` SHALL hold the old compiler's `-y` output
under six variants, each the defaults with one setting changed: `autoindent=0`, `autoindentsize=2`,
`indentsubs=0`, `autolayout=0`, `keywordcapitals=1` and `keywordlowercase=1`. Each output SHALL be kept under
`settings\<variant>\`, then the set's name and the program's path within it. A listed program SHALL have a
default recording.

#### Scenario: A listed program
- **WHEN** `settings.list` names `corpus/runtime_comparison/114_select_multiple.bas`
- **THEN** `tests\format\settings\autolayout=0\corpus\runtime_comparison\114_select_multiple.bas` holds `qb64pe -y`'s output with
  the defaults and `-f:autolayout=0`, and likewise for the five other variants

#### Scenario: A listed program the old compiler rejects
- **WHEN** `settings.list` names a program that has no default recording
- **THEN** the recording fails and names the line of the list

### Requirement: The variants recorded and compared
The format-corpus suite of `run_legacy_tests.py` SHALL record the variants with `qb64pe`, with the same rules as
the defaults and the variant's setting in place of that default. With any other compiler it SHALL compare them,
passing the variant's setting. A run limited to one set SHALL include that set's variants.

#### Scenario: Recording unchanged
- **WHEN** variants that are current are recorded again with `qb64pe`
- **THEN** no file under `tests\format\settings\` changes

#### Scenario: A variant differs
- **WHEN** `qb64rust -y -f:keywordcapitals=1` writes `Print` where `PRINT` is recorded
- **THEN** that variant of the program fails, and the result names the variant and the first line that differs

### Requirement: The variants in tier 1
`cargo test` SHALL format each variant's recordings in process with that variant's settings, and SHALL apply the
checks it applies to the default recordings: equal to the recording, the same typed tree as the source, and
unchanged when formatted again. A variant known to differ SHALL be named `settings/<variant>/<set>/<path>` in the
shrink-only list of known differences.

#### Scenario: A setting regresses
- **WHEN** a change to the formatter alters the indent of a SUB's body with `indentsubs=0`
- **THEN** `cargo test` fails and names the variant, the program and the line
