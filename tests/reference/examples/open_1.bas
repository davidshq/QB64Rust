$CONSOLE:ONLY
OPEN "open_1.tmp" FOR OUTPUT AS #1
PRINT #1, "first line"
CLOSE #1
OPEN "open_1.tmp" FOR APPEND AS #1
PRINT #1, "added line"
CLOSE #1
OPEN "open_1.tmp" FOR INPUT AS #1
DO UNTIL EOF(1)
    LINE INPUT #1, l$
    PRINT l$
LOOP
CLOSE #1
KILL "open_1.tmp"
END
