$CONSOLE
' Verification (m4-sub-gl, task 1.1): _glViewport in the main module with no handler (error 270); the program waits
' 2 s after the call.
_DEST _CONSOLE
PRINT "before"
_glViewport 0, 0, 1, 1
_DELAY 2
PRINT "after"
SYSTEM

SUB _GL
END SUB
