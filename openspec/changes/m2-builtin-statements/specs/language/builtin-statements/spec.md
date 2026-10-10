# Spec Delta

## Purpose

How built-in statements are compiled: how one is recognised and its arguments checked and converted, what a
runtime error in one does, and the behaviour of the statements that belong to no narrower capability (`SWAP`, the
`MID$` statement, `RANDOMIZE`, `SHELL`, `ENVIRON`, console `INPUT` and `LINE INPUT`), as measured with `qb64pe.exe`.

## ADDED Requirements

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
statement again. A statement with several items (`PRINT #`, `WRITE`, `INPUT #`, `READ`) SHALL stop at the first
item that raises, as the old compiler does.

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
every numeric type, `STRING`, fixed-length strings and user types as the old compiler accepts them. Two places of
different types SHALL be a compile error ("Type mismatch" in the old compiler), as SHALL an operand that is not a
place.

#### Scenario: Numbers and strings
- **WHEN** `a& = 1: b& = 2: s$ = "p": t$ = "q": SWAP a&, b&: SWAP s$, t$: PRINT a&; b&; s$; t$` runs
- **THEN** it prints ` 2  1 qp`

#### Scenario: Different types
- **WHEN** `SWAP a&, d#` is compiled
- **THEN** it is a compile error

#### Scenario: Elements
- **WHEN** `DIM x(3) AS INTEGER: x(1) = 7: x(2) = 9: SWAP x(1), x(2): PRINT x(1); x(2)` runs
- **THEN** it prints ` 9  7 `

### Requirement: MID$ statement
`MID$(s, start[, length]) = value` SHALL overwrite bytes of the string place `s` in place, never changing its
length, as the old compiler's runtime does for every start and length; the target SHALL be a string place.

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
runtime does, so that a program seeded with a constant prints the same `RND` values as with the old compiler.
`RANDOMIZE` without a seed in a `$CONSOLE:ONLY` program SHALL behave as the old compiler's does, as measured.

#### Scenario: Seeded sequence
- **WHEN** `RANDOMIZE 5: PRINT RND; RND; RND` runs
- **THEN** it prints the three values the old compiler's program prints

#### Scenario: Seed from the clock
- **WHEN** `RANDOMIZE TIMER: x = RND: PRINT x >= 0 AND x < 1` runs
- **THEN** it prints `-1`

### Requirement: SHELL and ENVIRON
`SHELL`, `SHELL command$`, `SHELL _HIDE …` and `SHELL _DONTWAIT …` (the words in either order) SHALL run the
command as the old compiler's runtime does, and `ENVIRON s$` SHALL set an environment variable of the program as it
does.

#### Scenario: Command output in order
- **WHEN** `PRINT "a": SHELL "echo b": PRINT "c"` runs in a `$CONSOLE:ONLY` program
- **THEN** the output is what the old compiler's program prints for it

#### Scenario: Setting and reading a variable
- **WHEN** `ENVIRON "QB64RUST_T=7": PRINT ENVIRON$("QB64RUST_T")` runs
- **THEN** it prints `7`

### Requirement: Console INPUT and LINE INPUT
In a `$CONSOLE:ONLY` program, `INPUT [;] ["prompt" {;|,}] target, …` and `LINE INPUT [;] ["prompt";] target$` SHALL
print the prompt, read from standard input and store into the targets as the old compiler's program does: the `? `
after a prompt ended by `;`, the split of a line at commas, the conversion of a field to each numeric type, and
what happens on a field that is no number, on too few or too many fields and at the end of the input, each as
measured. A prompt that is not a string literal SHALL be reported as the old compiler reports it.

#### Scenario: LINE INPUT with a prompt
- **WHEN** `LINE INPUT "name? "; s$: PRINT "["; s$; "]"` runs with the input line `Dave M`
- **THEN** it prints `name? [Dave M]`

#### Scenario: INPUT of a number and a string
- **WHEN** `INPUT "n"; a&, t$: PRINT a&; "["; t$; "]"` runs with the input line `42, hi there`
- **THEN** it prints `n? ` and then what the old compiler's program prints for that line

#### Scenario: Target is not a variable
- **WHEN** `INPUT 5` is compiled
- **THEN** it is a compile error
