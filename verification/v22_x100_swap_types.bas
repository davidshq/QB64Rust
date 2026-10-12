$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1): does the old compiler accept this? (100_swap_types)
TYPE r1
    n AS LONG
END TYPE
TYPE r2
    n AS LONG
END TYPE
DIM x AS r1, y AS r2
x.n = 1: y.n = 2
SWAP x, y
PRINT x.n; y.n
SYSTEM
