$CONSOLE:ONLY
' Verification (step 4 of M3, task 3.1 of m3-console-files): does the old compiler accept this, and what does it
' do? (66_exit_case_before_nested_select) EXIT CASE names its label by a count of the CASEs closed so far, so a
' SELECT with a CASE after it in the same CASE takes the jump: where does the program go on?
DIM n AS LONG
SELECT CASE 1
    CASE 1
        PRINT "a";
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
SELECT EVERYCASE 1
    CASE 1
        PRINT "b";
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
SELECT CASE 1
    CASE 1
        SELECT CASE 2
            CASE 2
                PRINT "c";
        END SELECT
        IF n = 0 THEN EXIT CASE
        PRINT " skipped";
    CASE ELSE
        PRINT " else";
END SELECT
PRINT " end"
SYSTEM
