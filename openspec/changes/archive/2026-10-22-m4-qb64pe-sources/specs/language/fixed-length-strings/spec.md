## ADDED Requirements

### Requirement: A suffixed name of a fixed-length string array
`name$(i)` SHALL be the element of the array of that name declared `AS STRING * n`, as a value and as a place,
and `name$` SHALL be that array where a whole array is named (`UBOUND(name$)`), in the main module and in a
procedure that shares it (`verification\v52_x40`–`x48`; QB64pe `source\utilities\hash.bas` relies on it). It
SHALL NOT make another, implicit array. `DIM name$(n)` beside such an array SHALL be a compile error (`v52_x45`).

#### Scenario: The $ name reads the element
- **WHEN** `REDIM a(1 TO 3) AS STRING * 4: a(2) = "ab": PRINT "["; a$(2); "]"; UBOUND(a$)` runs
- **THEN** it prints `[ab  ] 3`

#### Scenario: In a SUB, the shared array
- **WHEN** `DIM SHARED h(5) AS STRING * 8` is declared, and a SUB runs `h(1) = "x": PRINT LEN(h$(1))`
- **THEN** it prints ` 8 `

#### Scenario: The $ name as a place
- **WHEN** `DIM a(3) AS STRING * 2: a$(1) = "zzz": PRINT "["; a(1); "]"` runs
- **THEN** it prints `[zz]`

#### Scenario: An array of its own beside it
- **WHEN** a program has `DIM a(3) AS STRING * 4` and then `DIM a$(3)`
- **THEN** it is a compile error

### Requirement: A suffixed name of a fixed-length string variable
`name$` SHALL be the variable of that name declared `AS STRING * n`, as a value and as a place, in the main module
and in a procedure that shares it (`verification\v52_x39`, `x43`, `x44`; QB64pe `source\utilities\file.bas`
relies on it for `pathsep$`).

#### Scenario: The $ name reads and sets the variable
- **WHEN** `DIM s AS STRING * 4: s = "ab": PRINT "["; s$; "]": s$ = "x": PRINT "["; s; "]"` runs
- **THEN** it prints `[ab  ]` and `[x   ]`

#### Scenario: In a SUB, the shared variable
- **WHEN** `DIM SHARED pathsep AS STRING * 1` is declared, and a SUB runs `pathsep$ = "\": PRINT LEN(pathsep$)`
- **THEN** it prints ` 1 `, and `pathsep` holds `\` after the call

### Requirement: Declaring the $ name of a fixed-length string variable
`DIM name$`, `REDIM name$` and, in a procedure, `STATIC name$` beside a variable `name` declared `AS STRING * n`
SHALL be accepted and SHALL declare nothing new, whichever of `DIM` and `STATIC` declared `name`: `name$` stays the
fixed-length variable (`verification\v52_x49`, `x69`, `x70`, `x73`, `x74`). `SHARED name$` in a procedure SHALL bind the main module's variable-length `name$`, not the
fixed-length variable (`x71`).

#### Scenario: DIM of the $ name beside it
- **WHEN** `DIM s AS STRING * 4: DIM s$: s = "f": s$ = "v": PRINT "["; s; "]["; s$; "]"` runs
- **THEN** it prints `[v   ][v   ]`

#### Scenario: REDIM of the $ name beside it
- **WHEN** `DIM s AS STRING * 4: REDIM s$: s$ = "abcdefg": PRINT "["; s; "]["; s$; "]"` runs
- **THEN** it prints `[abcd][abcd]`
