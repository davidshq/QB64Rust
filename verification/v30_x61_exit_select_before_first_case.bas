$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (61_exit_select_before_first_case)
SELECT CASE 1
    EXIT SELECT
    CASE 1
END SELECT
PRINT "ok"
SYSTEM
