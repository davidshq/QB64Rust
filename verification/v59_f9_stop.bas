$CONSOLE
' Verification (m4-sub-gl, task 1.1): STOP inside SUB _GL (libqb's END tests for SUB _GL, SYSTEM's code tests for
' it; what does STOP do?). The main module waits 5 s and prints.
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
    STOP
END SUB
