$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x89_allowfullscreen_comma_only)
' _ALLOWFULLSCREEN with a comma and nothing else (accepted?).
k = 0
IF k = 1 THEN
    _ALLOWFULLSCREEN ,
END IF
PRINT "ok"
SYSTEM
