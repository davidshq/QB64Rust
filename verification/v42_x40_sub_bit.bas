$CONSOLE:ONLY
' Verification (m3-events, task 1.1): does the old compiler accept this? (40_sub_bit)
ON TIMER(1) s 1
PRINT "ok"
SYSTEM
h:
RETURN
SUB s (n AS _BIT)
END SUB
