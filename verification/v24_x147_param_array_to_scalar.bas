$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a whole array passed to a parameter that is no array.
DIM x(3) AS LONG
one x()
SYSTEM
SUB one (v AS LONG)
    PRINT v
END SUB
