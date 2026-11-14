$CONSOLE:ONLY
ON ERROR GOTO trap
WAIT &H378, 1
PRINT "A port libqb does not emulate: WAIT returns"
WAIT 70000, 1
PRINT "done"
END
trap:
PRINT "error"; ERR
RESUME NEXT
