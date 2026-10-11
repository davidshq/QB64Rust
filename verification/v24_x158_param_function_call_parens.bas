$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a FUNCTION with an array parameter called with the array and a
' second argument.
DIM x(3) AS LONG
x(2) = 7
PRINT pick(x(), 2)
SYSTEM
FUNCTION pick (a() AS LONG, i AS LONG)
    pick = a(i)
END FUNCTION
