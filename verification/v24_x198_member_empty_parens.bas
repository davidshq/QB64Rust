$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a member declared with empty parentheses.
TYPE t1
    v() AS LONG
END TYPE
DIM t AS t1
PRINT LEN(t)
SYSTEM
