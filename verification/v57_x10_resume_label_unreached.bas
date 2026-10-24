$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): RESUME of a main-module label in a SUB, never reached.
PRINT "ok"
SYSTEM
back:
SUB r
    RESUME back
END SUB
