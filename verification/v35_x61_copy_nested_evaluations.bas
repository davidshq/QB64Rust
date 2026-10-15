$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, task 4.3): a whole-value copy of an owner type nested in another, as a member and
' as an inline member array: how many times each place's index is evaluated, without and with a STRING in the
' type; and what the copy does to the target's member when the source's member is not allocated.
TYPE inner
    k AS LONG
    d(1) _DYNAMIC AS LONG
END TYPE
TYPE outer
    a AS LONG
    i AS inner
    r(1) AS inner
END TYPE
TYPE inners
    k AS LONG
    s AS STRING
    d(1) _DYNAMIC AS LONG
END TYPE
TYPE outers
    a AS LONG
    i AS inners
    r(1) AS inners
    t(1) AS STRING
END TYPE
TYPE flat
    d(1) _DYNAMIC AS LONG
END TYPE
TYPE owner
    s AS STRING
    d(1) _DYNAMIC AS LONG
END TYPE
ON ERROR GOTO h
REDIM a(2) AS outer, b(2) AS outer, c(2) AS outers, e(2) AS outers
b(1).a = 1: b(1).i.k = 2: b(1).i.d(1) = 3: b(1).r(1).k = 4: b(1).r(1).d(0) = 5
e(1).a = 1: e(1).i.s = "s": e(1).i.d(1) = 3: e(1).r(1).s = "r": e(1).r(1).d(0) = 5: e(1).t(1) = "t"
PRINT "1.";
a(one(1)) = b(one(2))
PRINT
PRINT "  "; a(1).a; a(1).i.k; a(1).i.d(1); a(1).r(1).k; a(1).r(1).d(0)
b(1).i.d(1) = 30: b(1).r(1).d(0) = 50
PRINT "  after a change of b(1):"; a(1).i.d(1); a(1).r(1).d(0)
PRINT "2.";
c(one(1)) = e(one(2))
PRINT
PRINT "  "; c(1).a; " "; c(1).i.s; c(1).i.d(1); " "; c(1).r(1).s; c(1).r(1).d(0); " "; c(1).t(1)
DIM f AS flat, g AS flat, o AS owner, p AS owner
f.d(1) = 7: o.d(1) = 7
ERASE g.d, p.d
f = g
PRINT "3. flat, source erased:";
PRINT LBOUND(f.d); UBOUND(f.d);
PRINT f.d(1)
o = p
PRINT "4. owner, source erased:";
PRINT LBOUND(o.d); UBOUND(o.d);
PRINT o.d(1)
REDIM p.d(2 TO 3) AS LONG: p.d(3) = 9
o = p
PRINT "5. owner, source redimmed:"; LBOUND(o.d); UBOUND(o.d); o.d(3)
o = o
PRINT "6. copied to itself:"; LBOUND(o.d); UBOUND(o.d); o.d(3)
SYSTEM
h:
PRINT "[error"; ERR; "]";
RESUME NEXT
FUNCTION one& (k AS LONG)
    PRINT k;
    one& = 1
END FUNCTION
