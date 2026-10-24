$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): RESUME of a label that exists nowhere, in a SUB.
PRINT "ok"
SYSTEM
SUB r
    RESUME nowhere
END SUB
