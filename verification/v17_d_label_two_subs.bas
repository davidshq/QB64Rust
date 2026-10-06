$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): the same label in two SUBs and in main, each GOTO stays in its body.
GOTO a
PRINT "skipped"
a:
PRINT "main a"
t1
t2
SYSTEM
SUB t1
    GOTO a
    PRINT "skipped"
    a:
    PRINT "t1 a"
END SUB
SUB t2
    GOTO a
    a:
    PRINT "t2 a"
END SUB
