$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): RESUME NEXT in a SUB with no error pending.
PRINT "before"
r
PRINT "after r"
SYSTEM
SUB r
    PRINT "in r"
    RESUME NEXT
    PRINT "after RESUME in r"
END SUB
