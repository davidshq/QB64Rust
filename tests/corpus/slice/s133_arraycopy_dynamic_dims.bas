$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Slice (m3-arraycopy): _ARRAYCOPY of _DYNAMIC member arrays that a REDIM gave another number of dimensions than
' they were declared with (verification\v41_l): error 9 and nothing copied, as source and as destination, whole,
' to a start and with ranges; a member REDIMmed back to its own number copies again. A plain array beside each is
' printed: -1 where nothing was copied.
TYPE dy
    _DYNAMIC n(1 TO 4) AS LONG
    _DYNAMIC g(1 TO 2, 1 TO 3) AS LONG
END TYPE
DIM i AS LONG, j AS LONG
DIM v AS dy, w(1 TO 2) AS dy
DIM src(1 TO 4) AS LONG, d(1 TO 10) AS LONG, m(1 TO 2, 1 TO 3) AS LONG
ON ERROR GOTO h
FOR i = 1 TO 4: src(i) = i * 100: NEXT
REDIM v.n(1 TO 2, 1 TO 3) AS LONG
REDIM w(2).g(1 TO 6) AS LONG
PRINT "1:";
FOR i = 1 TO 10: d(i) = -1: NEXT
_ARRAYCOPY v.n() TO d()
FOR i = 1 TO 10: PRINT d(i);: NEXT: PRINT
PRINT "2:";
_ARRAYCOPY v.n(1 TO 2) TO d(5)
FOR i = 1 TO 10: PRINT d(i);: NEXT: PRINT
PRINT "3:";
_ARRAYCOPY src() TO v.n()
_ARRAYCOPY src() TO v.n(1)
_ARRAYCOPY src(2 TO 3) TO v.n()
PRINT
PRINT "4:";
FOR i = 1 TO 2: FOR j = 1 TO 3: m(i, j) = -1: NEXT: NEXT
_ARRAYCOPY w(2).g() TO m()
_ARRAYCOPY w(2).g(1 TO 1, 1 TO 2) TO m(1, 1)
FOR i = 1 TO 2: FOR j = 1 TO 3: PRINT m(i, j);: NEXT: NEXT: PRINT
PRINT "5:";
_ARRAYCOPY m() TO w(2).g()
_ARRAYCOPY m() TO w(2).g(1, 1)
_ARRAYCOPY m(1 TO 2, 1 TO 2) TO w(2).g()
PRINT
PRINT "6:";
_ARRAYCOPY v.n() TO w(1).n()
PRINT
REDIM v.n(1 TO 4) AS LONG
_ARRAYCOPY src() TO v.n()
PRINT "7:";: FOR i = 1 TO 4: PRINT v.n(i);: NEXT: PRINT
SYSTEM
h:
PRINT " error"; ERR;
RESUME NEXT
