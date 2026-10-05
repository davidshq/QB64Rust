$CONSOLE:ONLY
' Verification (m2-parser-breadth, M4): NEXT in a single-line IF, closing the FOR outside it.
FOR i = 1 TO 3
    PRINT i
    IF i < 3 THEN NEXT
PRINT "end"
SYSTEM
