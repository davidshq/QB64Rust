$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (b2_func_without)
' an OpenGL FUNCTION in a program without SUB _GL.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    x = _glGetError
END IF
PRINT "ok"
SYSTEM
