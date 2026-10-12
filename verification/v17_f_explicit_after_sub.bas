$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): OPTION _EXPLICIT after a SUB definition.
SUB s
    y = 1
END SUB
OPTION _EXPLICIT
DIM x
x = 1
PRINT x
SYSTEM
