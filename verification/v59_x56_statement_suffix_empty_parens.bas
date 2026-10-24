$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, review): does the old compiler accept this? (x56_statement_suffix_empty_parens)
' an OpenGL SUB with a type suffix and empty parentheses as a statement.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    _glFlush% ()
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
