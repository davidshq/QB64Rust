$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (44_dynamic_static_word_order_static_first)
TYPE t
    _STATIC v(1) AS LONG
END TYPE
DIM x AS t
PRINT UBOUND(x.v)
SYSTEM
