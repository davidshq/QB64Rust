$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a member array declared with a bound that is not constant.
n = 3
TYPE t1
    v(n) AS LONG
END TYPE
DIM t AS t1
PRINT UBOUND(t.v)
SYSTEM
