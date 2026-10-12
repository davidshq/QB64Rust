$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 9): a member array named with empty parentheses as a value.
TYPE t1
    id AS LONG
    v(0 TO 3) AS LONG
END TYPE
DIM t AS t1
PRINT t.v()
SYSTEM
