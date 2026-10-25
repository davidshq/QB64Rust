$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x14_function_args_as_statement)
' an OpenGL FUNCTION with an argument as a statement.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    _glIsEnabled _GL_BLEND
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
