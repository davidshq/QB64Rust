$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x49_fixed_scalar_dollar_and_own)
' A STRING * n scalar and a variable name$ of its own.
DIM s AS STRING * 4
DIM s$
s = "f": s$ = "v"
PRINT "["; s; "]["; s$; "]"
SYSTEM
