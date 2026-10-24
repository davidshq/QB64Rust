$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): _SETBIT~&& with the suffix of its result (accepted?).
k = 0
IF k = 1 THEN
    q~&& = _SETBIT~&&(1, 0)
END IF
PRINT "ok"
SYSTEM
