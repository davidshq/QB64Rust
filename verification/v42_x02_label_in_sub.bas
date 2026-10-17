$CONSOLE:ONLY
' Verification (m3-events, task 1.1): does the old compiler accept this? (02_label_in_sub)
ON TIMER(1) GOSUB g
PRINT "ok"
SYSTEM
SUB s
g:
END SUB
