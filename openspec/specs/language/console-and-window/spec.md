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
wait for a key when the program's standard input is a console; when it is not (a file, a pipe, the null device)
they print the line and end at once (`study\39` row 2; a `QB64PE-DIVERGENCES.md` row); a trapped error is serviced
as under `$CONSOLE:ONLY`; an untrapped error prints the old compiler's message.

#### Scenario: A trapped error
- **WHEN** a program with `$CONSOLE`, `$SCREENHIDE` and a handler that prints `ERR` and does `RESUME NEXT` runs
  `_DEST _CONSOLE`, `PRINT "before"`, `ERROR 5`, `PRINT "after"`
- **THEN** it prints `before`, the handler's line with 5, and `after`

#### Scenario: An untrapped error
- **WHEN** a program with `$CONSOLE` and `$SCREENHIDE` runs `_DEST _CONSOLE`, `PRINT "before"` and `ERROR 5` on
  line 7, with `QB64PE_NOPROMPT=y`
- **THEN** it prints `before`, an empty line, `Runtime error: Line: 7 (in main module)` and `Illegal function
  call`, and exits with code 0

#### Scenario: END with its input from the null device
- **WHEN** a `$CONSOLE:ONLY` program runs `PRINT "before END"` and `END`, with its standard input the null device
- **THEN** it prints `before END`, an empty line and `Press any key to continue`, and exits with code 0 without
  waiting

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
be looked for in the folder of the main source file. Any other relative name SHALL be looked for relative to the
compiler root, as the old compiler looks in its own folder; a name with a single leading slash, on the compiler
root's drive. A line in a `$IF` branch that is not taken SHALL NOT count.

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

#### Scenario: Relative to the compiler root
- **WHEN** a program has `$EXEICON:'tests/compile_tests/icon/qb64pe.ico'` and is compiled with
  `--include-root tests/upstream/root`, under which that file lies
- **THEN** there is no diagnostic and the executable carries that icon (`icon/test`, `winresource/test`)

### Requirement: Errors of $EXEICON
The compiler SHALL report an error that is not "not supported yet" for a `$EXEICON` line without a name between
single quotes, for a second `$EXEICON` line of a program (whatever it names), for a name whose file is not
there, and for a file that is there but is no icon file, each at the line, as the old compiler does (for the
last, its C++ build fails). A name with a drive and no slash after it (`C:icon.ico`) SHALL be reported "not
supported yet". The line rules of every metacommand SHALL hold for it.

#### Scenario: No name
- **WHEN** a program has `$EXEICON`, `$EXEICON:''`, `$EXEICON:"./icon.ico"` or `$EXEICON:'./icon.ico`
- **THEN** the compiler reports that it expected `$EXEICON:'filename'`

#### Scenario: Twice
- **WHEN** a program has `$EXEICON:'./icon.ico'` on line 4 and a `$EXEICON` line further down
- **THEN** the compiler reports at the second line that `$EXEICON` is already defined in line 4

#### Scenario: A missing file
- **WHEN** a program has `$EXEICON:'./missing.ico'` and no such file lies beside it
- **THEN** the compiler reports that the file is not found, naming it as it is written in the program

#### Scenario: A file that is no icon
- **WHEN** a program has `$EXEICON:'./v29_inc/not_an_icon.ico'` and that file holds text
- **THEN** the compiler reports at the line, before any C++ is built, that the file is not an icon file, naming
  it as it is written in the program (`verification\v29_e16_not_an_icon`); an icon file of one or of five images
  gives no diagnostic

#### Scenario: A relative name without ./
- **WHEN** a program has `$EXEICON:'v29_inc/icon.ico'`, `$EXEICON:'missing.ico'` or `$EXEICON:'v29_none/icon.ico'`
  and no such file lies under the compiler root
- **THEN** the compiler reports that the file is not found, as the old compiler does (`verification\v29_e02`,
  `e03`, `e08`, `e18`)

#### Scenario: A drive with no slash after it
- **WHEN** a program has `$EXEICON:'C:icon.ico'`
- **THEN** the compiler reports "not supported yet" at the line (the old compiler looks in the folder it stands
  in on that drive)

#### Scenario: After a colon, and a tab before it
- **WHEN** a program has `l = 1: $EXEICON:'./icon.ico'` or a tab before `$EXEICON`
- **THEN** the compiler reports an error, as for any metacommand

