# language/conventional-memory Specification

## Purpose
The 64 KB segments of QuickBASIC's memory as libqb keeps them: `VARPTR` and `VARSEG` of an array of the main
module, which then lies in conventional memory, `DEF SEG`, `PEEK`, `POKE`, `BLOAD` and `BSAVE`, as the old
compiler's program behaves. What a program needs to load a picture saved by QuickBASIC into an array.

## Requirements

### Requirement: VARPTR and VARSEG of an array of the main module
For an array `a` of numbers declared in the main module, static or dynamic, named from any body, `VARPTR(a(i))`
SHALL be the element's index from the array's first element times the size of an element, with the bounds check
of an array read, and `VARSEG(a(i))` the segment of the array's data in libqb's conventional memory, the same for
every element. Both SHALL be `_INTEGER64` values to `PRINT` and to operators, as in the old compiler.

#### Scenario: Offsets
- **WHEN** `DIM a(1 TO 10) AS INTEGER, c(0 TO 4) AS LONG: PRINT VARPTR(a(1)); VARPTR(a(2)); VARPTR(c(3))` runs
- **THEN** it prints ` 0  2  12 `

#### Scenario: Two dimensions
- **WHEN** `DIM m(1 TO 3, 1 TO 4) AS INTEGER: PRINT VARPTR(m(2, 1)); VARPTR(m(1, 2))` runs
- **THEN** it prints ` 2  6 `, as with the old compiler

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

### Requirement: Arrays in conventional memory
An array named in a `VARPTR` or `VARSEG` anywhere in the program SHALL be allocated in libqb's conventional memory
as the old compiler allocates it: a static one in the order the arrays are declared, each rounded up to 16 bytes,
with error 257 when it is larger than 65,536 bytes or the memory is used up; a dynamic one at each allocation.
Every other use of the array (reads, stores, `ERASE`, `REDIM`, `LBOUND`, passing it to a procedure) SHALL behave
as for any array of its kind.

#### Scenario: Arrays lie one after the other
- **WHEN** `DIM a(1 TO 10) AS INTEGER, b(1 TO 10) AS INTEGER: PRINT VARSEG(b(1)) - VARSEG(a(1)); VARSEG(a(1)) =
  VARSEG(a(7))` runs
- **THEN** it prints ` 2 -1 `, as with the old compiler

#### Scenario: A dynamic array
- **WHEN** an array listed in `COMMON` and then given bounds by `DIM`, or made by `REDIM`, is named in `VARSEG`,
  erased, redimmed with `_PRESERVE` and passed to a SUB that redims it
- **THEN** `PEEK` through `DEF SEG = VARSEG(…)` reads its elements after each, as with the old compiler

#### Scenario: An array used before the line that names it
- **WHEN** a program stores into `a(3)`, and a later line, or a procedure, has `VARSEG(a(1))`
- **THEN** the array is in conventional memory from its `DIM` on, and the program prints what the old compiler's
  program prints

### Requirement: DEF SEG, PEEK and POKE
`DEF SEG = n` SHALL set the segment `PEEK`, `POKE`, `BLOAD` and `BSAVE` use, and `DEF SEG` alone SHALL set it back
to the default, each as the runtime call the old compiler writes; `POKE offset, value` SHALL store one byte and
`PEEK(offset)` SHALL give one, a LONG. Each numeric argument SHALL be converted as a LONG slot is. `DEF SEG` with a
value and no `=`, with `=` and no value, or with a string SHALL be a compile error.

#### Scenario: Bytes of an array
- **WHEN** `DIM a(1 TO 10) AS INTEGER`, then `DEF SEG = VARSEG(a(1)): POKE VARPTR(a(1)), 65: POKE VARPTR(a(1)) +
  1, 1: a(4) = &H1234: PRINT a(1); PEEK(VARPTR(a(4))); PEEK(VARPTR(a(4)) + 1)` runs
- **THEN** it prints ` 321  52  18 `

#### Scenario: A value that does not fit a byte
- **WHEN** `POKE VARPTR(a(5)), 300: PRINT a(5)` runs after `DEF SEG = VARSEG(a(1))`
- **THEN** it prints ` 44 `, as with the old compiler

#### Scenario: A rejected form
- **WHEN** a program has `DEF SEG 5`
- **THEN** the compiler reports an error that is not "not supported yet"

### Requirement: BLOAD and BSAVE
`BSAVE file$, offset, length` SHALL write the block at the offset in the current segment to a file, and `BLOAD
file$[, offset]` SHALL read such a file back to the offset, or to the offset it was saved from when none is given,
each as the runtime call the old compiler writes. A file that does not exist SHALL raise error 53.

#### Scenario: Saved and loaded into another array
- **WHEN** ten elements of an INTEGER array `a` are saved with `BSAVE "t.tmp", VARPTR(a(1)), 20` under `DEF SEG =
  VARSEG(a(1))` and loaded with `BLOAD "t.tmp", VARPTR(b(1))` under `DEF SEG = VARSEG(b(1))`
