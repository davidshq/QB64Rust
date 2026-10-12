$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 9): REDIM of a member array with the type before the names.
TYPE t1
    id AS LONG
    v(0 TO 3) AS LONG
END TYPE
DIM t AS t1
t.v(3) = 4
REDIM AS LONG t.v(0 TO 3)
PRINT t.v(3)
SYSTEM
