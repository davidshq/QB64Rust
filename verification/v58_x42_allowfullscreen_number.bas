$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x42_allowfullscreen_number)
' _ALLOWFULLSCREEN with a number.
k = 0
IF k = 1 THEN
    _ALLOWFULLSCREEN 1
END IF
PRINT "ok"
SYSTEM
