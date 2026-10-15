$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, task 5.2): the block of _MEM and _MEMELEMENT of a whole _DYNAMIC member and of an
' element of one (v35_x23, x24 measured the size): its offset against _OFFSET of the elements, size, type and
' element size, of a member of a variable and of an array element; a block after REDIM of the member; a member of
' a STRING * n and of a plain TYPE.
TYPE pt
    x AS INTEGER
    y AS LONG
END TYPE
TYPE t
    a AS INTEGER
    v(1) _DYNAMIC AS LONG
    s(2) _DYNAMIC AS STRING * 3
    q(1) _DYNAMIC AS pt
END TYPE
ON ERROR GOTO h
DIM x AS t
DIM m AS _MEM
REDIM p(2) AS t
m = _MEM(x.v())
PRINT "1."; m.SIZE; m.TYPE; m.ELEMENTSIZE; m.OFFSET = _OFFSET(x.v(0))
m = _MEM(x.v(1))
PRINT "2."; m.SIZE; m.TYPE; m.ELEMENTSIZE; m.OFFSET = _OFFSET(x.v(1))
m = _MEMELEMENT(x.v(1))
PRINT "3."; m.SIZE; m.TYPE; m.ELEMENTSIZE; m.OFFSET = _OFFSET(x.v(1))
m = _MEM(p(2).v())
PRINT "4."; m.SIZE; m.TYPE; m.ELEMENTSIZE; m.OFFSET = _OFFSET(p(2).v(0))
m = _MEM(p(2).v(0))
PRINT "5."; m.SIZE; m.TYPE; m.ELEMENTSIZE; m.OFFSET = _OFFSET(p(2).v(0))
m = _MEM(x.s())
PRINT "6."; m.SIZE; m.TYPE; m.ELEMENTSIZE
m = _MEM(x.s(1))
PRINT "7."; m.SIZE; m.TYPE; m.ELEMENTSIZE; m.OFFSET = _OFFSET(x.s(1))
m = _MEM(x.q())
PRINT "8."; m.SIZE; m.TYPE; m.ELEMENTSIZE
m = _MEM(x.q(1))
PRINT "9."; m.SIZE; m.TYPE; m.ELEMENTSIZE; m.OFFSET = _OFFSET(x.q(1))
x.v(1) = 1234
m = _MEM(x.v())
PRINT "10. _MEMGET:"; _MEMGET(m, m.OFFSET + 4, LONG)
REDIM x.v(0 TO 9) AS LONG
PRINT "11. after REDIM:"; _MEMEXISTS(m)
m = _MEM(x.v())
PRINT "12."; m.SIZE; _MEMEXISTS(m)
ERASE x.v
PRINT "13. after ERASE:"; _MEMEXISTS(m)
' Not here: _MEM of the erased member, m = _MEM(x.v()), ends the old program with a crash (exit code 139, recorded
' 2026-10-15 before this line was taken out).
SYSTEM
h:
PRINT "[error"; ERR; "]";
RESUME NEXT
