$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (24_mem_of_element)
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
DIM x AS t
DIM m AS _MEM
m = _MEM(x.v(1))
PRINT m.SIZE
SYSTEM
