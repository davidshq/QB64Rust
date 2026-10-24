$CONSOLE
' Verification (m4-statements-metacommands, task 1.1): PALETTE USING at run time: values, an index past the end, mode 13.
$SCREENHIDE
ON ERROR GOTO h
DIM a(15) AS INTEGER
FOR i = 0 TO 15: a(i) = 15 - i: NEXT
PALETTE USING a(0)
_DEST _CONSOLE
PRINT "pal 0"; HEX$(_PALETTECOLOR(0, 0)); " pal 15"; HEX$(_PALETTECOLOR(15, 0))
a(3) = -1: PALETTE USING a(0)
PRINT "pal 3"; HEX$(_PALETTECOLOR(3, 0))
PALETTE USING a(10)
PRINT "past end done"
a(2) = 64: PALETTE USING a(0)
PRINT "after 64"
DIM b(15) AS LONG
FOR i = 0 TO 15: b(i) = 1: NEXT
PALETTE USING b
PRINT "pal 9"; HEX$(_PALETTECOLOR(9, 0))
SYSTEM
h:
PRINT "error"; ERR
RESUME NEXT
