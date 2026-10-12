$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): DIM of an array inside a SUB: new on each call?
show
show
SYSTEM

SUB show
    DIM t(5) AS LONG
    t(1) = t(1) + 1
    PRINT "t(1):"; t(1)
END SUB
