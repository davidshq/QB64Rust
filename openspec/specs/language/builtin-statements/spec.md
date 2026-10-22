# language/builtin-statements Specification

## Purpose
How built-in statements are compiled: how one is recognised and its arguments checked and converted, what a
runtime error in one does, and the behaviour of the statements that belong to no narrower capability (`SWAP`, the
`MID$` statement, `RANDOMIZE`, `SHELL`, `ENVIRON`, console `INPUT` and `LINE INPUT`), as measured with `qb64pe.exe`.

## Requirements

### Requirement: Supported built-in statements
`sema` SHALL compile these built-in statements: `OPEN`, `CLOSE`, `SEEK`, `KILL`, `NAME`, `MKDIR`, `RMDIR`, `CHDIR`,
`PRINT #`, `WRITE`, `INPUT`, `LINE INPUT`, `READ`, `RESTORE`, `DATA`, `SWAP`, `MID$ … =`, `RANDOMIZE`, `SHELL`,
`ENVIRON`, `_DEST`, `_SOURCE`, `_ECHO`, the window statements `_SCREENHIDE`, `_SCREENSHOW`, `_SCREENICON`,
`_TITLE`, `_ICON`, `_MOUSESHOW`, `_MOUSEHIDE`, `_MOUSECURSOR`, `_WINDOWSIZELIMIT`, `_FREEIMAGE`, `_FULLSCREEN`
(`language/console-and-window`), the screen statements `SCREEN`, `CLS`, `COLOR`, `LOCATE`, `WIDTH`, `VIEW
PRINT`, `PSET`, `PRESET`, `LINE`, `CIRCLE`, `PAINT`, `DRAW`, `VIEW`, `WINDOW`, `PALETTE`, `PCOPY`, `_LIMIT`,
`_DELAY`, `SLEEP`, `_DISPLAY`, graphics `GET` and graphics `PUT` (`language/screen-output`), and the statements of
conventional memory `DEF SEG`, `POKE`, `BLOAD`, `BSAVE` (`language/conventional-memory`), and the sound
statements `SOUND`, `PLAY`, `BEEP` and the sound handle statements `_SNDCLOSE`, `_SNDPLAY`, `_SNDPLAYCOPY`, `_SNDPLAYFILE`, `_SNDPAUSE`, `_SNDSTOP`, `_SNDLOOP`, `_SNDVOL`, `_SNDBAL`, `_SNDSETPOS`, `_SNDLIMIT`, `_SNDRAW`, `_SNDRAWBATCH`, `_SNDRAWDONE`, `_MIDISOUNDBANK`, `_WAVE` (`language/sound`), `PRINT USING` (`language/print-using`), the statements of
files read and written by position `GET`, `PUT`, `FIELD`, `LSET`, `RSET`, `LOCK`, `UNLOCK`
(`language/record-files`), `LPRINT` and `CHAIN`, and the image, font and mouse statements
(`language/images`, `language/fonts`, `language/mouse`): `_SAVEIMAGE`, `_PUTIMAGE`, `_MAPTRIANGLE`,
`_DEPTHBUFFER`, `_CLEARCOLOR`, `_SETALPHA`, `_BLEND`, `_DONTBLEND`, `_PALETTECOLOR`, `_COPYPALETTE`,
`_AUTODISPLAY`, `_CLIPBOARDIMAGE`, `_PRINTIMAGE`, `_FONT`, `_FREEFONT`, `_PRINTSTRING`, `_PRINTMODE`,
`_UPRINTSTRING`, `_MAPUNICODE`, `_MOUSEMOVE`, and the statements of memory blocks (`language/mem-blocks`):
`_MEMGET`, `_MEMPUT`, `_MEMFILL`, `_MEMCOPY`, `_MEMFREE`, and `_ASSERT` (`language/asserts-and-debug`), and `CLEAR` and `_CLIPBOARD$ = s$`,
and `_KEYCLEAR` (`language/keyboard`), `_RESIZE`, `_CONSOLE`, `_SCREENMOVE`, `_ACCEPTFILEDROP`, `_FINISHDROP`
and `_MESSAGEBOX` (`language/console-and-window`), and `_CONTROLCHR` (`language/screen-output`). Any
other built-in
statement SHALL be reported "not supported yet" at its first word, and a supported
statement used in a form this change leaves out SHALL be reported "not supported yet" at that form.

