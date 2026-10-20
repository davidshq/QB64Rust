$CONSOLE:ONLY
' Verification (m4-devices-continue-paths, task 1.1): the accepted forms of ON STRIG and of the STRIG switch, with even button numbers.
ON STRIG(0) s 3
ON STRIG(4, 1) s2
ON STRIG(2.2) GOSUB h
ON STRIG(8, 1.6) GOSUB h
STRIG(0) ON
STRIG(4, 2) OFF
STRIG STOP
STRIG ON
STRIG(6) STOP
STRIG(2, 1) ON
PRINT "ok"
SYSTEM
h:
RETURN
SUB s (n AS INTEGER)
PRINT n
END SUB
SUB s2
END SUB
