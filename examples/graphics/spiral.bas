' Spirals: lines turn around the middle of the window and change color as they go. Press any key to
' draw a new one, or Esc to stop.
SCREEN _NEWIMAGE(640, 640, 32)
_TITLE "Spirals"
RANDOMIZE TIMER
DO
    CLS
    turn = RND * 10 + 85
    length = 2
    angle = 0
    x = 320: y = 320
    FOR i = 1 TO 300
        nx = x + COS(_D2R(angle)) * length
        ny = y + SIN(_D2R(angle)) * length
        LINE (x, y)-(nx, ny), _RGB32(128 + 127 * SIN(i / 20), 128 + 127 * SIN(i / 30), 255 - i / 2)
        x = nx: y = ny
        angle = angle + turn
        length = length + 1.5
    NEXT
    DO
        k$ = INKEY$
        _LIMIT 30
    LOOP UNTIL k$ <> ""
LOOP UNTIL k$ = CHR$(27)
SYSTEM
