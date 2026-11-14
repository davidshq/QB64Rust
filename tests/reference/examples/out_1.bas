$CONSOLE:ONLY
ON ERROR GOTO trap
OUT &H378, 1
PRINT "A port libqb does not emulate: no error"
OUT 70000, 1
PRINT "A port out of range: no error either"
END
trap:
PRINT "error"; ERR
RESUME NEXT
