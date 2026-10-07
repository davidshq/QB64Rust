$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): a member declared with a suffix instead of AS.
TYPE t
    x&
END TYPE
DIM p AS t
p.x = 5
PRINT p.x
SYSTEM
