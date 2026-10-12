$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): a.b used before DIM a AS t: a plain variable until the DIM line?
TYPE t
    b AS LONG
END TYPE
a.b = 3
PRINT "before DIM:"; a.b
DIM a AS t
PRINT "after DIM:"; a.b
a.b = 5
PRINT "after a.b = 5:"; a.b
SYSTEM
