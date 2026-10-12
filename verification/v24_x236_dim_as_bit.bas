$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1, for task 3.3): DIM AS _BIT * 4 with a scalar and an array, and DIM AS _FLOAT.
DIM AS _BIT * 4 b1, b2(3)
DIM AS _FLOAT f1
b1 = 9: b2(1) = 7: f1 = 2.5
PRINT b1; b2(1); f1; LEN(f1)
SYSTEM
