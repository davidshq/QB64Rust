# language/console-and-window Specification

## Purpose
How a program chooses a console, a window or both, where its text output and input go, and which window
built-ins are compiled. Every behaviour is the old compiler's, measured (`verification/v26_*`).

## Requirements

### Requirement: A program with a console
A program SHALL have a console when `$CONSOLE` or `$CONSOLE:ONLY` stands anywhere in its compiled code: on any
line, after the last statement that runs, inside a procedure, in an included file, once or several times. A
metacommand in a `$IF` branch that is not taken SHALL NOT count. A program with neither SHALL be compiled and
built with a window and no console, as the old compiler builds it (`language/screen-output`).

#### Scenario: $CONSOLE as the last line
- **WHEN** a program is `_DEST _CONSOLE`, `PRINT "ok"`, `SYSTEM`, `$SCREENHIDE`, `$CONSOLE`
- **THEN** it compiles and prints `ok`

#### Scenario: $CONSOLE inside a SUB
- **WHEN** `$CONSOLE` and `$SCREENHIDE` stand inside a SUB and the main module does `_DEST _CONSOLE` and `PRINT
  "ok"` before it calls the SUB
- **THEN** `ok` is printed

#### Scenario: Twice
- **WHEN** a program has `$CONSOLE` twice and `$SCREENHIDE` twice
- **THEN** it compiles as with each once

#### Scenario: Only in a branch that is not taken
- **WHEN** a program's only `$CONSOLE` stands between `$IF 0 THEN` and `$END IF`
- **THEN** the program is compiled as one with a window and no console

#### Scenario: Neither metacommand
- **WHEN** a program is `PRINT 1`
- **THEN** it compiles with no diagnostic, and its settings say a window and no console

### Requirement: $CONSOLE:ONLY
`$CONSOLE:ONLY` SHALL make the console the destination and the source at its line, each time it stands, and SHALL
make the program one without a window. A program with both `$CONSOLE:ONLY` and `$CONSOLE`, in either order, SHALL
be a program without a window.

#### Scenario: $CONSOLE, then $CONSOLE:ONLY
- **WHEN** a program is `$CONSOLE`, `$CONSOLE:ONLY`, `PRINT "printed with no _DEST"`, `SYSTEM`
- **THEN** it prints `printed with no _DEST`

#### Scenario: PRINT above $CONSOLE:ONLY
- **WHEN** a program is `PRINT "before both"`, `$CONSOLE:ONLY`, `$CONSOLE`, `PRINT "after both"`, `SYSTEM`
- **THEN** it prints `after both` only

#### Scenario: $CONSOLE:ONLY a second time
- **WHEN** a program is `$CONSOLE:ONLY`, `PRINT "console 1"`, `_DEST 0`, `PRINT "window"`, `$CONSOLE:ONLY`, `PRINT
  "console 2"`, `SYSTEM`
- **THEN** it prints `console 1` and `console 2`

#### Scenario: The shape of the upstream glut tests
- **WHEN** a program has `$CONSOLE:ONLY`, `PRINT "windows part"` and `SYSTEM` inside `$IF WINDOWS THEN`, and
  after it `$CONSOLE`, `_DEST _CONSOLE` and more statements
- **THEN** it prints `windows part` and ends, and the statements after it are compiled

### Requirement: $SCREENHIDE and $SCREENSHOW
`$SCREENHIDE` SHALL start the program with its window hidden and `$SCREENSHOW` with it shown; with several of
them, the last one in the file SHALL count, wherever they stand. Neither gives the program a console.

#### Scenario: Hidden window, output on the console
- **WHEN** a program is `$CONSOLE`, `$SCREENHIDE`, `_DEST _CONSOLE`, `PRINT "ok"`, `SYSTEM`
- **THEN** it prints `ok`, exits with code 0 and shows no window

#### Scenario: $SCREENSHOW after $SCREENHIDE
- **WHEN** a program has `$SCREENHIDE` and below it `$SCREENSHOW`
- **THEN** it is built to start with the window shown

