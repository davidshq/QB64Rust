# Spec Delta

## MODIFIED Requirements

### Requirement: SWAP
`SWAP a, b` SHALL exchange the values of two places (variables, array elements or members) of the same type, for
every numeric type but `_BIT`, strings and user types as the old compiler accepts them: two numeric types that
differ only in signedness count as the same (the bits are exchanged), any two strings go together (`STRING` and
`STRING * n` of any length, each side cut or padded to its own length), and two user types must be the same `TYPE`.
Two places of different types SHALL be a compile error ("Type mismatch" in the old compiler), as SHALL a `_BIT`
place, an operand that is not a place and the name of the FUNCTION the statement stands in. An operand in
parentheses SHALL be the place inside them, as in the old compiler (the parentheses make no copy); two whole
arrays SHALL exchange their first elements in memory (`language/arrays-and-types`, requirement "A whole array as
a value"); a member of an array element SHALL be a place as any other member is. An element operand whose index is out of range SHALL raise error 9 and the statement SHALL
then do what the old compiler's program does.

#### Scenario: Numbers and strings
- **WHEN** `a& = 1: b& = 2: s$ = "p": t$ = "q": SWAP a&, b&: SWAP s$, t$: PRINT a&; b&; s$; t$` runs
- **THEN** it prints ` 2  1 qp`

#### Scenario: Different types
- **WHEN** `SWAP a&, d#` is compiled
- **THEN** it is a compile error

#### Scenario: Elements
- **WHEN** `DIM x(3) AS INTEGER: x(1) = 7: x(2) = 9: SWAP x(1), x(2): PRINT x(1); x(2)` runs
- **THEN** it prints ` 9  7 `

#### Scenario: Signed and unsigned
- **WHEN** `l& = -1: u~& = 2: SWAP l&, u~&: PRINT l&; u~&` runs
- **THEN** it prints ` 2  4294967295 `

#### Scenario: Strings of different kinds
- **WHEN** `DIM f AS STRING * 3: s$ = "long one": f = "abc": SWAP s$, f: PRINT "["; s$; "]["; f; "]"` runs
- **THEN** it prints `[abc][lon]`

#### Scenario: Whole TYPE variables
- **WHEN** two variables of one `TYPE` with members `n` 1 and 2 are swapped and their `n` members printed
- **THEN** it prints ` 2  1 `

#### Scenario: A _BIT variable
- **WHEN** `DIM a AS _BIT, b AS _BIT: SWAP a, b` is compiled
- **THEN** it is a compile error, as in the old compiler

#### Scenario: An operand in parentheses
- **WHEN** `a& = 1: b& = 2: SWAP (a&), b&: PRINT a&; b&` runs
- **THEN** it prints ` 2  1 `, as the old compiler's program (`verification\v22_x103`)

### Requirement: A statement with a template after CALL
A supported built-in statement whose forms are read by a template SHALL compile after `CALL` as the old compiler
compiles it: the name alone is the statement with nothing after it, and the name with one pair of parentheses
around all that follows is the statement with what stands between them. Any other form SHALL be a compile error
that is not "not supported yet".

#### Scenario: SWAP after CALL
- **WHEN** `a& = 1: b& = 2: CALL SWAP(a&, b&): PRINT a&; b&` runs
- **THEN** it prints ` 2  1 `, as the old compiler's program (`verification\v26_x128`)

#### Scenario: Statements with a rule of their own
- **WHEN** a program has `CALL CLOSE`, `CALL CLOSE(1)` or `CALL RESET`, and another has `CALL MID$(a$, 1, 1, "x")`
- **THEN** the first three compile and close the files as `CLOSE`, `CLOSE 1` and `RESET` do, and the `MID$` statement
  is the old compiler's error "Command not implemented" (`verification\v57_s12`, `s13`, `s18`, `s14`)

#### Scenario: Operands in parentheses
- **WHEN** `SWAP a&, (b&)`, `SWAP (x(1)), x(2)`, `SWAP (s$), t$`, `SWAP (a&), (b&)` and `SWAP (r.n), s.n` run
- **THEN** each exchanges the places inside the parentheses, as the old compiler's program (`verification\v57_p01`–
  `p03`, `p11`, `p12`, `p16`)

#### Scenario: The template between the parentheses
- **WHEN** `CALL CLS`, `CALL LOCATE(, 2)`, `CALL LINE((1, 2)-(3, 4), 5, BF)`, `CALL SOUND(440, 1 WAIT)` and
  `CALL PLAY(a$, b$)` are compiled
- **THEN** each compiles to the runtime call of `CLS`, `LOCATE , 2`, `LINE (1, 2)-(3, 4), 5, BF`,
  `SOUND 440, 1 WAIT` and `PLAY a$, b$`

#### Scenario: Not one pair of parentheses around all that follows
- **WHEN** a program has `CALL CLS()`, `CALL LOCATE 1, 2`, `CALL SOUND(440, 1) WAIT` or `CALL VIEW PRINT`
- **THEN** the compiler reports an error that is not "not supported yet", as the old compiler rejects each

#### Scenario: No form matches
- **WHEN** a program has `CALL PSET(1, 2)`, `CALL SOUND(440)` or `CALL PLAY`
- **THEN** the compiler reports that the statement matches none of its forms

