$CONSOLE
' Verification (m4-sub-gl, task 1.1): how arguments are converted to an OpenGL routine's types, seen by reading the
' state back inside SUB _GL: SINGLE and DOUBLE values into GLint and GLsizei (_glViewport), a value above 32 bits
' into GLint, a negative into GLuint (_glStencilMask, read as GLint), 256 and 2 into GLboolean (_glColorMask), a
' DOUBLE into GLfloat (_glLineWidth), an out-of-range clear colour, -1 into GLenum (_glGetError), an _OFFSET held
' in a variable. The main module waits for three calls (10 s at most) and prints what the last one read.
DIM SHARED frames&, v1&(3), v2&(3), sm&, cm~%%(3), lw!, p~%%(3), er~&
t# = TIMER(.001)
DO
    _LIMIT 100
LOOP UNTIL frames& >= 3 OR TIMER(.001) - t# > 10
_DEST _CONSOLE
PRINT "frames >= 3:"; frames& >= 3
PRINT "viewport 1:"; v1&(0); v1&(1); v1&(2); v1&(3)
PRINT "viewport 2:"; v2&(0); v2&(1); v2&(2); v2&(3)
PRINT "stencil mask:"; sm&
PRINT "colour mask:"; cm~%%(0); cm~%%(1); cm~%%(2); cm~%%(3)
PRINT "line width:"; lw!
PRINT "pixel:"; p~%%(0); p~%%(1); p~%%(2); p~%%(3)
PRINT "error:"; er~&
SYSTEM

SUB _GL
    DIM o AS _OFFSET, d AS DOUBLE
    _glViewport 1.5, 2.5, 100.6, 50.4
    o = _OFFSET(v1&(0))
    _glGetIntegerv _GL_VIEWPORT, o
    d = 3.5
    _glViewport -2.5, d, 4294967306#, 20
    _glGetIntegerv _GL_VIEWPORT, _OFFSET(v2&(0))
    _glStencilMask -1
    _glGetIntegerv _GL_STENCIL_WRITEMASK, _OFFSET(sm&)
    _glColorMask 256, 2, 0, 1
    _glGetBooleanv _GL_COLOR_WRITEMASK, _OFFSET(cm~%%(0))
    _glColorMask 1, 1, 1, 1
    d = 1.75#
    _glLineWidth d
    _glGetFloatv _GL_LINE_WIDTH, _OFFSET(lw!)
    _glViewport 0, 0, _WIDTH, _HEIGHT
    _glClearColor 2, -1, 0, 1#
    _glClear _GL_COLOR_BUFFER_BIT
    _glReadPixels 0, 0, 1, 1, _GL_RGBA, _GL_UNSIGNED_BYTE, _OFFSET(p~%%(0))
    x = _glGetError
    _glEnable -1
    er~& = _glGetError
    frames& = frames& + 1
END SUB
