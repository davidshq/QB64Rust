$CONSOLE:ONLY
' Verification (m3-events, task 1.1): does the old compiler accept this? (39_sub_offset)
ON TIMER(1) s 5
PRINT "ok"
SYSTEM
h:
RETURN
SUB s (n AS _OFFSET)
END SUB
