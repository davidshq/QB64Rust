$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x20_assign_const)
' an assignment to an OpenGL constant.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    _GL_TRUE = 5
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
