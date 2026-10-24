$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, review): does the old compiler accept this? (x49_call_suffix)
' an OpenGL SUB with a type suffix after CALL, with arguments.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    CALL _glViewport&(0, 0, 1, 1)
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
