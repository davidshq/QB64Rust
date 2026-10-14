' TEST: typed
$CONSOLE:ONLY
' The block of a reference and of an address, and the address of a reference (m3-mem task 3.1 and 3.2, design D3
' and D6; verification\v32_x01_accepted): _MEM and _MEMELEMENT of a variable, an element, a member, a whole array
' and a whole member array (also named without parentheses), typed _MEM; _MEM(offset, size) with the size stored
' as an _OFFSET; _OFFSET of each, an _UNSIGNED _OFFSET, also of a variable-length string.
TYPE t
    a AS INTEGER
    v(0 TO 3) AS LONG
END TYPE
DIM m AS _MEM, k AS LONG, d AS DOUBLE, f AS STRING * 4, s AS STRING, u AS t, ua(3) AS t, b(10) AS INTEGER
DIM o AS _OFFSET, uo AS _UNSIGNED _OFFSET
m = _MEM(k)
m = _MEM(f)
m = _MEM(u)
m = _MEM(u.a)
m = _MEM(u.v(2))
m = _MEM(u.v())
m = _MEM(u.v)
m = _MEM(b())
m = _MEM(b(k))
m = _MEM(ua(1).v(2))
m = _MEMELEMENT(b(4))
m = _MEMELEMENT(u.a)
m = _MEM(o, 10)
m = _MEM(uo, d)
m = _MEM(_OFFSET(k), 4)
uo = _OFFSET(k)
uo = _OFFSET(s)
uo = _OFFSET(b())
uo = _OFFSET(ua(1).v(2))
PRINT _OFFSET(u.v) + 1; _OFFSET(b(2)) - _OFFSET(b())
SYSTEM
