$CONSOLE:ONLY
ON ERROR GOTO trap
PRINT "With no game controller:"; STRIG(0); STRIG(1); STRIG(4, 1)
ON STRIG(0) GOSUB pressed
STRIG(0) ON
STRIG OFF
x = STRIG(70000)
PRINT "done"
END
pressed:
PRINT "pressed"
RETURN
trap:
PRINT "STRIG(70000) raised error"; ERR
RESUME NEXT
