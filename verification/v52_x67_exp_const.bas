$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, review): does the old compiler accept this? (x67_exp_const)
' exp$ (a built-in without a required suffix) as a CONST.
CONST exp$ = "a"
PRINT exp$
SYSTEM
