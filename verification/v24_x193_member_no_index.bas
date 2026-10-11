$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a member array named without an index in a store and in PRINT.
TYPE t1
    v(0 TO 3) AS LONG
END TYPE
DIM t AS t1
t.v = 5
PRINT t.v
SYSTEM
