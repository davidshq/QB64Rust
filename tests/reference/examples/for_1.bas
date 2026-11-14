$CONSOLE:ONLY
FOR i = 1 TO 3
    PRINT "i ="; i
NEXT i
PRINT "After the loop, i ="; i
FOR n = 10 TO 0 STEP -5
    PRINT n;
NEXT
PRINT
FOR k = 5 TO 1
    PRINT "never"
NEXT k
PRINT "A loop from 5 to 1 does not run; k ="; k
END
