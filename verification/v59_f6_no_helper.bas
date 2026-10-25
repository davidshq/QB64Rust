$CONSOLE
' Verification (m4-sub-gl, task 1.1): the routines without a helper (no arguments and no pointer returned:
' _glFlush, _glFinish, _glGetError, _glLoadIdentity) and _gluPerspective (its own inline function) in the main
' module, where SUB _GL is not running: no error 270. _glGetError's value is the driver's (no context here): not
' printed.
ON ERROR GOTO h
_DEST _CONSOLE
PRINT "flush": _glFlush
PRINT "finish": _glFinish
PRINT "geterror": x = _glGetError
PRINT "identity": _glLoadIdentity
PRINT "perspective": _gluPerspective 45, 1, 1, 10
_DELAY 1
PRINT "end"
SYSTEM
h:
PRINT "handler"; ERR
RESUME NEXT

SUB _GL
END SUB
