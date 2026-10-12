$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1): does the old compiler accept this? (99_swap_signed_unsigned)
DIM l AS LONG, u AS _UNSIGNED LONG
l = -1: u = 2
SWAP l, u
PRINT l; u
SYSTEM