#### Scenario: Supported statement compiles
- **WHEN** `KILL "gone.tmp"` is compiled in a program with no other statement
- **THEN** there is no diagnostic

#### Scenario: With CALL
- **WHEN** `CALL KILL("gone.tmp")` is compiled
- **THEN** it compiles as `KILL "gone.tmp"` does, as with the old compiler

#### Scenario: Unsupported statement
- **WHEN** a program contains `STOP`
- **THEN** the only diagnostic is "not supported yet" at `STOP`

#### Scenario: Unsupported form of a supported statement
- **WHEN** a program contains `RESTORE 100`
- **THEN** the diagnostic is "not supported yet" and names line numbers

#### Scenario: A statement and a function of one name
- **WHEN** a program has `d& = _DEST` and `_DEST d&`
- **THEN** the first is the function and the second the statement, and both compile

#### Scenario: Two statements of one name
- **WHEN** a program has `VIEW PRINT 2 TO 10` and `VIEW (10, 10)-(50, 50)`
- **THEN** each compiles to the runtime call the old compiler writes for it

#### Scenario: The graphics form and the file form of one name
- **WHEN** a program has `PUT (10, 10), a, PSET` and `PUT #1, , x`
- **THEN** each compiles to the runtime call the old compiler writes for it: the first to the graphics entry, the
  second to the file entry

#### Scenario: A statement of two words
- **WHEN** a program has `DEF SEG = 0` and `DEF SEG`
- **THEN** each compiles to the runtime call the old compiler writes for it, and what the compiler reports for a
  `DEF FN` line does not change

#### Scenario: An image statement
- **WHEN** `_PUTIMAGE (0, 0), img` is compiled in a program where `img` is a LONG
- **THEN** there is no diagnostic

#### Scenario: A statement of memory blocks
- **WHEN** `DIM m AS _MEM: m = _MEMNEW(8): _MEMPUT m, m.OFFSET, 1 AS LONG: _MEMFREE m` is compiled
- **THEN** there is no diagnostic

### Requirement: Arguments of a built-in statement
Each argument of a built-in statement SHALL be checked for its kind (a string where the old compiler needs a
string, a number where it needs a number, a variable where it stores into one) and converted to its slot's type by
the rule built-in functions use for that slot type. A wrong kind or a wrong number of arguments SHALL be a compile
error, not a "not supported yet" mark.

#### Scenario: Number where a string is needed
- **WHEN** `KILL 5` is compiled
- **THEN** it is a compile error

#### Scenario: Wrong number of arguments
- **WHEN** `KILL "a", "b"` or `SEEK 1` is compiled
- **THEN** it is a compile error

#### Scenario: Float argument to an integer slot
- **WHEN** `CLOSE 1.5` runs with file 2 open
- **THEN** file 2 is closed, as with the old compiler (1.5 rounds half to even to 2)

### Requirement: Runtime errors of built-in statements
A built-in statement SHALL raise the runtime error the old compiler raises, and the error SHALL be serviced at the
end of that statement: the handler runs, `RESUME NEXT` continues with the next statement, and `RESUME` runs the
statement again. A statement with several items (`PRINT #`, `WRITE`, `INPUT #`) SHALL stop at the first item that
raises, as the old compiler does; `READ` SHALL go on as the old compiler's does (capability `language/data-read`).

#### Scenario: Error in a statement call
- **WHEN** `KILL "no-such-file.tmp"` runs under a handler that prints `ERR` and resumes next, followed by `PRINT "after"`
- **THEN** it prints 53 and then `after`

#### Scenario: RESUME runs the statement again
- **WHEN** `KILL f$` fails with 53 under a handler that creates the file and does `RESUME`
- **THEN** the statement runs again, its arguments evaluated again, and the file is removed

