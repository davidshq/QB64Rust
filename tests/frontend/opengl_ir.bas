' TEST: ir
$CONSOLE
' SUB _GL in the IR (m4-sub-gl task 3.1): the program asks for the OpenGL part of the runtime; SUB _GL is an
' ordinary SUB; each OpenGL routine named is a procedure of the OpenGL library, its parameters BYVAL; SYSTEM inside
' SUB _GL as anywhere.
DIM SHARED frames&
x~& = _glGetError
SYSTEM

SUB _GL
    _glClear _GL_COLOR_BUFFER_BIT
    frames& = frames& + 1
    IF frames& > 100 THEN SYSTEM
END SUB
