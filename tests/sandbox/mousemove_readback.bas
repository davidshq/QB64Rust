' _MOUSEMOVE in a graphics screen and in a text screen, read back by _MOUSEX and _MOUSEY (tests/sandbox/README.md).
SCREEN _NEWIMAGE(320, 200, 32)
_DELAY 1
OPEN "out.txt" FOR OUTPUT AS #1

_MOUSEMOVE 50, 60
_DELAY 0.5
DO WHILE _MOUSEINPUT: LOOP
PRINT #1, "graphics, after _MOUSEMOVE 50, 60:"; _MOUSEX; _MOUSEY

' In a text screen the place is a column and a row.
SCREEN 0
_DELAY 1
_MOUSEMOVE 10, 5
_DELAY 0.5
DO WHILE _MOUSEINPUT: LOOP
PRINT #1, "text, after _MOUSEMOVE 10, 5:"; _MOUSEX; _MOUSEY
CLOSE #1
SYSTEM
