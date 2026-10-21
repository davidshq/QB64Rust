$CONSOLE:ONLY
' Verification (m4-names-arrays, task 1.1): REDIM of two scalars and an array in one list in a FUNCTION.
PRINT f(2)
SYSTEM

FUNCTION f (n)
    REDIM a AS LONG, v(n) AS INTEGER, s AS STRING
    a = 40000: v(n) = 3: s = "z"
    f = a + v(n) + LEN(s)
END FUNCTION
