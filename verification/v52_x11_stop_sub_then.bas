$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x11_stop_sub_then)
' STOP in a SUB after THEN.
x = 1
s x
PRINT "after"
SYSTEM
SUB s (x)
    IF x = 1 THEN PRINT "e": STOP
    PRINT "not"
END SUB
