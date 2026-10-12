$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): a plain DIM array of the main module is not seen in a SUB: there the name
' is an implicit array of the SUB.
DIM x(5) AS LONG
x(1) = 11
ON ERROR GOTO h
show
PRINT "x(1):"; x(1)
SYSTEM

h:
PRINT "handler"; ERR
RESUME NEXT

SUB show
    PRINT "x(1) inside show:"; x(1)
    x(2) = 5
    PRINT "x(2) inside show:"; x(2)
END SUB
