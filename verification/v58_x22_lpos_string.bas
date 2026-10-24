$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x22_lpos_string)
' LPOS of a string.
k = 0
IF k = 1 THEN
    x = LPOS("a")
END IF
PRINT "ok"
SYSTEM
