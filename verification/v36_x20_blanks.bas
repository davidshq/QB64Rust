$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (20_blanks)
DIM a(2) AS LONG, b(2) AS LONG
b(2) = 3
a ( ) = b ( )
PRINT a(2)
PRINT "ok"
SYSTEM
