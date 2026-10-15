$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (06_both_markers)
TYPE t
    v(1) _DYNAMIC _STATIC AS LONG
END TYPE
DIM x AS t
PRINT UBOUND(x.v)
SYSTEM
