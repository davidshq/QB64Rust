$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m4-handlers-logging-checking, task 1.2): elements of arrays, member arrays and dynamic member
' arrays written and read below $CHECKING:OFF, within their bounds: the same elements as with checking on.
TYPE inner
    q(1 TO 3) AS INTEGER
END TYPE
TYPE t
    v(0 TO 3) AS LONG
    n(1) AS inner
    w(2) _DYNAMIC AS DOUBLE
END TYPE
DIM a(1 TO 3) AS LONG, g(2, -1 TO 1) AS INTEGER, s(2) AS STRING
DIM x AS t, xs(1 TO 2) AS t, i AS LONG, j AS LONG
REDIM dyn(5 TO 7) AS _INTEGER64
$CHECKING:OFF
FOR i = 1 TO 3: a(i) = i * 10: NEXT
FOR i = 0 TO 2: FOR j = -1 TO 1: g(i, j) = i * 10 + j: NEXT: NEXT
s(2) = "two"
FOR i = 5 TO 7: dyn(i) = i * 100: NEXT
FOR i = 0 TO 3: x.v(i) = i + 1: NEXT
x.n(1).q(3) = 13
xs(2).v(3) = 23
xs(2).n(0).q(1) = 201
FOR i = 0 TO 2: x.w(i) = i / 4: NEXT
REDIM x.w(1 TO 4) AS DOUBLE
x.w(4) = 4.5
xs(1).w(2) = 1.25
PRINT "1."; a(1); a(2); a(3); g(0, -1); g(2, 1); g(1, 0); " "; s(2); dyn(5); dyn(7)
PRINT "2."; x.v(0); x.v(3); x.n(1).q(3); xs(2).v(3); xs(2).n(0).q(1)
PRINT "3."; x.w(1); x.w(4); xs(1).w(2); LBOUND(x.w); UBOUND(x.w)
SWAP a(1), a(3)
PRINT "4."; a(1); a(3)
$CHECKING:ON
PRINT "5."; a(1); a(2); a(3); g(2, 1); x.v(3); x.w(4); dyn(6)
SYSTEM
