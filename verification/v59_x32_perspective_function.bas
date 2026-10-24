$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x32_perspective_function)
' _gluPerspective used as a function.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    x = _gluPerspective(1, 2, 3, 4)
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
