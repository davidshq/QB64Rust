$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x36_flush_empty_parens)
' an OpenGL statement without arguments, with empty parentheses.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    _glFlush ()
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
