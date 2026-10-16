$CONSOLE:ONLY
' Slice (m3-whole-array-assign task 3.2): whole-array assignment of array variables (verification\v36_a, v36_b,
' v36_c, v36_f, v36_x14, x17, x20, x36): every numeric type, two and three dimensions, a REDIM array on either
' side, an array assigned to itself, LET, the source in parentheses, a one-line IF; STRING, STRING * n, _BIT, user
' types of numbers, with strings, nested and with member arrays, _MEM; arrays of every place (SHARED, COMMON
' SHARED, local, STATIC, parameters, $DYNAMIC, used without DIM, a FUNCTION); the run-time errors (bounds that
' differ, arrays not allocated), after which nothing is copied.
COMMON SHARED cm() AS INTEGER
DIM SHARED cm(1 TO 3) AS INTEGER
DIM SHARED g(1 TO 3) AS INTEGER
DIM SHARED gs(1 TO 2) AS STRING
TYPE pair
    a AS LONG
    b AS SINGLE
END TYPE
TYPE txt
    n AS INTEGER
    s AS STRING
    f AS STRING * 3
END TYPE
TYPE outer
    p AS pair
    t AS txt
END TYPE
TYPE holder
    v(2) AS INTEGER
    w(1) AS STRING
END TYPE
ON ERROR GOTO h
DIM i AS LONG, j AS LONG, k AS LONG
DIM sb(1 TO 3) AS _BYTE, db(1 TO 3) AS _BYTE
DIM sbu(1 TO 3) AS _UNSIGNED _BYTE, dub(1 TO 3) AS _UNSIGNED _BYTE
DIM si(1 TO 3) AS INTEGER, di(1 TO 3) AS INTEGER
DIM sui(1 TO 3) AS _UNSIGNED INTEGER, dui(1 TO 3) AS _UNSIGNED INTEGER
DIM sl(1 TO 3) AS LONG, dl(1 TO 3) AS LONG
DIM sul(1 TO 3) AS _UNSIGNED LONG, dul(1 TO 3) AS _UNSIGNED LONG
DIM s64(1 TO 3) AS _INTEGER64, d64(1 TO 3) AS _INTEGER64
DIM su64(1 TO 3) AS _UNSIGNED _INTEGER64, du64(1 TO 3) AS _UNSIGNED _INTEGER64
DIM ss(1 TO 3) AS SINGLE, ds(1 TO 3) AS SINGLE
DIM sd(1 TO 3) AS DOUBLE, dd(1 TO 3) AS DOUBLE
DIM sf(1 TO 3) AS _FLOAT, df(1 TO 3) AS _FLOAT
DIM so(1 TO 3) AS _OFFSET, dof(1 TO 3) AS _OFFSET
DIM sx(1 TO 3) AS _UNSIGNED _OFFSET, dx(1 TO 3) AS _UNSIGNED _OFFSET
FOR i = 1 TO 3
    sb(i) = -i: sbu(i) = 250 + i: si(i) = -1000 * i: sui(i) = 65530 + i: sl(i) = -100000 * i
    sul(i) = 4294967290 + i: s64(i) = -10000000000 * i: su64(i) = 1844674407370955161 + i
    ss(i) = i + 0.25: sd(i) = i + 0.125: sf(i) = i + 0.0625: so(i) = 1000 + i: dx(i) = 7: sx(i) = 2000 + i
    dl(i) = 99
NEXT
db() = sb(): dub() = sbu(): di() = si(): dui() = sui(): dl() = sl(): dul() = sul(): d64() = s64()
du64() = su64(): ds() = ss(): dd() = sd(): df() = sf(): dof() = so(): dx() = sx()
PRINT "1. every numeric type:"
FOR i = 1 TO 3
    PRINT db(i); dub(i); di(i); dui(i); dl(i); dul(i); d64(i); du64(i); ds(i); dd(i); df(i); dof(i); dx(i)
