$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): a TYPE named like a keyword (PRINT).
TYPE print
    x AS LONG
END TYPE
DIM p AS print
p.x = 2
PRINT p.x
SYSTEM
