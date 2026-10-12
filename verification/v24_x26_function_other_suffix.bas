$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a FUNCTION typed LONG by DEFLNG called with the suffix of
' another type.
DEFLNG F
PRINT f!(3)
SYSTEM
FUNCTION f (x)
    f = x * 100000
END FUNCTION
