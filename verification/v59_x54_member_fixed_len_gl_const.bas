$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, review): does the old compiler accept this? (x54_member_fixed_len_gl_const)
' an OpenGL constant as the length of a STRING * n member.
TYPE t
    s AS STRING * _GL_TRUE
END TYPE
DIM v AS t
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    PRINT
END IF
PRINT LEN(v.s)
PRINT "ok"
SYSTEM

SUB _GL
END SUB
