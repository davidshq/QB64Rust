$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): a member used with its own type's suffix.
TYPE t
    x AS LONG
    s AS STRING
END TYPE
DIM p AS t
p.x& = 5
p.s$ = "five"
PRINT p.x; p.x&; p.s; p.s$
SYSTEM
