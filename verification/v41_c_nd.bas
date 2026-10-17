$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): multidimensional arrays, numbers and strings: whole to
' whole with the same bounds, whole to a start (into a larger destination), a range of each dimension to a start
' and to whole; 3D; STRING * n.
DIM i AS LONG, j AS LONG, k AS LONG
DIM m(1 TO 2, 0 TO 2) AS LONG, n(1 TO 2, 0 TO 2) AS LONG
DIM big(0 TO 4, 0 TO 5) AS LONG
FOR i = 1 TO 2: FOR j = 0 TO 2: m(i, j) = i * 10 + j: NEXT: NEXT

_ARRAYCOPY m() TO n()
PRINT "1. whole to whole:";: FOR i = 1 TO 2: FOR j = 0 TO 2: PRINT n(i, j);: NEXT: NEXT: PRINT

GOSUB clr: _ARRAYCOPY m() TO big(1, 2)
PRINT "2. whole to start (1, 2):": GOSUB show

FOR i = 0 TO 4: FOR j = 0 TO 5: big(i, j) = i * 10 + j: NEXT: NEXT
DIM small(0 TO 2, 0 TO 3) AS LONG
FOR i = 0 TO 2: FOR j = 0 TO 3: small(i, j) = -1: NEXT: NEXT
_ARRAYCOPY big(1 TO 2, 2 TO 4) TO small(1, 0)
PRINT "3. range to start (1, 0):";
FOR i = 0 TO 2: PRINT: FOR j = 0 TO 3: PRINT small(i, j);: NEXT: NEXT: PRINT

DIM other(0 TO 4, 0 TO 5) AS LONG
FOR i = 0 TO 4: FOR j = 0 TO 5: other(i, j) = -1: NEXT: NEXT
_ARRAYCOPY big(1 TO 2, 2 TO 4) TO other()
PRINT "4. range to whole:";
FOR i = 0 TO 4: PRINT: FOR j = 0 TO 5: PRINT other(i, j);: NEXT: NEXT: PRINT

DIM c3(1 TO 2, 1 TO 2, 1 TO 2) AS INTEGER, e3(1 TO 2, 1 TO 2, 1 TO 2) AS INTEGER
FOR i = 1 TO 2: FOR j = 1 TO 2: FOR k = 1 TO 2: c3(i, j, k) = i * 100 + j * 10 + k: NEXT: NEXT: NEXT
_ARRAYCOPY c3() TO e3()
PRINT "5. 3D whole:";: FOR i = 1 TO 2: FOR j = 1 TO 2: FOR k = 1 TO 2: PRINT e3(i, j, k);: NEXT: NEXT: NEXT: PRINT
FOR i = 1 TO 2: FOR j = 1 TO 2: FOR k = 1 TO 2: e3(i, j, k) = 0: NEXT: NEXT: NEXT
_ARRAYCOPY c3(2 TO 2, 1 TO 2, 2 TO 2) TO e3(1, 1, 1)
PRINT "6. 3D range to start:";: FOR i = 1 TO 2: FOR j = 1 TO 2: FOR k = 1 TO 2: PRINT e3(i, j, k);: NEXT: NEXT: NEXT: PRINT

DIM sm(1 TO 2, 1 TO 3) AS STRING, sn(1 TO 2, 1 TO 3) AS STRING, sb(0 TO 3, 0 TO 4) AS STRING
FOR i = 1 TO 2: FOR j = 1 TO 3: sm(i, j) = CHR$(64 + i) + CHR$(48 + j): NEXT: NEXT
_ARRAYCOPY sm() TO sn()
PRINT "7. strings whole:";: FOR i = 1 TO 2: FOR j = 1 TO 3: PRINT " "; sn(i, j);: NEXT: NEXT: PRINT
GOSUB sclr: _ARRAYCOPY sm() TO sb(1, 1)
PRINT "8. strings whole to start:": GOSUB sshow
GOSUB sclr: _ARRAYCOPY sm(1 TO 2, 2 TO 3) TO sb(0, 0)
PRINT "9. strings range to start:": GOSUB sshow
GOSUB sclr: _ARRAYCOPY sm(2 TO 2, 1 TO 3) TO sb()
PRINT "10. strings range to whole:": GOSUB sshow

DIM fx(1 TO 2, 1 TO 2) AS STRING * 2, fy(1 TO 2, 1 TO 2) AS STRING * 2
fx(1, 1) = "aa": fx(2, 1) = "bb": fx(1, 2) = "cc": fx(2, 2) = "dd"
_ARRAYCOPY fx() TO fy()
PRINT "11. STRING * 2 whole:"; fy(1, 1); fy(2, 1); fy(1, 2); fy(2, 2)
SYSTEM
clr:
FOR i = 0 TO 4: FOR j = 0 TO 5: big(i, j) = -1: NEXT: NEXT
RETURN
show:
FOR i = 0 TO 4: FOR j = 0 TO 5: PRINT big(i, j);: NEXT: PRINT: NEXT
RETURN
sclr:
FOR i = 0 TO 3: FOR j = 0 TO 4: sb(i, j) = "..": NEXT: NEXT
RETURN
sshow:
FOR i = 0 TO 3: FOR j = 0 TO 4: PRINT " "; sb(i, j);: NEXT: PRINT: NEXT
RETURN
