$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): SWAP of two elements of a _BIT * 4 array.
DIM b4(3) AS _BIT * 4
b4(0) = 3: b4(1) = 5
SWAP b4(0), b4(1)
PRINT b4(0); b4(1)
SYSTEM