### Requirement: $VERSIONINFO
A line whose upper case starts with `$VERSIONINFO` SHALL set one field of the executable's version resource, as in
the old compiler: the key stands between the line's first `:` and the first `=` after it, blanks trimmed, in any
letter case; the value is the rest of the line, trimmed, each `"` read as `'`, one pair of enclosing `'` removed.
The last line of a key SHALL count. A line in a `$IF` branch that is not taken SHALL NOT count.

#### Scenario: Every key
- **WHEN** a program sets `CompanyName`, `FileDescription`, `FILEVERSION#=1,2,3,4`, `PRODUCTVERSION#=5,6,7,8`,
  `FileVersion`, `InternalName`, `LegalCopyright`, `LegalTrademarks`, `OriginalFilename`, `ProductName`,
  `ProductVersion`, `Comments` and `Web`, each in single quotes
- **THEN** it compiles with no diagnostic, and the version resource of its executable holds each value, as the
  old compiler's does

#### Scenario: The string version follows the numeric one
- **WHEN** a program has `$VERSIONINFO:FILEVERSION#=1,2,3,4` and no `FileVersion`, or `FileVersion='text'` before
  `FILEVERSION#=1,2,3,4`
- **THEN** the string file version is `1,2,3,4` in the first and `text` in the second; a `FileVersion` after the
  numeric key replaces the digits

#### Scenario: Forms of a line
- **WHEN** a program has `$versioninfo:companyname="double quoted"`, `$VERSIONINFO : FileDescription = 'spaced'`,
  `$VERSIONINFO:InternalName='a:b=c'` and `$VERSIONINFOX:ProductName='prefix'`
- **THEN** the values are `double quoted`, `spaced`, `a:b=c` and `prefix`, as with the old compiler

#### Scenario: A value without quotes
- **WHEN** a program compiled with `-w` has `$VERSIONINFO:ProductName=no quotes` or
  `$VERSIONINFO:Comments='a' ' trailing comment`
- **THEN** each is the old compiler's warning "Missing string bracket delimiters", and the values are `no quotes`
  and `'a' ' trailing comment`

#### Scenario: Places
- **WHEN** a `$VERSIONINFO` line stands in a `$IF` branch that is not taken, in a SUB, and in an included file
- **THEN** the first sets nothing and the other two set their fields

#### Scenario: Rejected lines
- **WHEN** a program has `$VERSIONINFO`, `$VERSIONINFO:=x`, `$VERSIONINFO:CompanyName`, `$VERSIONINFO:Author='x'`,
  `$VERSIONINFO:FILEVERSION#=1,2,3` or `$VERSIONINFO:FILEVERSION#=1, 2,3,4`
- **THEN** each is a compile error at that line, not "not supported yet", where the old compiler rejects it

#### Scenario: Numeric parts the resource compiler refuses
- **WHEN** a program has `$VERSIONINFO:FILEVERSION#=1,,2,3`, `=,1,2,3`, `=70000,0,0,0` or `=08,0,0,0`
- **THEN** each is a compile error that names the part, where the old compiler's build fails; `=65535,0,0,0` and
  `=010,0,0,0` (octal, 8) compile

#### Scenario: A manifest Windows would refuse
- **WHEN** the company name, the product name or the description holds `<`, `&` or a byte that is not UTF-8
- **THEN** it is a compile error that names the character, where the old compiler's executable does not start;
  `>`, UTF-8 and an `&` in another value compile

### Requirement: $RESIZE
`$RESIZE:ON`, `$RESIZE:OFF`, `$RESIZE:STRETCH` and `$RESIZE:SMOOTH`, in any case of letters, SHALL build the
program with the window's resizing allowed or not and its scaling as the old compiler's build has them. With
several, the last in the file SHALL count. Every other form SHALL be a compile error, as in the old compiler, never
"not supported yet" (`verification\v53_x70`–`x81`).

#### Scenario: Resizing allowed
- **WHEN** a program with `$RESIZE:ON` is built
- **THEN** it is built with the window's resizing allowed and no scaling, as the old compiler builds it

#### Scenario: The last one counts
- **WHEN** a program has `$RESIZE:SMOOTH` and below it `$RESIZE:OFF`
- **THEN** it is built with resizing not allowed

#### Scenario: A form the old compiler does not know
- **WHEN** a program has `$RESIZE:BOGUS`, `$RESIZE` alone, `$RESIZE: ON` or `$RESIZE :ON` (a blank by the colon),
  `$RESIZE:ON:ON`, or `$RESIZE:ON` with a word or a comment after it
