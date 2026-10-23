$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x08_resize_fn_arg)
' the function _RESIZE with an argument.
k = 0
IF k = 1 THEN
    x = _RESIZE(1)
END IF
PRINT "ok"
SYSTEM
