$CONSOLE:ONLY
' Verification (m4-builtins-param-types, task 1.1): a SUB whose parameter's type is defined in a FUNCTION below it.
PRINT 1
SYSTEM
SUB use (p AS late)
END SUB
FUNCTION g
    TYPE late
        k AS LONG
    END TYPE
END FUNCTION
