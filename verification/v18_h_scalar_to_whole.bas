$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): a whole TYPE variable assigned a number.
TYPE pt
    x AS LONG
END TYPE
DIM p AS pt
p = 5
PRINT p.x
SYSTEM
