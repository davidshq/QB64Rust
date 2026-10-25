$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x23_own_sub_without)
' a SUB of the program named as an OpenGL routine, without SUB _GL.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    _glFlush
END IF
PRINT "ok"
SYSTEM

SUB _glFlush
    PRINT "mine"
END SUB
