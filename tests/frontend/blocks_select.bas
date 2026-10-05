' TEST: parse-ok
$CONSOLE:ONLY
' SELECT CASE and SELECT EVERYCASE in the forms the old compiler accepts (m2-parser-breadth task 6.2; measured in
' verification\v16_m4_select_forms, v16_m4_loop_forms)
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
SELECT EVERYCASE 5
    CASE IS > 1: PRINT "gt1"
    CASE 5: PRINT "five": EXIT CASE
    CASE 1 TO 9: PRINT "range"
END SELECT
SELECT CASE "b"
    CASE "a" TO "c", IS = "z"
        PRINT "string range"
        EXIT SELECT
    CASE ELSE
        PRINT "no"
END SELECT
SELECT CASE 1
END SELECT
SYSTEM
