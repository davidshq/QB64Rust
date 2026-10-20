$CONSOLE:ONLY
' Verification (m4-devices-continue-paths, task 1.1): _CONTINUE in a SUB whose only loop is around its call.
FOR i = 1 TO 2
    s
NEXT
SYSTEM
SUB s
    _CONTINUE
END SUB
