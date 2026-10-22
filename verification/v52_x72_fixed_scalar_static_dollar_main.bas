$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, review): does the old compiler accept this? (x72_fixed_scalar_static_dollar_main)
' STATIC s$ in the main module after DIM s AS STRING * 4.
DIM s AS STRING * 4
STATIC s$
s$ = "abcdefg"
PRINT "["; s; "]["; s$; "]"
SYSTEM
