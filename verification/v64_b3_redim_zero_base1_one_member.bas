$CONSOLE:ONLY
' Verification (SOMEDAY, member arrays): REDIM t.v(0) AS LONG under OPTION BASE 1, of a member declared
' v(1 TO 1), one element as v(0) under OPTION BASE 0 has.
TYPE t1
    v(1 TO 1) AS LONG
END TYPE
DIM t AS t1
OPTION BASE 1
REDIM t.v(0) AS LONG
PRINT LBOUND(t.v); UBOUND(t.v)
