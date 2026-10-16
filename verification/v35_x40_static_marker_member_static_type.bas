$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (40_static_marker_member_static_type)
TYPE t
    v(1) _STATIC AS LONG
    n AS LONG
END TYPE
DIM x AS t
PRINT LEN(x)
SYSTEM
