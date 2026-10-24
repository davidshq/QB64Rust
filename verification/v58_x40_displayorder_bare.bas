$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x40_displayorder_bare)
' _DISPLAYORDER without a word (accepted?).
k = 0
IF k = 1 THEN
    _DISPLAYORDER
END IF
PRINT "ok"
SYSTEM
