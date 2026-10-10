## MODIFIED Requirements

### Requirement: Diagnostics output
Each diagnostic SHALL be printed to stdout as `<file>:<line>:<column>: error: <message>`, with the file as given
on the command line and a 1-based byte column; a diagnostic marked "not supported yet" SHALL be printed as
`<file>:<line>:<column>: error: not supported yet: <message>`. A summary line with the error count SHALL follow,
saying how many of the errors are marked. A warning SHALL be printed as `<file>:<line>:<column>: warning:
<message>` only when `-w` is given, as `qb64pe` prints its warnings; warnings SHALL NOT be counted in the summary
and SHALL NOT change the exit status or stop the executable from being built.

#### Scenario: Unknown statement
- **WHEN** line 3 of `p.bas` begins at column 1 with an unsupported keyword
- **THEN** the output contains a line starting `p.bas:3:1: error: not supported yet:`

#### Scenario: Summary
- **WHEN** a program has three errors, two of them marked
- **THEN** the last line says `3 errors (2 not supported yet)`

#### Scenario: Warning shown with -w
- **WHEN** `qb64rust -w -x p.bas` compiles a program whose line 2 is `CONST c = 2 ^ 3 ^ 2`
- **THEN** the output contains a line starting `p.bas:2:` with `warning:`, the exit status is 0, and the
  executable exists

#### Scenario: Warning hidden without -w
- **WHEN** the same program is compiled with `qb64rust -q -m -x p.bas`
- **THEN** no warning is printed and the executable exists
