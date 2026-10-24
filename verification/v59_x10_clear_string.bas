$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x10_clear_string)
' _glClear with a string literal.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    _glClear "a"
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
