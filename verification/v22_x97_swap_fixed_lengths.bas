$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1): does the old compiler accept this? (97_swap_fixed_lengths)
DIM a AS LONG, b AS LONG, d AS DOUBLE, s AS STRING, t AS STRING, fx AS STRING * 3, fy AS STRING * 5
fx = "abc": fy = "vwxyz"
SWAP fx, fy
PRINT "["; fx; "]["; fy; "]"
SYSTEM
