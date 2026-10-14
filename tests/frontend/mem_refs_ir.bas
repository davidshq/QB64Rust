' TEST: ir
$CONSOLE:ONLY
' The nodes of m3-mem task 3.1 and 3.2 in the IR: a block of a reference (Mem, MemElement) and of an address
' (MemAt), stored into a _MEM variable, element or member; an address (AddressOf) as a value.
TYPE w
    a AS INTEGER
    mm AS _MEM
END TYPE
DIM m AS _MEM, u AS w, z(3) AS _MEM, k AS LONG, b(10) AS INTEGER, o AS _OFFSET
m = _MEM(k)
z(k) = _MEM(b(k))
u.mm = _MEMELEMENT(b())
m = _MEM(o, k)
PRINT _OFFSET(b(2))
SYSTEM
