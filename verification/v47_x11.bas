$CONSOLE:ONLY
' Verification (m4-devices-continue-paths, task 1.1): ON STRIG with an argument for a SUB that has none.
ON STRIG(1) s 3
PRINT "ok"
SYSTEM
SUB s
END SUB
