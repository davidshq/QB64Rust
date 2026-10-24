$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (b1_stmt_without)
' an OpenGL statement in a program without SUB _GL.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    _glViewport 0, 0, 1, 1
END IF
PRINT "ok"
SYSTEM
