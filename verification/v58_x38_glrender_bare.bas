$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x38_glrender_bare)
' _GLRENDER without a word.
k = 0
IF k = 1 THEN
    _GLRENDER
END IF
PRINT "ok"
SYSTEM
