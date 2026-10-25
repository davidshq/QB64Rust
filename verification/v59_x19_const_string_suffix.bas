$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x19_const_string_suffix)
' an OpenGL constant with the suffix $.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    PRINT _GL_TRUE$
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
