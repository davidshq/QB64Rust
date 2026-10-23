$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x30_messagebox_skip)
' the statement _MESSAGEBOX with its first slots left out.
k = 0
IF k = 1 THEN
    _MESSAGEBOX , , "info"
END IF
PRINT "ok"
SYSTEM
