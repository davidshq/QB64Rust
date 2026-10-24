$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x15_call_keyword)
' an OpenGL statement after CALL, with parentheses.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    CALL _glViewport(0, 0, 1, 1)
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
