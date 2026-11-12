$CONSOLE:ONLY
ON ERROR GOTO trap
PRINT "A port libqb does not emulate:"; INP(&H379)
x = INP(70000)
END
trap:
PRINT "error"; ERR
RESUME NEXT
