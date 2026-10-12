$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 9): REDIM of a member array with AS and its upper bound alone, below OPTION BASE 1: a member
' declared 1 TO 3 and one declared 0 TO 3.
OPTION BASE 1
TYPE t1
    v(1 TO 3) AS LONG
    w(0 TO 3) AS LONG
END TYPE
DIM t AS t1
t.v(3) = 4: t.w(3) = 5
REDIM t.v(3) AS LONG
PRINT t.v(3); t.w(3)
SYSTEM
