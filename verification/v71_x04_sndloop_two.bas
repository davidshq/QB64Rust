$CONSOLE:ONLY
' Verification (language-reference, second review): does the old compiler accept this? (x04_sndloop_two)
' _SNDLOOP with two arguments.
k = 0
IF k = 1 THEN
    _SNDLOOP h, 1
END IF
PRINT "ok"
SYSTEM
