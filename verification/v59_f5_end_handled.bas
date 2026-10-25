$CONSOLE
' Verification (m4-sub-gl, task 1.1): END and STOP inside SUB _GL under the main module's ON ERROR handler: the
' handler counts each error number; the main module waits for three calls (10 s at most) and prints the counts.
DIM SHARED frames&, after1&, after2&, e271&, eother&, lastother&
ON ERROR GOTO h
t# = TIMER(.001)
DO
    _LIMIT 100
LOOP UNTIL frames& >= 3 OR TIMER(.001) - t# > 10
_DEST _CONSOLE
PRINT "frames >= 3:"; frames& >= 3; "after end:"; after1& >= 3; "after stop:"; after2& >= 3
PRINT "271 seen:"; e271& > 0; "other:"; eother&; lastother&
SYSTEM
h:
IF ERR = 271 THEN e271& = e271& + 1 ELSE eother& = eother& + 1: lastother& = ERR
RESUME NEXT

SUB _GL
    frames& = frames& + 1
    END
    after1& = after1& + 1
    STOP
    after2& = after2& + 1
END SUB
