$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x11_cvsmbf_suffix)
' CVSMBF$ with a suffix.
k = 0
IF k = 1 THEN
    x = CVSMBF$("abcd")
END IF
PRINT "ok"
SYSTEM
