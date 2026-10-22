$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x24_exp_base_param)
' exp$ and base$ as parameters.
s "1", "2"
SYSTEM
SUB s (exp$, base$)
    PRINT exp$ + base$
END SUB
