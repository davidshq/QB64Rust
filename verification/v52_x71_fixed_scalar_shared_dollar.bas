$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, review): does the old compiler accept this? (x71_fixed_scalar_shared_dollar)
' SHARED s$ in a SUB after DIM s AS STRING * 4 in the main module.
DIM s AS STRING * 4
s = "m"
t
PRINT "["; s; "]"
SYSTEM
SUB t
    SHARED s$
    s$ = "abcdefg"
    PRINT "["; s$; "]"
END SUB
