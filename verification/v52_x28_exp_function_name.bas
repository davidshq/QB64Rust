$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x28_exp_function_name)
' A FUNCTION named exp$.
PRINT exp$("a")
SYSTEM
FUNCTION exp$ (a$)
    exp$ = a$ + "!"
END FUNCTION
