$CONSOLE:ONLY
' Default types in the main module (spec language/default-types): DEFINT, DEFLNG, DEFSNG, DEFDBL, DEFSTR with
' ranges, single letters, any letter case, a range written backwards and the underscore entry; a name before and
' after a statement (the plain name is the name plus the default at its line), a variable declared with AS, a
' later statement for the same letter, a statement that never runs (skipped by GOTO, in an IF block); what the
' default types: DIM, DIM SHARED, an array, a FOR variable, but not a CONST; _DEFINE with every type a suffix
' can give, STRING * n and _BIT * n included. LEN of a variable shows its type.
a = 1.5
DEFINT A-Z
b = 1.5
PRINT "1. a = 1.5 above DEFINT A-Z, b = 1.5 below: a, b, a!"; a; b; a!
x = 2.5
PRINT "2. x = 2.5: x, x / 2, LEN(x)"; x; x / 2; LEN(x)
DIM dd AS DOUBLE
dd = 1.5: s! = 2.5: n$ = "q"
PRINT "3. AS and suffixes win: "; dd; s!; n$; LEN(dd); LEN(s!)
DIM lg AS LONG
lg = 100000
DEFSTR L
PRINT "4. DIM lg AS LONG, then DEFSTR L: lg, LEN(lg)"; lg; LEN(lg)
DEFINT L
DEFDBL D
i = 1.5: d = 1.5
PRINT "5. DEFDBL D below DEFINT A-Z: i, d"; i; d
DEFSNG A-Z
DEFLNG a-c, x
b3 = 100000: d3 = 1.5: x3 = 70000: c3 = 2.5
PRINT "6. DEFLNG a-c, x: b3, d3, x3, c3"; b3; d3; x3; c3
DEFSNG A-Z
DEFSTR S
s = "text"
PRINT "7. DEFSTR S: "; s + "!"; LEN(s)
DEFSNG A-Z
GOTO skip
DEFLNG G
skip:
g = 2.5
PRINT "8. DEFLNG G skipped by GOTO: g, LEN(g)"; g; LEN(g)
IF 0 THEN
    DEFDBL H
END IF
h = 1# / 3
PRINT "9. DEFDBL H in an IF block that does not run: h"; h
DEFSNG A-Z
DEFINT z-w
v4 = 2.5: w4 = 2.5: x4 = 2.5: y4 = 2.5: z4 = 2.5
PRINT "10. DEFINT z-w: v4 to z4"; v4; w4; x4; y4; z4
DEFSNG A-Z
defint k, K, k-m, l
k5 = 2.5: l5 = 2.5: m5 = 2.5: n5 = 2.5
PRINT "11. defint k, K, k-m, l: k5 to n5"; k5; l5; m5; n5
DEFINT A-Z
__a = 2.5
PRINT "12. __a under DEFINT A-Z:"; __a; LEN(__a)
DEFINT _, y-z
__b = 2.5
DEFDBL y-_, y
__c = 2.5: y6 = 2.5: z6 = 2.5: x6 = 2.5
PRINT "    after DEFINT _, y-z: __b; after DEFDBL y-_, y: __c, y6, z6, x6"; __b; __c; y6; z6; x6
DEFSNG _, A-Z
DEFLNG A-Z: DEFINT Q: q7 = 2.5: r7 = 2.5
PRINT "13. two statements on one line: LEN(q7), LEN(r7)"; LEN(q7); LEN(r7)
DEFINT A-Z
DIM t(3), dv
DIM SHARED gv
t(1) = 2.5: dv = 2.5: gv = 2.5
PRINT "14. DIM t(3), dv and DIM SHARED gv under DEFINT A-Z:"; t(1); dv; gv; LEN(t(1)); LEN(dv); LEN(gv)
FOR fi = 1 TO 2: NEXT
PRINT "15. a FOR variable:"; fi; LEN(fi)
CONST ck = 2.5
DEFSTR S
CONST s1 = 5
DEFINT S
CONST kk = "abc"
PRINT "16. CONST ck = 2.5, CONST s1 = 5 under DEFSTR S, CONST kk = a string under DEFINT: "; ck; s1 + 1; kk + "!"
DEFSNG A-Z
_DEFINE a AS INTEGER: a1 = 2.5: PRINT "17. INTEGER"; a1; LEN(a1)
_DEFINE b AS LONG: b1 = 2.5: PRINT "    LONG"; b1; LEN(b1)
_DEFINE c AS SINGLE: c1 = 2.5: PRINT "    SINGLE"; c1; LEN(c1)
_DEFINE d AS DOUBLE: d1 = 1# / 3: PRINT "    DOUBLE"; d1; LEN(d1)
_DEFINE e AS _FLOAT: e1 = 2.5: PRINT "    _FLOAT"; e1; LEN(e1)
_DEFINE f AS _BYTE: f1 = -1: PRINT "    _BYTE"; f1; LEN(f1)
_DEFINE g AS _UNSIGNED _BYTE: g1 = -1: PRINT "    _UNSIGNED _BYTE"; g1; LEN(g1)
_DEFINE h AS _UNSIGNED INTEGER: h1 = -1: PRINT "    _UNSIGNED INTEGER"; h1; LEN(h1)
_DEFINE i AS _UNSIGNED LONG: i1 = -1: PRINT "    _UNSIGNED LONG"; i1; LEN(i1)
_DEFINE j AS _INTEGER64: j1 = 5000000000.5: PRINT "    _INTEGER64"; j1; LEN(j1)
_DEFINE k AS _UNSIGNED _INTEGER64: k1 = -1: PRINT "    _UNSIGNED _INTEGER64"; k1; LEN(k1)
_DEFINE l AS _OFFSET: l1 = -1: PRINT "    _OFFSET"; l1; LEN(l1)
_DEFINE m AS _UNSIGNED _OFFSET: m1 = 5: PRINT "    _UNSIGNED _OFFSET"; m1; LEN(m1)
_DEFINE n AS STRING: n1 = "abc": PRINT "    STRING "; n1; LEN(n1)
_DEFINE o-q, s AS LONG
o1 = 2.5: p1 = 2.5: q1 = 2.5: r1 = 2.5: s2 = 2.5
PRINT "18. _DEFINE o-q, s AS LONG: LEN of o1, p1, q1, s2, and r1"; LEN(o1); LEN(p1); LEN(q1); LEN(s2); r1
_Define U As _Unsigned _Byte
u = -1
PRINT "19. _Define U As _Unsigned _Byte: u"; u
_DEFINE v AS STRING * 5
v1 = "abcdefgh"
PRINT "20. _DEFINE v AS STRING * 5: ["; v1; "]"; LEN(v1)
_DEFINE w AS _BIT * 4
_DEFINE x AS _UNSIGNED _BIT
_DEFINE y AS _BIT
w1 = 7: PRINT "21. _BIT * 4, _UNSIGNED _BIT, _BIT:"; w1;
w1 = 9: x1 = 1: y1 = 1: PRINT w1; x1; y1
_DEFINE _ AS DOUBLE
__d = 1# / 3
PRINT "22. _DEFINE _ AS DOUBLE: __d"; __d
SYSTEM
