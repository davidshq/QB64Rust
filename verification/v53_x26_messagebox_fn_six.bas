$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x26_messagebox_fn_six)
' the function _MESSAGEBOX with six arguments.
k = 0
IF k = 1 THEN
    r& = _MESSAGEBOX("a", "b", "ok", "info", 1, 2)
END IF
PRINT "ok"
SYSTEM
