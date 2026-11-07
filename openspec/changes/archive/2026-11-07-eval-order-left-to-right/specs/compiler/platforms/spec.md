## MODIFIED Requirements

### Requirement: A program behaves as QB64pe's on its platform
A program built on Linux SHALL behave as the same program built by QB64pe 4.7.0 at the reference commit on Linux,
apart from the rows of `QB64PE-DIVERGENCES.md`. Where QB64pe's Linux and Windows builds differ, the Linux build
SHALL follow QB64pe's Linux build, with one exception. Where they differ only in the order in which the parts of
one expression, call or statement are evaluated, the Linux build SHALL follow the Windows build's order (spec
`language/evaluation-order`). `_OS$` SHALL report `[LINUX][64BIT]`.

#### Scenario: Comma zones on the Linux console
- **WHEN** a `$CONSOLE:ONLY` program prints `1, 2` on Linux
- **THEN** the items are separated as QB64pe's Linux build separates them (D-011), not in 10-column zones

#### Scenario: The platform's name
- **WHEN** a program prints `_OS$` on Linux x64
- **THEN** it prints `[LINUX][64BIT]`

#### Scenario: The order of calls on Linux
- **WHEN** a program built on Linux prints `f(1) - f(2)` for a FUNCTION `f` that adds 1 to a SHARED counter and
  returns it, with the counter at 0
- **THEN** it prints `-1`, as QB64pe's Windows build does, where QB64pe's Linux build prints `1`