#### Scenario: Raising argument
- **WHEN** `KILL CHR$(-1)` runs under a handler that prints `ERR` and resumes next
- **THEN** the handler prints 5 once, the argument's error, as with the old compiler

### Requirement: SWAP
`SWAP a, b` SHALL exchange the values of two places (variables, array elements or members) of the same type, for
every numeric type but `_BIT`, strings and user types as the old compiler accepts them: two numeric types that
differ only in signedness count as the same (the bits are exchanged), any two strings go together (`STRING` and
`STRING * n` of any length, each side cut or padded to its own length), and two user types must be the same `TYPE`.
Two places of different types SHALL be a compile error ("Type mismatch" in the old compiler), as SHALL a `_BIT`
place, an operand that is not a place and the name of the FUNCTION the statement stands in. An operand in
parentheses, two whole arrays and a member of an array element, which the old compiler accepts, SHALL be reported
"not supported yet". An element operand whose index is out of range SHALL raise error 9 and the statement SHALL
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

### Requirement: MID$ statement
`MID$(s, start[, length]) = value` SHALL overwrite bytes of the string place `s` in place, never changing its
length, as the old compiler's runtime does for every start and length: a start below 1 or past the end and a
length below 1 SHALL change nothing and raise nothing. The target SHALL be a string place (a variable, a
fixed-length string, an element or a member); a member of an array element SHALL be reported "not supported yet".

#### Scenario: Replacement longer than the target
- **WHEN** `s$ = "q": MID$(s$, 1, 1) = "ZZ": PRINT s$` runs
- **THEN** it prints `Z`

#### Scenario: Without a length
- **WHEN** `s$ = "abcdef": MID$(s$, 3) = "XY": PRINT s$` runs
- **THEN** it prints `abXYef`

#### Scenario: Target is not a string place
- **WHEN** `MID$("abc", 1) = "x"` is compiled
- **THEN** it is a compile error

### Requirement: RANDOMIZE
`RANDOMIZE`, `RANDOMIZE n` and `RANDOMIZE USING n` SHALL seed the random number generator as the old compiler's
runtime does, so that a program seeded with a constant prints the same `RND` values as with the old compiler:
`RANDOMIZE USING n` SHALL give the same sequence wherever it runs, and `RANDOMIZE n` one that also depends on the
values drawn before it.
`RANDOMIZE` without a seed SHALL print `Random-number seed (-32768 to 32767)? ` and read the seed from standard
input, as the old compiler's program does.

#### Scenario: Seeded sequence
- **WHEN** `RANDOMIZE 5: PRINT RND; RND; RND` runs
- **THEN** it prints the three values the old compiler's program prints

#### Scenario: USING repeats
- **WHEN** `RANDOMIZE USING 5: a = RND: b = RND: RANDOMIZE USING 5: PRINT a = RND` runs
- **THEN** it prints `-1`

#### Scenario: Seed from the clock
- **WHEN** `RANDOMIZE TIMER: x = RND: PRINT x >= 0 AND x < 1` runs
- **THEN** it prints `-1`

### Requirement: SHELL and ENVIRON
`SHELL`, `SHELL command$`, `SHELL _HIDE …` and `SHELL _DONTWAIT …` (the words in either order) SHALL run the
command as the old compiler's runtime does, and `ENVIRON s$` SHALL set an environment variable of the program as it
does. A command that is no string SHALL be a compile error. `CALL SHELL(c$)`, which the old compiler accepts, MAY
be reported "not supported yet".

#### Scenario: Words in either order
- **WHEN** `SHELL _HIDE _DONTWAIT c$` and `SHELL _DONTWAIT _HIDE c$` are compiled
- **THEN** the first calls the runtime's entry for `_HIDE` and the second its entry for `_DONTWAIT`, each with the
  flag for its second word, as the old compiler's C++ does

#### Scenario: An empty command
- **WHEN** `SHELL _HIDE ""` runs under a handler
- **THEN** error 5 is raised and the program goes on

