$CONSOLE:ONLY
' Draw on an image 12 pixels wide and 5 high, then print each pixel's color as a digit.
canvas& = _NEWIMAGE(12, 5, 13)
_DEST canvas&
LINE (0, 0)-(11, 4), 1, B
LINE (2, 1)-(4, 3), 2, BF
LINE (6, 1)-(10, 3), 3
LINE -STEP(0, -2), 4
_DEST _CONSOLE
_SOURCE canvas&
FOR y = 0 TO 4
    row$ = ""
    FOR x = 0 TO 11
        row$ = row$ + LTRIM$(STR$(POINT(x, y)))
    NEXT
    PRINT row$
NEXT
END
