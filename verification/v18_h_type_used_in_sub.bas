$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): a main-module TYPE used in a SUB and as a FUNCTION's local.
TYPE pt
    x AS LONG
END TYPE
PRINT twice&(4)
SYSTEM

FUNCTION twice& (n AS LONG)
    DIM p AS pt
    p.x = n * 2
    twice& = p.x
END FUNCTION
