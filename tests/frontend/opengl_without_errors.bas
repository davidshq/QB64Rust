' TEST: check-fail
$CONSOLE
' The OpenGL names in a program without SUB _GL (m4-sub-gl task 3.1; verification\v59_b*, x23, x42, x47): none of
' them exists, each a real error (the old compiler's message after each); DECLARE SUB _GL () is no SUB _GL.
DECLARE SUB _GL ()
_glViewport 0, 0, 1, 1
' b1, x47: Syntax error
x = _glGetError
' b2: Invalid variable name
PRINT _GL_TRUE
' b3: Invalid variable name
_glFlush& = 3
' x42: Invalid variable name
_gluLookAt 0, 0, 0, 0, 0, 0, 0, 1, 0
' (x33 with SUB _GL): Syntax error
SYSTEM

SUB _glFlush
END SUB
' x23 above: Invalid name
