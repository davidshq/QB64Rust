$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1): does the old compiler accept this? (72_restore_duplicate_label)
lab:
DATA 1
RESTORE lab
READ a: PRINT a
SYSTEM
SUB p
    lab:
    DATA 2
END SUB
