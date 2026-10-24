$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x45_const_in_gl_bound)
' an OpenGL constant as the bound of a DIM.
DIM a(_GL_TRUE) AS LONG
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    PRINT
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
