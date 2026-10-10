$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1): does the old compiler accept this? (113_mid_four_arguments)
DIM a AS LONG, b AS LONG, d AS DOUBLE, s AS STRING, t AS STRING, fx AS STRING * 3, fy AS STRING * 5
MID$(s, 1, 1, 1) = "x"
SYSTEM
