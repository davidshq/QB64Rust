## ADDED Requirements

### Requirement: Parts are evaluated in the Windows build's order on every platform
The parts of an expression and the arguments of a call or statement SHALL be evaluated in the order that QB64pe
4.7.0's Windows build evaluates them in, on every platform. That is the order of QB64pe's C++, which is not
always the BASIC order. This covers:
- the arguments of a SUB, a FUNCTION, a library procedure, a built-in function or a built-in statement;
- the operands of an operator;
- the indexes of one element;
- the target and the value of a string assignment.

#### Scenario: A built-in statement's arguments
- **WHEN** `FIELD #fn, wd AS e(ix)` is run with FUNCTIONs `fn`, `wd` and `ix` that each print their name
- **THEN** it prints `fn ix wd` on Windows and on Linux

### Requirement: Which parts the order applies to
The order SHALL be kept wherever another order could change what the program does:
- one part calls a FUNCTION, a library procedure or a built-in that changes program state, and another part is
  not a constant; or
- two parts can each raise an error, unless both can raise only error 9 from index checks.

#### Scenario: Two FUNCTION calls as operands
- **WHEN** a program has a FUNCTION `f` that adds 1 to a SHARED counter and returns it, and runs
  `PRINT f(1) - f(2)` with the counter at 0
- **THEN** it prints `-1` on Windows and on Linux

#### Scenario: A variable read and a FUNCTION that changes it
- **WHEN** `x` is 1 and a FUNCTION `g` sets `x` to 10 and returns 0, and the program prints `x + g`
- **THEN** it prints `1` on Windows and on Linux

#### Scenario: Two errors in one call's arguments
- **WHEN** a SUB is called as `two x(9), ASC(empty$)`, where `x` has no element 9 and `empty$` is `""`, under an
  `ON ERROR` handler that prints `ERR`
- **THEN** the handler reports error 9 on Windows and on Linux

#### Scenario: A built-in function's arguments
- **WHEN** a program prints `_MIN(pa, pb)` where FUNCTIONs `pa` and `pb` each print their name
- **THEN** it prints `(a) (b)` before the result, on Windows and on Linux

### Requirement: Orders the C++ already fixes stay as they are
A part that QB64pe's Windows build evaluates only when needed SHALL still be evaluated only when needed: the
right side of `_ANDALSO` and `_ORELSE`, and the branch of `_IIF` that is not chosen. Each item of a `PRINT`,
`WRITE` or `INPUT` SHALL still be evaluated after the previous item is done. A store into an element of an array
SHALL evaluate the element's indexes before the value, as a step of its own. A number stored through a member with
an index on the way (`r(i).n = v`) SHALL be evaluated before that index, as both of QB64pe's builds do.

#### Scenario: `_IIF` with FUNCTIONs in both branches
- **WHEN** a program evaluates `_IIF(c, f(1), f(2))` with `c` true, where `f` prints its argument
- **THEN** it prints `1` only

#### Scenario: A store into an element: the index first
- **WHEN** a program runs `a(f(1)) = f(2)`, where `f` prints its argument and returns it
- **THEN** it prints `1` before `2`, on Windows and on Linux

#### Scenario: A number stored through a member: the value first
- **WHEN** a program runs `r(f(1)).n = f(2)`, where `r` is an array of a `TYPE` with a `LONG` member `n`
- **THEN** it prints `2` before `1`, on Windows and on Linux

#### Scenario: Two index checks
- **WHEN** a program prints `a(i) + b(j)` with both indexes out of range under an `ON ERROR` handler
- **THEN** the handler reports error 9 once, as QB64pe's Windows build does
