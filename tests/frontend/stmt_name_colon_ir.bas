' TEST: ir
' A built-in statement's name and a colon at the start of a line is the statement with no argument, never a label
' (m3-window-output task 5.2; measured for 17 statements, verification\v27_x58): one built-in operation with every
' slot absent, and the rest of the line goes on. Civil War Strategy writes `CLS : LOCATE 1, 1` so ten times. A
' name that is no statement stays a label
DIM l AS LONG
CLS: l = 1
CLS : LOCATE 1, 30: COLOR 11
SCREEN: l = 3
COLOR: l = 4
LOCATE: l = 5
WIDTH: l = 6
VIEW: l = 7
WINDOW: l = 8
PALETTE: l = 9
SLEEP: l = 10
_DISPLAY: l = 11
CLOSE: l = 12
RANDOMIZE: l = 13
SHELL: l = 14
_SCREENHIDE: l = 15
_MOUSEHIDE: l = 16
_FREEIMAGE: l = 17
IF l THEN
    CLS : l = 18
END IF
CLS:
again: l = 19
GOTO again
SYSTEM
