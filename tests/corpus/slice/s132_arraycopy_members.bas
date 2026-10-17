$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Slice (m3-arraycopy): _ARRAYCOPY of member arrays (verification\v41_d, v41_e, v41_k): of a plain TYPE, of a
' variable, of an element, nested, of numbers, STRING * n and STRING, the members beside them unchanged, the order
' of the indexes on the way; _DYNAMIC member arrays of one and two dimensions and STRING, after a REDIM of the
' member, an erased one. Left out, as the new compiler differs on purpose: an index outside its bounds on the way
' (DIVERGENCES.md D-004), a CLI test.
TYPE t
    k AS INTEGER
    b(1 TO 6) AS LONG
    s(0 TO 2) AS STRING * 3
    v(0 TO 3) AS STRING
END TYPE
TYPE o
    tag AS LONG
    inner AS t
END TYPE
TYPE dy
    _DYNAMIC n(1 TO 4) AS LONG
    _DYNAMIC g(1 TO 2, 1 TO 3) AS INTEGER
    _DYNAMIC s(0 TO 2) AS STRING
END TYPE
DIM i AS LONG, j AS LONG
DIM v AS t, tt(1 TO 2) AS t, oo AS o, oa(0 TO 1) AS o
DIM src(1 TO 4) AS LONG, dst(0 TO 7) AS LONG
FOR i = 1 TO 4: src(i) = i * 11: NEXT

PRINT "-- plain members"
_ARRAYCOPY src() TO v.b(2)
FOR i = 1 TO 6: PRINT v.b(i);: NEXT: PRINT
_ARRAYCOPY src(2 TO 3) TO tt(2).b(5)
FOR i = 1 TO 6: PRINT tt(2).b(i);: NEXT: PRINT
FOR i = 1 TO 6: PRINT tt(1).b(i);: NEXT: PRINT
_ARRAYCOPY v.b() TO dst()
FOR i = 0 TO 7: PRINT dst(i);: NEXT: PRINT
_ARRAYCOPY v.b(2 TO 4) TO oo.inner.b()
FOR i = 1 TO 6: PRINT oo.inner.b(i);: NEXT: PRINT
_ARRAYCOPY oo.inner.b() TO oa(1).inner.b(1)
FOR i = 1 TO 6: PRINT oa(1).inner.b(i);: NEXT: PRINT
PRINT oa(1).tag; oa(1).inner.k; oa(0).inner.b(1)
v.s(0) = "abc": v.s(1) = "def": v.s(2) = "ghi"
_ARRAYCOPY v.s(1 TO 2) TO tt(1).s(0)
PRINT tt(1).s(0); tt(1).s(1); "["; tt(1).s(2); "]"
DIM fs(0 TO 2) AS STRING * 3
_ARRAYCOPY v.s() TO fs()
PRINT fs(0); fs(1); fs(2)
FOR i = 1 TO 6: v.b(i) = i: NEXT
_ARRAYCOPY v.b(1 TO 4) TO v.b(3)
FOR i = 1 TO 6: PRINT v.b(i);: NEXT: PRINT
_ARRAYCOPY src() TO tt(ix(2)).b(ix(1))
FOR i = 1 TO 6: PRINT tt(2).b(i);: NEXT: PRINT
_ARRAYCOPY tt(ix(1)).b(ix(2) TO ix(3)) TO dst(ix(0))
FOR i = 0 TO 7: PRINT dst(i);: NEXT: PRINT

PRINT "-- STRING members"
DIM w(1 TO 2) AS t, a(1 TO 3) AS STRING
a(1) = "one": a(2) = "two": a(3) = "three"
_ARRAYCOPY a() TO v.v(1)
FOR i = 0 TO 3: PRINT " ["; v.v(i); "]";: NEXT: PRINT
a(2) = "changed"
PRINT " ["; v.v(2); "]"
_ARRAYCOPY v.v(1 TO 2) TO w(2).v()
FOR i = 0 TO 3: PRINT " ["; w(2).v(i); "]";: NEXT: PRINT
_ARRAYCOPY w(2).v() TO v.v()
FOR i = 0 TO 3: PRINT " ["; v.v(i); "]";: NEXT: PRINT
_ARRAYCOPY v.v(0 TO 2) TO v.v(1)
FOR i = 0 TO 3: PRINT " ["; v.v(i); "]";: NEXT: PRINT
PRINT v.k; w(1).k

PRINT "-- _DYNAMIC members"
DIM x AS dy, y(1 TO 2) AS dy
DIM src2(1 TO 4) AS LONG, back(1 TO 10) AS LONG
FOR i = 1 TO 4: src2(i) = i * 100: NEXT
_ARRAYCOPY src2() TO x.n()
FOR i = 1 TO 4: PRINT x.n(i);: NEXT: PRINT
REDIM x.n(1 TO 10) AS LONG
_ARRAYCOPY src2() TO x.n(7)
FOR i = 1 TO 10: PRINT x.n(i);: NEXT: PRINT
_ARRAYCOPY x.n() TO back()
FOR i = 1 TO 10: PRINT back(i);: NEXT: PRINT
_ARRAYCOPY x.n(7 TO 9) TO y(2).n(1)
FOR i = 1 TO 4: PRINT y(2).n(i);: NEXT: PRINT
DIM gs(1 TO 2, 1 TO 3) AS INTEGER
FOR i = 1 TO 2: FOR j = 1 TO 3: gs(i, j) = i * 10 + j: NEXT: NEXT
_ARRAYCOPY gs() TO y(1).g()
FOR i = 1 TO 2: FOR j = 1 TO 3: PRINT y(1).g(i, j);: NEXT: NEXT: PRINT
_ARRAYCOPY y(1).g(1 TO 2, 2 TO 3) TO x.g(1, 1)
FOR i = 1 TO 2: FOR j = 1 TO 3: PRINT x.g(i, j);: NEXT: NEXT: PRINT
DIM ss(0 TO 2) AS STRING
ss(0) = "x": ss(1) = "yy": ss(2) = "zzz"
_ARRAYCOPY ss() TO x.s()
FOR i = 0 TO 2: PRINT " ["; x.s(i); "]";: NEXT: PRINT
_ARRAYCOPY x.s(1 TO 2) TO y(2).s(0)
FOR i = 0 TO 2: PRINT " ["; y(2).s(i); "]";: NEXT: PRINT
ss(1) = "changed"
_ARRAYCOPY x.s() TO ss()
FOR i = 0 TO 2: PRINT " ["; ss(i); "]";: NEXT: PRINT
ON ERROR GOTO h
ERASE x.n
PRINT "erased member as source:";
_ARRAYCOPY x.n() TO back()
PRINT " done"
SYSTEM
h:
PRINT " error"; ERR;
RESUME NEXT

FUNCTION ix& (n AS LONG)
    PRINT "(ix"; n; ")";
    ix& = n
END FUNCTION
