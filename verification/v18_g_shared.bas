$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): DIM SHARED arrays used in a SUB and a FUNCTION.
DIM SHARED s(5) AS LONG
s(1) = 1
fill
PRINT "s(2) after fill:"; s(2); total&
SYSTEM

SUB fill
    s(2) = s(1) + 1
END SUB

FUNCTION total&
    total& = s(1) + s(2)
END FUNCTION
