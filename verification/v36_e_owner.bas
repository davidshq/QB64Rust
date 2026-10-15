$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-whole-array-assign, before its design): whole-array assignment of arrays whose type has a
' _DYNAMIC member (owner types): a DIM target (copied in place) and a REDIM target (a new block), the members'
' own bounds copied, strings, a nested owner type, a _DYNAMIC member array to another,
' the target REDIMmed.
TYPE own
    a AS INTEGER
    v(1) _DYNAMIC AS LONG
    s AS STRING
END TYPE
TYPE nest
    o AS own
    w(0) _DYNAMIC AS STRING
END TYPE
ON ERROR GOTO h
DIM i AS LONG
DIM src(1 TO 2) AS own, dst(1 TO 2) AS own
REDIM rsrc(1 TO 2) AS own, rdst(1 TO 2) AS own
src(1).a = 5: REDIM src(1).v(3 TO 5): src(1).v(4) = 44: src(1).s = "one"
src(2).v(1) = 21
dst(1).v(0) = 999: dst(2).s = "old"
PRINT "1. DIM target:";
dst() = src()
src(1).v(4) = -1: src(1).s = "gone"
PRINT dst(1).a; LBOUND(dst(1).v); UBOUND(dst(1).v); dst(1).v(4); " "; dst(1).s; dst(2).v(1); "["; dst(2).s; "]"
PRINT "2. REDIM target:";
rsrc(2).s = "r2": REDIM rsrc(2).v(0 TO 2): rsrc(2).v(2) = 22
rdst() = rsrc()
PRINT UBOUND(rdst(2).v); rdst(2).v(2); " "; rdst(2).s
PRINT "3. DIM target from REDIM source:";
dst() = rsrc()
PRINT UBOUND(dst(2).v); dst(2).v(2); " "; dst(2).s
PRINT "4. nested owner type:";
DIM ns(1) AS nest, nd(1) AS nest
REDIM ns(1).o.v(0 TO 2): ns(1).o.v(2) = 12: ns(1).w(0) = "w": ns(1).o.s = "os"
nd() = ns()
ns(1).w(0) = "x"
PRINT nd(1).o.v(2); " "; nd(1).w(0); " "; nd(1).o.s
PRINT "5. a _DYNAMIC member to another, other bounds:";
DIM p AS own, q AS own
REDIM p.v(1 TO 2): p.v(2) = 8
q.v() = p.v()
PRINT LBOUND(q.v); UBOUND(q.v)
PRINT "6. the same bounds:";
REDIM q.v(1 TO 2)
q.v() = p.v()
PRINT q.v(2)
PRINT "7. a plain array to a _DYNAMIC member and back:";
DIM a(1 TO 2) AS LONG
a(1) = 31
q.v() = a(): a(2) = -9: a() = p.v()
PRINT q.v(1); a(2)
SYSTEM
h:
PRINT " error"; ERR;
RESUME NEXT
