$CONSOLE:ONLY
' Verification (m2-parser-breadth, M6, review): EXIT FOR inside a $IF inside a FOR.
FOR i = 1 TO 3
$IF WIN THEN
    IF i = 2 THEN EXIT FOR
$END IF
    PRINT i
NEXT
PRINT "after"; i
SYSTEM
