$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a FUNCTION defined above DEFDBL for its letter and called below
' it without a suffix.
FUNCTION f (x)
    f = 100000 + 1# / 3
END FUNCTION
DEFDBL F
DIM r AS DOUBLE
r = f(3): PRINT r
SYSTEM
