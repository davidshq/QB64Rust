$CONSOLE
' Verification (m4-sub-gl, task 1.1): SUB _GL STATIC (as the upstream program opengl/test has it): a local keeps
' its value from call to call. The main module waits for three calls (10 s at most) and prints.
DIM SHARED frames&, last&
t# = TIMER(.001)
DO
    _LIMIT 100
LOOP UNTIL frames& >= 3 OR TIMER(.001) - t# > 10
_DEST _CONSOLE
PRINT "frames >= 3:"; frames& >= 3
PRINT "local kept:"; last& >= 3
SYSTEM

SUB _GL STATIC
    _glViewport 0, 0, _WIDTH, _HEIGHT
    n& = n& + 1
    last& = n&
    frames& = frames& + 1
END SUB
