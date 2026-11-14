$CONSOLE:ONLY
OPEN "loc_1.tmp" FOR BINARY AS #1
s$ = "abcdef"
PUT #1, , s$
PRINT "After writing 6 bytes, LOC is"; LOC(1)
c$ = " "
GET #1, 2, c$
PRINT "After reading byte 2, LOC is"; LOC(1)
CLOSE #1
KILL "loc_1.tmp"
END
