$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x30_offset_array)
' a whole array where an _OFFSET goes.
DIM v&(3)
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    _glGetIntegerv _GL_VIEWPORT, v&()
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
