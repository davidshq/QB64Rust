$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x08_asc_function_place)
' A FUNCTION's result as the place, outside the FUNCTION.
ASC(f$) = 65
PRINT f$
SYSTEM
FUNCTION f$
    f$ = "xy"
END FUNCTION
