$CONSOLE:ONLY
' Verification (m4-builtins-param-types, task 1.1): _CAST of a whole user-type value.
TYPE pt
    x AS LONG
END TYPE
DIM p AS pt
x = _CAST(LONG, p)
PRINT x
SYSTEM
