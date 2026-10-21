$CONSOLE:ONLY
' Verification (m4-builtins-param-types, task 1.1): a SUB whose parameter's type is defined in an IF block of the main module above it.
IF 0 THEN
    TYPE early
        k AS LONG
    END TYPE
END IF
DIM v AS early
v.k = 3
use v
SYSTEM
SUB use (p AS early)
    PRINT p.k
END SUB
