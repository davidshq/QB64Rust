$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x45_fps_string)
' _FPS with a string.
k = 0
IF k = 1 THEN
    _FPS "a"
END IF
PRINT "ok"
SYSTEM
