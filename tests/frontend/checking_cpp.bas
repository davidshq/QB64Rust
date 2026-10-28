' TEST: cpp
$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' $CHECKING in C++ (m4-handlers-logging-checking tasks 3.2 and 3.3, design D3-D5): below OFF a statement has no
' do{...}while(r); and no event check, a label no event check, and an index of an array, a member array and a
' dynamic member array no array_check (qb64pe.bas 15627-15640, 16136-16149, 16335-16348); the same statements
' beside, with checking on; a SUB defined below OFF; an included file that turns checking off.
TYPE t
    v(1 TO 3) AS LONG
    g(1, 2) AS INTEGER
    w(2) _DYNAMIC AS DOUBLE
END TYPE
DIM a(1 TO 3) AS LONG, m(2, -1 TO 1) AS INTEGER, x AS t, i AS LONG
a(i) = 1
m(i, i) = 2
x.v(i) = 3
x.g(i, 1) = 4
x.w(i) = 5
$CHECKING:OFF
a(i) = 1
m(i, i) = 2
x.v(i) = 3
x.g(i, 1) = 4
x.w(i) = 5
lbl:
PRINT a(2)
DIM mb AS _MEM, l AS LONG, d AS DOUBLE, s5 AS STRING * 5, sv AS STRING
mb = _MEMNEW(16)
_MEMGET mb, mb.OFFSET, l
_MEMGET mb, mb.OFFSET, s5
PRINT _MEMGET(mb, mb.OFFSET, LONG); _MEMGET(mb, mb.OFFSET, STRING * 2)
_MEMPUT mb, mb.OFFSET, l
_MEMPUT mb, mb.OFFSET, s5
_MEMPUT mb, mb.OFFSET, sv + "x"
_MEMPUT mb, mb.OFFSET, 2.5 AS DOUBLE
_MEMFILL mb, mb.OFFSET, 16, l
_MEMFILL mb, mb.OFFSET, 16, sv + "x"
_MEMFILL mb, mb.OFFSET, 16, 7 AS _BYTE
_MEMFILL mb, mb.OFFSET, 16, d AS DOUBLE
' An element as the block: below OFF the block is not written, nor its index.
DIM mbs(1) AS _MEM
mbs(1) = mb
_MEMGET mbs(i + 1), mb.OFFSET, l
_MEMPUT mbs(i + 1), mb.OFFSET, l
_MEMFILL mbs(i + 1), mb.OFFSET, 16, l
PRINT _MEMGET(mbs(i + 1), mb.OFFSET, LONG)
' A whole array in a file: below OFF the call alone, with checking error 9 while it has no storage (D-042).
PUT #1, , a()
$CHECKING:ON
_MEMGET mb, mb.OFFSET, l
_MEMFILL mb, mb.OFFSET, 16, 7 AS _BYTE
_MEMGET mbs(i + 1), mb.OFFSET, l
PUT #1, , a()
s
'$INCLUDE:'inc/checking_off.bi'
PRINT "after the include"
$CHECKING:ON
SYSTEM

$CHECKING:OFF
SUB s
    PRINT "s"
END SUB
$CHECKING:ON
