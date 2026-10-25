$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x17_no_prefix)
' an OpenGL statement without the underscore (glViewport).
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    glViewport 0, 0, 1, 1
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
