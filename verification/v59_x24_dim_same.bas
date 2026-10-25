$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x24_dim_same)
' a variable named as an OpenGL routine, beside SUB _GL.
DIM _glFlush AS LONG
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    PRINT
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
