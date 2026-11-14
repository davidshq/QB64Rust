$CONSOLE:ONLY
f = FREEFILE
OPEN "freefile_1.tmp" FOR OUTPUT AS #f
PRINT "The first free number was"; f; "and the next is"; FREEFILE
CLOSE #f
KILL "freefile_1.tmp"
END
