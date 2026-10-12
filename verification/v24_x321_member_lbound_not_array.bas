$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 9): LBOUND of a member that is no array.
TYPE t1
    id AS LONG
    v(0 TO 3) AS LONG
END TYPE
DIM t AS t1
PRINT LBOUND(t.id)
SYSTEM
