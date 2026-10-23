$CONSOLE:ONLY
' Verification (m4-devices-continue-paths, task 1.1): ON STRIG with a SUB that needs an argument, given none.
ON STRIG(1) s
PRINT "ok"
SYSTEM
SUB s (n AS INTEGER)
END SUB
