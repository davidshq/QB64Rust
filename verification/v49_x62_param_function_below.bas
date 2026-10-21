$CONSOLE:ONLY
' Verification (m4-builtins-param-types, task 1.1): a FUNCTION parameter whose type is defined below it.
DIM v AS late
PRINT f(v)
SYSTEM
FUNCTION f (p AS late)
    f = p.k
END FUNCTION
TYPE late
    k AS LONG
END TYPE
