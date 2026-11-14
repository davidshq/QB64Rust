$CONSOLE:ONLY
REDIM scores(1 TO 3) AS INTEGER
scores(1) = 90: scores(2) = 80: scores(3) = 70
REDIM _PRESERVE scores(1 TO 5) AS INTEGER
PRINT "After _PRESERVE:"; scores(1); scores(3); scores(5)
REDIM _RETAIN scores(3 TO 6) AS INTEGER
PRINT "After _RETAIN to 3 TO 6:"; scores(3); scores(4); scores(6)
REDIM scores(1 TO 2) AS INTEGER
PRINT "After a plain REDIM:"; scores(1); UBOUND(scores)
END
