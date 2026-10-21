$CONSOLE:ONLY
' Verification (m4-builtins-param-types, task 1.1): a SUB never called whose parameter's type is defined below it.
PRINT 1
SYSTEM
SUB use (p AS late)
END SUB
TYPE late
    k AS LONG
END TYPE
