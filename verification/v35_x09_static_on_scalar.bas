$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (09_static_on_scalar)
TYPE t
    n AS LONG _STATIC
END TYPE
DIM x AS t
PRINT x.n
SYSTEM