#### Scenario: $SCREENHIDE alone
- **WHEN** a program is `$SCREENHIDE`, `PRINT "x"`, `SYSTEM`
- **THEN** it compiles with no diagnostic, as a program with a hidden window and no console

### Requirement: Forms of the console and window metacommands
`$CONSOLE`, `$CONSOLE:ONLY`, `$SCREENHIDE` and `$SCREENSHOW` SHALL be accepted in any letter case, with blanks
before and after, alone on their line. Every other form the old compiler rejects SHALL be an error of the program,
never "not supported yet".

#### Scenario: Case and blanks
- **WHEN** a program starts with `   $console` and `   $ScreenHide   `
- **THEN** both are accepted

#### Scenario: $CONSOLE:OFF and blanks around the colon
- **WHEN** a program has the line `$CONSOLE:OFF` or `$CONSOLE : ONLY`
- **THEN** the compile fails with an error at that line that is not "not supported yet"

#### Scenario: A word or a comment after the metacommand
- **WHEN** a program has `$CONSOLE junk`, `$CONSOLE:ONLY junk`, `$SCREENHIDE junk`, `$SCREENSHOW junk`, `$CONSOLE
  ' a comment`, `$CONSOLE:ONLY ' a comment` or `$SCREENHIDE ' a comment`
- **THEN** the compile fails with an error at that line that is not "not supported yet"

#### Scenario: A statement on the same line
- **WHEN** a program has `$CONSOLE: x = 1` or `x = 1: $CONSOLE`
- **THEN** the compile fails with an error at that line that is not "not supported yet"

#### Scenario: A tab before the metacommand
- **WHEN** a line is a tab followed by `$CONSOLE`
- **THEN** the compile fails with an error at that line that is not "not supported yet"

### Requirement: _DEST and _SOURCE
`_DEST handle` SHALL make the image or console with that handle the destination of text output, and `_SOURCE
handle` the source; the handle SHALL be a number of any numeric type, converted as the old compiler converts it.
The functions `_DEST` and `_SOURCE` SHALL give the current handles. A destination set in a procedure SHALL stay in
force after it returns. Output SHALL reach standard output only while the console is the destination.

#### Scenario: Switching the destination
- **WHEN** a program with `$CONSOLE` and `$SCREENHIDE` prints `window 1`, saves `d& = _DEST`, and then prints
  `console 1` after `_DEST _CONSOLE`, `window 2` after `_DEST d&`, `console 2` after `_DEST _CONSOLE`, `window 3`
  after `_DEST 0` and `console 3` after `_DEST _CONSOLE`
- **THEN** standard output holds `console 1`, `console 2` and `console 3`, one per line

#### Scenario: The handles
- **WHEN** that program prints `d&`, and `_DEST`, `_SOURCE` and `_CONSOLE` after `_DEST _CONSOLE`
- **THEN** it prints -2, -10, -2 and -10, and `_SOURCE` is -10 after `_SOURCE _CONSOLE`

#### Scenario: Set in a procedure
- **WHEN** a SUB does `_DEST _CONSOLE` and the main module prints after calling it
- **THEN** that output is on standard output

#### Scenario: A handle that does not exist
- **WHEN** `_DEST -99` runs under a handler with the console as destination
- **THEN** error 258 is raised and the next `PRINT` still goes to the console

#### Scenario: Rejected forms
- **WHEN** a program has `_DEST` with no argument, `_DEST "a"`, or `_CONSOLE = 1`
- **THEN** the compile fails with an error that is not "not supported yet"

### Requirement: _ECHO
`_ECHO text$` SHALL write the string and a line end to the console whatever the destination is, and SHALL leave the
destination as it was. In a program with no console it SHALL be a compile error, as in the old compiler.

#### Scenario: Echo with the window as destination
- **WHEN** a program with `$CONSOLE` and `$SCREENHIDE` runs `_ECHO "echo 1"`, `PRINT "window"`, `_ECHO "echo 2: "
  + STR$(_DEST)`
- **THEN** standard output holds `echo 1` and `echo 2: -2`

