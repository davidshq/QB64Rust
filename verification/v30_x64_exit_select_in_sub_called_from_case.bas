$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (64_exit_select_in_sub_called_from_case)
SELECT CASE 1
    CASE 1
        t
END SELECT
SYSTEM
SUB t
    EXIT SELECT
END SUB
