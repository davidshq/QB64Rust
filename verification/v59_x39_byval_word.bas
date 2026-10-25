$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x39_byval_word)
' BYVAL before an OpenGL argument.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    _glFlush: _glViewport BYVAL 0, 0, 1, 1
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
