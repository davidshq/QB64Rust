$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x74_date_nothing)
' DATE$ = with nothing after it.
k = 0
IF k = 1 THEN
    DATE$ =
END IF
PRINT "ok"
SYSTEM
