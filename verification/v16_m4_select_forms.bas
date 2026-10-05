$CONSOLE:ONLY
' Verification (m2-parser-breadth, M4): CASE forms; statements between SELECT CASE and the first CASE.
FOR x = 0 TO 6
    SELECT CASE x
        CASE IS < 1: PRINT x; "is < 1"
        CASE 1, 2: PRINT x; "1, 2"
        CASE 3 TO 4, IS = 6: PRINT x; "3 TO 4, IS = 6"
        CASE ELSE: PRINT x; "else"
    END SELECT
NEXT
SELECT CASE 1
    ' a comment before the first CASE
    CASE 1: PRINT "comment ok"
END SELECT
SYSTEM
