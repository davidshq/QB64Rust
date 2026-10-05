$CONSOLE:ONLY
' Verification (m2-parser-breadth, M4, group 6): DO/LOOP with conditions at either end, WHILE/WEND, EXIT WHILE, SELECT EVERYCASE, EXIT SELECT, EXIT CASE, CASE forms, FOR with STEP and a suffixed variable.
i = 0
DO WHILE i < 2: i = i + 1: LOOP
DO UNTIL i = 4: i = i + 1: LOOP
DO: i = i + 1: LOOP WHILE i < 6
DO: i = i + 1: LOOP UNTIL i = 8
PRINT i
WHILE i > 0
    i = i - 1
    IF i = 5 THEN EXIT WHILE
WEND
PRINT i
FOR k% = 10 TO 1 STEP -3: PRINT k%;: NEXT k%
PRINT
SELECT EVERYCASE 5
    CASE IS > 1: PRINT "gt1"
    CASE 5: PRINT "five": EXIT CASE
    CASE 1 TO 9: PRINT "range"
END SELECT
SELECT CASE "b"
    CASE "a" TO "c", IS = "z"
        PRINT "string range"
        EXIT SELECT
        PRINT "not here"
    CASE ELSE
        PRINT "no"
END SELECT
SELECT CASE 1
END SELECT
PRINT "empty select"
SYSTEM
