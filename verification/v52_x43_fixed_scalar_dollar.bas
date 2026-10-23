$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x43_fixed_scalar_dollar)
' name$ of a STRING * n scalar.
DIM s AS STRING * 4
s = "ab"
PRINT "["; s$; "]"
SYSTEM
