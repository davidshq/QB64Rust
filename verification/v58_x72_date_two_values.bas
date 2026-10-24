$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x72_date_two_values)
' DATE$ = with two values.
k = 0
IF k = 1 THEN
    DATE$ = "a", "b"
END IF
PRINT "ok"
SYSTEM
