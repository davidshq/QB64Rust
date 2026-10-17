$CONSOLE:ONLY
' Verification (m3-events, task 1.1): does the old compiler accept this? (36_sub_two_args)
ON TIMER(1) s 1, 2
PRINT "ok"
SYSTEM
h:
RETURN
SUB s (n AS LONG)
END SUB
