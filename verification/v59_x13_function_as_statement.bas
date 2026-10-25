$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x13_function_as_statement)
' an OpenGL FUNCTION without arguments as a statement.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    _glGetError
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
