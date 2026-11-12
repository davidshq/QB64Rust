$CONSOLE:ONLY
OPEN "lof_1.tmp" FOR BINARY AS #1
s$ = "abcdef"
PUT #1, , s$
PRINT "The file holds"; LOF(1); "bytes."
CLOSE #1
KILL "lof_1.tmp"
END
