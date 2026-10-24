$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x06_call_gl)
' a call of SUB _GL by its name.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    _GL
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