- **THEN** `b` holds the ten values of `a`, and the file is 27 bytes long

#### Scenario: Loaded in a procedure into a shared array
- **WHEN** a SUB runs `DEF SEG = VARSEG(g(1)): BLOAD "t.tmp", VARPTR(g(1)): DEF SEG` for an array `g` of the main
  module declared `DIM SHARED`
- **THEN** `g` holds the saved values after the call

#### Scenario: No such file
- **WHEN** `BLOAD "missing.tmp", VARPTR(b(1))` runs under a handler with `RESUME NEXT`
- **THEN** error 53 is raised and the program goes on

#### Scenario: A picture of Civil War Strategy
- **WHEN** a screen program loads the game's `MTN.VGA` into `DIM mtn(1 TO 1564)` as the game does and draws it
  with `PUT (380, 30), mtn, PSET`
- **THEN** the state of its screen is the one recorded with the old compiler

### Requirement: Variables in conventional memory
A variable of the main module that `VARPTR`, `VARSEG`, `SADD` or `VARPTR$` names anywhere in the program SHALL
live in libqb's conventional memory, in the old compiler's order and sizes (with every `_BIT` scalar of the main
module, named or not): `VARPTR` SHALL give the offset the old compiler's program gives, and `VARSEG` its segment
(80 for a variable). `PEEK` and `POKE` through them SHALL reach the variable's bytes, and a string SHALL keep its
descriptor there through every assignment. A `TYPE` variable or array element that holds a `STRING` (a member,
or a member of a member) SHALL take the old compiler's place and size there, 8 bytes for each `STRING` member;
its strings SHALL be made as for a `TYPE` variable anywhere else, empty, where the old compiler's program leaves
them out and crashes at the first use of one (`QB64PE-DIVERGENCES.md` D-037).

#### Scenario: Offsets of scalars
- **WHEN** the programs `verification\v56_a1`–`a3` print `VARPTR` and `VARSEG` of scalars of every type, strings,
  fixed-length strings and `_BIT` variables, declared in several orders
- **THEN** each prints what the old compiler's program printed

#### Scenario: Bytes through PEEK and POKE
- **WHEN** `DIM i AS INTEGER: i = 258: DEF SEG = VARSEG(i): PRINT PEEK(VARPTR(i)); PEEK(VARPTR(i) + 1): POKE
  VARPTR(i), 3: PRINT i` runs
- **THEN** it prints ` 2  1 `, then ` 259 `

#### Scenario: A string keeps its place
- **WHEN** a string named by `VARPTR` is assigned, grown, swapped, changed by `MID$`, `READ` and `LSET`, and passed
  to a SUB, and its `VARPTR`, `SADD` and descriptor printed after each (`verification\v56_a4`)
- **THEN** each value is the old compiler's

#### Scenario: Elements and members
- **WHEN** the programs `verification\v56_a6`, `a7`, `t01`, `t06`–`t09` print `VARPTR` and `VARSEG` of elements of
  arrays of a `TYPE`, of strings and of fixed-length strings, of members through member arrays and nested types,
  and of an index outside its bounds
- **THEN** each prints what the old compiler's program printed: the element's offset and the member's for a
  `TYPE`, the element's offset for fixed-length strings, the descriptor's offset for strings, error 9 for an index
  outside

#### Scenario: A TYPE that holds a string, in conventional memory
- **WHEN** the programs `verification\v56_x08`, `x26` and `x27` print `VARPTR` of a `STRING` member, of a `LONG`
  member beside it and of a member of an array element of such a `TYPE`
- **THEN** each prints what the old compiler's program printed

#### Scenario: Its string used
- **WHEN** `verification\v56_a5` prints `VARPTR(r.v); VARSEG(r)`, sets `r.v = 5`, prints it, then sets `r.n = "abc"`
  and prints `r.n; LEN(r.n)`
- **THEN** it prints the old compiler's first two lines, then `abc 3`, and ends normally (D-037: the old
  compiler's program crashes there)

### Requirement: SADD
`SADD(s$)` of a variable-length string variable or element, or of `s$()` (its first element), SHALL give the
offset of its characters in conventional memory. `SADD` of anything else SHALL be a compile error, with the old
compiler's wording where it has one.

#### Scenario: SADD as the string grows
- **WHEN** the program `verification\v56_b1` prints `SADD` of a string before and after it grows, and of an
  element of an array of strings
- **THEN** it prints what the old compiler's program printed

#### Scenario: SADD of a fixed-length string or a literal
- **WHEN** a program has `l = SADD(fx)` for a `STRING * 4`, or `l = SADD("a")`
- **THEN** it is a compile error with the old compiler's wording, "SADD only accepts variable-length string
  variables", as a note (`verification\v28_x55`, `x56`)

