' TEST: typed
$CONSOLE:ONLY
' The built-ins that give a block, stored into a _MEM variable, element or member (m3-mem task 2.2, design D2;
' verification\v32_x01_accepted): _MEMNEW with a size of each kind (an _OFFSET slot: a float rounded, the others
' converted as a store), _MEMIMAGE bare and with a handle, _MEMSOUND with one and two arguments; and _MEMEXISTS,
' a LONG, whose argument is a block (a place of type _MEM, not a value).
TYPE w
    a AS INTEGER
    mm AS _MEM
END TYPE
DIM m AS _MEM, u AS w, z(3) AS _MEM, k AS LONG, d AS DOUBLE, q AS _INTEGER64, o AS _OFFSET
m = _MEMNEW(64)
m = _MEMNEW(k + 1)
m = _MEMNEW(d)
m = _MEMNEW(1.5)
m = _MEMNEW(q)
u.mm = _MEMNEW(o)
z(2) = _MEMIMAGE
m = _MEMIMAGE(k)
m = _MEMIMAGE(d)
PRINT _MEMEXISTS(m); _MEMEXISTS(u.mm); _MEMEXISTS(z(2))
k = _MEMEXISTS(m) + 1
SYSTEM
m = _MEMSOUND(k)
m = _MEMSOUND(k, 2)