#### Scenario: Command output in order
- **WHEN** `PRINT "a": SHELL "echo b": PRINT "c"` runs in a `$CONSOLE:ONLY` program
- **THEN** the output is what the old compiler's program prints for it

#### Scenario: Setting and reading a variable
- **WHEN** `ENVIRON "QB64RUST_T=7": PRINT ENVIRON$("QB64RUST_T")` runs
- **THEN** it prints `7`

### Requirement: Console INPUT and LINE INPUT
In a `$CONSOLE:ONLY` program, `INPUT [;] ["prompt" {;|,}] target, …` and `LINE INPUT [;] ["prompt";] target$` SHALL
print the prompt, read from standard input and store into the targets as the old compiler's program does: the `? `
after a prompt ended by `;` (never for `LINE INPUT`), the split of a line at commas, the conversion of a field to
each numeric type, and what happens on a field that is no number and on too few or too many fields, each as
measured: the program never asks again; a character that does not fit its field is dropped and the text so far is
printed again; targets without a field are 0 or empty. A `;` before the prompt SHALL leave the output on the same
line after the answer. One `,` after the last target SHALL be accepted. A `_BIT` variable SHALL be accepted as a
target and keep its value, as in the old compiler. A prompt that is not a string literal, a target that is no
variable, and a `LINE INPUT` target that is no string or not alone SHALL be compile errors.

#### Scenario: A character that does not fit
- **WHEN** `INPUT "byte"; b` with `b` a `_BYTE` runs with the input line `300`
- **THEN** it prints `byte? 30` and `b` is 30

#### Scenario: Fewer fields than targets
- **WHEN** `INPUT l, m` runs with the input line `5`
- **THEN** `l` is 5 and `m` is 0

#### Scenario: Trailing comma
- **WHEN** `INPUT l,` is compiled
- **THEN** it compiles and reads one value into `l`

#### Scenario: Prompt is a variable
- **WHEN** `INPUT p$; l` is compiled
- **THEN** it is a compile error

#### Scenario: LINE INPUT with a prompt
- **WHEN** `LINE INPUT "name? "; s$: PRINT "["; s$; "]"` runs with the input line `Dave M`
- **THEN** it prints `name? [Dave M]`

#### Scenario: INPUT of a number and a string
- **WHEN** `INPUT "n"; a&, t$: PRINT a&; "["; t$; "]"` runs with the input line `42, hi there`
- **THEN** it prints `n? ` and then what the old compiler's program prints for that line

#### Scenario: Target is not a variable
- **WHEN** `INPUT 5` is compiled
- **THEN** it is a compile error

### Requirement: A statement with a template after CALL
A supported built-in statement whose forms are read by a template SHALL compile after `CALL` as the old compiler
compiles it: the name alone is the statement with nothing after it, and the name with one pair of parentheses
around all that follows is the statement with what stands between them. Any other form SHALL be a compile error
that is not "not supported yet".

#### Scenario: Statements with a rule of their own
- **WHEN** a program has `CALL SWAP(a, b)`
- **THEN** it MAY be reported "not supported yet" (as `CLOSE` and the `MID$` statement after `CALL`)

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

### Requirement: CHAIN
`CHAIN name$` SHALL compile, for any string expression, to the runtime call the old compiler writes. No name, a
number or two arguments SHALL be a compile error.

#### Scenario: The call
- **WHEN** `CHAIN "next" + ".bas"` is compiled by both compilers
- **THEN** the runtime calls are the same

#### Scenario: A number
- **WHEN** a program has `CHAIN 5` or `CHAIN`
- **THEN** each is a compile error that is not "not supported yet"

### Requirement: LPRINT
`LPRINT` SHALL compile in every form console `PRINT` has (items, `;` and `,`, `TAB` and `SPC`, `USING`), to the
runtime calls the old compiler writes for the printer.

#### Scenario: Items and zones
- **WHEN** `LPRINT "a"; 1, "b"` and `LPRINT` are compiled by both compilers
- **THEN** the runtime calls are the same

#### Scenario: TAB to the printer
- **WHEN** `LPRINT TAB(5); "x"` is compiled by both compilers
- **THEN** the runtime calls are the same

