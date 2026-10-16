$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (17_const_bound)
CONST n = 4
TYPE t
    v(n) _DYNAMIC AS LONG
END TYPE
DIM x AS t
PRINT UBOUND(x.v)
SYSTEM
