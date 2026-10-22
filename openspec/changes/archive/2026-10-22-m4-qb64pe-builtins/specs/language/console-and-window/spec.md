# Spec Delta

## ADDED Requirements

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
`_SELECTFOLDERDIALOG$`) and `_SCREENMOVE` SHALL compile, in every form the old compiler accepts, to its runtime
calls. No test opens a dialog or moves the window.

#### Scenario: Optional arguments of a dialog
- **WHEN** a program has `_MESSAGEBOX "t", "m"`, `r& = _MESSAGEBOX("t", "m", "yesno", "warning")`, `f$ =
  _OPENFILEDIALOG$("t")` and `f$ = _SELECTFOLDERDIALOG$`
- **THEN** each compiles, and the call-site check finds each call equal to the old compiler's

#### Scenario: The window moved
- **WHEN** a program has `_SCREENMOVE 10, 20` and `_SCREENMOVE _MIDDLE`
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
