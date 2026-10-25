$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x42_name_as_variable_without)
' a variable named as an OpenGL routine, without SUB _GL.
_DEST _CONSOLE
_glFlush& = 3
PRINT _glFlush&
SYSTEM
