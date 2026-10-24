$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x25_func_suffix)
' an OpenGL FUNCTION with the suffix of its type (~&).
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    x = _glGetError~&
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
