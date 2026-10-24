$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x62_inputbox_number)
' _INPUTBOX$ with a number.
k = 0
IF k = 1 THEN
    x$ = _INPUTBOX$(1)
END IF
PRINT "ok"
SYSTEM
