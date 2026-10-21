$CONSOLE:ONLY
' Verification (m4-builtins-param-types, task 1.1): a SUB whose parameter's type is defined in its own body.
DIM v AS own
v.k = 2
use v
SYSTEM
SUB use (p AS own)
    TYPE own
        k AS LONG
    END TYPE
    PRINT p.k
END SUB
