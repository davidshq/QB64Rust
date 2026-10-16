' TEST: check-fail
$CONSOLE:ONLY
' The old compiler's errors for the forms of m3-mem it rejects (verification\v32_x02 to x120), one per line: a
' store to a member of a block, a number for a block, a block as a value, a member a program cannot name.
TYPE t
    a AS INTEGER
    v(0 TO 3) AS LONG
END TYPE
DIM m AS _MEM, k AS LONG, z(2) AS _MEM, s AS STRING, o AS _OFFSET, u AS t, bt AS _BIT, ba(3) AS _BIT
DIM sa(3) AS STRING
m.SIZE = 4
m.OFFSET = 4
z(1).TYPE = 3
' READ and INPUT # store as an assignment does (x117, x118); console INPUT does not.
READ m.SIZE
INPUT #1, m.SIZE
m = 5
m = k
PRINT m
PRINT m + 1
k = m.LOCK_ID
k = m.FOO
DIM u AS _UNSIGNED _MEM
' A built-in that gives a block: stored only into a _MEM place (x02, x03, x80, x93-x96, x114).
PRINT _MEMEXISTS(_MEMNEW(4))
PRINT _MEMNEW(4).SIZE
k = _MEMNEW(4)
x _MEMNEW(4)
CALL x(_MEMNEW(4))
' A block as an argument: a place of type _MEM (x67-x73).
_MEMFREE k
_MEMFREE _MEMNEW(4)
PRINT _MEMEXISTS(k)
_MEMCOPY m, 0, 8 TO k, 0
_MEMCOPY k, 0, 8 TO m, 0
' Numbers, counts and forms (x74-x79, x87, x88, x65, x66, x69, x70).
m = _MEMNEW("a")
m = _MEMIMAGE("a")
m = _MEMNEW
m = _MEMNEW(1, 2)
m = _MEMIMAGE(1, 2)
m = _MEMIMAGE()
PRINT _MEMEXISTS
m = _MEMSOUND
m = _MEMSOUND(1, 2, 3)
_MEMFREE
_MEMFREE m, m
_MEMCOPY m, 0, 8, m, 0
_MEMCOPY m, 0, 8 TO m
' The block of a reference and of an address, and an address (x04, x07-x24, x115, x116).
m = _MEMELEMENT(u.v())
m = _MEM(s)
m = _MEMELEMENT(s)
m = _MEM(5, 4)
m = _MEM(k, 4)
m = _MEM(5)
m = _MEM(k + 1)
m = _MEMELEMENT(5)
m = _MEM(bt)
m = _MEM(ba())
m = _MEM(sa())
m = _MEM(sa(1))
m = _MEM()
m = _MEM(o, 4, 1)
m = _MEM(o, "a")
m = _MEMELEMENT(k, 1)
k = _MEM(k)
m = _MEM
PRINT _OFFSET(5)
PRINT _OFFSET(k + 1)
PRINT _OFFSET(ba())
PRINT _OFFSET(bt)
PRINT _OFFSET()
' Reading and writing through a block (x25-x64, x90).
_MEMPUT m, m.OFFSET, k + 1
_MEMPUT m, m.OFFSET, "ab" AS LONG
_MEMPUT m, m.OFFSET, 1 AS STRING
_MEMPUT m, m.OFFSET, 1 AS STRING * 4
_MEMPUT m, m.OFFSET, 1 AS t
_MEMPUT m, m.OFFSET, 1 AS _BIT
_MEMPUT m, m.OFFSET, 1 AS FOO
_MEMPUT k, 0, 1 AS LONG
_MEMPUT m, m.OFFSET
_MEMPUT m, "a", 1 AS LONG
_MEMPUT m, m.OFFSET, bt
_MEMPUT m, m.OFFSET, sa()
_MEMGET m, m.OFFSET
_MEMGET m, m.OFFSET, 5
_MEMGET m, m.OFFSET, k + 1
_MEMGET m, m.OFFSET, k, 1
_MEMGET k, 0, k
_MEMGET m, m.OFFSET, bt
CALL _MEMGET(m, m.OFFSET, k)
PRINT _MEMGET(m, m.OFFSET, STRING)
PRINT _MEMGET(m, m.OFFSET, _BIT)
PRINT _MEMGET(m, m.OFFSET, _BIT * 4)
PRINT _MEMGET(m, m.OFFSET, _UNSIGNED STRING * 4)
PRINT _MEMGET(m, m.OFFSET, FOO)
PRINT _MEMGET(k, 0, LONG)
PRINT _MEMGET(m, m.OFFSET)
PRINT _MEMGET(m, m.OFFSET, 5)
PRINT _MEMGET(m, "a", LONG)
PRINT _MEMGET(m, m.OFFSET, t)
k = _MEMGET(m, m.OFFSET, STRING * 4)
u = _MEMGET(m, 0, LONG)
_MEMFILL m, m.OFFSET, 8, 1 AS STRING
_MEMFILL m, m.OFFSET, 8, 1 AS t
_MEMFILL m, m.OFFSET, 8, 1 AS _BIT
_MEMFILL m, m.OFFSET, 8
_MEMFILL m, m.OFFSET, 8, k + 1
_MEMFILL m, m.OFFSET, 8, "a" AS LONG
_MEMFILL k, 0, 8, 1 AS LONG
SYSTEM

SUB x (p AS _MEM)
END SUB
