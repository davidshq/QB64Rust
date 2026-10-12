$CONSOLE:ONLY
' Verification (m2-control-flow-slice, 4.3): OPTION _EXPLICIT, main has DIM x AS LONG, a SUB has SHARED x without AS (SINGLE).
OPTION _EXPLICIT
DIM x AS LONG
x = 4
s
SYSTEM
SUB s
    SHARED x
    PRINT x
END SUB
