$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x73_date_no_equals)
' DATE$ with a string and no equals sign.
k = 0
IF k = 1 THEN
    DATE$ "x"
END IF
PRINT "ok"
SYSTEM
