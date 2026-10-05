$CONSOLE:ONLY
' Verification (m2-parser-breadth, M4): NEXT inside a multi-line IF, closing the FOR outside it.
FOR i = 1 TO 3
    IF i < 3 THEN
        PRINT i
    NEXT
END IF
PRINT "end"
SYSTEM
