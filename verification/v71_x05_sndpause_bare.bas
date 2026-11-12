$CONSOLE:ONLY
' Verification (language-reference, second review): does the old compiler accept this? (x05_sndpause_bare)
' _SNDPAUSE with no handle.
k = 0
IF k = 1 THEN
    _SNDPAUSE
END IF
PRINT "ok"
SYSTEM
