$CONSOLE
' Verification (m4-sub-gl, task 1.1): a plain SUB _GL is called once a frame. It counts its calls and clears the
' frame; the main module waits for three calls (10 s at most), prints the constant the SUB used, and ends.
DIM SHARED frames&
t# = TIMER(.001)
DO
    _LIMIT 100
LOOP UNTIL frames& >= 3 OR TIMER(.001) - t# > 10
_DEST _CONSOLE
PRINT "frames >= 3:"; frames& >= 3
PRINT _GL_COLOR_BUFFER_BIT
SYSTEM

SUB _GL
    _glClear _GL_COLOR_BUFFER_BIT
    frames& = frames& + 1
END SUB
