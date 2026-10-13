$CONSOLE:ONLY
' Verification (step 4 of M3, measured before its design): EXIT SELECT. In a CASE and in CASE ELSE, inside an IF
' and inside a loop within the CASE (which block it leaves), in a nested SELECT (the inner one only), with SELECT
' EVERYCASE, on a single-line IF, in a SUB, and EXIT CASE beside it.
DIM n AS LONG, k AS LONG
FOR n = 1 TO 4
    SELECT CASE n
        CASE 1
            PRINT "one";
            EXIT SELECT
            PRINT " not reached";
        CASE 2
            PRINT "two";
            IF n = 2 THEN EXIT SELECT
            PRINT " not reached";
        CASE 3
            PRINT "three";
            IF n = 3 THEN
                PRINT " in IF";
                EXIT SELECT
            END IF
            PRINT " not reached";
        CASE ELSE
            PRINT "else";
            EXIT SELECT
            PRINT " not reached";
    END SELECT
    PRINT " after"; n
NEXT
PRINT "inside a loop within the CASE"
SELECT CASE 1
    CASE 1
        FOR k = 1 TO 3
            PRINT k;
            IF k = 2 THEN EXIT SELECT
        NEXT
        PRINT "after the loop";
        DO
            EXIT SELECT
        LOOP
END SELECT
PRINT "after"; k
k = 0
SELECT CASE 1
    CASE 1
        DO
            k = k + 1
            WHILE k < 5
                k = k + 1
                IF k = 3 THEN EXIT SELECT
            WEND
        LOOP
END SELECT
PRINT "after"; k
PRINT "nested"
SELECT CASE 1
    CASE 1
        SELECT CASE 2
            CASE 2
                PRINT "inner";
                EXIT SELECT
                PRINT " not reached";
        END SELECT
        PRINT " outer goes on";
        EXIT SELECT
        PRINT " not reached";
END SELECT
PRINT " after"
PRINT "a loop around the SELECT"
FOR k = 1 TO 3
    SELECT CASE k
        CASE 2
            EXIT SELECT
        CASE ELSE
            PRINT k;
    END SELECT
NEXT
PRINT
PRINT "EVERYCASE"
SELECT EVERYCASE 5
    CASE 5
        PRINT "first";
        EXIT SELECT
        PRINT " not reached";
    CASE IS > 1
        PRINT " second";
    CASE ELSE
        PRINT " else";
END SELECT
PRINT " after"
SELECT EVERYCASE 5
    CASE 5
        PRINT "first";
        EXIT CASE
        PRINT " not reached";
    CASE IS > 1
        PRINT " second";
        EXIT CASE
        PRINT " not reached";
    CASE 7
        PRINT " third";
END SELECT
PRINT " after"
PRINT "in a SUB"
t 1: t 2
SYSTEM
SUB t (n AS LONG)
    SELECT CASE n
        CASE 1: PRINT "one": EXIT SELECT: PRINT "not reached"
        CASE 2: PRINT "two": EXIT SUB
    END SELECT
    PRINT "end of t"; n
END SUB
