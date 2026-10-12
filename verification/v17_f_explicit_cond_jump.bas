$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): OPTION _EXPLICIT inside an IF block.
IF 1 THEN
    OPTION _EXPLICIT
END IF
x = 1
PRINT x
SYSTEM
