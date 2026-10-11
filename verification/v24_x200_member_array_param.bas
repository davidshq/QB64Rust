$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a member array passed whole to an array parameter.
TYPE t1
    v(0 TO 3) AS LONG
END TYPE
DIM t AS t1
fill t.v()
PRINT t.v(1); t.v(3)
SYSTEM
SUB fill (a() AS LONG)
    DIM i AS LONG
    FOR i = LBOUND(a) TO UBOUND(a): a(i) = i * 2: NEXT
END SUB
