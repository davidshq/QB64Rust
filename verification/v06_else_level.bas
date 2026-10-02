$CONSOLE:ONLY
' Verification: ELSE while a FOR inside the IF block is still open (qb64pe.bas 6627-6638).
' QB4.5 rejects this. The old compiler searches down for the IF but updates the top entry.
x = 0
IF x THEN
    FOR i = 1 TO 2
        PRINT "then"; i
ELSE
    PRINT "else"
    NEXT
END IF
PRINT "end"
SYSTEM
