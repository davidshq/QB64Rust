$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 9): a name written like an element of a member array, of a member the type does not have.
TYPE t1
    id AS LONG
    v(0 TO 3) AS LONG
END TYPE
DIM t AS t1
t.m(1) = 2
SYSTEM
