$CONSOLE:ONLY
' Verification (m4-builtins-param-types, task 1.1): _CAST of a member and of an element passed to a SUB (accepted).
TYPE pt
    x AS LONG
END TYPE
DIM p AS pt, a(2) AS SINGLE
p.x = 70000: a(1) = 2.75
show _CAST(INTEGER, p.x), _CAST(_BYTE, a(1))
SYSTEM
SUB show (i AS INTEGER, b AS _BYTE)
    PRINT i; b
END SUB
