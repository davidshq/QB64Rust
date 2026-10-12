$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1, for task 3.3): a DIM AS type list that starts with an array, and one with two arrays.
DIM AS LONG b(3), a
DIM AS INTEGER c(1 TO 2), d(2, 2)
b(3) = 100000: a = 5: c(2) = 7: d(2, 2) = 8
PRINT b(3); a; c(2); d(2, 2); LEN(c(1))
SYSTEM
