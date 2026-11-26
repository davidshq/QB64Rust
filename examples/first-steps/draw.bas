' Draw with the mouse: hold the left button to draw, press a number key 1 to 9 for a color,
' C to clear the picture, and Esc to stop.
SCREEN _NEWIMAGE(640, 480, 32)
_TITLE "Draw with the mouse"
ink~& = _RGB32(255, 255, 255)
drawing = 0
DO
    WHILE _MOUSEINPUT
        x = _MOUSEX
        y = _MOUSEY
        IF _MOUSEBUTTON(1) THEN
            IF drawing THEN LINE (lastX, lastY)-(x, y), ink~& ELSE PSET (x, y), ink~&
            drawing = -1
        ELSE
            drawing = 0
        END IF
        lastX = x
        lastY = y
    WEND
    k$ = INKEY$
    IF k$ >= "1" AND k$ <= "9" THEN ink~& = colorOf~&(VAL(k$))
    IF UCASE$(k$) = "C" THEN CLS
    _LIMIT 120
LOOP UNTIL k$ = CHR$(27)
SYSTEM

' One of nine colors, by number.
FUNCTION colorOf~& (n)
    SELECT CASE n
        CASE 1: colorOf~& = _RGB32(255, 255, 255)
        CASE 2: colorOf~& = _RGB32(255, 64, 64)
        CASE 3: colorOf~& = _RGB32(64, 200, 64)
        CASE 4: colorOf~& = _RGB32(64, 128, 255)
        CASE 5: colorOf~& = _RGB32(255, 220, 0)
        CASE 6: colorOf~& = _RGB32(255, 128, 0)
        CASE 7: colorOf~& = _RGB32(200, 64, 255)
        CASE 8: colorOf~& = _RGB32(0, 220, 220)
        CASE ELSE: colorOf~& = _RGB32(128, 128, 128)
    END SELECT
END FUNCTION
