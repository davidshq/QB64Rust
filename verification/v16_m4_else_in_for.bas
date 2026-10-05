$CONSOLE:ONLY
' Verification (m2-parser-breadth, M4, group 6): ELSE inside a FOR inside a block IF.
IF 1 THEN
    FOR i = 1 TO 2
    ELSE
    NEXT
END IF
SYSTEM
