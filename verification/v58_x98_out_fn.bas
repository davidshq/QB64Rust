$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x98_out_fn)
' OUT as a function.
k = 0
IF k = 1 THEN
    x = OUT(1, 2)
END IF
PRINT "ok"
SYSTEM
