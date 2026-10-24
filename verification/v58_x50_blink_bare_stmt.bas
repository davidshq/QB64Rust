$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x50_blink_bare_stmt)
' _BLINK as a statement with no word.
k = 0
IF k = 1 THEN
    _BLINK
END IF
PRINT "ok"
SYSTEM
