$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (b4_const_without_explicit)
' an OpenGL constant under OPTION _EXPLICIT without SUB _GL.
OPTION _EXPLICIT
DIM k AS SINGLE
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    PRINT _GL_TRUE
END IF
PRINT "ok"
SYSTEM
