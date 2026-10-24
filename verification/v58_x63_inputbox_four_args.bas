$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x63_inputbox_four_args)
' _INPUTBOX$ with four arguments.
k = 0
IF k = 1 THEN
    x$ = _INPUTBOX$("a", "b", "c", "d")
END IF
PRINT "ok"
SYSTEM
