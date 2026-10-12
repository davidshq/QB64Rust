# Spec Delta

## Purpose

The default type of a name that carries neither a type suffix nor an `AS` clause: SINGLE, or what `DEFINT`,
`DEFLNG`, `DEFSNG`, `DEFDBL`, `DEFSTR` and `_DEFINE` set for its first letter, with the old compiler's observed
behaviour (`verification\v24_a_*`).

## ADDED Requirements

### Requirement: Default type by first letter
`DEFINT`, `DEFLNG`, `DEFSNG`, `DEFDBL` and `DEFSTR`, each followed by letters and letter ranges (`A-Z`, `i`,
`a-c, x`), SHALL set the default type of names beginning with those letters to INTEGER, LONG, SINGLE, DOUBLE and
`STRING`. Letters SHALL be compared without regard to case; a range MAY be written backwards (`z-w`); `_` SHALL be
a letter of its own, for names that start with an underscore, after Z in a range (`y-_`). A name with a type
suffix or declared with `AS` SHALL keep that type. Without any such statement the default type SHALL be SINGLE.
A statement without letters, with anything but single letters, `-` and `,`, or with an `AS` clause SHALL be a
compile error.

#### Scenario: DEFINT A-Z
- **WHEN** `DEFINT A-Z: x = 2.5: PRINT x; x / 2` runs
- **THEN** it prints ` 2  1 ` (`x` is an INTEGER; the division of an INTEGER prints as SINGLE)

#### Scenario: A range and single letters
- **WHEN** `DEFLNG a-c, x` is followed by `b = 100000: d = 1.5: x = 70000: PRINT b; d; x`
- **THEN** it prints ` 100000  1.5  70000 `

#### Scenario: Suffix and AS win
- **WHEN** `DEFINT A-Z` is followed by `DIM d AS DOUBLE: d = 1.5: s! = 2.5: n$ = "q": PRINT d; s!; n$`
- **THEN** it prints ` 1.5  2.5 q`

#### Scenario: DEFSTR
- **WHEN** `DEFSTR S: s = "text": PRINT s + "!"` runs
- **THEN** it prints `text!`

#### Scenario: The underscore entry
- **WHEN** `DEFINT A-Z: __a = 2.5: PRINT __a` runs, and in another program `DEFINT _, a: __a = 2.5: PRINT __a`
- **THEN** the first prints ` 2.5 ` and the second ` 2 `

#### Scenario: Malformed statement
- **WHEN** `DEFINT`, `DEFINT 1`, `DEFINT ab`, `DEFINT a-`, `DEFINT a b`, `DEFINT a,` or `DEFINT a AS LONG` is
  compiled
- **THEN** each is a compile error

### Requirement: Default types apply from the statement on
A default-type statement SHALL apply to every line below it in the file, whether or not the statement is reached
when the program runs, and whatever block or procedure it stands in: one inside a SUB or FUNCTION SHALL go on
applying after the procedure ends, to the procedures and main-module lines below it. A procedure above the
statement SHALL keep the earlier default for its parameters, its result and its variables. A later statement for
the same letter SHALL replace the earlier one. A name without a suffix SHALL be the name with the default type at
its line: a variable made under another default SHALL NOT change and SHALL NOT be the variable such a name means
after the statement, unless it was declared with `AS`, which is found by its bare name whatever the default.

#### Scenario: Before and after
- **WHEN** `a = 1.5: DEFINT A-Z: b = 1.5: PRINT a; b; a!` runs
- **THEN** it prints ` 0  2  1.5 ` (after the statement `a` is `a%`, a new variable)

#### Scenario: Declared with AS
- **WHEN** `DIM d AS LONG: d = 100000: DEFINT A-Z: PRINT d` runs
- **THEN** it prints ` 100000 `

#### Scenario: A later statement replaces an earlier one
- **WHEN** `DEFINT A-Z: DEFDBL D: i = 1.5: d = 1.5: PRINT i; d` runs
- **THEN** it prints ` 2  1.5 `

#### Scenario: A statement that does not run
- **WHEN** `GOTO skip`, `DEFLNG G`, `skip: g = 2.5: PRINT g; LEN(g)` stand on three lines
- **THEN** it prints ` 2  4 `

#### Scenario: Procedure after the statement
- **WHEN** `DEFINT A-Z` stands above `SUB s: v = 2.5: PRINT v: END SUB` and the main module calls `s`
- **THEN** it prints ` 2 `

