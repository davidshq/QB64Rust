$CONSOLE:ONLY
' Verification (m3-events, task 1.1): does the old compiler accept this? (37_label_main_and_sub)
ON TIMER(1) GOSUB h
PRINT "ok"
SYSTEM
h:
RETURN
SUB s
h:
END SUB
