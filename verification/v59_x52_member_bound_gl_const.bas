$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, review): does the old compiler accept this? (x52_member_bound_gl_const)
' an OpenGL constant as the bound of a TYPE member array.
TYPE t
    a(_GL_TRUE) AS LONG
END TYPE
DIM v AS t
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    PRINT
END IF
PRINT UBOUND(v.a)
PRINT "ok"
SYSTEM

SUB _GL
END SUB
