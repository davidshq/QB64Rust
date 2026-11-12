$CONSOLE:ONLY
' Verification (language-reference, second review): does the old compiler accept this? (x06_sndstop_bare)
' _SNDSTOP with no handle.
k = 0
IF k = 1 THEN
    _SNDSTOP
END IF
PRINT "ok"
SYSTEM
