$CONSOLE
' Verification (m4-sub-gl, task 1.1): SUB _GL in an included file (v59_inc\gl.bm), and an OpenGL name used in the
' main module before the include line.
DIM SHARED frames&
PRINT "const:"; _GL_TRUE
t# = TIMER(.001)
DO
    _LIMIT 100
LOOP UNTIL frames& >= 3 OR TIMER(.001) - t# > 10
_DEST _CONSOLE
PRINT "frames >= 3:"; frames& >= 3
SYSTEM

'$INCLUDE: 'v59_inc\gl.bm'
