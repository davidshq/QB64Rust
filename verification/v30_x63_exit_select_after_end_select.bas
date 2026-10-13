$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (63_exit_select_after_end_select)
SELECT CASE 1
    CASE 1
END SELECT
EXIT SELECT
PRINT "ok"
SYSTEM
