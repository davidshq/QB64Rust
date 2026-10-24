$CONSOLE
' Verification (m4-sub-gl, task 1.1): what SUB _GL reads back from OpenGL. It clears the frame to red, reads one
' pixel with _glReadPixels, the viewport with _glGetIntegerv, _glIsEnabled before and after _glEnable, _glGetError
' after a good call and after a bad enum; the renderer is printed only as "named" (it is the machine's). The main
' module waits for the SUB to have run three times (10 s at most) and prints what the last run read.
DIM SHARED p~%%(3), v&(3), frames&, en1~%%, en2~%%, er1~&, er2~&, rn$
t# = TIMER(.001)
DO
    _LIMIT 100
LOOP UNTIL frames& >= 3 OR TIMER(.001) - t# > 10
_DEST _CONSOLE
IF frames& < 3 THEN PRINT "frames:"; frames&: SYSTEM
PRINT "frames >= 3"
PRINT "renderer named:"; LEN(rn$) > 0
PRINT "pixel:"; p~%%(0); p~%%(1); p~%%(2); p~%%(3)
PRINT "viewport:"; v&(0); v&(1); v&(2); v&(3)
PRINT "enabled:"; en1~%%; en2~%%
PRINT "errors:"; er1~&; er2~&
SYSTEM

SUB _GL
    rn$ = _glGetString(_GL_RENDERER)
    _glClearColor 1, 0, 0, 1
    _glClear _GL_COLOR_BUFFER_BIT
    _glReadPixels 0, 0, 1, 1, _GL_RGBA, _GL_UNSIGNED_BYTE, _OFFSET(p~%%(0))
    _glGetIntegerv _GL_VIEWPORT, _OFFSET(v&(0))
    _glDisable _GL_DEPTH_TEST
    en1~%% = _glIsEnabled(_GL_DEPTH_TEST)
    _glEnable _GL_DEPTH_TEST
    en2~%% = _glIsEnabled(_GL_DEPTH_TEST)
    _glDisable _GL_DEPTH_TEST
    er1~& = _glGetError
    _glEnable 12345
    er2~& = _glGetError
    frames& = frames& + 1
END SUB
