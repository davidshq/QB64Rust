$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, review): does the old compiler accept this? (x53_fixed_len_gl_const)
' an OpenGL constant as the length of a STRING * n variable.
DIM s AS STRING * _GL_TRUE
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    PRINT
END IF
PRINT LEN(s)
PRINT "ok"
SYSTEM

SUB _GL
END SUB
