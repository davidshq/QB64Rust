$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x17_out_string)
' OUT with a string.
k = 0
IF k = 1 THEN
    OUT "a", 1
END IF
PRINT "ok"
SYSTEM
