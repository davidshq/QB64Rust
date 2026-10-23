$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x25_exp_implicit)
' exp$ as a variable not declared.
exp$ = "1"
PRINT exp$
SYSTEM
