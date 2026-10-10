$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1): does the old compiler accept this? (98_swap_integer_long)
DIM i AS INTEGER, l AS LONG
i = 1: l = 2
SWAP i, l
PRINT i; l
SYSTEM
