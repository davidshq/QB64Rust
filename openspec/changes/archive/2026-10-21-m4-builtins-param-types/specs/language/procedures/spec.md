## MODIFIED Requirements

### Requirement: User-type parameters
A parameter declared `name AS type` with a user type SHALL take a place of that type (a variable, an array
element, a member, the procedure's own parameter) by reference: the procedure's stores to its members change the
caller's, in a SUB and in a FUNCTION. An element with an index outside its bounds SHALL raise error 9 and the
procedure SHALL NOT be entered. An argument of another type, a number, and a place in parentheses (an internal
error of the old compiler) SHALL be compile errors. A FUNCTION SHALL NOT have a user type as its result. The parameter's `TYPE` SHALL be defined by a block above the procedure's header in the file (at the top level or in a procedure above); one defined only below it SHALL be the old compiler's error "Illegal SUB/FUNCTION parameter".

#### Scenario: Member changed in the procedure
- **WHEN** `SUB move (p AS pt)` does `p.x = p.x + 1` and is called as `move q` and as `move a(2)`
- **THEN** `q.x` and `a(2).x` are each 1 higher afterwards

#### Scenario: Another type
- **WHEN** a variable of another user type, a number, or `(q)` in parentheses is passed to `SUB move (p AS pt)`
- **THEN** it is a compile error

#### Scenario: Element outside
- **WHEN** `move a(9)` runs for `DIM a(2) AS pt` under a handler that prints `ERR` and resumes next
- **THEN** the handler prints 9 and no element changes

#### Scenario: A parameter's type defined below the procedure
- **WHEN** `SUB use (p AS late)` stands above the program's `TYPE late` block, at the top level or in a SUB below
- **THEN** it is the compile error "Illegal SUB/FUNCTION parameter" at the header, as in the old compiler

#### Scenario: A parameter's type defined in a procedure above
- **WHEN** `TYPE late` is defined in a SUB above `SUB use (p AS late)`
- **THEN** the program compiles and runs as with the old compiler
