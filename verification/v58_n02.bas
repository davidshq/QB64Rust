$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): _COLORCHOOSERDIALOG& with the suffix of its result (accepted?).
k = 0
IF k = 1 THEN
    c~& = _COLORCHOOSERDIALOG&("t", 1)
END IF
PRINT "ok"
SYSTEM
