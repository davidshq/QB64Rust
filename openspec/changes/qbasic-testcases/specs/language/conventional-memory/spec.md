# Spec Delta

## MODIFIED Requirements

### Requirement: What VARPTR and VARSEG accept
An argument of `VARPTR` or `VARSEG` that is not a variable or an array element (a literal, an expression, a
function call) SHALL be a compile error. `VARPTR` and `VARSEG` of a variable, array or parameter of a SUB or
FUNCTION SHALL be taken as those of the main module are (requirements "A procedure's variables in conventional
memory" and "Parameters whose address is taken"), and `VARPTR` and `VARSEG` of a whole array `x()` that
is not of numbers SHALL be reported "not supported yet". `VARPTR` of a
`_BIT` array SHALL be the old compiler's error "VARPTR cannot reference _BIT type arrays", and `VARPTR` of a whole
`TYPE` variable a compile error (the old compiler fails there, `verification\v56_t02`). `VARPTR` and `VARSEG` of a
fixed-length string member SHALL be those of its bytes, as of a number member (`verification\v61_a4`). `VARPTR` of the `STRING`
member of an array element and `VARPTR$` of a `STRING` member SHALL be compile errors (the old compiler's C++
build fails, `v60_d7`, `d8`). Every other variable,
element and member of the main module SHALL be taken (requirement "Variables in conventional memory"), `_BIT`
scalars included. A name alone that is an array of strings, of fixed-length strings or of a `TYPE` SHALL name the
scalar of that name, as in the old compiler.

#### Scenario: Not a variable
- **WHEN** a program has `l& = VARPTR(5)` or `l& = VARSEG(i% + 1)`
- **THEN** the compiler reports an error that is not "not supported yet"

#### Scenario: A scalar
- **WHEN** a program of the main module has `l& = VARPTR(i%)`, `VARSEG(d#)`, `VARPTR(t$)`, `VARPTR(fx)` for a
  `STRING * 8`, `VARPTR(r(1))` for an array of a `TYPE`, `VARPTR(one.y)` for a member, or `VARSEG(one)`
- **THEN** it compiles with no diagnostic (`verification\v28_x01_accepted`, `v56_t01`, `t03`)

#### Scenario: A whole TYPE variable
- **WHEN** a program has `l& = VARPTR(one)` for a `TYPE` variable `one`
- **THEN** it is a compile error that is not "not supported yet"

#### Scenario: A variable of a SUB
- **WHEN** a SUB has `DIM k AS INTEGER: PRINT VARPTR(k)`
- **THEN** it compiles with no diagnostic

#### Scenario: A TYPE that holds a string
- **WHEN** a program has `PRINT VARPTR(x.v)` or `VARPTR(x.n)` for `x` of a `TYPE` with a `STRING` member `n`
- **THEN** it compiles with no diagnostic

#### Scenario: A STRING member where the old build fails
- **WHEN** a program has `VARPTR(e(2).n)` for an array `e` of that `TYPE`, or `VARPTR$(x.n)`
- **THEN** it is a compile error that is not "not supported yet"

## ADDED Requirements

### Requirement: A procedure's variables in conventional memory
A variable of a SUB or FUNCTION that `VARPTR`, `VARSEG`, `SADD` or `VARPTR$` names SHALL live in libqb's
conventional memory: a local one allocated on each call below what is in use and given back when the procedure
ends, a `STATIC` one allocated once when the program starts, after the main module's, and kept; its arrays
likewise. Offsets, segments and the bytes `PEEK` and `POKE` reach SHALL be the old compiler's
(`verification\v61`).

#### Scenario: Locals on each call
- **WHEN** a SUB with `DIM k AS INTEGER, l AS LONG` and `STATIC st AS INTEGER` prints `VARPTR(k); VARSEG(k);
  VARPTR(l); VARPTR(st)` and `PEEK(VARPTR(k))` under `DEF SEG = VARSEG(k)`, and is called twice
- **THEN** each call prints what the old compiler's program prints (` 65516  80  65512  65518 ` with nothing else
  in conventional memory), and the `PEEK` gives the low byte of `k`

#### Scenario: Recursion
- **WHEN** a recursive FUNCTION prints `VARPTR(z)` of its local `z AS INTEGER` at each depth
- **THEN** each depth's offset is 2 below its caller's, as with the old compiler

#### Scenario: A string of a FUNCTION
- **WHEN** a FUNCTION prints `SADD(m$)` and `VARPTR(m$)` of its local string `m$`
- **THEN** it prints what the old compiler's program prints

#### Scenario: Arrays of a procedure
- **WHEN** a SUB with `DIM w(1 TO 3) AS INTEGER` and `STATIC v(1 TO 2) AS LONG` sets `w(1) = 5` and prints
  `VARPTR(w(2)); PEEK(VARPTR(w(1))); VARPTR(v(2))` under `DEF SEG = VARSEG(w(1))`
- **THEN** it prints ` 2  5  4 `, as with the old compiler

### Requirement: Parameters whose address is taken
When `VARPTR`, `VARSEG`, `SADD` or `VARPTR$` names a parameter, a member of one or an element of one, the
argument of every call SHALL be in conventional memory: a variable or array of the caller moves there, a `TYPE` or
array parameter passed on carries the need to its own callers, and a numeric expression gets a slot of its call
site, placed among the variables of its body in source order. `VARPTR` and `VARSEG` of a scalar parameter, and
`VARSEG` of a place in a `TYPE` parameter, SHALL be those of where its argument lies; `VARPTR` of a place in a
`TYPE` parameter, and `VARPTR$` of a parameter, SHALL be its offset from libqb's `dblock` cut to 16 bits, as the
old compiler's, wherever the argument lies (`verification\v61_p9`, `v61_p10`).

