' TEST: typed
$CONSOLE
' SUB _GL and the OpenGL names (m4-sub-gl task 3.1; measured, verification\v59_*): a constant is an _INTEGER64 (with
' & and && as for a CONST), in a DIM bound and a CASE; a FUNCTION's result type (_glGetError an _UNSIGNED LONG,
' _glIsEnabled an _UNSIGNED _BYTE, _glGetString a STRING); an argument converted to the parameter's type, an
' integer as written (not folded), a float rounded into a GLint, a literal 0 as NULL but not _GL_FALSE; a LONG and
' a whole array where an _OFFSET goes (its first element); CALL with parentheses; the names in any letter case;
' DECLARE SUB _GL (); a routine named first in a SUB above SUB _GL; a constant as the length of a STRING * n (x53).
DECLARE SUB _GL ()
DIM SHARED v&(3), p~%%(3)
DIM a(_GL_TRUE) AS LONG
DIM f AS STRING * _GL_TRUE
PRINT _GL_COLOR_BUFFER_BIT; _GL_COLOR_BUFFER_BIT&; _gl_true&&
SELECT CASE k
    CASE _GL_TRUE
        PRINT
END SELECT
e~& = _glGetError
b~%% = _glIsEnabled(_GL_BLEND) + 1
s$ = _glGetString(_GL_VERSION)
_glViewport 1.5, 2, 3&, 4#
CALL _glViewport(0, 0, 1, 1)
_glColorMask 256, _GL_FALSE, 0, -1
_glGetIntegerv _GL_VIEWPORT, _OFFSET(v&(0))
_glGetIntegerv _GL_VIEWPORT, v&
_glGetIntegerv _GL_VIEWPORT, v&()
_GLFLUSH
_gluPerspective 45, 1.5, 1, 10&
SYSTEM

SUB paint_it
    _glClear _GL_COLOR_BUFFER_BIT OR _GL_DEPTH_BUFFER_BIT
END SUB

SUB _GL STATIC
    paint_it
    _glReadPixels 0, 0, 1, 1, _GL_RGBA, _GL_UNSIGNED_BYTE, _OFFSET(p~%%(0))
END SUB
