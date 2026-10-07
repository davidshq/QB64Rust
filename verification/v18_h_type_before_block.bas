$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): a TYPE used before its block.
DIM p AS pt
TYPE pt
    x AS LONG
END TYPE
p.x = 5
PRINT p.x
SYSTEM
