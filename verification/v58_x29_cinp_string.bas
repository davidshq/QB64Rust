$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x29_cinp_string)
' _CINP of a string.
k = 0
IF k = 1 THEN
    x = _CINP("a")
END IF
PRINT "ok"
SYSTEM
