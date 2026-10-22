$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x26_exp_function_after)
' exp$ as a variable, then the function EXP.
exp$ = "2"
PRINT exp$; EXP(0)
SYSTEM
