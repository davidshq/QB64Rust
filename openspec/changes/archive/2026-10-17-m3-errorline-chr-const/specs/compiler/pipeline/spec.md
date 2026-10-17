# Spec Delta

## MODIFIED Requirements

### Requirement: No follow-on errors after an unsupported declaration
After the first declaration reported "not supported yet", in file order, only "not supported yet" errors SHALL be
reported by the check of the rest of the program; syntax errors SHALL still be reported. Names the old compiler
knows but the new one does not yet (such as the constants of the auto-included files) SHALL be reported "not
supported yet", not as reserved names.

#### Scenario: Unsupported declaration
- **WHEN** a program has `CONST c = EXP(1)` and then `PRINT c`
- **THEN** the only error is the "not supported yet" error at the `CONST`

#### Scenario: Real error after an unsupported declaration
- **WHEN** a program has `CONST c = EXP(1)` and, later, a statement that stores a string in a number variable
- **THEN** the only error is the "not supported yet" error at the `CONST`, and the compile fails

#### Scenario: Real error before an unsupported declaration
- **WHEN** a program has a statement that stores a string in a number variable and, later, `CONST c = EXP(1)`
- **THEN** both errors are reported

#### Scenario: Auto-included constant
- **WHEN** a program contains `PRINT _TRUE`
- **THEN** the error at `_TRUE` is marked "not supported yet"
