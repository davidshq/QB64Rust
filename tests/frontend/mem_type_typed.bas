' TEST: typed
$CONSOLE:ONLY
' The predefined type _MEM (m3-mem task 2.1, design D1; verification\v32_x01_accepted): a variable, a static and a
' dynamic array of it, a member of a TYPE, a parameter, a whole copy, SWAP, LEN and the six members a program can
' name, each read with its type (four _OFFSET, two LONG).
TYPE w
    a AS INTEGER
    mm AS _MEM
END TYPE
DIM m AS _MEM, n AS _MEM, u AS w, z(3) AS _MEM
REDIM r(2) AS _MEM
n = m
SWAP m, n
u.mm = z(1)
PRINT LEN(m); LEN(u); LEN(z(2))
PRINT m.OFFSET; m.SIZE; m.TYPE; m.ELEMENTSIZE; m.IMAGE; m.SOUND
PRINT u.mm.SIZE; z(1).OFFSET + 1; r(2).ELEMENTSIZE
x m
SYSTEM

SUB x (p AS _MEM)
    STATIC s AS _MEM
    s = p
    PRINT p.SIZE; s.IMAGE
END SUB
