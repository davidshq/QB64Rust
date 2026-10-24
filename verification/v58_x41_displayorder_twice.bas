$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x41_displayorder_twice)
' _DISPLAYORDER with one word twice (accepted?).
k = 0
IF k = 1 THEN
    _DISPLAYORDER _SOFTWARE, _SOFTWARE
END IF
PRINT "ok"
SYSTEM
