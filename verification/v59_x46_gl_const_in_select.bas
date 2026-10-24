$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x46_gl_const_in_select)
' an OpenGL constant in a CASE.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    SELECT CASE k: CASE _GL_TRUE: PRINT: END SELECT
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
