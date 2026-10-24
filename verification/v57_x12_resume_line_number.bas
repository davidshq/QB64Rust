$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): RESUME of a main-module line number in a SUB.
PRINT "ok"
SYSTEM
100 PRINT "x"
SUB r
    RESUME 100
END SUB
