$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): every element type _ARRAYCOPY takes, one copy each of a range
' to a start (source 1 TO 3, elements 2 and 3 to destination 0 TO 3 at 1), printed with the elements left.
DIM i AS LONG

DIM b1(1 TO 3) AS _BYTE, b2(0 TO 3) AS _BYTE
FOR i = 1 TO 3: b1(i) = -i: NEXT: _ARRAYCOPY b1(2 TO 3) TO b2(1)
PRINT "_BYTE:";: FOR i = 0 TO 3: PRINT b2(i);: NEXT: PRINT

DIM ub1(1 TO 3) AS _UNSIGNED _BYTE, ub2(0 TO 3) AS _UNSIGNED _BYTE
FOR i = 1 TO 3: ub1(i) = 250 + i: NEXT: _ARRAYCOPY ub1(2 TO 3) TO ub2(1)
PRINT "_UNSIGNED _BYTE:";: FOR i = 0 TO 3: PRINT ub2(i);: NEXT: PRINT

DIM n1(1 TO 3) AS INTEGER, n2(0 TO 3) AS INTEGER
FOR i = 1 TO 3: n1(i) = -1000 * i: NEXT: _ARRAYCOPY n1(2 TO 3) TO n2(1)
PRINT "INTEGER:";: FOR i = 0 TO 3: PRINT n2(i);: NEXT: PRINT

DIM un1(1 TO 3) AS _UNSIGNED INTEGER, un2(0 TO 3) AS _UNSIGNED INTEGER
FOR i = 1 TO 3: un1(i) = 65530 + i: NEXT: _ARRAYCOPY un1(2 TO 3) TO un2(1)
PRINT "_UNSIGNED INTEGER:";: FOR i = 0 TO 3: PRINT un2(i);: NEXT: PRINT

DIM l1(1 TO 3) AS LONG, l2(0 TO 3) AS LONG
FOR i = 1 TO 3: l1(i) = -100000 * i: NEXT: _ARRAYCOPY l1(2 TO 3) TO l2(1)
PRINT "LONG:";: FOR i = 0 TO 3: PRINT l2(i);: NEXT: PRINT

DIM ul1(1 TO 3) AS _UNSIGNED LONG, ul2(0 TO 3) AS _UNSIGNED LONG
FOR i = 1 TO 3: ul1(i) = 4294967290 + i: NEXT: _ARRAYCOPY ul1(2 TO 3) TO ul2(1)
PRINT "_UNSIGNED LONG:";: FOR i = 0 TO 3: PRINT ul2(i);: NEXT: PRINT

DIM q1(1 TO 3) AS _INTEGER64, q2(0 TO 3) AS _INTEGER64
FOR i = 1 TO 3: q1(i) = -9000000000000 * i: NEXT: _ARRAYCOPY q1(2 TO 3) TO q2(1)
PRINT "_INTEGER64:";: FOR i = 0 TO 3: PRINT q2(i);: NEXT: PRINT

DIM uq1(1 TO 3) AS _UNSIGNED _INTEGER64, uq2(0 TO 3) AS _UNSIGNED _INTEGER64
FOR i = 1 TO 3: uq1(i) = 18446744073709551610~&& + i: NEXT: _ARRAYCOPY uq1(2 TO 3) TO uq2(1)
PRINT "_UNSIGNED _INTEGER64:";: FOR i = 0 TO 3: PRINT uq2(i);: NEXT: PRINT

DIM f1(1 TO 3) AS SINGLE, f2(0 TO 3) AS SINGLE
FOR i = 1 TO 3: f1(i) = i / 3: NEXT: _ARRAYCOPY f1(2 TO 3) TO f2(1)
PRINT "SINGLE:";: FOR i = 0 TO 3: PRINT f2(i);: NEXT: PRINT

DIM d1(1 TO 3) AS DOUBLE, d2(0 TO 3) AS DOUBLE
FOR i = 1 TO 3: d1(i) = i / 3: NEXT: _ARRAYCOPY d1(2 TO 3) TO d2(1)
PRINT "DOUBLE:";: FOR i = 0 TO 3: PRINT d2(i);: NEXT: PRINT

DIM g1(1 TO 3) AS _FLOAT, g2(0 TO 3) AS _FLOAT
FOR i = 1 TO 3: g1(i) = i / 3: NEXT: _ARRAYCOPY g1(2 TO 3) TO g2(1)
PRINT "_FLOAT:";: FOR i = 0 TO 3: PRINT g2(i);: NEXT: PRINT

DIM o1(1 TO 3) AS _OFFSET, o2(0 TO 3) AS _OFFSET
FOR i = 1 TO 3: o1(i) = i * 7: NEXT: _ARRAYCOPY o1(2 TO 3) TO o2(1)
PRINT "_OFFSET:";: FOR i = 0 TO 3: PRINT o2(i);: NEXT: PRINT

DIM uo1(1 TO 3) AS _UNSIGNED _OFFSET, uo2(0 TO 3) AS _UNSIGNED _OFFSET
FOR i = 1 TO 3: uo1(i) = i * 9: NEXT: _ARRAYCOPY uo1(2 TO 3) TO uo2(1)
PRINT "_UNSIGNED _OFFSET:";: FOR i = 0 TO 3: PRINT uo2(i);: NEXT: PRINT

DIM fs1(1 TO 3) AS STRING * 3, fs2(0 TO 3) AS STRING * 3
FOR i = 1 TO 3: fs1(i) = CHR$(64 + i) + "x": NEXT: _ARRAYCOPY fs1(2 TO 3) TO fs2(1)
PRINT "STRING * 3:";: FOR i = 0 TO 3: PRINT "["; fs2(i); "]";: NEXT: PRINT

DIM vs1(1 TO 3) AS STRING, vs2(0 TO 3) AS STRING
FOR i = 1 TO 3: vs1(i) = STRING$(i, 96 + i): NEXT: vs2(0) = "keep": _ARRAYCOPY vs1(2 TO 3) TO vs2(1)
PRINT "STRING:";: FOR i = 0 TO 3: PRINT "["; vs2(i); "]";: NEXT: PRINT
vs1(2) = "changed"
PRINT "STRING after the source changed:"; "["; vs2(1); "]"

DEFINT A-Z
DIM di1(1 TO 3), di2(0 TO 3)
FOR i = 1 TO 3: di1(i) = i * 11: NEXT: _ARRAYCOPY di1(2 TO 3) TO di2(1)
PRINT "DEFINT:";: FOR i = 0 TO 3: PRINT di2(i);: NEXT: PRINT
SYSTEM
