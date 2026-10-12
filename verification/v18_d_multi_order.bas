$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): several indexes: the order they are evaluated in, and which error ERR
' reports when two of them raise.
DIM m(3, 3) AS LONG
ON ERROR GOTO h
m(ix&(1), ix&(2)) = 5
PRINT m(1, 2)
PRINT m(ix&(1), ix&(2))
m(9, ASC("")) = 1
PRINT "after m(9, ASC(empty)) = 1"
m(ASC(""), 9) = 1
PRINT "after m(ASC(empty), 9) = 1"
SYSTEM

h:
PRINT "handler"; ERR
RESUME NEXT

FUNCTION ix& (n AS LONG)
    PRINT "index"; n
    ix& = n
END FUNCTION
