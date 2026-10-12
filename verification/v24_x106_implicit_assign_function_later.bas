$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): name(3) = 5 where a FUNCTION of that name is defined below.
f(3) = 5
PRINT f(3)
SYSTEM
FUNCTION f (n)
    f = n * 2
END FUNCTION
