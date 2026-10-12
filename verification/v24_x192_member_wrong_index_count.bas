$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a member array of one dimension named with two indexes.
TYPE t1
    v(0 TO 3) AS LONG
END TYPE
DIM t AS t1
t.v(1, 2) = 5
SYSTEM
