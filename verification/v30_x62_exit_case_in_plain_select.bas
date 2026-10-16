$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (62_exit_case_in_plain_select)
SELECT CASE 1
    CASE 1
        PRINT "a"
        EXIT CASE
        PRINT "b"
END SELECT
PRINT "ok"
SYSTEM
