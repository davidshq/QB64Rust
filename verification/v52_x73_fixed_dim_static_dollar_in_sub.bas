$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, review): does the old compiler accept this? (x73_fixed_dim_static_dollar_in_sub)
' STATIC s$ after DIM s AS STRING * 2 in a SUB called twice: the same variable, or a static one of its own?
t
t
SYSTEM
SUB t
    DIM s AS STRING * 2
    STATIC s$
    s$ = s$ + "a"
    PRINT "["; s; "]["; s$; "]"
END SUB