#### Scenario: No console
- **WHEN** a program without `$CONSOLE` and `$CONSOLE:ONLY` has `_ECHO "x"`
- **THEN** the compile fails with an error that says `_ECHO` needs `$CONSOLE` or `$CONSOLE:ONLY`

### Requirement: Console input in a $CONSOLE program
In a program with `$CONSOLE`, `INPUT` and `LINE INPUT` SHALL be compiled as under `$CONSOLE:ONLY`, and SHALL read
standard input while the console is the destination, with the prompts and the echo of `$CONSOLE:ONLY`. `_SOURCE
_CONSOLE` SHALL NOT be needed for it.

#### Scenario: INPUT after _DEST _CONSOLE
- **WHEN** a program with `$CONSOLE` and `$SCREENHIDE` runs `_DEST _CONSOLE`, `INPUT "n"; a`, `PRINT "a ="; a`,
  `LINE INPUT "l? "; l$`, `PRINT "l$ = ["; l$; "]"` with the input lines `5` and `hello there`
- **THEN** it prints `n? a = 5 ` and `l? l$ = [hello there]`

### Requirement: The end of a program and errors with a hidden window
A program with `$CONSOLE` SHALL end and report errors as the old compiler's program does: `SYSTEM` leaves at once;
`END`, and running off the last line, print the old compiler's "Press any key to continue" on the console and
wait; a trapped error is serviced as under `$CONSOLE:ONLY`; an untrapped error prints the old compiler's message.

#### Scenario: A trapped error
- **WHEN** a program with `$CONSOLE`, `$SCREENHIDE` and a handler that prints `ERR` and does `RESUME NEXT` runs
  `_DEST _CONSOLE`, `PRINT "before"`, `ERROR 5`, `PRINT "after"`
- **THEN** it prints `before`, the handler's line with 5, and `after`

#### Scenario: An untrapped error
- **WHEN** a program with `$CONSOLE` and `$SCREENHIDE` runs `_DEST _CONSOLE`, `PRINT "before"` and `ERROR 5` on
  line 7, with `QB64PE_NOPROMPT=y`
- **THEN** it prints `before`, an empty line, `Runtime error: Line: 7 (in main module)` and `Illegal function
  call`, and exits with code 0

### Requirement: Window built-ins are compiled
The window built-ins SHALL be compiled in every form the old compiler accepts, each with the runtime call the old
compiler writes for it: the statements `_SCREENHIDE`, `_SCREENSHOW`, `_SCREENICON`, `_TITLE`, `_ICON`,
`_MOUSESHOW`, `_MOUSEHIDE`, `_MOUSECURSOR`, `_WINDOWSIZELIMIT`, `_FREEIMAGE`, and the functions `_SCREENEXISTS`,
`_SCREENICON`, `_TITLE$`, `_DESKTOPWIDTH`, `_DESKTOPHEIGHT`, `_SCREENX`, `_SCREENY`, `_WINDOWHANDLE`,
`_WINDOWHASFOCUS`, `_NEWIMAGE`. What they do in a window is not part of this capability yet.

#### Scenario: A statement with parenthesised pairs
- **WHEN** a program has `_WINDOWSIZELIMIT`, `_WINDOWSIZELIMIT (320, 240)`, `_WINDOWSIZELIMIT -(800, 600)` and
  `_WINDOWSIZELIMIT (320, 240)-(800, 600)`
- **THEN** each compiles, and the call-site check finds each call equal to the old compiler's

#### Scenario: Optional arguments
- **WHEN** a program has `_MOUSECURSOR img`, `_MOUSECURSOR img, (16, 16)`, `_ICON`, `_MOUSESHOW`, `_MOUSEHIDE`,
  `PRINT _WINDOWHANDLE <> 0` and `PRINT _WINDOWHANDLE(1) <> 0`
- **THEN** each compiles, and the call-site check finds each call equal to the old compiler's

#### Scenario: Compiled and never run
- **WHEN** an upstream `glut` test is compiled and run on Windows
- **THEN** it prints what its `.output` file holds, the window built-in after its `SYSTEM` never running

