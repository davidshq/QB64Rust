' TEST: ir
$CONSOLE:ONLY
' _MEMFREE and _MEMCOPY (m3-mem task 2.2, design D4; verification\v32_x01_accepted): each a plain call whose
' blocks are places of type _MEM (a variable, an element, a member), the offsets and the size _OFFSET values;
' both also after CALL.
TYPE w
    a AS INTEGER
    mm AS _MEM
END TYPE
DIM m AS _MEM, n AS _MEM, u AS w, z(3) AS _MEM, k AS LONG, d AS DOUBLE
_MEMFREE m
_MEMFREE z(1)
_MEMFREE u.mm
CALL _MEMFREE(n)
_MEMCOPY m, m.OFFSET, 8 TO n, n.OFFSET
_MEMCOPY u.mm, k, d TO z(k), 1.5
CALL _MEMCOPY(m, 0, 4 TO m, 4)
SYSTEM
