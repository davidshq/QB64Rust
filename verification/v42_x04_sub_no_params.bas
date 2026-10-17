$CONSOLE:ONLY
' Verification (m3-events, task 1.1): does the old compiler accept this? (04_sub_no_params)
ON TIMER(1) s 5
PRINT "ok"
SYSTEM
h:
RETURN
SUB s
END SUB
