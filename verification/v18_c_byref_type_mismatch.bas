$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): an element of another type passed to a LONG parameter: a copy?
DIM d(3) AS DOUBLE
DIM i(3) AS INTEGER
d(1) = 2.5: bump d(1)
PRINT "bump d(1):"; d(1)
i(1) = 2: bump i(1)
PRINT "bump i(1):"; i(1)
SYSTEM

SUB bump (v AS LONG)
    PRINT "in bump"; v
    v = v + 1
END SUB
