$CONSOLE
$SCREENHIDE
' Verification (m4-sub-gl, task 1.1): SUB _GL in a program whose window is hidden ($SCREENHIDE): is the SUB called?
' The main module waits for three calls (3 s at most) and prints whether they came.
DIM SHARED frames&
t# = TIMER(.001)
DO
    _LIMIT 100
LOOP UNTIL frames& >= 3 OR TIMER(.001) - t# > 3
_DEST _CONSOLE
PRINT "frames >= 3:"; frames& >= 3
SYSTEM

SUB _GL
    frames& = frames& + 1
END SUB
