$CONSOLE:ONLY
' Verification (m4-builtins-param-types, task 1.1): an array parameter whose type is defined below the SUB.
DIM v(2) AS late
usea v()
SYSTEM
SUB usea (p() AS late)
    PRINT UBOUND(p)
END SUB
TYPE late
    k AS LONG
END TYPE
