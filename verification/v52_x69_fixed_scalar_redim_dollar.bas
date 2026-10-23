$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, review): does the old compiler accept this? (x69_fixed_scalar_redim_dollar)
' REDIM s$ (a scalar) after DIM s AS STRING * 4: the same variable or one of its own?
DIM s AS STRING * 4
REDIM s$
s$ = "abcdefg"
PRINT "["; s; "]["; s$; "]"
SYSTEM
