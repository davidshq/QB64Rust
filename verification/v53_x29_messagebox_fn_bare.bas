$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x29_messagebox_fn_bare)
' the function _MESSAGEBOX with no argument.
k = 0
IF k = 1 THEN
    r& = _MESSAGEBOX
END IF
PRINT "ok"
SYSTEM
