$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 9): a member array as an array argument without its parentheses.
TYPE t1
    id AS LONG
    v(0 TO 3) AS LONG
END TYPE
DIM t AS t1
fill t.v
PRINT t.v(1)
SYSTEM
SUB fill (a() AS LONG)
    a(1) = 5
END SUB