### Requirement: CLEAR
`CLEAR [a][, [b][, c]]` SHALL reset what the old compiler's runtime resets: every variable of the main module and
every `STATIC` and `SHARED` variable to zero or empty, each static array's elements, each dynamic array released,
the `DATA` position, the open files closed, the error handler and the `RETURN` points.

#### Scenario: Variables reset
- **WHEN** `a = 5: s$ = "x": DIM b(3): b(1) = 2: CLEAR: PRINT a; LEN(s$); b(1)` runs
- **THEN** it prints ` 0  0  0 `, as with the old compiler

#### Scenario: A dynamic array released
- **WHEN** `REDIM d(5): CLEAR` is followed by `PRINT d(1)` under a handler that prints `ERR`
- **THEN** the handler prints 9, as with the old compiler

#### Scenario: A STATIC variable and the DATA position
- **WHEN** a SUB counts calls in a `STATIC` variable and a `READ` follows `CLEAR`
- **THEN** the count starts again from 1 and the `READ` gives the first `DATA` item, as with the old compiler

#### Scenario: RETURN after CLEAR
- **WHEN** a `GOSUB` runs `CLEAR`, sets a handler and runs `RETURN`
- **THEN** the handler gets error 3, as with the old compiler

#### Scenario: Strings in a user type's member array
- **WHEN** the upstream program `arrays/95_type_codegen_clear_inline_owner` runs
- **THEN** it prints `PASS 95_type_codegen_clear_inline_owner.bas`

### Requirement: The forms of CLEAR
`CLEAR`'s arguments SHALL be numbers, evaluated and ignored, and `CALL CLEAR` SHALL be `CLEAR`. `CLEAR` in a SUB
or FUNCTION, with a parenthesis anywhere among its arguments, with a fourth argument or with a string SHALL be the
old compiler's errors.

#### Scenario: Arguments ignored
- **WHEN** `x = 5: CLEAR x + 1, , 2 * 3: PRINT x` runs
- **THEN** it prints ` 0 `, as with the old compiler

#### Scenario: In a SUB
- **WHEN** a SUB holds `CLEAR`
- **THEN** it is the compile error "CLEAR cannot be used inside a SUB/FUNCTION", as in the old compiler

#### Scenario: A parenthesis
- **WHEN** a program has `CLEAR F(1)`
- **THEN** it is the compile error "Incorrect number of arguments", as in the old compiler

#### Scenario: Wrong forms
- **WHEN** a program has `CLEAR , , LEN("a")`, `CLEAR 1, 2, 3, 4` or `CLEAR "a"`
- **THEN** each is a compile error, as in the old compiler

### Requirement: ASC statement
`ASC(s$) = n` and `ASC(s$, i) = n` SHALL set the byte at position 1, or at `i`, of the string place `s$` to the
low byte of `n`, as the old compiler's inline code does: the place, `i` and `n` evaluated in that order, each
number converted to LONG, error 5 for an empty string or a position out of range. A place that is not a string,
and an expression that is not a place, SHALL be compile errors, as `verification\v52` records them.

#### Scenario: One byte set
- **WHEN** `a$ = "a/b": ASC(a$, 2) = 92: PRINT a$` runs
- **THEN** it prints `a\b`

#### Scenario: The first byte
- **WHEN** `a$ = "xyz": ASC(a$) = 65: PRINT a$` runs
- **THEN** it prints `Ayz`

#### Scenario: Out of range
- **WHEN** `a$ = "ab": ASC(a$, 3) = 65` runs under a handler that prints `ERR`, and so does `b$ = "": ASC(b$) = 65`
- **THEN** each handler prints 5, and the strings are unchanged

#### Scenario: A value beyond a byte
- **WHEN** `a$ = "a": ASC(a$) = 321: PRINT ASC(a$)` runs
- **THEN** it prints 65, the low byte (`verification\v52_d_asc`)

#### Scenario: Not a string place
- **WHEN** a program has `ASC(n%) = 65` or `ASC("ab") = 65`
- **THEN** it is a compile error
