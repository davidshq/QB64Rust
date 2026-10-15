$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Slice (m3-whole-array-assign task 4.3): whole-array assignment with member arrays and owner types
' (verification\v36_d, v36_e, v36_x33, x46, x47, x49-x51): member to member, member to array and back, members
' of array elements, STRING and user-type member arrays, a member's bounds that differ (error 9), the target's
' place found before the source's, an error handled before the statement; arrays of a type with _DYNAMIC members
' (a DIM and a REDIM target, the members' own bounds, strings, a nested owner type), a _DYNAMIC member array to
' another and to a plain array, inline and _DYNAMIC member arrays of an owner type, _MEM of a member of an
' element after the copy. Where the new compiler differs on purpose (an index outside on the way, DIVERGENCES.md
' D-004; an owner type with other bounds, D-022) nothing is here: CLI tests hold those.
TYPE leaf
    k AS LONG
    s AS STRING
END TYPE
TYPE box
    v(1 TO 3) AS LONG
    w(1) AS STRING
    l(1) AS leaf
END TYPE
TYPE t2
    v(1) AS LONG
END TYPE
TYPE own
    a AS INTEGER
    v(1) _DYNAMIC AS LONG
    s AS STRING
END TYPE
TYPE nest
    o AS own
    w(0) _DYNAMIC AS STRING
END TYPE
TYPE own2
    v(1) _DYNAMIC AS LONG
    k AS LONG
END TYPE
TYPE ibox
    l(1) AS own2
END TYPE
TYPE dbox
    d(1) _DYNAMIC AS own2
END TYPE
ON ERROR GOTO h
DIM x AS box, y AS box, i AS LONG
DIM a(1 TO 3) AS LONG
DIM b(0 TO 2) AS LONG
DIM e(2) AS box
FOR i = 1 TO 3: x.v(i) = i * 5: NEXT
x.w(0) = "w0": x.w(1) = "w1": x.l(1).k = 9: x.l(1).s = "leaf"
PRINT "1. member to member:";
y.v() = x.v(): y.w() = x.w(): y.l() = x.l()
x.w(1) = "changed": x.l(1).s = "changed"
PRINT y.v(1); y.v(3); " "; y.w(0); " "; y.w(1); y.l(1).k; " "; y.l(1).s
PRINT "2. member to array:";
a() = x.v()
FOR i = 1 TO 3: PRINT a(i);: NEXT: PRINT
PRINT "3. array to member:";
a(2) = -1
x.v() = a()
FOR i = 1 TO 3: PRINT x.v(i);: NEXT: PRINT
PRINT "4. members of array elements:";
e(1).v(2) = 77: e(1).w(1) = "e1"
e(2).v() = e(1).v(): e(2).w() = e(1).w()
PRINT e(2).v(2); " "; e(2).w(1)
PRINT "5. other bounds (error):";
b() = x.v()
PRINT " after:"; b(0)
PRINT "6. a member of an element from an array:";
DIM e2(2) AS t2, a2(1) AS LONG
a2(1) = 4
e2(1).v() = a2()
PRINT e2(1).v(1)
PRINT "7. the target's place first:";
e2(1).v(1) = 9
e2(f(2)).v() = e2(f(1)).v()
PRINT e2(2).v(1)
PRINT "8. an error handled before:";
a2(1) = 3
ERROR 7: e2(1).v() = a2()
PRINT e2(1).v(1)
PRINT "9. DIM target of an owner type:";
DIM src(1 TO 2) AS own, dst(1 TO 2) AS own
REDIM rsrc(1 TO 2) AS own, rdst(1 TO 2) AS own
src(1).a = 5: REDIM src(1).v(3 TO 5): src(1).v(4) = 44: src(1).s = "one"
src(2).v(1) = 21
dst(1).v(0) = 999: dst(2).s = "old"
dst() = src()
src(1).v(4) = -1: src(1).s = "gone"
PRINT dst(1).a; LBOUND(dst(1).v); UBOUND(dst(1).v); dst(1).v(4); " "; dst(1).s; dst(2).v(1); "["; dst(2).s; "]"
PRINT "10. REDIM target:";
rsrc(2).s = "r2": REDIM rsrc(2).v(0 TO 2): rsrc(2).v(2) = 22
rdst() = rsrc()
PRINT UBOUND(rdst(2).v); rdst(2).v(2); " "; rdst(2).s
PRINT "11. DIM target from REDIM source:";
dst() = rsrc()
PRINT UBOUND(dst(2).v); dst(2).v(2); " "; dst(2).s
PRINT "12. nested owner type:";
DIM ns(1) AS nest, nd(1) AS nest
REDIM ns(1).o.v(0 TO 2): ns(1).o.v(2) = 12: ns(1).w(0) = "w": ns(1).o.s = "os"
nd() = ns()
ns(1).w(0) = "x"
PRINT nd(1).o.v(2); " "; nd(1).w(0); " "; nd(1).o.s
PRINT "13. a _DYNAMIC member to another, other bounds:";
DIM p AS own, q AS own
REDIM p.v(1 TO 2): p.v(2) = 8
q.v() = p.v()
PRINT LBOUND(q.v); UBOUND(q.v)
PRINT "14. the same bounds:";
REDIM q.v(1 TO 2)
q.v() = p.v()
PRINT q.v(2)
PRINT "15. a plain array to a _DYNAMIC member and back:";
DIM pa(1 TO 2) AS LONG
pa(1) = 31
q.v() = pa(): pa(2) = -9: pa() = p.v()
PRINT q.v(1); pa(2)
PRINT "16. an inline member array of an owner type:";
DIM ib AS ibox, oa(1) AS own2
oa(1).k = 5
ib.l() = oa()
PRINT ib.l(1).k;
oa(1).k = 0
oa() = ib.l()
PRINT oa(1).k
PRINT "17. a _DYNAMIC member array of an owner type:";
DIM db AS dbox
oa(1).k = 6
db.d() = oa()
PRINT db.d(1).k;
oa(1).k = 0
oa() = db.d()
PRINT oa(1).k
PRINT "18. _MEM of a member of an element:";
REDIM ms(1) AS own2, md(1) AS own2
DIM md2(1) AS own2
ms(1).k = 4
DIM m AS _MEM, m2 AS _MEM
m = _MEM(md(1).k): m2 = _MEM(md2(1).k)
md() = ms(): md2() = ms()
PRINT _MEMEXISTS(m); _MEMEXISTS(m2); md(1).k; md2(1).k
SYSTEM
h:
PRINT " error"; ERR;
RESUME NEXT

FUNCTION f (n)
    PRINT " f"; n;
    f = n
END FUNCTION
