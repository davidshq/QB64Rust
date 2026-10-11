$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): inside a FUNCTION with a DEFINT of its own, the result named
' without a suffix.
DIM r AS DOUBLE
r = f(3): PRINT r
SYSTEM
FUNCTION f (x)
    DEFINT A-Z
    f = 100000 + 1# / 3
END FUNCTION
