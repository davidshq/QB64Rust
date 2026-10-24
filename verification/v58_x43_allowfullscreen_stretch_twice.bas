$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x43_allowfullscreen_stretch_twice)
' _ALLOWFULLSCREEN _STRETCH, _STRETCH.
k = 0
IF k = 1 THEN
    _ALLOWFULLSCREEN _STRETCH, _STRETCH
END IF
PRINT "ok"
SYSTEM
