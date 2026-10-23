$CONSOLE:ONLY
' Verification (m4-devices-continue-paths, task 1.1): the accepted forms of ON STRIG and of the STRIG switch.
ON STRIG(1) s 3
ON STRIG(0, 1) s2
ON STRIG(1.7) GOSUB h
ON STRIG(2, 1.6) GOSUB h
STRIG(1) ON
STRIG(1, 2) OFF
STRIG STOP
STRIG ON
STRIG(0) STOP
PRINT "ok"
SYSTEM
h:
RETURN
SUB s (n AS INTEGER)
PRINT n
END SUB
SUB s2
END SUB
