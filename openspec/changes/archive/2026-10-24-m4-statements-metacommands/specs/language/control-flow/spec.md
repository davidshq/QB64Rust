# Spec Delta

## ADDED Requirements

### Requirement: RUN
`RUN` SHALL start the program again from its first line, and `RUN label` or `RUN line` at that label or line
number of the main module, also from a SUB or FUNCTION. Before that, the screen, the colours, the palette, `RND`'s
seed, the key traps and the keyboard buffer SHALL be reset and every variable cleared as by `CLEAR`, as in the old
compiler (measured, `verification\v57_r01`–`r19`). Any other name SHALL be the old compiler's error.

#### Scenario: RUN again from the top
- **WHEN** a program prints a counter kept in a file, adds one to it, and runs `RUN` while it is below 3
- **THEN** the program's output is the old compiler's: each run starts with its variables zero and the same `RND`
  value (`verification\v57_r01`)

#### Scenario: RUN from a SUB or FUNCTION
- **WHEN** a SUB or a FUNCTION runs `RUN`
- **THEN** the program starts again from its first line (`verification\v57_r06`, `r07`)

#### Scenario: RUN a label from a SUB
- **WHEN** a SUB runs `RUN again`, where `again:` is a label of the main module
- **THEN** the program goes on at `again:` with every variable cleared, as the old compiler's program does
  (`verification\v57_r04`; in a `$CONSOLE:ONLY` program its later output is lost, as with the old compiler)

#### Scenario: A label that is also a SUB
- **WHEN** a program has the label `one:` and a `SUB one`, and runs `RUN one`
- **THEN** the compile fails with an error at `RUN one`, with "Name already in use" as its note
  (`verification\v28_x86`)

#### Scenario: A label that does not exist
- **WHEN** a program runs `RUN nowhere` and no label `nowhere` exists, or `RUN inside` with `inside:` a label of a SUB
- **THEN** the compile fails with an error, with "Label 'nowhere' not defined" as its note (`verification\v57_r12`,
  `r13`)

#### Scenario: A numeric variable or expression
- **WHEN** a program runs `RUN n` with `n` a variable, or `RUN 1 + 2`
- **THEN** the compile fails with "Label 'n' not defined" or "Illegal string-number conversion" as its note
  (`verification\v57_r14`, `r15`)

#### Scenario: Variables kept elsewhere
- **WHEN** a program runs `RUN` with a `COMMON SHARED` variable and a SUB's `STATIC` variable set
- **THEN** both are 0 in the next run (`verification\v57_r18`, `r19`)

### Requirement: RUN of a file
`RUN file$` SHALL start the program `file$` names and end this one, as the old compiler's program does; when the
file cannot be started it SHALL raise error 53, and the program SHALL go on after the `RUN`.

#### Scenario: A file that cannot be started
- **WHEN** a program runs `RUN "nosuch.exe"` under a handler that prints `ERR` and resumes next
- **THEN** it prints 53, then its line after the `RUN` (`verification\v57_r08`, `r16`)

#### Scenario: A name that means a string
- **WHEN** a program runs `RUN p` with `p` a string `CONST` (named with or without `$`) or, under `DEFSTR A-Z`, a
  variable assigned or not, under a handler that prints `ERR` and resumes next
- **THEN** `p` is the file name: it prints 53, then its line after the `RUN`; `RUN p` with `p` a numeric `CONST` is
  "Label 'p' not defined" (`verification\v57_r20`–`r24`)

#### Scenario: A program that can be started
- **WHEN** a program runs `RUN "sort"` with no input
- **THEN** this program prints nothing after the `RUN`, as with the old compiler (`verification\v57_r09`)
