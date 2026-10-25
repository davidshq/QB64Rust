$CONSOLE
' Verification (m4-sub-gl, task 1.1): SUB _GL with _GLRENDER _BEHIND: still called each frame (three calls, 10 s at
' most), and what _glGetError reads on the first call.
DIM SHARED frames&, e~&
_GLRENDER _BEHIND
t# = TIMER(.001)
DO
    _LIMIT 100
LOOP UNTIL frames& >= 3 OR TIMER(.001) - t# > 10
_DEST _CONSOLE
PRINT "frames >= 3:"; frames& >= 3; "error:"; e~&
SYSTEM

SUB _GL
    IF frames& = 0 THEN e~& = _glGetError
    frames& = frames& + 1
END SUB
