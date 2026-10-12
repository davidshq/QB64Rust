$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1): does the old compiler accept this? (73_data_between_subs)
READ a: PRINT a
SYSTEM
SUB p
END SUB
DATA 1
SUB q
END SUB
