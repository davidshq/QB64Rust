$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (37_dim_member)
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
DIM x AS t
DIM x.v(3) AS LONG
PRINT UBOUND(x.v)
SYSTEM
