' The mouse in a graphics mode (m3-images-fonts-mouse, task 4.3; mouse steps of the keys file, tools/screen/README.md):
' the queue read event by event with _MOUSEINPUT, and for each event _MOUSEX, _MOUSEY, the three buttons,
' _MOUSEWHEEL and, after the first move, the movement; a dot where each event is. The steps: a click (the spec's
' scenario: four events at 40, 30), a move, the right and the middle button, the wheel down and up, a last move.
' The movement of the first move counts from the real cursor, so it is not printed (design D4).
DIM n AS INTEGER, k AS STRING
SCREEN 12
WHILE _MOUSEINPUT: WEND
GOSUB announce
DO: k = INKEY$: LOOP WHILE k = ""
PRINT " n   x   y b1 b2 b3 wheel  dx  dy"
DO WHILE _MOUSEINPUT
    n = n + 1
    PSET (_MOUSEX, _MOUSEY), 10 + n MOD 6
    PRINT USING "### ### ###"; n; _MOUSEX; _MOUSEY;
    PRINT USING " ## ## ## #####"; _MOUSEBUTTON(1); _MOUSEBUTTON(2); _MOUSEBUTTON(3); _MOUSEWHEEL;
    IF n > 2 THEN PRINT USING " ### ###"; _MOUSEMOVEMENTX; _MOUSEMOVEMENTY ELSE PRINT
LOOP
PRINT "events:"; n; "key: "; k
PRINT "after:"; _MOUSEINPUT; _MOUSEX; _MOUSEY; _MOUSEBUTTON(2); _MOUSEWHEEL; _MOUSEWHEEL(1)
PRINT "hidden:"; _MOUSEHIDDEN; "disabled:"; _MOUSEDISABLED
SYSTEM
' It moves the machine's cursor, so no test runs it (design D3); its call is compared by tests\callsite.
_MOUSEMOVE 10, 20
announce:
DEF SEG = 0: POKE 1264, (PEEK(1264) + 1) AND 255: DEF SEG
RETURN
