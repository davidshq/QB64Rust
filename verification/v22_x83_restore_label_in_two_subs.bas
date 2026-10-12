$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1): does the old compiler accept this? (83_restore_label_in_two_subs)
RESTORE lab
READ a: PRINT a
SYSTEM
SUB p
    lab:
    DATA 1
END SUB
SUB q
    lab:
    DATA 2
END SUB
