$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): a() = b() of the other kinds of element: STRING (a
' copy of each string), STRING * n, _BIT and _UNSIGNED _BIT * n, a TYPE of numbers, a TYPE with a STRING and a
' STRING * n, a nested TYPE, a TYPE with member arrays, _MEM.
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
DIM i AS LONG
PRINT "1. STRING:";
DIM s(2) AS STRING, d(2) AS STRING
s(0) = "zero": s(1) = "one": s(2) = "two": d(1) = "old"
d() = s()
s(1) = "changed"
PRINT " "; d(0); " "; d(1); " "; d(2); " "; s(1)
PRINT "2. STRING * 4:";
DIM sf(2) AS STRING * 4, df(2) AS STRING * 4
sf(0) = "A": sf(1) = "BBBB": sf(2) = "CCCCCC"
df() = sf()
sf(0) = "Z"
PRINT " ["; df(0); "]["; df(1); "]["; df(2); "]"
PRINT "3. _BIT:";
DIM sbit(0 TO 19) AS _BIT, dbit(0 TO 19) AS _BIT
FOR i = 0 TO 19: IF i MOD 3 = 0 THEN sbit(i) = -1
NEXT
dbit() = sbit()
FOR i = 0 TO 19: PRINT dbit(i);: NEXT: PRINT
PRINT "4. _UNSIGNED _BIT * 3:";
DIM sb3(0 TO 9) AS _UNSIGNED _BIT * 3, db3(0 TO 9) AS _UNSIGNED _BIT * 3
FOR i = 0 TO 9: sb3(i) = i MOD 8: NEXT
db3() = sb3()
FOR i = 0 TO 9: PRINT db3(i);: NEXT: PRINT
PRINT "5. TYPE of numbers:";
DIM sp(1 TO 2) AS pair, dp(1 TO 2) AS pair
sp(1).a = 1024: sp(2).b = 3.5
dp() = sp()
PRINT dp(1).a; dp(2).b
PRINT "6. TYPE with STRING and STRING * 3:";
DIM st(1) AS txt, dt(1) AS txt
st(0).n = 10: st(0).s = "first": st(0).f = "abc": st(1).s = "second"
dt() = st()
st(0).s = "changed"
PRINT dt(0).n; dt(0).s; " "; dt(0).f; " "; dt(1).s; " "; st(0).s
PRINT "7. nested TYPE:";
DIM so(1) AS outer, dout(1) AS outer
so(1).p.a = 7: so(1).t.s = "inner": so(1).t.f = "xyz"
dout() = so()
so(1).t.s = "gone"
PRINT dout(1).p.a; dout(1).t.s; " "; dout(1).t.f
PRINT "8. TYPE with member arrays:";
DIM sh(1) AS holder, dh(1) AS holder
sh(0).v(2) = 42: sh(1).w(1) = "deep"
dh() = sh()
sh(1).w(1) = "shallow"
PRINT dh(0).v(2); dh(1).w(1)
PRINT "9. _MEM:";
DIM sm(1) AS _MEM, dm(1) AS _MEM
DIM z AS LONG
sm(0) = _MEM(z)
dm() = sm()
PRINT dm(0).SIZE; dm(0).OFFSET = _OFFSET(z)
SYSTEM
