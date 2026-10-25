$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x12_sub_as_function)
' an OpenGL SUB used as a function.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    x = _glFlush
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
