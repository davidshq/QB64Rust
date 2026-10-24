$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x16_statement_parens)
' an OpenGL statement with its arguments in parentheses.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    _glViewport (0, 0, 1, 1)
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
