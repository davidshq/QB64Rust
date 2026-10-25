$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x43_const_of_gl_const)
' a CONST whose value names an OpenGL constant, after a use of it.
PRINT _GL_TRUE
CONST k2 = _GL_TRUE
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    PRINT
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
