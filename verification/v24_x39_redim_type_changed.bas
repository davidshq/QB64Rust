$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): REDIM of an array with another numeric type.
REDIM x(5) AS LONG
REDIM x(5) AS INTEGER
x(1) = 100000: PRINT x(1); LEN(x(1))
SYSTEM
