$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x01_sub_gl_param)
' SUB _GL with a parameter.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    PRINT
END IF
PRINT "ok"
SYSTEM

SUB _GL (a AS LONG)
END SUB
