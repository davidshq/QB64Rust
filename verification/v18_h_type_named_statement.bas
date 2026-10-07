$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): a TYPE named like a built-in statement (CLS).
TYPE cls
    x AS LONG
END TYPE
DIM p AS cls
p.x = 2
PRINT p.x
SYSTEM
