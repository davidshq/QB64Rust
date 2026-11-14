$CONSOLE:ONLY
OPEN "write_1.tmp" FOR OUTPUT AS #1
WRITE #1, "Davout", 26000, 0.9
CLOSE #1
OPEN "write_1.tmp" FOR INPUT AS #1
LINE INPUT #1, l$
CLOSE #1
PRINT l$
OPEN "write_1.tmp" FOR INPUT AS #1
INPUT #1, n$, men, morale
CLOSE #1
PRINT n$; men; morale
KILL "write_1.tmp"
END
