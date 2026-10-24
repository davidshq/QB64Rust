$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x25_capslock_bare_stmt)
' _CAPSLOCK as a statement with no word.
k = 0
IF k = 1 THEN
    _CAPSLOCK
END IF
PRINT "ok"
SYSTEM
