$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a number assigned to an element of a user type.
TYPE t1
    x AS LONG
END TYPE
DIM a(2) AS t1
a(1) = 5
SYSTEM
