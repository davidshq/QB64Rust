$CONSOLE:ONLY
' Verification (m4-sub-gl, task 1.1): SUB _GL in a $CONSOLE:ONLY program (no window): does it compile, and is the
' SUB ever called? The main module waits 1 s and prints the count.
DIM SHARED frames&
t# = TIMER(.001)
DO
    _LIMIT 100
LOOP UNTIL TIMER(.001) - t# > 1
PRINT "frames:"; frames&
PRINT _GL_COLOR_BUFFER_BIT
SYSTEM

SUB _GL
    frames& = frames& + 1
END SUB
