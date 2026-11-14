$CONSOLE:ONLY
DIM n AS LONG, s AS STRING * 4
OPEN "put_1.tmp" FOR BINARY AS #1
n = 1000000: s = "abcd"
PUT #1, 1, n
PUT #1, , s
PRINT "LOF is"; LOF(1)
CLOSE #1
KILL "put_1.tmp"
END
