$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x100_files_fn)
' FILES as a function.
k = 0
IF k = 1 THEN
    x = FILES("a")
END IF
PRINT "ok"
SYSTEM
