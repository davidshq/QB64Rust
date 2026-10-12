$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1): does the old compiler accept this? (101_swap_arrays)
DIM x(2) AS LONG, y(2) AS LONG
x(0) = 1: y(0) = 2
SWAP x(), y()
PRINT x(0); y(0)
SYSTEM