NEXT
PRINT "2. independent of its source:";
sl(2) = 5: PRINT dl(2); sl(2)
PRINT "3. two dimensions:"
DIM m(-1 TO 1, 2 TO 4) AS INTEGER, n(-1 TO 1, 2 TO 4) AS INTEGER
FOR i = -1 TO 1: FOR j = 2 TO 4: m(i, j) = 10 * i + j: NEXT: NEXT
n() = m()
FOR i = -1 TO 1: FOR j = 2 TO 4: PRINT n(i, j);: NEXT: PRINT: NEXT
PRINT "4. three dimensions:"
DIM c(1, 1, 1) AS DOUBLE, e(1, 1, 1) AS DOUBLE
FOR i = 0 TO 1: FOR j = 0 TO 1: FOR k = 0 TO 1: c(i, j, k) = 100 * i + 10 * j + k + 0.5: NEXT: NEXT: NEXT
e() = c()
FOR i = 0 TO 1: FOR j = 0 TO 1: FOR k = 0 TO 1: PRINT e(i, j, k);: NEXT: NEXT: NEXT: PRINT
PRINT "5. REDIM arrays, either side:";
REDIM r(2 TO 4) AS LONG, q(2 TO 4) AS LONG
DIM f(2 TO 4) AS LONG
FOR i = 2 TO 4: r(i) = i * 7: NEXT
q() = r(): f() = q()
FOR i = 2 TO 4: PRINT q(i); f(i);: NEXT: PRINT
PRINT "6. itself:";
r() = r()
FOR i = 2 TO 4: PRINT r(i);: NEXT: PRINT
PRINT "7. LET, parentheses, blanks, a one-line IF:";
DIM x(2), y(2)
x(2) = 2.5
LET y() = (x())
PRINT y(2);
x(1) = 1.5
y ( ) = x ( )
PRINT y(1);
x(0) = 3
IF i > 0 THEN y() = x() ELSE PRINT "no"
PRINT y(0)
PRINT "8. STRING:";
DIM s(2) AS STRING, d(2) AS STRING
s(0) = "zero": s(1) = "one": s(2) = "two": d(1) = "old"
d() = s()
s(1) = "changed"
PRINT " "; d(0); " "; d(1); " "; d(2); " "; s(1)
PRINT "9. STRING * 4:";
DIM sfx(2) AS STRING * 4, dfx(2) AS STRING * 4
sfx(0) = "A": sfx(1) = "BBBB": sfx(2) = "CCCCCC"
dfx() = sfx()
sfx(0) = "Z"
PRINT " ["; dfx(0); "]["; dfx(1); "]["; dfx(2); "]"
PRINT "10. _BIT:";
DIM sbit(0 TO 19) AS _BIT, dbit(0 TO 19) AS _BIT
FOR i = 0 TO 19: IF i MOD 3 = 0 THEN sbit(i) = -1
NEXT
dbit() = sbit()
FOR i = 0 TO 19: PRINT dbit(i);: NEXT: PRINT
PRINT "11. _UNSIGNED _BIT * 3:";
DIM sb3(0 TO 9) AS _UNSIGNED _BIT * 3, db3(0 TO 9) AS _UNSIGNED _BIT * 3
FOR i = 0 TO 9: sb3(i) = i MOD 8: NEXT
db3() = sb3()
FOR i = 0 TO 9: PRINT db3(i);: NEXT: PRINT
PRINT "12. TYPE of numbers:";
DIM sp(1 TO 2) AS pair, dp(1 TO 2) AS pair
sp(1).a = 1024: sp(2).b = 3.5
dp() = sp()
PRINT dp(1).a; dp(2).b
PRINT "13. TYPE with STRING and STRING * 3:";
DIM st(1) AS txt, dt(1) AS txt
st(0).n = 10: st(0).s = "first": st(0).f = "abc": st(1).s = "second"
dt() = st()
st(0).s = "changed"
PRINT dt(0).n; dt(0).s; " "; dt(0).f; " "; dt(1).s; " "; st(0).s
PRINT "14. nested TYPE:";
DIM sn(1) AS outer, dn(1) AS outer
sn(1).p.a = 7: sn(1).t.s = "inner": sn(1).t.f = "xyz"
dn() = sn()
sn(1).t.s = "gone"
PRINT dn(1).p.a; dn(1).t.s; " "; dn(1).t.f
PRINT "15. TYPE with member arrays:";
DIM sh(1) AS holder, dh(1) AS holder
sh(0).v(2) = 42: sh(1).w(1) = "deep"
dh() = sh()
sh(1).w(1) = "shallow"
PRINT dh(0).v(2); dh(1).w(1)
PRINT "16. _MEM:";
DIM sm(1) AS _MEM, dm(1) AS _MEM
DIM z AS LONG
sm(0) = _MEM(z)
dm() = sm()
PRINT dm(0).SIZE; dm(0).OFFSET = _OFFSET(z)
DIM a(1 TO 3) AS INTEGER, b(1 TO 3) AS INTEGER
FOR i = 1 TO 3: g(i) = i: a(i) = 10 * i: NEXT
gs(1) = "g1": gs(2) = "g2"
PRINT "17. SHARED into COMMON (conventional memory), from a SUB:";
FromShared
FOR i = 1 TO 3: PRINT cm(i);: NEXT: PRINT
PRINT "18. local and STATIC arrays of a SUB:";
Locals: Locals: PRINT
PRINT "19. array parameters, both sides:";
Params a(), b()
FOR i = 1 TO 3: PRINT b(i);: NEXT: PRINT
PRINT "20. a parameter and a SHARED array:";
ToParam b()
FOR i = 1 TO 3: PRINT b(i);: NEXT: PRINT
PRINT "21. a parameter of strings:";
DIM t(1 TO 2) AS STRING
StrParam t()
PRINT " "; t(1); " "; t(2)
PRINT "22. a parameter whose dimensions only the copy gives:";
OnlyCopy a()
PRINT "23. a SUB declared STATIC:";
StaticSub: StaticSub: PRINT
'$DYNAMIC
DIM dy(1 TO 3) AS INTEGER
'$STATIC
PRINT "24. $DYNAMIC:";
dy() = a()
FOR i = 1 TO 3: PRINT dy(i);: NEXT: PRINT
PRINT "25. used without DIM:";
u(4) = 44
DIM w(10) AS SINGLE
w() = u()
PRINT w(4); UBOUND(w)
PRINT "26. in a FUNCTION:"; Fn(2)
DIM ea(0 TO 5) AS LONG, eb(1 TO 6) AS LONG, ec(0 TO 4) AS LONG, ed(0 TO 6) AS LONG
DIM em(2, 3) AS LONG, en(2, 4) AS LONG
FOR i = 0 TO 5: ea(i) = i + 1: NEXT
eb(1) = 99: ec(0) = 98: ed(0) = 97
PRINT "27. same count, moved:";: eb() = ea(): PRINT eb(1)
PRINT "28. fewer:";: ec() = ea(): PRINT ec(0)
PRINT "29. more:";: ed() = ea(): PRINT ed(0)
PRINT "30. second dimension:";: en() = em(): PRINT " done"
REDIM er(0 TO 5) AS LONG
ERASE er
PRINT "31. target erased:";: er() = ea(): PRINT " done"
PRINT "32. source erased:";: ea() = er(): PRINT ea(0)
'$DYNAMIC
DIM nv(0 TO 5) AS LONG
'$STATIC
ERASE nv
PRINT "33. target never allocated again:";: nv() = ea(): PRINT " done"
DIM es(0 TO 5) AS LONG
ERASE es
PRINT "34. static after ERASE:";: es() = ea(): PRINT es(5)
DIM s1(2) AS STRING, s2(3) AS STRING
s2(0) = "keep"
PRINT "35. strings:";: s2() = s1(): PRINT " "; s2(0)
SYSTEM
h:
PRINT " error"; ERR;
RESUME NEXT

SUB FromShared
    cm() = g()
END SUB

SUB Locals
    STATIC nn AS INTEGER
    DIM l(2) AS LONG
    STATIC sl2(2) AS LONG
    nn = nn + 1
    l(1) = 100 + nn
    PRINT sl2(1);
    sl2() = l()
    PRINT sl2(1);
END SUB

SUB Params (p() AS INTEGER, pq() AS INTEGER)
    pq() = p()
END SUB

SUB ToParam (pq() AS INTEGER)
    g(2) = -2
    pq() = g()
END SUB

SUB StrParam (pq() AS STRING)
    pq() = gs()
END SUB

SUB OnlyCopy (p() AS INTEGER)
    DIM pq(1 TO 3) AS INTEGER
    pq(2) = 6
    p() = pq()
    PRINT p(2)
END SUB

SUB StaticSub STATIC
    DIM sa(2) AS LONG, sbb(2) AS LONG
    sa(1) = sa(1) + 1
    sbb() = sa()
    PRINT sbb(1);
END SUB

FUNCTION Fn (kk AS LONG)
    DIM fx(3) AS LONG, fy(3) AS LONG
    fx(kk) = kk * 3
    fy() = fx()
    Fn = fy(kk)
END FUNCTION
