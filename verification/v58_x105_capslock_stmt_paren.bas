$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x105_capslock_stmt_paren)
' _CAPSLOCK (ON) in parentheses.
k = 0
IF k = 1 THEN
    _CAPSLOCK (ON)
END IF
PRINT "ok"
SYSTEM
