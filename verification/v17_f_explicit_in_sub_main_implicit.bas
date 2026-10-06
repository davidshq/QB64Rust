$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): OPTION _EXPLICIT inside a SUB while main uses an implicit variable.
y = 1
PRINT y
s
SYSTEM
SUB s
    OPTION _EXPLICIT
END SUB
