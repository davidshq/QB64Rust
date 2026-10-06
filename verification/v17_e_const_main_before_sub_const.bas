$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): main uses a name before a SUB, the SUB defines a CONST of that name.
pc = 1
PRINT pc
s
SYSTEM
SUB s
    CONST pc = 9
    PRINT pc
END SUB
