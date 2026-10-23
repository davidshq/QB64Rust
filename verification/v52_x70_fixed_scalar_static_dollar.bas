$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, review): does the old compiler accept this? (x70_fixed_scalar_static_dollar)
' STATIC s$ after STATIC s AS STRING * 2 in a SUB: the same variable or one of its own?
t
SYSTEM
SUB t
    STATIC s AS STRING * 2
    STATIC s$
    s$ = "abcdefg"
    PRINT "["; s; "]["; s$; "]"
END SUB
