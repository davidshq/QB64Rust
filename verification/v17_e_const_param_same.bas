$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): a SUB parameter with the name of a main CONST defined before it.
CONST k = 4
s 9
SYSTEM
SUB s (k)
    PRINT k
END SUB
