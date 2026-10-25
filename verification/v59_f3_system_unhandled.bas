$CONSOLE
' Verification (m4-sub-gl, task 1.1): SYSTEM inside SUB _GL with no handler (error 271). The main module waits
' 5 s and prints; whether it gets there is the measurement.
DIM SHARED frames&
t# = TIMER(.001)
DO
    _LIMIT 100
LOOP UNTIL TIMER(.001) - t# > 5
_DEST _CONSOLE
PRINT "main after wait:"; frames& > 0
SYSTEM

SUB _GL
    frames& = frames& + 1
    _DEST _CONSOLE: PRINT "in sub _gl"
    SYSTEM
END SUB
