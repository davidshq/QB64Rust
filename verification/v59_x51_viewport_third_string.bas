$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, review): does the old compiler accept this? (x51_viewport_third_string)
' _glViewport with a string as its third argument.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    _glViewport 0, 0, "a", 1
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
