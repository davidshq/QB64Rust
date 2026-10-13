$CONSOLE:ONLY
' Slice (m3-console-files, task 3.1; from verification\v30_f_exit_select, v30_x62, x65, x66): EXIT SELECT and EXIT
' CASE, 12 numbered lines. EXIT SELECT from a CASE, from an IF and from loops inside it, from CASE ELSE; a loop
' around the SELECT goes on; the inner of two SELECTs; SELECT EVERYCASE, where EXIT CASE goes on with the next CASE
' and the block still counts as matched; a single-line IF; in a SUB; a string selector; and EXIT CASE before a
' SELECT in the same CASE, which goes on after that inner SELECT (the old compiler's count of CASEs).
DIM n AS LONG, k AS LONG, v AS STRING
PRINT "1:";
FOR n = 1 TO 4
    SELECT CASE n
        CASE 1
            PRINT " one";
            EXIT SELECT
            PRINT " not reached";
        CASE 2
            PRINT " two";
            IF n = 2 THEN EXIT SELECT
            PRINT " not reached";
        CASE 3
            PRINT " three";
            IF n = 3 THEN
                PRINT " in IF";
                EXIT SELECT
            END IF
            PRINT " not reached";
        CASE ELSE
            PRINT " else";
            EXIT SELECT
            PRINT " not reached";
    END SELECT
    PRINT n;
NEXT
PRINT
PRINT "2:";
SELECT CASE 1
    CASE 1
        FOR k = 1 TO 3
            PRINT k;
            IF k = 2 THEN EXIT SELECT
        NEXT
        PRINT "not reached";
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
PRINT "3: after"; k
PRINT "4:";
SELECT CASE 1
    CASE 1
        SELECT CASE 2
            CASE 2
                PRINT " inner";
                EXIT SELECT
                PRINT " not reached";
        END SELECT
        PRINT " outer goes on";
        EXIT SELECT
        PRINT " not reached";
END SELECT
PRINT " after"
PRINT "5:";
FOR k = 1 TO 4
    SELECT CASE k
        CASE 2
            EXIT SELECT
        CASE 4
            EXIT FOR
        CASE ELSE
            PRINT k;
    END SELECT
NEXT
PRINT "after"; k
PRINT "6:";
SELECT EVERYCASE 5
    CASE 5
        PRINT " first";
        EXIT SELECT
        PRINT " not reached";
    CASE IS > 1
        PRINT " second";
    CASE ELSE
        PRINT " else";
END SELECT
PRINT " after"
PRINT "7:";
SELECT EVERYCASE 5
    CASE 5
        PRINT " first";
        EXIT CASE
        PRINT " not reached";
    CASE IS > 1
        PRINT " second";
        EXIT CASE
        PRINT " not reached";
    CASE 7
        PRINT " third";
    CASE ELSE
        PRINT " else";
END SELECT
PRINT " after"
PRINT "8:";
SELECT EVERYCASE 5
    CASE 1
        PRINT " first";
    CASE ELSE
        PRINT " else";
        EXIT CASE
        PRINT " not reached";
END SELECT
SELECT CASE 1
    CASE 1
        PRINT " plain";
        EXIT CASE
        PRINT " not reached";
    CASE ELSE
        PRINT " else";
END SELECT
PRINT " after"
PRINT "9:";
n = 0
SELECT CASE 1
    CASE 1
        IF n THEN PRINT " a"; ELSE EXIT SELECT
        PRINT " not reached";
END SELECT
t 1: t 2: t 3
PRINT " after"
v = "b"
PRINT "10:";
SELECT CASE v + "c"
    CASE "bc"
        PRINT " string";
        IF LEN(v) = 1 THEN EXIT SELECT
        PRINT " not reached";
    CASE ELSE
        PRINT " else";
END SELECT
PRINT " after"
PRINT "11:";
SELECT CASE 1
    CASE 1
        PRINT " a";
        IF n = 0 THEN EXIT CASE
        PRINT " skipped";
        SELECT CASE 2
            CASE 2
                PRINT " inner";
            CASE 3
                PRINT " inner 3";
        END SELECT
        PRINT " after the inner SELECT";
    CASE ELSE
        PRINT " else";
END SELECT
PRINT " end"
PRINT "12:";
SELECT EVERYCASE 1
    CASE 1
        PRINT " b";
        IF n = 0 THEN EXIT CASE
        PRINT " skipped";
        SELECT EVERYCASE 2
            CASE 2
                PRINT " inner";
            CASE IS > 1
                PRINT " inner second";
            CASE ELSE
                PRINT " inner else";
        END SELECT
        PRINT " after the inner SELECT";
    CASE IS > 0
        PRINT " second";
    CASE ELSE
        PRINT " else";
END SELECT
PRINT " end"
SYSTEM
SUB t (n AS LONG)
    SELECT CASE n
        CASE 1: PRINT " one";: EXIT SELECT: PRINT " not reached";
        CASE 2: PRINT " two";: EXIT SUB
        CASE ELSE: EXIT CASE
    END SELECT
    PRINT " end of t"; n;
END SUB
