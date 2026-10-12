$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): an array and a FUNCTION of the same name.
DIM f(3) AS LONG
f(1) = 2
PRINT f(1)
SYSTEM

FUNCTION f (n)
    f = n * 100
END FUNCTION
