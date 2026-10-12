$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1): does the old compiler accept this? (82_duplicate_label_no_restore)
lab:
DATA 1
READ a: PRINT a
SYSTEM
SUB p
    lab:
    DATA 2
END SUB
