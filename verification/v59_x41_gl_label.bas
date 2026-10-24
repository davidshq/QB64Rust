$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x41_gl_label)
' a label named _GL beside SUB _GL.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    GOTO _GL
END IF
PRINT "ok"
SYSTEM

_GL:
PRINT "label"
SYSTEM

SUB _GL
END SUB
