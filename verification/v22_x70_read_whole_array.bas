$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1): does the old compiler accept this? (70_read_whole_array)
DIM a(3) AS LONG
DATA 1,2,3,4
READ a()
PRINT a(0); a(3)
SYSTEM
