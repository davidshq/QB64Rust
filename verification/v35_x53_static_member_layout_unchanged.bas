$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (53_static_member_layout_unchanged)
TYPE t
    a AS INTEGER
    v(1) _STATIC AS LONG
    b AS INTEGER
END TYPE
DIM x AS t
PRINT LEN(x); _OFFSET(x.b) - _OFFSET(x.a)
SYSTEM
