$CONSOLE:ONLY
' Verification (m4-memory-records, task 1.1): a SUB parameter named p$5.
q "a"
PRINT "ok"
SYSTEM
SUB q (p$5)
    PRINT p$5
END SUB
