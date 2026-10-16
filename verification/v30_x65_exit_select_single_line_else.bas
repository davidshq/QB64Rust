$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (65_exit_select_single_line_else)
DIM n AS LONG
SELECT CASE 1
    CASE 1
        IF n THEN PRINT "a" ELSE EXIT SELECT
        PRINT "b"
END SELECT
PRINT "ok"
SYSTEM
