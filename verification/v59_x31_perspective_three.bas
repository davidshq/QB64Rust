$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x31_perspective_three)
' _gluPerspective with three arguments.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    _gluPerspective 1, 2, 3
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
