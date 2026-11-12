$CONSOLE:ONLY
n = _ENVIRONCOUNT
PRINT "Some variables:"; n > 0
PRINT "The last one has an = in it:"; INSTR(ENVIRON$(n), "=") > 0
PRINT "Past the last: ["; ENVIRON$(n + 1); "]"
END
