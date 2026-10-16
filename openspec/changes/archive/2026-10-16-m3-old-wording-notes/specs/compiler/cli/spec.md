## MODIFIED Requirements

### Requirement: Diagnostics output
Each diagnostic SHALL be printed to stdout as `<file>:<line>:<column>: error: <message>`, with the file as given
on the command line and a 1-based byte column; a diagnostic marked "not supported yet" SHALL be printed as
`<file>:<line>:<column>: error: not supported yet: <message>`. An error not so marked whose wording by the old
compiler is in the table made from recorded output SHALL be followed by `<file>:<line>:<column>: note: QB64pe:
<that wording>`, the names and numbers of this error filled in. A summary line with the error count SHALL follow,
saying how many of the errors are marked. A warning SHALL be printed as `<file>:<line>:<column>: warning:
<message>` only when `-w` is given, as `qb64pe` prints its warnings; warnings SHALL NOT be counted in the summary
and SHALL NOT change the exit status or stop the executable from being built.

#### Scenario: Unknown statement
- **WHEN** line 3 of `p.bas` begins at column 1 with an unsupported keyword
- **THEN** the output contains a line starting `p.bas:3:1: error: not supported yet:`

#### Scenario: The old compiler's wording
- **WHEN** line 9 of `p.bas` is `m = _MEM()`
- **THEN** the error `p.bas:9:5: error: `_MEM` with empty parentheses` is followed by `p.bas:9:5: note: QB64pe:
  Expected (...)`

#### Scenario: A name in the old wording
- **WHEN** `OPTION _EXPLICIT` is set and line 2 is `count = 1`
- **THEN** the note reads `QB64pe: Variable 'count' (SINGLE) not defined`, with the name the error gives

#### Scenario: A form no recording shows
- **WHEN** line 3 is `count& = 1`, and every recorded pair of that message had a name without a type suffix
- **THEN** the error has no note (the old compiler drops the suffix there, which no recording showed)

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
