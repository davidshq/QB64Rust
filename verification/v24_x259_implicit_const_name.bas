$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, 5.1): an array used without DIM named like a CONST.
CONST k = 1
k(1) = 2
PRINT k(1)
SYSTEM
