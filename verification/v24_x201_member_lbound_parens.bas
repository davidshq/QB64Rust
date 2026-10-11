$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): LBOUND of a member array written with empty parentheses.
TYPE t1
    v(0 TO 3) AS LONG
END TYPE
DIM t AS t1
PRINT LBOUND(t.v())
SYSTEM
