$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 9): REDIM of a member array declared 0 TO 3 with AS and its upper bound alone, below
' OPTION BASE 1.
TYPE t1
    id AS LONG
    v(0 TO 3) AS LONG
END TYPE
DIM t AS t1
OPTION BASE 1
t.v(3) = 4
REDIM t.v(3) AS LONG
PRINT t.v(3)
SYSTEM
