$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x28_offset_from_long)
' a LONG variable where an _OFFSET goes.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    _glGetIntegerv _GL_VIEWPORT, v&
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
