$CONSOLE:ONLY
' Take a 3-by-2 rectangle of an image with GET, put it back twice with PUT, and print each pixel's color.
DIM pic(10) AS INTEGER
canvas& = _NEWIMAGE(10, 4, 13)
_DEST canvas&
_SOURCE canvas&
LINE (0, 0)-(2, 1), 5, BF
PSET (1, 0), 7
GET (0, 0)-(2, 1), pic()
PUT (4, 0), pic(), PSET
LINE (4, 2)-(9, 3), 6, BF
PUT (7, 2), pic(), XOR
_DEST _CONSOLE
PRINT "Width in bits:"; pic(0); " Height:"; pic(1)
FOR y = 0 TO 3
    row$ = ""
    FOR x = 0 TO 9
        row$ = row$ + LTRIM$(STR$(POINT(x, y)))
    NEXT
    PRINT row$
NEXT
END
