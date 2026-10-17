$CONSOLE:ONLY
' Verification (m3-events, task 1.1): does the old compiler accept this? (06_sub_unsigned)
ON TIMER(1) s 5
PRINT "ok"
SYSTEM
h:
RETURN
SUB s (n AS _UNSIGNED INTEGER)
END SUB
