## MODIFIED Requirements

### Requirement: Syntax diagnostics
For every open document the server SHALL publish the parser's diagnostics of the program the document is, after
every change, under the uri of the file each diagnostic is in, with severity error and source `qb64rust`. It SHALL
NOT publish anything from the later stages (no "not supported yet" of `sema`, no type errors). A diagnostic the
parser itself marks "not supported yet" (a syntax error at a BASIC word, an expression nested too deep) is
published with the marker in its message. An error with the old compiler's wording (requirement "Diagnostics
output" of `compiler/cli`) SHALL have it as a second line of its message, `QB64pe: <wording>`.

#### Scenario: Error appears and goes
- **WHEN** the user types `x = 1 +` on a line and then completes it to `x = 1 + 2`
- **THEN** a diagnostic appears on that line after the first edit and is gone after the second, without a save

#### Scenario: Unsupported statement is no diagnostic
- **WHEN** an open document contains `OPEN "f" FOR OUTPUT AS #1`, which the compiler does not compile yet
- **THEN** the server publishes no diagnostic for that line

#### Scenario: Error in an included file
- **WHEN** the open main file includes `lib.bi` and `lib.bi` has a syntax error on line 3
- **THEN** the diagnostic is published under `lib.bi`'s uri at line 3, and none under the main file for it

#### Scenario: The old compiler's wording
- **WHEN** a parser error the table holds is published
- **THEN** its message is the error's, a line end, and `QB64pe: ` with the old compiler's wording
