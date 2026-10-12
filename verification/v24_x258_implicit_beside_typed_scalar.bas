$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, 5.1): an array used without DIM whose plain name a DIM ... AS typed as
' a scalar: which type the array has.
DIM x AS LONG
x = 7
x(3) = 2.5
PRINT x; x(3); LEN(x(3)); UBOUND(x)
SYSTEM