#### Scenario: A TYPE parameter through a chain of calls
- **WHEN** the main module passes `DIM s AS st` to a SUB `outer (x AS st)` that passes `x` to `inner (y AS st)`,
  and `inner` prints `VARPTR(y.b); VARSEG(y.b)` and `POKE`s a byte of `y.b`
- **THEN** it prints what the old compiler's program prints, and `s.b` holds the poked byte after the call

#### Scenario: An array parameter
- **WHEN** a SUB `fill (z() AS INTEGER)` prints `VARPTR(z(2))` and `PEEK(VARPTR(z(2)))` under `DEF SEG =
  VARSEG(z(1))`, and is called with a static array, a dynamic one and an array of another SUB
- **THEN** each call prints what the old compiler's program prints, and a `POKE` through it reaches the caller's
  array

#### Scenario: A TYPE parameter called with an element of a TYPE array
- **WHEN** a SUB `s (p AS rec)`, `rec` with a member array `m(5) AS INTEGER`, prints `VARSEG(p.m(2)); VARPTR(p.m(2))`
  and pokes 42 there under `DEF SEG = VARSEG(p.m(2))`, and is called as `s ra(1)` for an array `ra` of `rec`
- **THEN** it prints the segment nearest the element and its offset from `dblock` (4177 and 20), as the old
  compiler's program does, and `ra(1).m(2)` is unchanged: the poke misses it, as with the old compiler

#### Scenario: An expression argument
- **WHEN** a SUB `t (p AS INTEGER)` prints `VARPTR(p); VARSEG(p)`, and is called as `t 3`, `t (x%)` and `t x% + 1`
- **THEN** each call site prints the offset of its own slot, as with the old compiler, and `x%` is unchanged

#### Scenario: An element argument
- **WHEN** `t a(2)` is called for an `INTEGER` array `a`, and `t` prints `VARSEG(p)` and pokes 42 through `p`
- **THEN** it prints the array's segment, and `a(2)` holds 42 after the call, as with the old compiler

#### Scenario: Slots among variables
- **WHEN** the main module has `DIM a AS INTEGER`, then `t 3`, then `DIM b AS INTEGER`, and prints `VARPTR(a)` and
  `VARPTR(b)`
- **THEN** `t` prints the offset between the two, as with the old compiler

### Requirement: Variables that cannot be passed to such a parameter
A plain variable not of a user type (a number, a string, a fixed-length string, a scalar parameter passed on)
passed to a parameter whose address is taken SHALL be a compile error (the old compiler fails there,
`verification\v61_px1`-`px8`).

#### Scenario: A scalar variable argument
- **WHEN** a program passes `x%` to `t` above
- **THEN** it is a compile error that is not "not supported yet"

#### Scenario: A scalar parameter passed on
- **WHEN** a SUB `a (x AS INTEGER)` passes `x` to `t` above
- **THEN** it is a compile error that is not "not supported yet"

### Requirement: CALL ABSOLUTE
`CALL ABSOLUTE([argument, …] offset)` SHALL run the machine code at the offset in the current `DEF SEG` segment
through libqb's emulator, as the old compiler's program does: a variable argument passed as its offset (the
variable in conventional memory), any other value as the offset of a slot holding it, a `SINGLE`, `DOUBLE` or
`_INTEGER64` by its type.

#### Scenario: Code that returns
- **WHEN** a program pokes the bytes of a far return into an INTEGER array and runs `DEF SEG = VARSEG(code(0)):
  CALL ABSOLUTE(VARPTR(code(0)))`, then prints `"back"`
- **THEN** it prints `back`, as the old compiler's program does

#### Scenario: Code that writes an argument
- **WHEN** the machine code stores a number through its first argument's offset, and the program passes `v%`
- **THEN** `v%` holds that number after the call, as with the old compiler

#### Scenario: Mouse programs
- **WHEN** QB64pe's `qbasic_testcases` `n54/big/tictac` and `misc/cabsmous` are compiled
- **THEN** each builds with no diagnostic

### Requirement: The name ABSOLUTE
`CALL ABSOLUTE(…)` SHALL be the statement of requirement "CALL ABSOLUTE" also where the program has a SUB named
`ABSOLUTE`. `ABSOLUTE` without `CALL`, and `CALL ABSOLUTE` without parentheses, SHALL be compile errors where the
program has no such SUB (the old compiler's "Syntax error" and "Expected CALL sub-name (...)",
`verification\v61_c5`-`c7`).

#### Scenario: A SUB named ABSOLUTE
- **WHEN** a program has `SUB ABSOLUTE (n AS INTEGER)`, runs `CALL ABSOLUTE(VARPTR(code(0)))` over a far return
  and then `ABSOLUTE 5`
- **THEN** the machine code runs and returns, and the SUB is called once, by `ABSOLUTE 5`

#### Scenario: ABSOLUTE without CALL
- **WHEN** a program with no SUB named `ABSOLUTE` has `ABSOLUTE (VARPTR(code(0)))`
- **THEN** it is a compile error that is not "not supported yet"
