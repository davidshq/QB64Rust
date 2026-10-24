$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x115_input_hash_only)
' INPUT$ with a # and no number.
k = 0
IF k = 1 THEN
    s$ = INPUT$(1, #)
END IF
PRINT "ok"
SYSTEM
