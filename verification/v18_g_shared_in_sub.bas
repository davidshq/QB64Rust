$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): SHARED a() inside a SUB for a main-module array.
DIM x(5) AS LONG
x(1) = 11
show
SYSTEM

SUB show
    SHARED x() AS LONG
    PRINT "x(1):"; x(1)
END SUB
