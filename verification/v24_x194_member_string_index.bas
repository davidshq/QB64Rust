$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a member array named with a string index.
TYPE t1
    v(0 TO 3) AS LONG
END TYPE
DIM t AS t1
t.v("a") = 5
SYSTEM
