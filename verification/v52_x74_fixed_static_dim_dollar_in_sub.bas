$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, review): does the old compiler accept this? (x74_fixed_static_dim_dollar_in_sub)
' DIM s$ after STATIC s AS STRING * 2 in a SUB called twice: the same variable, or a local one of its own?
t
t
SYSTEM
SUB t
    STATIC s AS STRING * 2
    DIM s$
    s$ = s$ + "a"
    PRINT "["; s; "]["; s$; "]"
END SUB
