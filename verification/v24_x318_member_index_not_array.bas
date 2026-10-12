$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 9): an index after a member that is no array.
TYPE t1
    id AS LONG
    v(0 TO 3) AS LONG
END TYPE
DIM t AS t1
t.id(1) = 2
PRINT t.id(1)
SYSTEM
