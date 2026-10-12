$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 9): ERASE of a member array written with empty parentheses.
TYPE t1
    id AS LONG
    v(0 TO 3) AS LONG
END TYPE
DIM t AS t1
t.v(1) = 5
ERASE t.v()
PRINT t.v(1)
SYSTEM
