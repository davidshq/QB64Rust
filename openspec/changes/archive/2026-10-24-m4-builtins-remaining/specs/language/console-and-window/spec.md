# Spec Delta

## MODIFIED Requirements

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

## ADDED Requirements

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
