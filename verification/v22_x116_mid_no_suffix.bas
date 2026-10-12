$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1): does the old compiler accept this? (116_mid_no_suffix)
DIM a AS LONG, b AS LONG, d AS DOUBLE, s AS STRING, t AS STRING, fx AS STRING * 3, fy AS STRING * 5
s = "abc"
MID(s, 1) = "x"
PRINT s
SYSTEM
