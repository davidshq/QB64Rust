$CONSOLE:ONLY
x = 7: s$ = "men"
PRINT "Ney has"; 3000; s$
PRINT "[" + "joined" + "]"
PRINT x; -x; 1.5
PRINT "no line end ";
PRINT "here"
? "A question mark is PRINT."
OPEN "print_1.tmp" FOR OUTPUT AS #1
PRINT #1, "a", "b", 5
CLOSE #1
OPEN "print_1.tmp" FOR INPUT AS #1
LINE INPUT #1, l$
CLOSE #1
KILL "print_1.tmp"
PRINT "[" + l$ + "]"
END
