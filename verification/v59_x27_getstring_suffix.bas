$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x27_getstring_suffix)
' _glGetString with the suffix $.
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    a$ = _glGetString$(_GL_VERSION)
END IF
PRINT "ok"
SYSTEM

SUB _GL
END SUB
