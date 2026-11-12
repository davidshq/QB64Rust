$CONSOLE:ONLY
ON ERROR GOTO trap
PRINT "With no game controller:"; STICK(0); STICK(1); STICK(2, 1)
x = STICK(0, 0)
PRINT "done"
END
trap:
PRINT "STICK(0, 0) raised error"; ERR
RESUME NEXT
