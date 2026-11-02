$CONSOLE:ONLY
' Verification (SOMEDAY, member arrays): REDIM t.v(0) AS LONG under OPTION BASE 1, of a member declared
' v(0 TO 3). In a DIM and a TYPE, the number 0 under OPTION BASE 1 has no rule of its own (1 TO 0).
TYPE t1
    v(0 TO 3) AS LONG
END TYPE
DIM t AS t1
OPTION BASE 1
REDIM t.v(0) AS LONG
PRINT LBOUND(t.v); UBOUND(t.v)
