$CONSOLE:ONLY
' Verification (m2-control-flow-slice, 4.3): OPTION _EXPLICIT, SHARED in a SUB that comes before the main module's DIM of the name.
OPTION _EXPLICIT
SUB s
    SHARED w AS LONG
    PRINT w
END SUB
DIM w AS LONG
w = 4
s
SYSTEM
