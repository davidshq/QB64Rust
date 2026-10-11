$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a user-type variable passed to a LONG parameter.
TYPE t1
    x AS LONG
END TYPE
DIM q AS t1
one q
SYSTEM
SUB one (n AS LONG)
    PRINT n
END SUB
