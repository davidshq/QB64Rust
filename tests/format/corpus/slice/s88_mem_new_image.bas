$Console:Only
' Slice (m3-mem task 2.2): the type _MEM and the blocks of built-ins, numbered lines. The size of a block and of a
' TYPE with one; new memory, its members, a copy of a block, freeing (also through the copy, an element and a
' member); a block of 0 bytes and a size that is a float; the pixels of a 32-bit and of a 256-colour image, of the
' destination, freed with _MEMFREE and with _FREEIMAGE; _MEMCOPY between two blocks. _MEMSOUND needs a sound
' handle, which no supported statement gives: it is held below SYSTEM, compiled and not reached.
Type w
    a As Integer
    mm As _MEM
End Type
Dim m As _MEM, n As _MEM, u As w, z(3) As _MEM, img As Long, img2 As Long
Print "1."; Len(m); Len(u); Len(z(1))
m = _MemNew(16)
Print "2."; _MemExists(m); m.SIZE; m.TYPE; m.ELEMENTSIZE; m.IMAGE; m.OFFSET <> 0
n = m
Print "3."; n.OFFSET = m.OFFSET; n.SIZE; _MemExists(n)
_MemFree m
Print "4."; _MemExists(m); _MemExists(n)
m = _MemNew(0)
Print "5."; m.OFFSET; m.SIZE; _MemExists(m)
u.mm = _MemNew(2.5)
Print "6."; u.mm.SIZE; _MemExists(u.mm)
_MemFree u.mm
Print "7."; _MemExists(u.mm)
z(2) = _MemNew(8)
Print "8."; _MemExists(z(2)); z(2).SIZE
Call _MemFree(z(2))
Print "9."; _MemExists(z(2)); _MemExists(z(3))
img = _NewImage(4, 2, 32)
m = _MemImage(img)
Print "10."; m.SIZE; m.TYPE; m.ELEMENTSIZE; m.IMAGE = img; _MemExists(m)
_Dest img
n = _MemImage
_Dest _Console
Print "11."; n.SIZE; n.IMAGE = img
_MemFree n
Print "12."; _MemExists(n); _MemExists(m)
img2 = _NewImage(3, 3, 256)
m = _MemImage(img2)
Print "13."; m.SIZE; m.TYPE; m.ELEMENTSIZE
_FreeImage img2
Print "14."; _MemExists(m)
m = _MemNew(8)
n = _MemNew(8)
_MemCopy m, m.OFFSET, 8 To n, n.OFFSET
_MemCopy m, m.OFFSET, 4 To m, m.OFFSET + 4
Print "15."; _MemExists(m); _MemExists(n)
Swap m, n
Print "16."; m.SIZE; n.SIZE
System
z(1) = _MemSound(1)
z(1) = _MemSound(1, 2)
