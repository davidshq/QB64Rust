$CONSOLE:ONLY
OPEN "seek_1.tmp" FOR BINARY AS #1
s$ = "abcdef"
PUT #1, , s$
SEEK #1, 3
PRINT "The position is"; SEEK(1)
c$ = " "
GET #1, , c$
PRINT "The byte there is "; c$
CLOSE #1
KILL "seek_1.tmp"
END
