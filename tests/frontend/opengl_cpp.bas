' TEST: cpp
$CONSOLE
' SUB _GL in C++ (m4-sub-gl task 3.2; the old compiler's -z output, design D4): SUB__GL declared with the program's
' SUBs and defined as one; a routine with a helper called by its call_ name, one without by the C function, the
' constant an _INTEGER64 literal, a FUNCTION's result with no cast, _glGetString's pointer read as a string, a
' literal 0 as NULL, _gluPerspective; SYSTEM inside SUB _GL tests for it, and in this program so does STOP
' (DIVERGENCES.md D-036: the old compiler's program hangs).
DIM SHARED p~%%(3), e~&, s$
SYSTEM

SUB helper
    _glFlush
END SUB

SUB _GL
    helper
    _glClearColor 1, 0, 0, 1
    _glClear _GL_COLOR_BUFFER_BIT
    _glReadPixels 0, 0, 1, 1, _GL_RGBA, _GL_UNSIGNED_BYTE, _OFFSET(p~%%(0))
    e~& = _glGetError + 1
    s$ = _glGetString(_GL_VERSION)
    _gluPerspective 45, 1.5, 1, 10
    IF e~& = 7 THEN STOP
    SYSTEM
END SUB
