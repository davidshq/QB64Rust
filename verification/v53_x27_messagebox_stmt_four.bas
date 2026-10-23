$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x27_messagebox_stmt_four)
' the statement _MESSAGEBOX with four arguments.
k = 0
IF k = 1 THEN
    _MESSAGEBOX "a", "b", "ok", "info"
END IF
PRINT "ok"
SYSTEM
