$CONSOLE
' Verification (m4-sub-gl, task 1.1): a routine with a helper called in a SUB of the program that the main module
' calls (SUB _GL is not running): error 270; the program waits 2 s after the call.
_DEST _CONSOLE
PRINT "before"
s
_DELAY 2
PRINT "after"
SYSTEM

SUB s
    _glClear _GL_COLOR_BUFFER_BIT
END SUB

SUB _GL
END SUB
