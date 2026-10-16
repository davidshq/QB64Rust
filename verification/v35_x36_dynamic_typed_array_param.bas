$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (36_dynamic_typed_array_param)
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
DIM x AS t
REDIM x.v(0 TO 3) AS LONG
x.v(3) = 3
PRINT s&(x.v())
FUNCTION s& (a() AS LONG)
    s& = UBOUND(a) + a(3)
END FUNCTION
SYSTEM
