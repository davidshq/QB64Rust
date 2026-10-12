$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1): does the old compiler accept this? (85_data_in_line_if)
IF 0 THEN DATA 1, 2
READ a, b: PRINT a; b
SYSTEM
