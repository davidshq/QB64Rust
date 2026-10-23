$CONSOLE:ONLY
' Verification (m4-devices-continue-paths, task 1.1): ON STRIG with a SUB whose parameter is SINGLE.
ON STRIG(1) s 3
PRINT "ok"
SYSTEM
SUB s (n AS SINGLE)
END SUB
