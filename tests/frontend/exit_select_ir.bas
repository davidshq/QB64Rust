' TEST: ir
$CONSOLE:ONLY
' EXIT SELECT and EXIT CASE in the IR (m3-console-files task 3.1, design D5; verification\v30_f_exit_select,
' v30_x62, x65, x66): a jump. EXIT SELECT goes to the end of the SELECT whose CASE it stands in, also from an IF
' or a loop inside that CASE; EXIT CASE to the end of its CASE body, before what ends the CASE (the jump out of a
' plain SELECT, the mark of SELECT EVERYCASE), and in a CASE ELSE to the end of the SELECT
DIM n AS LONG, k AS LONG
' A plain SELECT: from a CASE, from inside an IF, from CASE ELSE; EXIT CASE leaves it too
SELECT CASE n
    CASE 1
        EXIT SELECT
    CASE 2
        IF k THEN EXIT SELECT
        PRINT "b"
    CASE 3
        EXIT CASE
        PRINT "c"
    CASE ELSE
        EXIT CASE
END SELECT
' EVERYCASE: EXIT SELECT leaves the block, EXIT CASE goes on with the next CASE
SELECT EVERYCASE n
    CASE 1
        EXIT CASE
        PRINT "a"
    CASE IS > 0
        EXIT SELECT
    CASE ELSE
        EXIT SELECT
END SELECT
' From a loop inside the CASE; a loop around the SELECT is not left, and EXIT FOR still finds it
FOR k = 1 TO 3
    SELECT CASE k
        CASE 1
            DO
                EXIT SELECT
            LOOP
        CASE 2
            EXIT FOR
    END SELECT
NEXT
' A SELECT inside a CASE: the inner one
SELECT CASE n
    CASE 1
        SELECT CASE k
            CASE 2
                EXIT SELECT
        END SELECT
        EXIT SELECT
END SELECT
SYSTEM
