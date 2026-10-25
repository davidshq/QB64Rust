$CONSOLE
' Verification (m4-sub-gl, task 1.1): SYSTEM inside SUB _GL under the main module's ON ERROR handler (error 271):
' the handler prints ERR and resumes after the statement; the main module waits for three calls (10 s at most).
DIM SHARED frames&, after&
ON ERROR GOTO h
t# = TIMER(.001)
DO
    _LIMIT 100
LOOP UNTIL frames& >= 3 OR TIMER(.001) - t# > 10
_DEST _CONSOLE
PRINT "frames >= 3:"; frames& >= 3; "after:"; after& >= 3
SYSTEM
h:
IF ERR <> 271 OR frames& = 0 THEN
    _DEST _CONSOLE: PRINT "error"; ERR; "frames"; frames&
END IF
RESUME NEXT

SUB _GL
    frames& = frames& + 1
    SYSTEM
    after& = after& + 1
END SUB
