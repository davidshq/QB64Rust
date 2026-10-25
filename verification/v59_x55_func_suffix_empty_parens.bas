$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, review): does the old compiler accept this? (x55_func_suffix_empty_parens)
' an OpenGL FUNCTION with a type suffix and empty parentheses.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    x = _glGetError%()
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
