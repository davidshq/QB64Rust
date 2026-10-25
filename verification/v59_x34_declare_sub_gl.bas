$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x34_declare_sub_gl)
' DECLARE SUB _GL beside SUB _GL.
DECLARE SUB _GL ()
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    PRINT
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
