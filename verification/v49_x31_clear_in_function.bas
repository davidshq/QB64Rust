$CONSOLE:ONLY
' Verification (m4-builtins-param-types, task 1.1): CLEAR in a FUNCTION.
PRINT f
SYSTEM
FUNCTION f
    CLEAR
    f = 1
END FUNCTION
