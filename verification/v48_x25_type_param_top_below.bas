$CONSOLE:ONLY
' Verification (m4-names-arrays, task 1.1): a SUB whose parameter's type is a top-level TYPE block below the SUB.
DIM v AS late
v.k = 6
use v
SYSTEM

SUB use (p AS late)
    PRINT p.k; LEN(p)
END SUB

TYPE late
    k AS _UNSIGNED INTEGER
END TYPE
