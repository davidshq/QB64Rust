$CONSOLE:ONLY
' Verification (wasm-pointer-width, design D3 and D8): _MEM where a pointer is W bytes. LEN of a _MEM is libqb's
' block (five pointers, the 8-byte lock id, two LONGs, rounded up to 8: 56 or 40). A block of a TYPE with an
' _OFFSET member, _MEMPUT and _MEMGET at its members' offsets, an array of _MEM stored and read back, and the
' TYPE value and ELEMENTSIZE of a block of an _OFFSET.
$IF 32BIT THEN
    CONST W = 4
$ELSE
    CONST W = 8
$END IF
TYPE rec
    a AS LONG
    p AS _OFFSET
    b AS INTEGER
END TYPE
DIM m AS _MEM, blocks(3) AS _MEM, v AS rec, a AS LONG, q AS _OFFSET
PRINT LEN(m) = 5 * W + 16 + (8 - W); LEN(blocks()) = 4 * LEN(m)
m = _MEM(v)
PRINT m.SIZE = 6 + W; m.ELEMENTSIZE = 6 + W
_MEMPUT m, m.OFFSET, 77 AS LONG
_MEMPUT m, m.OFFSET + 4 + W, 9 AS INTEGER
PRINT v.a; v.b
v.p = 1234
_MEMGET m, m.OFFSET + 4, q
PRINT q
blocks(1) = m
blocks(3) = _MEM(a)
a = 5
PRINT blocks(1).SIZE = 6 + W; blocks(3).SIZE; _MEMGET(blocks(3), blocks(3).OFFSET, LONG)
PRINT blocks(2).SIZE; blocks(1).OFFSET = m.OFFSET; blocks(3).OFFSET = blocks(1).OFFSET
m = _MEM(q)
PRINT m.SIZE = W; m.ELEMENTSIZE = W; (m.TYPE AND 8192) <> 0; (m.TYPE AND 15) = W
m = _MEMNEW(32)
_MEMFILL m, m.OFFSET, 32, 3 AS _BYTE
_MEMPUT m, m.OFFSET + 8, q
_MEMGET m, m.OFFSET + 8, q
PRINT m.SIZE; _MEMGET(m, m.OFFSET + 31, _BYTE); q
_MEMFREE m
SYSTEM
