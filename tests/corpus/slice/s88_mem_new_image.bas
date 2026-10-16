$CONSOLE:ONLY
' Slice (m3-mem task 2.2): the type _MEM and the blocks of built-ins, numbered lines. The size of a block and of a
' TYPE with one; new memory, its members, a copy of a block, freeing (also through the copy, an element and a
' member); a block of 0 bytes and a size that is a float; the pixels of a 32-bit and of a 256-colour image, of the
' destination, freed with _MEMFREE and with _FREEIMAGE; _MEMCOPY between two blocks. _MEMSOUND needs a sound
' handle, which no supported statement gives: it is held below SYSTEM, compiled and not reached.
TYPE w
    a AS INTEGER
    mm AS _MEM
END TYPE
DIM m AS _MEM, n AS _MEM, u AS w, z(3) AS _MEM, img AS LONG, img2 AS LONG
PRINT "1."; LEN(m); LEN(u); LEN(z(1))
m = _MEMNEW(16)
PRINT "2."; _MEMEXISTS(m); m.SIZE; m.TYPE; m.ELEMENTSIZE; m.IMAGE; m.OFFSET <> 0
n = m
PRINT "3."; n.OFFSET = m.OFFSET; n.SIZE; _MEMEXISTS(n)
_MEMFREE m
PRINT "4."; _MEMEXISTS(m); _MEMEXISTS(n)
m = _MEMNEW(0)
PRINT "5."; m.OFFSET; m.SIZE; _MEMEXISTS(m)
u.mm = _MEMNEW(2.5)
PRINT "6."; u.mm.SIZE; _MEMEXISTS(u.mm)
_MEMFREE u.mm
PRINT "7."; _MEMEXISTS(u.mm)
z(2) = _MEMNEW(8)
PRINT "8."; _MEMEXISTS(z(2)); z(2).SIZE
CALL _MEMFREE(z(2))
PRINT "9."; _MEMEXISTS(z(2)); _MEMEXISTS(z(3))
img = _NEWIMAGE(4, 2, 32)
m = _MEMIMAGE(img)
PRINT "10."; m.SIZE; m.TYPE; m.ELEMENTSIZE; m.IMAGE = img; _MEMEXISTS(m)
_DEST img
n = _MEMIMAGE
_DEST _CONSOLE
PRINT "11."; n.SIZE; n.IMAGE = img
_MEMFREE n
PRINT "12."; _MEMEXISTS(n); _MEMEXISTS(m)
img2 = _NEWIMAGE(3, 3, 256)
m = _MEMIMAGE(img2)
PRINT "13."; m.SIZE; m.TYPE; m.ELEMENTSIZE
_FREEIMAGE img2
PRINT "14."; _MEMEXISTS(m)
m = _MEMNEW(8)
n = _MEMNEW(8)
_MEMCOPY m, m.OFFSET, 8 TO n, n.OFFSET
_MEMCOPY m, m.OFFSET, 4 TO m, m.OFFSET + 4
PRINT "15."; _MEMEXISTS(m); _MEMEXISTS(n)
SWAP m, n
PRINT "16."; m.SIZE; n.SIZE
SYSTEM
z(1) = _MEMSOUND(1)
z(1) = _MEMSOUND(1, 2)
