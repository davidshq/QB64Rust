$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): OPTION _EXPLICIT inside a SUB.
s
SYSTEM
SUB s
    OPTION _EXPLICIT
    DIM x
    x = 1
    PRINT x
END SUB