### Requirement: $EXEICON
A line `$EXEICON:'name'` SHALL give the program's executable the icon in the file `name`, and SHALL be, where it
stands, the statement `_ICON` with no argument, as in the old compiler. The name stands between the first two
single quotes of the line; what follows the second SHALL be dropped. A name that starts with `./` or `.\` SHALL
be looked for in the folder of the main source file. A line in a `$IF` branch that is not taken SHALL NOT count.

#### Scenario: The game's line
- **WHEN** a program with `$EXEICON: './cws.ico'` is compiled and `cws.ico` lies beside it
- **THEN** there is no diagnostic, the line compiles to the call the old compiler writes for it, and the
  executable carries the icon

#### Scenario: Reached in a console program
- **WHEN** a program with `$CONSOLE:ONLY` runs `PRINT "before"`, the line `$EXEICON:'./icon.ico'` and
  `PRINT "after"`
- **THEN** it prints both lines and ends, as with the old compiler

#### Scenario: What follows the name
- **WHEN** a program has `$EXEICON:'./icon.ico' and more`, `$EXEICON:'./icon.ico' ' the icon` or
  `$EXEICON:'./icon.ico': PRINT "same line"`
- **THEN** each compiles as `$EXEICON:'./icon.ico'` does, and the `PRINT` of the third is not part of the
  program, as with the old compiler

#### Scenario: Case, blanks and backslashes
- **WHEN** a program has `   $exeicon:'  ./icon.ico  '` or a name written `.\icon.ico`
- **THEN** each compiles as `$EXEICON:'./icon.ico'` does

#### Scenario: Inside a SUB and after the last statement
- **WHEN** the line stands inside a SUB that is never called, or after `SYSTEM` as the last line
- **THEN** the program compiles and its executable carries the icon

#### Scenario: In a comment
- **WHEN** a program has `'$EXEICON:'./missing.ico'` or `REM $EXEICON:'./missing.ico'`
- **THEN** the line is a comment: there is no diagnostic, no statement and no icon, as with the old compiler

#### Scenario: With an _ICON statement
- **WHEN** a program has `$EXEICON:'./icon.ico'` and a line `_ICON`
- **THEN** it compiles, each line to the call the old compiler writes for it

#### Scenario: A branch that is not taken
- **WHEN** `$EXEICON:'./missing.ico'` stands between `$IF 0 THEN` and `$END IF`
- **THEN** there is no diagnostic and the executable carries no icon of the program's

### Requirement: Errors of $EXEICON
The compiler SHALL report an error that is not "not supported yet" for a `$EXEICON` line without a name between
single quotes, for a second `$EXEICON` line of a program (whatever it names), and for a name whose file is not
there, each at the line, as the old compiler does. A relative name that does not start with `./` or `.\` SHALL be
reported "not supported yet". The line rules of every metacommand SHALL hold for it.

#### Scenario: No name
- **WHEN** a program has `$EXEICON`, `$EXEICON:''`, `$EXEICON:"./icon.ico"` or `$EXEICON:'./icon.ico`
- **THEN** the compiler reports that it expected `$EXEICON:'filename'`

#### Scenario: Twice
- **WHEN** a program has `$EXEICON:'./icon.ico'` on line 4 and a `$EXEICON` line further down
- **THEN** the compiler reports at the second line that `$EXEICON` is already defined in line 4

#### Scenario: A missing file
- **WHEN** a program has `$EXEICON:'./missing.ico'` and no such file lies beside it
- **THEN** the compiler reports that the file is not found, naming it as it is written in the program

#### Scenario: A relative name without ./
- **WHEN** a program has `$EXEICON:'icons/icon.ico'` or `$EXEICON:'/icons/icon.ico'`
- **THEN** the compiler reports "not supported yet" at the line (the old compiler looks in its own folder, or on
  its own drive)

#### Scenario: After a colon, and a tab before it
- **WHEN** a program has `l = 1: $EXEICON:'./icon.ico'` or a tab before `$EXEICON`
- **THEN** the compiler reports an error, as for any metacommand
