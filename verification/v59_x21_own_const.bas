$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x21_own_const)
' a CONST of a program named as an OpenGL constant.
CONST _GL_TRUE = 5
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    PRINT
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
