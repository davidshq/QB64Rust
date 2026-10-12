$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): COMMON x (no parentheses), then DIM x(3).
COMMON x
DIM x(3)
x = 1: x(2) = 2: PRINT x; x(2)
SYSTEM
