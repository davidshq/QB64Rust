$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x08_mksmbf_string)
' MKSMBF$ of a string.
k = 0
IF k = 1 THEN
    s$ = MKSMBF$("a")
END IF
PRINT "ok"
SYSTEM
