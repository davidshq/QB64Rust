$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): does the old compiler accept this? (x47_gl_names_with_declare_only)
' an OpenGL statement where the program has DECLARE SUB _GL but no SUB _GL.
DECLARE SUB _GL ()
_DEST _CONSOLE
k = 0
IF k = 1 THEN
    _glFlush
END IF
PRINT "ok"
SYSTEM