#### Scenario: Procedure before the statement
- **WHEN** `SUB sa (p): v = 2.5: PRINT LEN(p); LEN(v): END SUB` stands above `DEFDBL A-Z` and is called below it
- **THEN** it prints ` 4  4 ` (both SINGLE)

#### Scenario: A statement inside a procedure
- **WHEN** `SUB s1` holds `DEFINT A-Z`, and below `END SUB` the main module has `w = 2.5: PRINT LEN(w)`
- **THEN** it prints ` 2 `

### Requirement: What a default type gives its type to
The default type SHALL be the type of every name that may be written without a suffix and has no `AS` clause: a
variable used or declared with `DIM`, `STATIC`, `SHARED`, `COMMON` or `REDIM`, an array, a `FOR` variable, a
parameter, and the result of a FUNCTION. It SHALL NOT type a `CONST`, which takes the type of its value. A
FUNCTION whose result is typed by a default SHALL be callable with the matching suffix and, whatever the default
where the call stands, without one; a call with another suffix SHALL be a compile error (the old compiler: "Name
already in use"). In a procedure a plain name SHALL mean the procedure's own variable of that name and the
default type where there is one (a parameter, a local variable), also where a main-module `DIM SHARED … AS` types
that name; where there is none (another type, or a `DEFxxx` in the body has changed the default) it SHALL mean the
shared variable. A main-module variable bound by `SHARED` is not one of the procedure's own.

#### Scenario: Parameter named like a shared variable
- **WHEN** `DIM SHARED x AS LONG: x = 7` is followed by `PRINT f(1.5)` and `FUNCTION f (x): f = x * 2: END FUNCTION`
- **THEN** it prints ` 3 `

#### Scenario: Array and parameter
- **WHEN** `DEFINT A-Z` is followed by `DIM t(3): t(1) = 2.5: PRINT t(1)` and by `SUB p (n): PRINT n / 2: END SUB`
  called as `p 5`
- **THEN** it prints ` 2 ` and ` 2.5 `

#### Scenario: FUNCTION result
- **WHEN** `DEFLNG F` stands above `FUNCTION f (x): f = x * 100000: END FUNCTION` and the main module prints
  `f(3)` and `f&(3)`
- **THEN** it prints ` 300000 ` twice

#### Scenario: FUNCTION called where the default has changed
- **WHEN** `DEFLNG F`, then `DEFSNG F: PRINT f(3)`, then `DEFLNG F` stand above that FUNCTION
- **THEN** it prints ` 300000 `, and `PRINT f!(3)` in its place is a compile error

#### Scenario: CONST
- **WHEN** `DEFSTR S: CONST s1 = 5: PRINT s1 + 1` runs
- **THEN** it prints ` 6 `

#### Scenario: FOR variable typed STRING
- **WHEN** `DEFSTR S: FOR s = 1 TO 3: NEXT` is compiled
- **THEN** it is a compile error

### Requirement: _DEFINE
`_DEFINE letters AS type` SHALL set the default type of those letters, by the same rules as the `DEFxxx`
statements, to any type a variable may have by a suffix: a numeric type with or without `_UNSIGNED`, `_OFFSET`,
`_FLOAT`, `_BIT` and `_BIT * n`, `STRING`, and `STRING * n` with n above 0. Any other type SHALL be a compile
error (the old compiler: "Cannot convert type (…) to symbol"): a user type, `_MEM`, an unknown name, `_UNSIGNED`
alone or before `STRING`, SINGLE, DOUBLE or `_FLOAT`, and `STRING * 0`. A name typed this way SHALL follow the
rules of that type where it stands (a `_BIT` parameter is a compile error, `language/procedures`).

#### Scenario: Unsigned default
- **WHEN** `_DEFINE u AS _UNSIGNED _BYTE: u = -1: PRINT u` runs
- **THEN** it prints ` 255 `

#### Scenario: Fixed-length string default
- **WHEN** `_DEFINE a AS STRING * 5: a1 = "abcdefgh": PRINT "["; a1; "]"; LEN(a1)` runs
- **THEN** it prints `[abcde] 5 `

#### Scenario: A type that cannot be a default
- **WHEN** `_DEFINE a AS rec` (a user type), `_DEFINE a AS _MEM`, `_DEFINE a AS _UNSIGNED STRING`,
  `_DEFINE a AS _UNSIGNED SINGLE` or `_DEFINE a AS STRING * 0` is compiled
- **THEN** each is a compile error
