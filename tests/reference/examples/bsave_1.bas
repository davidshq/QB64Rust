$CONSOLE:ONLY
DIM a(1 TO 10) AS INTEGER
FOR i = 1 TO 10: a(i) = i * 11: NEXT
DEF SEG = VARSEG(a(1))
BSAVE "bsave_1.tmp", VARPTR(a(1)), 20
DEF SEG
OPEN "bsave_1.tmp" FOR BINARY AS #1
PRINT "20 bytes and a header of 7:"; LOF(1)
CLOSE #1
KILL "bsave_1.tmp"
END
