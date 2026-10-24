$CONSOLE
' Verification (m4-sub-gl, task 1.1): the SUB's name in lower case with empty parentheses (sub _gl ()), the OpenGL
' names in other letter cases (_GLCLEAR, _gl_color_buffer_bit), and a SUB defined before SUB _GL that uses an
' OpenGL routine and is called from it.
DIM SHARED frames&
t# = TIMER(.001)
DO
    _LIMIT 100
LOOP UNTIL frames& >= 3 OR TIMER(.001) - t# > 10
_DEST _CONSOLE
PRINT "frames >= 3:"; frames& >= 3
PRINT _gl_color_buffer_bit; _Gl_Depth_Buffer_Bit
SYSTEM

SUB helper
    _GLCLEAR _gl_color_buffer_bit
END SUB

sub _gl ()
    helper
    frames& = frames& + 1
END SUB
