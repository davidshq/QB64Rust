$CONSOLE
' Verification (m4-sub-gl, task 1.1): _glGetString (a helper, its pointer read as a string) in the main module,
' and the program waits 2 s after it: error 270.
_DEST _CONSOLE
PRINT "before"
a$ = _glGetString(_GL_VERSION)
_DELAY 2
PRINT "after"
SYSTEM

SUB _GL
END SUB
