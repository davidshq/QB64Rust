$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x11_clear_string_var)
' _glClear with a string variable.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    _glClear a$
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
