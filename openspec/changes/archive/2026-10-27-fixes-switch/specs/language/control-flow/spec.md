# Spec Delta

## MODIFIED Requirements

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
  (`verification\v57_r04`); in a `$CONSOLE:ONLY` program its later output still goes to the console (requirement
  "RUN in a console-only program")

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

## ADDED Requirements

### Requirement: RUN leaves the active procedures first
A `RUN` reached inside a SUB or FUNCTION SHALL first leave every active procedure through its exit path (as `END`
from a procedure does), so that a program which runs `RUN` from a SUB any number of times neither grows its stack
nor keeps the procedures' strings, whatever the fixes (`study\39` row 14; a `QB64PE-DIVERGENCES.md` row).

#### Scenario: RUN from a SUB many times
- **WHEN** a program keeps a counter in its process's environment (`ENVIRON`, which `RUN` does not clear) and runs
  `RUN` from a SUB until the counter reaches 100,000
- **THEN** it ends normally with the counter at 100,000

### Requirement: RUN in a console-only program
In a `$CONSOLE:ONLY` program, the output after `RUN`, `RUN label` or `RUN line` SHALL go to the console as at the
first start, whatever the fixes (`study\39` row 14; a `QB64PE-DIVERGENCES.md` row).

#### Scenario: Output after RUN label
- **WHEN** a `$CONSOLE:ONLY` program prints `one`, runs `RUN again` once, and `again:` is followed by `PRINT "two"`
- **THEN** its console output holds `one` and then `two`

### Requirement: Fix run-in-procedure
With the fix `run-in-procedure` on, `RUN` in any form inside a SUB or FUNCTION SHALL be a compile error at it, with
a note to set a flag and run `RUN` from the main module, as in QuickBASIC 4.5 (`verification\qb45\q45_run_from_sub`).

#### Scenario: RUN in a SUB
- **WHEN** a SUB holds `RUN` and the fix is on
- **THEN** the compile fails with an error at `RUN`, not marked "not supported yet"
