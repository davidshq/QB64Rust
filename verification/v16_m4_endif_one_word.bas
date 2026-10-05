$CONSOLE:ONLY
' Verification (m2-parser-breadth, M4): ENDIF as one word; ELSE IF as two words
IF 1 THEN
    PRINT "one"
ENDIF
IF 0 THEN
    PRINT "no"
ELSE IF 1 THEN
        PRINT "else if"
    END IF
END IF
SYSTEM