- **THEN** it is a compile error, as in the old compiler, and not "not supported yet"

#### Scenario: Lower case
- **WHEN** a program has `$resize:on`
- **THEN** it is built as with `$RESIZE:ON`

#### Scenario: In a SUB and without a window
- **WHEN** `$RESIZE:ON` is in a SUB, or in a `$CONSOLE:ONLY` program
- **THEN** it is accepted, as in the old compiler

### Requirement: _RESIZE and the size it asks for
The statement `_RESIZE [{ON|OFF}][, {_STRETCH|_SMOOTH}]` and the functions `_RESIZE`, `_RESIZEWIDTH` and
`_RESIZEHEIGHT` SHALL compile to the old compiler's runtime calls. In a program whose window the user has not
resized, the functions SHALL give what the old compiler's program gives.

#### Scenario: Each form of the statement
- **WHEN** a program has `_RESIZE ON`, `_RESIZE OFF`, `_RESIZE ON, _STRETCH`, `_RESIZE , _SMOOTH` and `_RESIZE`
- **THEN** each compiles, and the call-site check finds each call equal to the old compiler's

#### Scenario: No resize by the user
- **WHEN** a screen program with `$RESIZE:ON` prints `_RESIZE; _RESIZEWIDTH; _RESIZEHEIGHT`
- **THEN** its screen holds what the old compiler's program's screen holds

### Requirement: The console shown and hidden
`_CONSOLE ON` and `_CONSOLE OFF` SHALL compile to the old compiler's runtime call. In a program with a console of
its own, `_CONSOLE OFF` SHALL hide the console's window and `_CONSOLE ON` SHALL show it again, as libqb does.
Output SHALL still go to the console while it is off, as in the old compiler's program (measured with the console a
pipe, `verification\v53_a_window`). In a program without a console, both SHALL do nothing.

#### Scenario: Output while the console is off
- **WHEN** a `$CONSOLE` program runs `_DEST _CONSOLE: PRINT "a": _CONSOLE OFF: PRINT "b": _CONSOLE ON: PRINT "c"`
  with its console a pipe
- **THEN** its output is `a`, `b` and `c` on three lines, as the old compiler's program's output is

#### Scenario: A wrong form
- **WHEN** a program has `_CONSOLE`, `_CONSOLE 1` or `_CONSOLE ON, OFF` as a statement
- **THEN** each is a compile error, as in the old compiler

### Requirement: _EXIT
`_EXIT` SHALL compile to the old compiler's runtime call. From its first call, the program SHALL no longer end when
its window is closed or Ctrl+Break is pressed; `_EXIT` SHALL give the old compiler's value for such a request, and
0 when none came.

#### Scenario: No request to end
- **WHEN** a program prints `_EXIT` and ends with `SYSTEM`
- **THEN** it prints ` 0 ` and ends

### Requirement: Dialogs and the window's place are compiled
The dialogs (`_MESSAGEBOX` as statement and function, `_OPENFILEDIALOG$`, `_SAVEFILEDIALOG$`,
`_SELECTFOLDERDIALOG$`, `_COLORCHOOSERDIALOG`, `_INPUTBOX$`), `_NOTIFYPOPUP`, `_SCREENMOVE`, `_SCREENCLICK` and
`_SCREENPRINT` SHALL compile, in every form the old compiler accepts, to its runtime calls, and the forms it
rejects SHALL be its errors. No test opens a dialog, shows a notification, moves the window, or clicks or types
on the desktop.

#### Scenario: Optional arguments of a dialog
- **WHEN** a program has `_MESSAGEBOX "t", "m"`, `r& = _MESSAGEBOX("t", "m", "yesno", "warning")`, `f$ =
  _OPENFILEDIALOG$("t")` and `f$ = _SELECTFOLDERDIALOG$`
- **THEN** each compiles, and the call-site check finds each call equal to the old compiler's

#### Scenario: The window moved
- **WHEN** a program has `_SCREENMOVE 10, 20` and `_SCREENMOVE _MIDDLE`
- **THEN** each compiles, and the call-site check finds each call equal to the old compiler's

