$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x49_display_fn_arg)
' the function _DISPLAY with an argument.
k = 0
IF k = 1 THEN
    x = _DISPLAY(1)
END IF
PRINT "ok"
SYSTEM
