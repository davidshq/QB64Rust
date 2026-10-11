$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a FUNCTION typed LONG by DEFLNG called without a suffix after
' DEFSNG for its letter.
DEFLNG F
DEFSNG F
PRINT f(3)
SYSTEM
DEFLNG F
FUNCTION f (x)
    f = x * 100000
END FUNCTION
