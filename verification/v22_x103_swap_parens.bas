$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1): does the old compiler accept this? (103_swap_parens)
DIM a AS LONG, b AS LONG, d AS DOUBLE, s AS STRING, t AS STRING, fx AS STRING * 3, fy AS STRING * 5
a = 1: b = 2
SWAP (a), b
PRINT a; b
SYSTEM
