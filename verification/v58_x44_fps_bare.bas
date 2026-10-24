$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x44_fps_bare)
' _FPS without an argument.
k = 0
IF k = 1 THEN
    _FPS
END IF
PRINT "ok"
SYSTEM