#### Scenario: A colour and a line of text asked for
- **WHEN** a program has `c& = _COLORCHOOSERDIALOG`, `c& = _COLORCHOOSERDIALOG("t", &HFF0000)`,
  `a$ = _INPUTBOX$` and `a$ = _INPUTBOX$("t", "m", "d")`
- **THEN** each compiles, and the call-site check finds each call equal to the old compiler's

#### Scenario: A notification
- **WHEN** a program has `_NOTIFYPOPUP`, `_NOTIFYPOPUP "t"` and `_NOTIFYPOPUP "t", "m", "warning"`
- **THEN** each compiles, and the call-site check finds each call equal to the old compiler's

#### Scenario: A click and keys on the desktop
- **WHEN** a program has `_SCREENCLICK 10, 20`, `_SCREENCLICK 10, 20, 2` and `_SCREENPRINT "abc"`
- **THEN** each compiles, and the call-site check finds each call equal to the old compiler's

### Requirement: Files dropped on the window
`_ACCEPTFILEDROP` as statement and function, `_TOTALDROPPEDFILES`, `_DROPPEDFILE$` with and without its `$`, and
`_FINISHDROP` SHALL compile, in every form the old compiler accepts, to its runtime calls. With no file dropped,
they SHALL give what the old compiler's program gives.

#### Scenario: Dropped files
- **WHEN** a program has `_ACCEPTFILEDROP`, `_ACCEPTFILEDROP OFF`, `PRINT _ACCEPTFILEDROP`,
  `PRINT _TOTALDROPPEDFILES`, `f$ = _DROPPEDFILE$(1)`, `f$ = _DROPPEDFILE` and `_FINISHDROP`
- **THEN** each compiles, and the call-site check finds each call equal to the old compiler's

#### Scenario: Nothing dropped
- **WHEN** a program with a window runs `_ACCEPTFILEDROP: PRINT _ACCEPTFILEDROP; _TOTALDROPPEDFILES: _FINISHDROP`
- **THEN** it prints what the old compiler's program prints

### Requirement: The function _SCREENHIDE
`_SCREENHIDE` as a value SHALL be compiled with the runtime call the old compiler writes for it
(`func__screenhide`), and SHALL give what that call gives: -1 when the window is hidden, 0 when it is not and in a
`$CONSOLE:ONLY` program (measured, `verification\v57_h01`–`h04`). The statement `_SCREENHIDE` SHALL stay as it
is.

#### Scenario: Compiled beside the window statements
- **WHEN** a program has `n = _SCREENHIDE` after its `SYSTEM`, beside the other window statements after `CALL`
- **THEN** it compiles and prints `ok` (`verification\v26_x74_window_accepted`), and the call-site check finds
  the call equal to the old compiler's

#### Scenario: After $SCREENHIDE
- **WHEN** a program with `$CONSOLE` and `$SCREENHIDE` prints `_SCREENHIDE`
- **THEN** it prints `-1` (`verification\v57_h01`)

#### Scenario: After the statement and _SCREENSHOW
- **WHEN** a program with `$CONSOLE` runs the statement `_SCREENHIDE`, prints `_SCREENHIDE`, runs `_SCREENSHOW` and
  prints it again
- **THEN** it prints `-1`, then `0` (`verification\v57_h03`)

### Requirement: INPUT into a place in parentheses
`INPUT (a)` and `LINE INPUT (a$)` SHALL read into the variable inside the parentheses, as the old compiler's
program does (measured, `verification\v57_p06`, `p07`).

#### Scenario: INPUT
- **WHEN** `INPUT (a): PRINT a` runs with the input `4`
- **THEN** it prints the prompt `? ` and ` 4 `, as the old compiler's program (`verification\v57_p06`)

### Requirement: The console's own statements and functions
`_CONSOLETITLE title$`, `_CONSOLECURSOR`, `_CONSOLEFONT`, and the functions `_CINP` and `_CONSOLEINPUT` SHALL
compile in every form the old compiler accepts with its runtime calls, and SHALL do what its program does in a
program with a console. `_CONSOLEFONT` changes the font of the console the program shares, so no test runs it.

#### Scenario: The console's title and cursor
- **WHEN** a `$CONSOLE:ONLY` program runs `_CONSOLETITLE "t"`, `_CONSOLECURSOR _HIDE`, `_CONSOLECURSOR _SHOW, 50`
  and prints `"ok"`
- **THEN** it prints what the old compiler's program prints

