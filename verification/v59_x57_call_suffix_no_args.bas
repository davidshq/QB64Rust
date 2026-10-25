$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, review): does the old compiler accept this? (x57_call_suffix_no_args)
' an OpenGL SUB with a type suffix after CALL, without arguments.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    CALL _glFlush%
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
