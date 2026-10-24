$CONSOLE
' Verification (m4-sub-gl, task 1.1): _glGetString of a name OpenGL does not know (12345): the driver returns NULL.
' What does the program get (or does it stop)? The main module waits for one call (10 s at most).
DIM SHARED frames&, a$, l&
t# = TIMER(.001)
DO
    _LIMIT 100
LOOP UNTIL frames& >= 1 OR TIMER(.001) - t# > 10
_DEST _CONSOLE
PRINT "frames >= 1:"; frames& >= 1
PRINT "["; a$; "]"; l&
SYSTEM

SUB _GL
    IF frames& THEN EXIT SUB
    a$ = _glGetString(12345)
    l& = LEN(a$)
    frames& = frames& + 1
END SUB
