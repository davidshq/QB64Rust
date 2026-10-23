$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x44_controlchr_bare)
' the statement _CONTROLCHR with no word.
k = 0
IF k = 1 THEN
    _CONTROLCHR
END IF
PRINT "ok"
SYSTEM
