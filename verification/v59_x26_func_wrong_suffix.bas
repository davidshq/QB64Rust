$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x26_func_wrong_suffix)
' an OpenGL FUNCTION with another suffix (%).
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    x = _glGetError%
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
