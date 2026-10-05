$CONSOLE:ONLY
' Verification (m2-parser-breadth, M4): NEXT j, i closes two FOR blocks; NEXT without a variable.
FOR i = 1 TO 2
    FOR j = 1 TO 2
        PRINT i; j
NEXT j, i
FOR k = 1 TO 2
    PRINT "k"; k
NEXT
SYSTEM
