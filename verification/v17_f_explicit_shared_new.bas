$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): OPTION _EXPLICIT, SHARED in a SUB naming a variable main never declared.
OPTION _EXPLICIT
s
SYSTEM
SUB s
    SHARED w AS LONG
    w = 1
    PRINT w
END SUB
