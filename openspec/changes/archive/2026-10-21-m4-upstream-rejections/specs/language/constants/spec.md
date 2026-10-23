## ADDED Requirements

### Requirement: Malformed CONST expressions
In a `CONST` expression, an operator word (`OR`, `AND`, `XOR`, `EQV`, `IMP`, `MOD`) where an operand is expected
SHALL be a compile error, and so SHALL a name that is no constant defined above the `CONST` and none of the
auto-included file (measured, `verification\v51_x40`–`x54`). The name of a `CONST` above that this compiler did
not take SHALL stay "not supported yet", as SHALL an operator where an operand is expected outside a `CONST`.

#### Scenario: An operator as the whole expression
- **WHEN** a program has `CONST foobar = OR` (upstream `const/bad_single_arg`)
- **THEN** it is a compile error at `OR`, not "not supported yet", and its note is "Unexpected element 'OR'"

#### Scenario: An operator after NOT
- **WHEN** a program has `CONST foobar = NOT OR` (upstream `const/not_bad_identifier`)
- **THEN** it is a compile error at `OR`, not "not supported yet"

#### Scenario: An undefined name after an operator
- **WHEN** a program has `CONST foobar = 20 OR UNDEFINED` and nothing in it is named `UNDEFINED` (upstream
  `const/undefined_argument`)
- **THEN** it is a compile error at `UNDEFINED`, not "not supported yet", and its note is "Unexpected element
  'UNDEFINED'"

#### Scenario: An undefined name first
- **WHEN** a program has `CONST foobar = UNDEFINED OR 2` (upstream `const/undefined_argument_first`)
- **THEN** it is the same compile error

#### Scenario: A constant defined below
- **WHEN** `CONST a = b + 1` stands above `CONST b = 2` (`verification\v51_x45`)
- **THEN** it is a compile error at `b`, and its note is "Unexpected element 'B'"

#### Scenario: Names of other things
- **WHEN** a `CONST` names a variable, an array, a SUB, a FUNCTION or a label, or an undefined name with a suffix
  (`verification\v51_x46`–`x49`, `x52`, `x53`)
- **THEN** it is a compile error at the name, with the note "Unexpected element '…'"

#### Scenario: Outside a CONST
- **WHEN** a statement that is no `CONST` has an operator where an operand is expected (`x = OR`)
- **THEN** the diagnostic stays "not supported yet"
