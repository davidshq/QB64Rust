$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x09_mksmbf_no_suffix)
' MKSMBF without its suffix.
k = 0
IF k = 1 THEN
    s$ = MKSMBF(1.5)
END IF
PRINT "ok"
SYSTEM
