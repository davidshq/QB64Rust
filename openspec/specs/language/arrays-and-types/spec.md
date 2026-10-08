# language/arrays-and-types Specification

## Purpose
Static arrays of the main module, their elements, `LBOUND`/`UBOUND`, user types (`TYPE`) with numeric, string and
nested members, and member access, with the old compiler's observed behaviour. Every rule is measured with
`qb64pe.exe` (`verification\v18_*`); the scenarios are tests in `tests\corpus\slice\` or `tests\frontend\`.

## Requirements

### Requirement: Static arrays
`DIM name(bounds) [AS type]` and `DIM SHARED name(bounds) [AS type]` in the main module, with bounds that are
constant expressions, SHALL declare an array of the slice's numeric types, of `STRING` or of a user type: each
dimension is `upper` (lower bound 0) or `lower TO upper`, each bound rounded half to even to a whole number; every
element starts at zero or as an empty string. The `DIM` line SHALL do nothing when it runs. An array SHALL be a name
plus a type, in a name space of its own beside scalar variables. An upper bound below its lower bound, and a second
`DIM` of the same array, SHALL be compile errors. Every other way to make an array (bounds that are not constant,
`DIM` of an array inside a SUB or FUNCTION, `REDIM`, `STATIC`, `OPTION BASE`, `$DYNAMIC`, `SHARED name()`, array
parameters, and an array used without `DIM`) SHALL be reported "not supported yet".

#### Scenario: Bounds
- **WHEN** `DIM b(-2 TO 3, 5) AS LONG` is declared and `b(-2, 0)` and `b(3, 5)` are assigned and printed
- **THEN** both values are printed, and every other element is 0

#### Scenario: Float bounds
- **WHEN** `DIM a(2.5)` is declared
- **THEN** `PRINT LBOUND(a); UBOUND(a)` prints `0  2`

#### Scenario: Array and scalar of the same name
- **WHEN** `DIM a(3)`, `a = 5` and `a(1) = 2` run
- **THEN** `PRINT a; a(1)` prints `5  2`

#### Scenario: DIM run twice
- **WHEN** `DIM a(5) AS LONG` stands in a `FOR` body that sets `a(1) = a(1) + 1` and runs twice
- **THEN** `a(1)` is 2 after the loop

#### Scenario: Array in a SUB
- **WHEN** `DIM t(5)` stands inside a SUB
- **THEN** the compiler reports it "not supported yet"

#### Scenario: Array used without DIM
- **WHEN** `z(3) = 4` is compiled and no array `z` is declared
- **THEN** the compiler reports it "not supported yet"

### Requirement: Array elements
`name(i, …)` SHALL name an element of an array visible at that point, with one index per dimension; each index
SHALL be converted to a 64-bit integer as for an assignment (a float rounded half to even). An index outside its
own dimension's bounds SHALL raise error 9, and a read with such an index SHALL give the value of the array's first
element. A wrong number of indexes and a string index SHALL be compile errors. An element SHALL be readable,
assignable, and passable by reference to a parameter of its exact type; in parentheses or to a parameter of another
type it SHALL be passed as a copy. An element SHALL NOT be a `FOR` variable (compile error).

#### Scenario: Index out of range
- **WHEN** `x(11) = 5` runs for `DIM x(10)` under an error handler that prints `ERR` and resumes next
- **THEN** the handler prints 9 and no element of `x` changes

#### Scenario: Each dimension checked
- **WHEN** `PRINT m(1, 4)` runs for `DIM m(2, 3)` under the same handler
- **THEN** the handler prints 9

#### Scenario: Float index
- **WHEN** `x(i) = i * 10` for `i` from 0 to 10 and `PRINT x(1.5); x(2.5); x(-0.5)` runs
- **THEN** it prints `20  20  0`

#### Scenario: Read with a bad index
- **WHEN** `x(0) = 77: y = x(11)` runs under the same handler
- **THEN** the handler prints 9 and `y` is 77

#### Scenario: Element passed by reference
- **WHEN** `SUB bump (v AS LONG)` adds 1 to `v` and is called as `bump x(3)` for a LONG array `x`
- **THEN** `x(3)` is 1 higher afterwards; called as `bump (x(3))` it is unchanged

#### Scenario: Element with a bad index passed to a SUB
- **WHEN** `bump x(9)` runs for `DIM x(5) AS LONG` under the same handler
- **THEN** the handler prints 9 and `bump` does not run

#### Scenario: Wrong number of indexes
- **WHEN** `x(1, 2) = 5` is compiled for `DIM x(10)`
- **THEN** the compiler reports an error

### Requirement: LBOUND and UBOUND
`LBOUND(array[, dimension])` and `UBOUND(array[, dimension])` SHALL give the lower and upper bound of a dimension
of an array (the first without `dimension`), typed `_INTEGER64`. The dimension SHALL be rounded half to even; a
dimension outside 1 to the array's number of dimensions SHALL raise error 9 when it runs. A name that is no array
SHALL be reported "not supported yet" (the old compiler makes an implicit array); `LBOUND(a())` SHALL be a compile
error.

#### Scenario: Both bounds
- **WHEN** `PRINT LBOUND(b); UBOUND(b); LBOUND(b, 2); UBOUND(b, 2)` runs for `DIM b(-2 TO 3, 5)`
- **THEN** it prints `-2  3  0  5`

#### Scenario: Typed _INTEGER64
- **WHEN** `PRINT UBOUND(b) / 7` runs for the same array
- **THEN** it prints `.4285714285714285`

#### Scenario: Dimension out of range
- **WHEN** `PRINT LBOUND(b, 3)` runs for the same array under a handler that prints `ERR` and resumes next
- **THEN** the handler prints 9

### Requirement: User types
A `TYPE name … END TYPE` block in the main module SHALL define a user type whose members are of the slice's numeric
types or of user types defined in earlier blocks; the program's types SHALL be known before its statements (a type
may be used above its block). `DIM v AS name` SHALL declare a variable of that type, in every storage class a scalar
variable may have; every member starts at zero, a local one on each call. Members that are arrays, `STRING`,
fixed-length strings, unsigned or `_BIT` types, and a `TYPE` block inside a SUB or FUNCTION, SHALL be reported "not
supported yet"; a member declared with a type suffix SHALL be a compile error. A whole user-type value in an
expression, a `PRINT` item or stored into a scalar, and a number stored into a user-type variable, SHALL be compile
errors; an assignment of one user-type variable to another and a user-type argument or parameter SHALL be reported
"not supported yet".

#### Scenario: Members of each type
- **WHEN** a type with an INTEGER, a LONG, an `_INTEGER64`, a SINGLE, a DOUBLE and a `_FLOAT` member is declared,
  each member assigned and printed
- **THEN** each value is printed as a variable of that type would be

#### Scenario: Nested type
- **WHEN** `TYPE inner: v AS LONG: END TYPE` (as a block) and `TYPE outer: i AS inner: END TYPE` are declared, `DIM
  o AS outer` and `o.i.v = 7` runs
- **THEN** `PRINT o.i.v` prints `7`

#### Scenario: Type used before its block
- **WHEN** `DIM p AS pt` stands above `TYPE pt … END TYPE`
- **THEN** the program compiles and `p` has the type's members

#### Scenario: Local user-type variable
- **WHEN** a SUB with `DIM p AS pt` prints `p.x` and then sets it to 5, and is called twice
- **THEN** it prints `0` both times

#### Scenario: Whole value in PRINT
- **WHEN** `PRINT p` is compiled for a user-type variable `p`
- **THEN** the compiler reports an error

### Requirement: Member access and dotted names
`a.b` SHALL be a member access when a scalar variable `a` of a user type is visible at that point, and a plain
variable named `a.b` otherwise, also before a later `DIM a AS t` (measured, `verification/v16_m8_*`,
`v18_h_dotted_before_dim`); when an array `a` of a user type is visible and no such scalar, `a.b` SHALL be a compile
error. A member that its type does not have, a member written with another type's suffix, and a member of an
element of a numeric array SHALL be compile errors. `a(i).m` SHALL be the member `m` of element `a(i)`. A member SHALL
be readable, assignable, and passable by reference to a parameter of its exact type.

#### Scenario: Dotted name without a TYPE
- **WHEN** `a.b = 3: a = 4: PRINT a.b; a` runs and no user-type variable `a` exists
- **THEN** it prints `3  4`

#### Scenario: Dotted name before the DIM
- **WHEN** `a.b = 3: PRINT a.b` runs, then `DIM a AS t` (a type with a LONG member `b`), then `PRINT a.b`
- **THEN** it prints `3`, then `0`

#### Scenario: Unknown member
- **WHEN** `a.c = 6` is compiled for a variable `a` of a type without a member `c`
- **THEN** the compiler reports an error

#### Scenario: Member of an element
- **WHEN** `a(1).b = 8` runs for `DIM a(2) AS t`
- **THEN** `PRINT a(1).b; a(2).b` prints `8  0`

#### Scenario: Member passed by reference
- **WHEN** `bump p.x` is called for a LONG member `x`
- **THEN** `p.x` is 1 higher afterwards
