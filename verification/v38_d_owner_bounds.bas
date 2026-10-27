$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-measured-copies, before its design; QB64PE-DIVERGENCES.md D-022): a whole-array assignment of arrays
' of a type with a _DYNAMIC member array whose bounds differ. The old compiler raises error 9 and goes on: it
' copies as many elements as the target has (AppendDynUDTArrayAssign, qb64pe.bas 16855), in place for a static
' target and into a new block for a dynamic one. Each case prints the target's elements after the copy.
' 1. static target, as many elements, other bounds
' 2. static target, fewer elements
' 3. static target, more elements (reads past the source)
' 4. dynamic target (REDIM), as many elements, other bounds
' 5. dynamic target, fewer elements
' 6. two dimensions of the same count in another shape (2 x 2 from 4 x 1)
' 7. a source that ERASE released, to an allocated target
' 8. the members of the elements copied: are they the source's (deep copy)?
TYPE own
    v(1) _DYNAMIC AS LONG
    k AS LONG
END TYPE
ON ERROR GOTO h
DIM i AS LONG
DIM s(1 TO 2) AS own
s(1).k = 4: s(2).k = 5: s(1).v(0) = 40: s(2).v(1) = 51
DIM d1(0 TO 1) AS own, d2(0 TO 0) AS own, d3(1 TO 3) AS own
d3(3).k = 7
PRINT "1.";: d1() = s(): PRINT d1(0).k; d1(1).k
PRINT "2.";: d2() = s(): PRINT d2(0).k
PRINT "3.";: d3() = s(): PRINT d3(1).k; d3(2).k; d3(3).k
REDIM r4(5 TO 6) AS own, r5(0 TO 0) AS own
PRINT "4.";: r4() = s(): PRINT r4(5).k; r4(6).k; LBOUND(r4); UBOUND(r4)
PRINT "5.";: r5() = s(): PRINT r5(0).k
REDIM m(1 TO 4, 0 TO 0) AS own, n(0 TO 1, 0 TO 1) AS own
FOR i = 1 TO 4: m(i, 0).k = 60 + i: NEXT
PRINT "6.";: n() = m(): PRINT n(0, 0).k; n(1, 0).k; n(0, 1).k; n(1, 1).k
REDIM e(1 TO 2) AS own, g(1 TO 2) AS own
g(1).k = 9
ERASE e
PRINT "7.";: g() = e(): PRINT g(1).k; g(2).k
PRINT "8.";: d1(0).v(0) = 99: PRINT d1(0).v(0); s(1).v(0); d1(1).v(1)
PRINT "ok"
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
