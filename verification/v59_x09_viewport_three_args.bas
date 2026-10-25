$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x09_viewport_three_args)
' _glViewport with three arguments.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    _glViewport 0, 0, 1
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
