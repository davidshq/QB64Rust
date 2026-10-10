$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1): does the old compiler accept this? (106_swap_bit_widths)
DIM x AS _BIT * 3, y AS _BIT * 5
x = 1: y = 2
SWAP x, y
PRINT x; y
SYSTEM
