$CONSOLE:ONLY
' Verification (language-reference, second review): does the old compiler accept this? (x03_sndloop_bare)
' _SNDLOOP with no handle.
k = 0
IF k = 1 THEN
    _SNDLOOP
END IF
PRINT "ok"
SYSTEM
