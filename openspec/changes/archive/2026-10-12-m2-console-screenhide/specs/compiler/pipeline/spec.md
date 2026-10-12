# Spec Delta

## ADDED Requirements

### Requirement: Precompiler flags
A `$IF` or `$ELSEIF` condition that names `_CONSOLE_`, `_EXPLICIT_`, `_EXPLICITARRAY_`, `_ASSERTS_`, `_DEBUG_` or
`_SOCKETS_` SHALL be evaluated with the value the old compiler gives the flag from the whole program, the same on
every line of the code that is compiled. The values SHALL be settled as the old compiler settles them, by
compiling again until they no longer change. Such a condition SHALL NOT be reported "not supported yet", except
where the old compiler's own passes disagree about the flag's value on that line.

#### Scenario: Above the line that sets it
- **WHEN** a program tests `$IF _CONSOLE_ = 1 THEN` on a line above its `$CONSOLE`
- **THEN** the branch is taken

#### Scenario: $CONSOLE:ONLY alone
- **WHEN** a program with `$CONSOLE:ONLY` and no `$CONSOLE` tests `_CONSOLE_`
- **THEN** its value is 2

#### Scenario: $CONSOLE:ONLY, then $CONSOLE
- **WHEN** a program has `$CONSOLE:ONLY` and below it `$CONSOLE`
- **THEN** `_CONSOLE_` is 1

#### Scenario: OPTION _EXPLICIT
- **WHEN** a program has `OPTION _EXPLICIT` on its last line and `$IF _EXPLICIT_ THEN` and `$IF _EXPLICITARRAY_
  THEN` above it
- **THEN** both branches are taken

#### Scenario: OPTION _EXPLICITARRAY
- **WHEN** a program has `OPTION _EXPLICITARRAY` on its last line and tests both flags above it
- **THEN** the `_EXPLICITARRAY_` branch is taken and the `_EXPLICIT_` branch is not

#### Scenario: Between two lines that set the flag differently
- **WHEN** a program has `$CONSOLE:ONLY`, then `$IF _CONSOLE_ = 2 THEN` … `$END IF`, then `$CONSOLE`
- **THEN** that `$IF` is reported "not supported yet" (the old compiler's two passes disagree about it: its
  program prints nothing from the branches, and a `CONST` in them is "Name already in use"), and no branch is
  compiled with a guessed value

#### Scenario: Nothing sets a flag
- **WHEN** a program with none of these features tests `$IF _CONSOLE_ THEN`, `$IF _ASSERTS_ THEN`, `$IF _DEBUG_
  THEN` and `$IF _SOCKETS_ THEN`
- **THEN** none of the branches is taken

#### Scenario: A metacommand inside the branch it changes
- **WHEN** a program's only `$CONSOLE` stands inside `$IF _CONSOLE_ = 0 THEN` … `$END IF`
- **THEN** the program has a console and `_CONSOLE_` is 1 everywhere else in it, as with the old compiler

#### Scenario: $CONSOLE:ONLY inside the branch it changes
- **WHEN** a program's only `$CONSOLE:ONLY` stands inside `$IF _CONSOLE_ = 0 THEN` … `$END IF`
- **THEN** `_CONSOLE_` is 2, and the program has a console and a window (the line that would make it one without
  a window is not compiled), as with the old compiler

#### Scenario: OPTION _EXPLICIT inside the branch it changes
- **WHEN** a program's only `OPTION _EXPLICIT` stands inside `$IF _EXPLICIT_ = 0 THEN` … `$END IF`, and a line
  below uses a variable that is not declared
- **THEN** `_EXPLICIT_` is 1 and that use is the error of a variable that is not declared, as with the old
  compiler (the option is in force by its flag, though its line is not compiled in the last round)

#### Scenario: Three rounds
- **WHEN** a program's only `$CONSOLE` stands inside `$IF _EXPLICIT_ THEN` … `$END IF` and its only `OPTION
  _EXPLICIT` on the last line
- **THEN** the program has a console, and `_CONSOLE_` and `_EXPLICIT_` are 1 on every line

#### Scenario: A setting inside a branch a line above has switched off
- **WHEN** a program has `$CONSOLE` on its first line and below it `OPTION _EXPLICIT` inside `$IF _CONSOLE_ = 0
  THEN` … `$END IF`
- **THEN** `_EXPLICIT_` is 0: the branch is not taken in any round, as with the old compiler

### Requirement: Program settings in the IR
The IR SHALL say of a program, in terms that name no libqb symbol and no make flag: whether it has a console,
whether it has a window, whether the window is hidden at the start, and which optional parts of the runtime its
built-ins need. The IR dump SHALL show them.

#### Scenario: Console and hidden window
- **WHEN** a program with `$CONSOLE` and `$SCREENHIDE` is lowered
- **THEN** the IR says it has a console and a window that starts hidden

#### Scenario: Console only
- **WHEN** a program with `$CONSOLE:ONLY` is lowered
- **THEN** the IR says it has a console and no window

### Requirement: The build follows the program
The emitter and the driver SHALL give a program the start-up settings and the make flags the old compiler gives
it: a console, a console without a window, a window hidden at the start, and each optional part of the runtime
one of its built-ins needs. Two builds that differ in any of them SHALL NOT share a cached executable.

#### Scenario: A program with a console and a window
- **WHEN** a program with `$CONSOLE`, `$SCREENHIDE` and `_DEST _CONSOLE` is built and run
- **THEN** it links against the runtime with the window, shows no window, and its output equals the old
  compiler's

#### Scenario: A built-in that needs a part of the runtime
- **WHEN** a program names `_ICON`
- **THEN** it is built with the make flag the old compiler passes for the icon dependency

#### Scenario: Reference clone unchanged
- **WHEN** a program with `$CONSOLE` is built for the first time
- **THEN** `git status --porcelain` in the reference clone prints the same as before the build
