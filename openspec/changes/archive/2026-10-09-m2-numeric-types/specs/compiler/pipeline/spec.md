## ADDED Requirements

### Requirement: No follow-on errors after an unsupported declaration
After the first declaration reported "not supported yet", in file order, only "not supported yet" errors SHALL be
reported by the check of the rest of the program; syntax errors SHALL still be reported. Names the old compiler
knows but the new one does not yet (such as the constants of the auto-included files) SHALL be reported "not
supported yet", not as reserved names.

#### Scenario: Unsupported declaration
- **WHEN** a program has `DIM m AS _MEM` and then `PRINT LEN(m)`
- **THEN** the only error is the "not supported yet" error at the `DIM`

#### Scenario: Real error after an unsupported declaration
- **WHEN** a program has `DIM m AS _MEM` and, later, a statement that stores a string in a number variable
- **THEN** the only error is the "not supported yet" error at the `DIM`, and the compile fails

#### Scenario: Real error before an unsupported declaration
- **WHEN** a program has a statement that stores a string in a number variable and, later, `DIM m AS _MEM`
- **THEN** both errors are reported

#### Scenario: Auto-included constant
- **WHEN** a program contains `PRINT _TRUE`
- **THEN** the error at `_TRUE` is marked "not supported yet"

## REMOVED Requirements

### Requirement: No follow-on errors after an unsupported construct
**Reason**: Its scenarios use `DIM s AS STRING * 5` as the unsupported declaration, which this change makes
supported; a MODIFIED block cannot drop a scenario.
**Migration**: The same rule, word for word, is the added requirement "No follow-on errors after an unsupported
declaration", whose scenarios use `DIM m AS _MEM`.
