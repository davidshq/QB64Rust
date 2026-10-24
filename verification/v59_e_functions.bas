$CONSOLE
' Verification (m4-sub-gl, task 1.1): the OpenGL FUNCTIONs inside SUB _GL: _glGetString's result as a STRING
' (its length only: the text is the driver's), the type of _glGetError's result (0 - 1 and the suffix of a
' variable), _glIsEnabled in an expression, _glGenLists and _glRenderMode, each read in the last call of three.
DIM SHARED frames&, a$, l1&, ge1#, ge2#, ie#, gl~&, rm&, il~%%
t# = TIMER(.001)
DO
    _LIMIT 100
LOOP UNTIL frames& >= 3 OR TIMER(.001) - t# > 10
_DEST _CONSOLE
PRINT "frames >= 3:"; frames& >= 3
PRINT "version named:"; LEN(a$) > 0; l1& > 0
PRINT "error minus 1:"; ge1#; ge2#
PRINT "enabled + 1:"; ie#
PRINT "list named:"; gl~& > 0; il~%%
PRINT "render mode:"; rm&
SYSTEM

SUB _GL
    a$ = _glGetString(_GL_VERSION)
    l1& = LEN(_glGetString(_GL_VENDOR))
    ge1# = _glGetError - 1
    e~& = _glGetError
    ge2# = e~& - 1
    _glDisable _GL_BLEND
    ie# = _glIsEnabled(_GL_BLEND) + 1
    IF gl~& THEN _glDeleteLists gl~&, 1
    gl~& = _glGenLists(1)
    il~%% = _glIsList(gl~&)
    rm& = _glRenderMode(_GL_RENDER)
    frames& = frames& + 1
END SUB
