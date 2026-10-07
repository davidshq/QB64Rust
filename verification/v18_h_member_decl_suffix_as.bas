$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): a member declared with a suffix and AS.
TYPE t
    x& AS LONG
END TYPE
DIM p AS t
p.x = 5
PRINT p.x
SYSTEM
