$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): ERASE of a variable of a user type.
TYPE t1
    v(0 TO 3) AS LONG
END TYPE
DIM t AS t1
ERASE t
SYSTEM
