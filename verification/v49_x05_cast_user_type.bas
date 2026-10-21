$CONSOLE:ONLY
' Verification (m4-builtins-param-types, task 1.1): _CAST to a user type.
TYPE pt
    x AS LONG
END TYPE
x = _CAST(pt, 1)
PRINT x
SYSTEM
