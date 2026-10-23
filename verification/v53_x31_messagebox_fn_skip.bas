$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x31_messagebox_fn_skip)
' the function _MESSAGEBOX with slots left out.
k = 0
IF k = 1 THEN
    r& = _MESSAGEBOX(, "m", , , 0)
END IF
PRINT "ok"
SYSTEM
