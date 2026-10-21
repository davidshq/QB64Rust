$CONSOLE:ONLY
' Verification (m4-handlers-logging-checking, task 1.2): _MEMGET (function and statement), _MEMPUT and _MEMFILL of
' each kind of value below $CHECKING:OFF, within a valid block: the results are those of checking on.
TYPE pt
    x AS INTEGER
    y AS LONG
    s AS STRING * 3
END TYPE
DIM m AS _MEM, o AS _OFFSET
DIM b AS _BYTE, ub AS _UNSIGNED _BYTE, i AS INTEGER, l AS LONG, q AS _INTEGER64
DIM sg AS SINGLE, d AS DOUBLE, ofs AS _OFFSET, fs AS STRING * 4, vs AS STRING, p AS pt, p2 AS pt
DIM arr(1 TO 4) AS LONG
m = _MEMNEW(64)
$CHECKING:OFF
_MEMFILL m, m.OFFSET, 64, 0 AS _BYTE
_MEMPUT m, m.OFFSET, -5 AS _BYTE
_MEMPUT m, m.OFFSET + 1, 200 AS _UNSIGNED _BYTE
_MEMPUT m, m.OFFSET + 2, -1234 AS INTEGER
_MEMPUT m, m.OFFSET + 4, 123456789 AS LONG
_MEMPUT m, m.OFFSET + 8, 1234567890123 AS _INTEGER64
_MEMPUT m, m.OFFSET + 16, 1.5 AS SINGLE
_MEMPUT m, m.OFFSET + 20, 2.25 AS DOUBLE
PRINT "1."; _MEMGET(m, m.OFFSET, _BYTE); _MEMGET(m, m.OFFSET + 1, _UNSIGNED _BYTE); _MEMGET(m, m.OFFSET + 2, INTEGER);
PRINT _MEMGET(m, m.OFFSET + 4, LONG); _MEMGET(m, m.OFFSET + 8, _INTEGER64); _MEMGET(m, m.OFFSET + 16, SINGLE);
PRINT _MEMGET(m, m.OFFSET + 20, DOUBLE)
_MEMGET m, m.OFFSET, b
_MEMGET m, m.OFFSET + 1, ub
_MEMGET m, m.OFFSET + 2, i
_MEMGET m, m.OFFSET + 4, l
_MEMGET m, m.OFFSET + 8, q
_MEMGET m, m.OFFSET + 16, sg
_MEMGET m, m.OFFSET + 20, d
PRINT "2."; b; ub; i; l; q; sg; d
ofs = 4096
_MEMPUT m, m.OFFSET + 48, ofs
ofs = 0
_MEMGET m, m.OFFSET + 48, ofs
PRINT "3."; ofs; _MEMGET(m, m.OFFSET + 48, _OFFSET)
fs = "abcd"
_MEMPUT m, m.OFFSET, fs
vs = "WXYZ"
_MEMPUT m, m.OFFSET + 4, vs
vs = SPACE$(8)
_MEMGET m, m.OFFSET, vs
fs = ""
_MEMGET m, m.OFFSET + 4, fs
PRINT "4. "; vs; " "; fs; " "; _MEMGET(m, m.OFFSET + 1, STRING * 2)
_MEMPUT m, m.OFFSET, "hello"
PRINT "5. "; _MEMGET(m, m.OFFSET, STRING * 5)
p.x = 7: p.y = -8: p.s = "pq"
_MEMPUT m, m.OFFSET + 16, p
_MEMGET m, m.OFFSET + 16, p2
PRINT "6."; p2.x; p2.y; "["; p2.s; "]"; _MEMGET(m, m.OFFSET + 18, LONG)
_MEMFILL m, m.OFFSET, 8, 65 AS _UNSIGNED _BYTE
PRINT "7. "; _MEMGET(m, m.OFFSET, STRING * 8)
_MEMFILL m, m.OFFSET, 8, -2 AS INTEGER
PRINT "8."; _MEMGET(m, m.OFFSET, INTEGER); _MEMGET(m, m.OFFSET + 6, INTEGER)
_MEMFILL m, m.OFFSET, 16, 2.5 AS DOUBLE
PRINT "9."; _MEMGET(m, m.OFFSET + 8, DOUBLE)
fs = "ab"
_MEMFILL m, m.OFFSET, 9, fs
PRINT "10. "; _MEMGET(m, m.OFFSET, STRING * 9)
l = 99
_MEMFILL m, m.OFFSET, 8, l
PRINT "11."; _MEMGET(m, m.OFFSET + 4, LONG)
_MEMFILL m, m.OFFSET, 6, p
PRINT "12."; _MEMGET(m, m.OFFSET, INTEGER)
DIM ma AS _MEM
ma = _MEM(arr())
_MEMPUT ma, ma.OFFSET + 4, 42 AS LONG
_MEMGET ma, ma.OFFSET + 8, arr(4)
PRINT "13."; arr(2); arr(4); _MEMGET(ma, ma.OFFSET + 4, LONG)
o = _OFFSET(l)
_MEMGET m, o, i
PRINT "14."; i
$CHECKING:ON
_MEMFREE ma
_MEMFREE m
SYSTEM
