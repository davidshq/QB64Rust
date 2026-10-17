$CONSOLE:ONLY
' Verification (m3-events, task 1.1): does the old compiler accept this? (03_sub_needs_arg)
ON TIMER(1) s
PRINT "ok"
SYSTEM
h:
RETURN
SUB s (n AS LONG)
END SUB
