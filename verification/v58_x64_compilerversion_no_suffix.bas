$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x64_compilerversion_no_suffix)
' _COMPILERVERSION without its suffix.
k = 0
IF k = 1 THEN
    x$ = _COMPILERVERSION
END IF
PRINT "ok"
SYSTEM
