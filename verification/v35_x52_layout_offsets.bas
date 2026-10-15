$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (52_layout_offsets)
TYPE t
    a AS INTEGER
    v(1) _DYNAMIC AS LONG
    b AS INTEGER
END TYPE
TYPE o
    c AS _BYTE
    i AS t
    d AS _BYTE
END TYPE
DIM x AS t, y AS o
DIM m AS _MEM
m = _MEM(x)
PRINT m.SIZE
PRINT _OFFSET(x.a) - m.OFFSET; _OFFSET(x.b) - m.OFFSET
m = _MEM(y)
PRINT m.SIZE; _OFFSET(y.i.a) - m.OFFSET; _OFFSET(y.i.b) - m.OFFSET; _OFFSET(y.d) - m.OFFSET
REDIM r(1) AS t
PRINT _OFFSET(r(1).a) - _OFFSET(r(0).a)
SYSTEM
