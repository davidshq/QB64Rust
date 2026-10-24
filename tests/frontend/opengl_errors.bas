' TEST: check-fail
$CONSOLE
' SUB _GL and the OpenGL names (m4-sub-gl task 3.1): the forms the old compiler rejects in a program with SUB _GL,
' each a real error (verification\v59_x*; the old compiler's message after each). Accepted forms: opengl_typed.bas;
' without SUB _GL: opengl_without_errors.bas.
CONST k1 = _GL_TRUE
' x43: Unexpected element '_GL_TRUE'
_GL
' x06: Cannot call SUB _GL directly
CALL _GL
' x07: Cannot call SUB _GL directly
_GL&
' x40: Syntax error
_glViewport 0, 0, 1
' x09: Syntax error
_glClear "a"
' x10: Number required for sub
_glClear a$
' x11: Number required for sub
x = _glFlush
' x12: Invalid variable name
_glGetError
' x13: Syntax error
_glIsEnabled _GL_BLEND
' x14: Syntax error
_glViewport (0, 0, 1, 1)
' x16: Syntax error
glViewport 0, 0, 1, 1
' x17: Syntax error
PRINT _GL_TRUE$
' x19: Type mismatch
_GL_TRUE = 5
' x20: Expected variable =, look for conflict with a CONST name
x = _glGetError~&
' x25: Invalid variable name
x = _glGetError%
' x26: Invalid variable name
a$ = _glGetString$(_GL_VERSION)
' x27: Invalid array name
_glGetIntegerv _GL_VIEWPORT, "a"
' x29: 2nd sub argument requires a number
_gluPerspective 1, 2, 3
' x31: Syntax error
x = _gluPerspective(1, 2, 3, 4)
' x32: Invalid array name
_gluLookAt 0, 0, 0, 0, 0, 0, 0, 1, 0
' x33: Syntax error
_glFlush ()
' x36: SUB does not require any arguments
_glViewport 0, 0, 1, 1,
' x37: Syntax error
_glViewport 0, , 1, 1
' x38: Syntax error
_glViewport BYVAL 0, 0, 1, 1
' x39: Expected operator in equation
_glFlush%
' x48: Syntax error
CALL _glViewport&(0, 0, 1, 1)
' x49: Expected CALL sub-name [(...)]
_glClear 1, 2
' x50: Syntax error
_glViewport 0, 0, "a", 1
' x51: 3rd sub argument requires a number
DIM _glFlush AS LONG
' x24: Invalid variable name
SYSTEM

SUB _GL
    CONST k2 = _GL_TRUE
    ' x44: Unexpected element '_GL_TRUE'
    IF k = 1 THEN _GL
    ' x08: Cannot call SUB _GL directly
END SUB

SUB _GL
END SUB
' x05 above: Invalid name

SUB _GL (a AS LONG)
END SUB
' x01 above: Invalid name

FUNCTION _GL
END FUNCTION
' x03 above: Invalid name

SUB _GL$
END SUB
' x35 above: Type symbols after a SUB name are invalid

SUB _glFinish
END SUB
' x22 above (with _glFlush): Invalid name
