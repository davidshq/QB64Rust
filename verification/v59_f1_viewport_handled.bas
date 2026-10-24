$CONSOLE
' Verification (m4-sub-gl, task 1.1): _glViewport (a routine with a helper) in the main module, where SUB _GL is not
' running, under ON ERROR: error 270 is a critical error (libqb's error_handle.cpp), so the handler does not run.
' The program waits 2 s after the call, so that nothing after it is printed while the program ends.
ON ERROR GOTO h
_DEST _CONSOLE
PRINT "before"
_glViewport 0, 0, 1, 1
_DELAY 2
PRINT "after"
SYSTEM
h:
PRINT "handler"; ERR
RESUME NEXT

SUB _GL
END SUB
