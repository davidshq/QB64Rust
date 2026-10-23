$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x46_controlchr_fn_arg)
' the function _CONTROLCHR with an argument.
k = 0
IF k = 1 THEN
    x = _CONTROLCHR(1)
END IF
PRINT "ok"
SYSTEM
