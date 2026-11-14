$CONSOLE:ONLY
OPEN "eof_1.tmp" FOR OUTPUT AS #1
PRINT #1, "one": PRINT #1, "two"
CLOSE #1
OPEN "eof_1.tmp" FOR INPUT AS #1
n = 0
WHILE NOT EOF(1)
    LINE INPUT #1, l$
    n = n + 1
WEND
CLOSE #1
PRINT "Read"; n; "lines."
KILL "eof_1.tmp"
END
