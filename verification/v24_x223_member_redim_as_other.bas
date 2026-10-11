$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): REDIM of a LONG member array with AS INTEGER.
TYPE t1
    v(0 TO 3) AS LONG
END TYPE
DIM t AS t1
REDIM t.v(0 TO 3) AS INTEGER
PRINT UBOUND(t.v)
SYSTEM
