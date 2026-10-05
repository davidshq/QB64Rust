$CONSOLE:ONLY
' Verification (m2-parser-breadth, M4): EXIT FOR inside an IF inside a DO inside a FOR; EXIT DO from a nested FOR.
FOR i = 1 TO 3
    DO
        IF i = 2 THEN EXIT FOR
        PRINT "i"; i
        EXIT DO
    LOOP
NEXT
PRINT "after for"; i
DO
    FOR j = 1 TO 3
        IF j = 2 THEN EXIT DO
        PRINT "j"; j
    NEXT
LOOP
PRINT "after do"; j
SYSTEM
