$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1): does the old compiler accept this? (117_mid_function_result)
PRINT f$
SYSTEM
FUNCTION f$
    f$ = "abc"
    MID$(f$, 1) = "x"
END FUNCTION
