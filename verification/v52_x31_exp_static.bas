$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x31_exp_static)
' exp$ in STATIC in a SUB.
s
s
SYSTEM
SUB s
    STATIC exp$
    exp$ = exp$ + "e"
    PRINT exp$
END SUB
