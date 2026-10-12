$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): an array used without DIM with one index, then with two.
z(1) = 1
z(1, 2) = 1
PRINT z(1)
SYSTEM
