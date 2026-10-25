$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x33_glu_other)
' a glu routine not in the table (_gluLookAt).
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    _gluLookAt 0, 0, 0, 0, 0, 0, 0, 1, 0
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
