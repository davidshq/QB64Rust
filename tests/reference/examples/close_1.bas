$CONSOLE:ONLY
OPEN "close_1.tmp" FOR OUTPUT AS #1
OPEN "close_1b.tmp" FOR OUTPUT AS #2
CLOSE #1, #2
CLOSE #7
PRINT "Closing a file that is not open is no error."
KILL "close_1.tmp": KILL "close_1b.tmp"
END