#### Scenario: SADD of a number or a TYPE
- **WHEN** a program has `l = SADD(i)` for an INTEGER, or `l = SADD(r)` for a `TYPE` variable
- **THEN** it is a compile error (`verification\v56_x03`, `x09`)

### Requirement: VARPTR$
`VARPTR$(v)` of a number, a `_BIT`, a string, a fixed-length string, a `TYPE` variable, a member, an element of an
array of strings or `s$()` SHALL give the old compiler's string: a type byte, then an offset (for a fixed-length
string its descriptor's, for a member its variable's). `VARPTR$` of an element of an array that is not of
variable-length strings SHALL be the old compiler's error.

#### Scenario: The bytes of VARPTR$
- **WHEN** the programs `verification\v56_b1`–`b5`, `t04` and `t05` print `ASC` of each byte of `VARPTR$` of each
  type
- **THEN** each prints what the old compiler's program printed

#### Scenario: VARPTR$ of a number array
- **WHEN** a program has `t = VARPTR$(a(1))` for an INTEGER array
- **THEN** it is a compile error with the note "VARPTR$ only accepts variable-length string arrays"
  (`verification\v28_x57`)

### Requirement: VARPTR and VARSEG in HEX$, OCT$ and _BIN$
`VARPTR`, `VARSEG`, `SADD` or `_SCREENEXISTS` as the direct argument of `HEX$`, `OCT$` or `_BIN$` SHALL compile,
passed to the runtime as the old compiler passes it (the width 0), and SHALL give the old compiler's string.

#### Scenario: HEX$ of VARPTR
- **WHEN** `DIM a(1 TO 10) AS INTEGER: PRINT HEX$(VARPTR(a(9))); OCT$(VARSEG(a(1)) - VARSEG(a(1)))` runs
- **THEN** it prints `10` and `0`

#### Scenario: The forms measured
- **WHEN** the program `verification\v56_c_radix_width` runs
- **THEN** it prints what the old compiler's program printed (`HEX$(_SCREENEXISTS)` is empty where the value is -1)

### Requirement: INP, OUT and WAIT
`INP(port)`, `OUT port, value` and `WAIT port, and[, xor]` SHALL compile with the old compiler's libqb calls and
SHALL do what libqb's emulated ports do: the palette ports `&H3C0` (`OUT`), `&H3C7`, `&H3C8` and `&H3C9`, the
keyboard port `&H60`, and the display status port `&H3DA`; any other port SHALL do what the old compiler's
program does, and a port out of range SHALL raise error 6 (measured, `verification\v58`).

#### Scenario: The palette through its ports
- **WHEN** a program in screen 13 runs `OUT &H3C8, 1: OUT &H3C9, 63: OUT &H3C9, 0: OUT &H3C9, 0`, then
  `OUT &H3C7, 1` and prints `INP(&H3C9)` three times
- **THEN** it prints what the old compiler's program prints

#### Scenario: Waiting for the display
- **WHEN** a program runs `WAIT &H3DA, 8` and prints `"ok"`
- **THEN** it ends and prints `ok`, as the old compiler's program does

#### Scenario: A port libqb does not emulate
- **WHEN** a program prints `INP(&H379)` and runs `OUT &H378, 1`
- **THEN** it prints and ends as the old compiler's program does, error or value

#### Scenario: A port out of range
- **WHEN** a program evaluates `INP(70000)` under a handler that prints `ERR`
- **THEN** it prints ` 6 `

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

### Requirement: BLOAD and BSAVE past the block
A `BLOAD` or `BSAVE` whose start plus length runs past the end of libqb's block of conventional memory SHALL raise
error 5 before reading or writing anything. Inside the block they SHALL work as with the old compiler, wherever
the array their address came from ends (`study\39` row 5; a `QB64PE-DIVERGENCES.md` row).

#### Scenario: A BLOAD at the top of the block
- **WHEN** a program with a handler that prints `ERR` and resumes next runs `DEF SEG = &HFFFF` and `BLOAD f$, &HFFF0`
  for a file saved with a length of 1000
- **THEN** the handler prints 5, and nothing past the block is written

#### Scenario: A file longer than 64 KB with size 0
- **WHEN** a `BLOAD` reads a file whose header gives size 0 and whose data is 200,000 bytes long, to an address
  less than 200,000 bytes from the end of the block
- **THEN** it raises error 5

#### Scenario: The last bytes of the block
- **WHEN** a program saves the block's last 19 bytes with `DEF SEG = &HFFFF` and `BSAVE f$, &HFFF0, 19`, and loads
  them back with `BLOAD f$, &HFFF0`
- **THEN** neither raises an error, as with the old compiler

#### Scenario: A BSAVE at the top of the block
- **WHEN** a program runs `DEF SEG = &HFFFF` and `BSAVE f$, &HFFF0, 1000` under a handler
- **THEN** it raises error 5 and `f$` is not created
