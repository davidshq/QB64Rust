$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a member array declared with a CONST bound and an expression.
CONST top = 3
TYPE t1
    v(top) AS LONG
    w(top * 2 - 1) AS LONG
END TYPE
DIM t AS t1
PRINT UBOUND(t.v); UBOUND(t.w); LEN(t)
SYSTEM
