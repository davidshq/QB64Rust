$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (08_marker_on_scalar)
TYPE t
    n _DYNAMIC AS LONG
END TYPE
DIM x AS t
PRINT x.n
SYSTEM
