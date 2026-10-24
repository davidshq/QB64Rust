$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x44_const_in_sub_gl)
' a CONST inside SUB _GL whose value names an OpenGL constant.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    PRINT
END IF
PRINT "ok"
SYSTEM

SUB _GL
    CONST k2 = _GL_TRUE
END SUB
