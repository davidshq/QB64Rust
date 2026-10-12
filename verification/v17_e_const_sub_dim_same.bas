$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): a SUB DIMs a local with the name of a main CONST defined before it.
CONST k = 4
s
SYSTEM
SUB s
    DIM k AS LONG
    k = 7
    PRINT k
END SUB
