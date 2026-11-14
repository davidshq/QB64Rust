$CONSOLE:ONLY
FOR i = 1 TO 10
    IF i * i > 20 THEN EXIT FOR
NEXT i
PRINT "The first square above 20 is of"; i
SELECT CASE i
    CASE 5
        PRINT "five"
        IF i > 4 THEN EXIT SELECT
        PRINT "not printed"
END SELECT
Check 0
Check 2
END

SUB Check (v)
    IF v = 0 THEN EXIT SUB
    PRINT "v ="; v
END SUB
