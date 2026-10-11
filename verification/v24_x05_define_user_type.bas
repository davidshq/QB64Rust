$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): _DEFINE with a user type.
TYPE rec
    n AS LONG
END TYPE
_DEFINE a AS rec
a1.n = 5: PRINT a1.n
SYSTEM