#### Scenario: Console input with no key waiting
- **WHEN** a `$CONSOLE:ONLY` program with no input waiting prints `_CONSOLEINPUT` and `_CINP`
- **THEN** it prints what the old compiler's program prints (`verification\v58`)

#### Scenario: The font is compiled
- **WHEN** a program has `_CONSOLEFONT "Consolas", 16`
- **THEN** it compiles, and the call-site check finds the call equal to the old compiler's

### Requirement: The window's display settings
The statements `_GLRENDER`, `_DISPLAYORDER`, `_ALLOWFULLSCREEN` and `_FPS`, and the functions `_SCALEDWIDTH`,
`_SCALEDHEIGHT`, `_SMOOTH` and `_DESKTOPREFRESHRATE`, SHALL compile in every form the old compiler accepts with
its runtime calls, and the forms it rejects SHALL be its errors (measured, `verification\v58`).

#### Scenario: The forms of the statements
- **WHEN** a program has `_GLRENDER _BEHIND`, `_DISPLAYORDER _HARDWARE, _SOFTWARE`, `_ALLOWFULLSCREEN`,
  `_ALLOWFULLSCREEN _STRETCH, _SMOOTH`, `_ALLOWFULLSCREEN _OFF`, `_FPS 30` and `_FPS _AUTO`
- **THEN** each compiles, and the call-site check finds each call equal to the old compiler's

#### Scenario: The values in a window
- **WHEN** a screen program prints `_SCALEDWIDTH; _SCALEDHEIGHT; _SMOOTH` after `SCREEN _NEWIMAGE(320, 200, 32)`
  and `_DESKTOPREFRESHRATE > 0`
- **THEN** it prints what the old compiler's program prints

#### Scenario: A rejected form
- **WHEN** a program has a form `v58` records the old compiler rejecting, such as `_GLRENDER` alone, `_FPS 30, 1`,
  `_FPS "a"`, `_ALLOWFULLSCREEN 1`, `_ALLOWFULLSCREEN _STRETCH, _STRETCH`, `_DISPLAYORDER _SOFTWARE,` or
  `_SCALEDWIDTH(1)`
- **THEN** the compile fails with an error whose note is the old compiler's message (`verification\v58_x37`–`x48`,
  `x89`, `x90`)

#### Scenario: Forms the template allows
- **WHEN** a program has `_DISPLAYORDER` alone, `_DISPLAYORDER _SOFTWARE, _SOFTWARE`, `_FPS` alone or
  `_CONSOLECURSOR` alone
- **THEN** each compiles, as the old compiler compiles it (`verification\v58_x33`, `x40`, `x41`, `x44`)

### Requirement: Console input at the end of the input
In a console program, `INPUT`, `LINE INPUT` and `INPUT$(n)` with no file number SHALL raise error 62 (Input past
end of file), trappable, when standard input has ended and nothing of the line was read, and SHALL never wait for
input that cannot come; a last line without a newline SHALL be read as a line (`study\39` row 2; a
`QB64PE-DIVERGENCES.md` row).

#### Scenario: INPUT after the last line
- **WHEN** a `$CONSOLE:ONLY` program with `ON ERROR GOTO h` (printing `ERR`, then `RESUME NEXT`) reads two
  `INPUT` lines from a standard input holding one line
- **THEN** the first `INPUT` reads the line, the second raises error 62, and the program goes on to its end

#### Scenario: LINE INPUT with no handler
- **WHEN** a `$CONSOLE:ONLY` program runs `LINE INPUT s$` with its standard input the null device and no handler,
  with `QB64PE_NOPROMPT=y`
- **THEN** it ends with libqb's message for error 62 and does not wait

#### Scenario: The variables after the error
- **WHEN** `a = 7: b$ = "old"`, then `INPUT a, b$` raises error 62 under a handler that resumes next, and the
  program prints them
- **THEN** it prints 0 and an empty string, the values `INPUT` gives them before it reads

#### Scenario: INPUT$ from a file
- **WHEN** a `$CONSOLE:ONLY` program whose standard input holds `abcdef` and a newline runs `s$ = INPUT$(3)` and
  then `t$ = INPUT$(10)` under a handler that resumes next
- **THEN** `s$` is `abc`, and the second raises error 62 after reading the rest of the input

#### Scenario: A last line without a newline
- **WHEN** a `$CONSOLE:ONLY` program runs `LINE INPUT s$: PRINT s$` with standard input `abc` and no newline
- **THEN** it prints `abc`
