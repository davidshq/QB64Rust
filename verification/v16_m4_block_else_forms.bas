$CONSOLE:ONLY
' Verification (m2-parser-breadth, M4, group 6): ELSE and ELSEIF ... THEN followed by a statement on the same line; ELSE and END IF after a colon.
FOR x = 0 TO 2
    IF x = 0 THEN
        PRINT "zero"
    ELSEIF x = 1 THEN PRINT "one (same line)"
    ELSE PRINT "else (same line)": PRINT "second"
    END IF
NEXT
IF 0 THEN
    PRINT "a": ELSE PRINT "b"
END IF
IF 1 THEN
    PRINT "c": END IF
SYSTEM
