$CONSOLE:ONLY
' Verification (m3-window-output, task 5.2): a built-in statement's name and a colon at the start of a line
' is the statement with no argument, not a label; as code that is not reached. The first line runs: CLS in a
' program with a console, then the rest of its line.
DIM l AS LONG
CLS : PRINT "ok"
SYSTEM
CLS: l = 1
SCREEN: l = 2
COLOR: l = 3
LOCATE: l = 4
WIDTH: l = 5
VIEW: l = 6
WINDOW: l = 7
PALETTE: l = 8
SLEEP: l = 9
_DISPLAY: l = 10
CLOSE: l = 11
RANDOMIZE: l = 12
SHELL: l = 13
_SCREENHIDE: l = 14
_ICON: l = 15
_MOUSEHIDE: l = 16
_FREEIMAGE: l = 17
CLS : l = 20
CLS:
SLEEP:
