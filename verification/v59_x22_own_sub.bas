$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x22_own_sub)
' a SUB of the program named as an OpenGL routine, beside SUB _GL.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    PRINT
END IF
PRINT "ok"
SYSTEM

SUB _glFlush
END SUB

SUB _GL
END SUB
