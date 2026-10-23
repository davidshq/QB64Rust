$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x44_fixed_scalar_dollar_store)
' name$ as a place of a STRING * n scalar.
DIM s AS STRING * 4
s$ = "x"
PRINT "["; s; "]"
SYSTEM
