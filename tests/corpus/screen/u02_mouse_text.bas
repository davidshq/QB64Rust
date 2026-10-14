' The mouse in a text mode (m3-images-fonts-mouse, task 4.3): _MOUSEX and _MOUSEY are the column and row of the
' window pixel of each event, kept inside the screen; the same pixels as in u01_mouse_queue, the corners of the
' window, and a click; an X where each event is. _MOUSEHIDDEN and _MOUSEDISABLED of a window whose cursor
' nothing hid or disabled.
DIM n AS INTEGER, k AS STRING
SCREEN 0: WIDTH 80, 25
COLOR 7, 1: CLS
WHILE _MOUSEINPUT: WEND
GOSUB announce
DO: k = INKEY$: LOOP WHILE k = ""
LOCATE 1, 30: PRINT " n  x  y b1"
DO WHILE _MOUSEINPUT
    n = n + 1
    LOCATE _MOUSEY, _MOUSEX: COLOR 14: PRINT "X"; : COLOR 7
    LOCATE 1 + n, 30: PRINT USING "## ## ## ##"; n; _MOUSEX; _MOUSEY; _MOUSEBUTTON(1)
LOOP
LOCATE 20, 30: PRINT "events:"; n; "key: "; k
LOCATE 21, 30: PRINT "hidden:"; _MOUSEHIDDEN; "disabled:"; _MOUSEDISABLED
SYSTEM
announce:
DEF SEG = 0: POKE 1264, (PEEK(1264) + 1) AND 255: DEF SEG
RETURN
