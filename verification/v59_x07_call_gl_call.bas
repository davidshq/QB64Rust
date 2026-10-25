$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x07_call_gl_call)
' a call of SUB _GL with CALL.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    CALL _GL
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
