# language/builtin-statements Specification

## Purpose
How built-in statements are compiled: how one is recognised and its arguments checked and converted, what a
runtime error in one does, and the behaviour of the statements that belong to no narrower capability (`SWAP`, the
`MID$` statement, `RANDOMIZE`, `SHELL`, `ENVIRON`, console `INPUT` and `LINE INPUT`), as measured with `qb64pe.exe`.

## Requirements

### Requirement: Supported built-in statements
`sema` SHALL compile these built-in statements: `OPEN`, `CLOSE`, `SEEK`, `KILL`, `NAME`, `MKDIR`, `RMDIR`, `CHDIR`,
`PRINT #`, `WRITE`, `INPUT`, `LINE INPUT`, `READ`, `RESTORE`, `DATA`, `SWAP`, `MID$ … =`, `RANDOMIZE`, `SHELL`,
`ENVIRON`. Any other built-in statement SHALL be reported "not supported yet" at its first word, and a supported
statement used in a form this change leaves out SHALL be reported "not supported yet" at that form.

#### Scenario: Supported statement compiles
- **WHEN** `KILL "gone.tmp"` is compiled in a program with no other statement
- **THEN** there is no diagnostic

#### Scenario: With CALL
- **WHEN** `CALL KILL("gone.tmp")` is compiled
- **THEN** it compiles as `KILL "gone.tmp"` does, as with the old compiler

#### Scenario: Unsupported statement
- **WHEN** a program contains `LOCATE 1, 1`
- **THEN** the only diagnostic is "not supported yet" at `LOCATE`

#### Scenario: Unsupported form of a supported statement
- **WHEN** a program contains `RESTORE 100`
- **THEN** the diagnostic is "not supported yet" and names line numbers

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
