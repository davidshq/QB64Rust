$CONSOLE:ONLY
FOR score = 0 TO 100 STEP 25
    SELECT CASE score
        CASE 0
            PRINT score; "nothing yet"
        CASE 1 TO 49
            PRINT score; "keep going"
        CASE 50, 70
            PRINT score; "a round number"
        CASE IS >= 100
            PRINT score; "full marks"
        CASE ELSE
            PRINT score; "nearly there"
    END SELECT
NEXT

word$ = "dog"
SELECT CASE word$
    CASE "a" TO "m": PRINT word$; " is in the first half of the alphabet"
    CASE ELSE: PRINT word$; " is in the second half"
END SELECT

' SELECT EVERYCASE runs every CASE that matches.
SELECT EVERYCASE 12
    CASE IS > 10: PRINT "more than 10"
    CASE IS > 5: PRINT "more than 5"
    CASE IS > 20: PRINT "more than 20"
END SELECT
END
