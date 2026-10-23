$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x35_exp_function_param)
' exp$ as a FUNCTION's parameter, beside a call of EXP.
PRINT f$("3")
SYSTEM
FUNCTION f$ (exp$)
    f$ = exp$ + STR$(EXP(0))
END FUNCTION
