$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-arraycopy, after its review): _DYNAMIC member arrays that a REDIM gave another number of
' dimensions than they were declared with, as source and destination of each form. The plain arrays beside them
' are printed: -1 where nothing was copied.
TYPE dy
    _DYNAMIC n(1 TO 4) AS LONG
    _DYNAMIC g(1 TO 2, 1 TO 3) AS LONG
END TYPE
DIM i AS LONG, j AS LONG
DIM v AS dy
DIM src(1 TO 4) AS LONG, d(1 TO 10) AS LONG, m(1 TO 2, 1 TO 3) AS LONG
ON ERROR GOTO h
FOR i = 1 TO 4: src(i) = i * 100: NEXT
REDIM v.n(1 TO 2, 1 TO 3) AS LONG
REDIM v.g(1 TO 6) AS LONG
PRINT "1. 1D member now 2D, whole to d():";
FOR i = 1 TO 10: d(i) = -1: NEXT
_ARRAYCOPY v.n() TO d()
FOR i = 1 TO 10: PRINT d(i);: NEXT: PRINT
PRINT "2. 1D member now 2D, range to d(5):";
FOR i = 1 TO 10: d(i) = -1: NEXT
_ARRAYCOPY v.n(1 TO 2) TO d(5)
FOR i = 1 TO 10: PRINT d(i);: NEXT: PRINT
PRINT "3. src() to the 1D member now 2D:";
_ARRAYCOPY src() TO v.n()
PRINT " done"
PRINT "4. src() to the 1D member now 2D at 1:";
_ARRAYCOPY src() TO v.n(1)
PRINT " done"
PRINT "5. 2D member now 1D, whole to m():";
FOR i = 1 TO 2: FOR j = 1 TO 3: m(i, j) = -1: NEXT: NEXT
_ARRAYCOPY v.g() TO m()
FOR i = 1 TO 2: FOR j = 1 TO 3: PRINT m(i, j);: NEXT: NEXT: PRINT
PRINT "6. 2D member now 1D, ranges to m(1, 1):";
FOR i = 1 TO 2: FOR j = 1 TO 3: m(i, j) = -1: NEXT: NEXT
_ARRAYCOPY v.g(1 TO 1, 1 TO 2) TO m(1, 1)
FOR i = 1 TO 2: FOR j = 1 TO 3: PRINT m(i, j);: NEXT: NEXT: PRINT
PRINT "7. m() to the 2D member now 1D:";
_ARRAYCOPY m() TO v.g()
PRINT " done"
PRINT "8. m() to the 2D member now 1D at (1, 1):";
_ARRAYCOPY m() TO v.g(1, 1)
PRINT " done"
SYSTEM
h:
PRINT " error"; ERR;
RESUME NEXT
