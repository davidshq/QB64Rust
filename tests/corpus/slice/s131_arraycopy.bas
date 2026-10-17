$CONSOLE:ONLY
' Slice (m3-arraycopy): _ARRAYCOPY of arrays (verification\v41_a, v41_b, v41_c, v41_f, v41_g, v41_h): the six
' forms, overlap and an array copied to itself, every element type but _FLOAT, several dimensions with numbers and
' strings, the places of an array, the run-time errors (error 9, the destination unchanged), the order in which the
' bounds and starts are evaluated and their rounding. Left out, as the new compiler differs on purpose: _FLOAT
' arrays (DIVERGENCES.md D-025) and an index outside its bounds on the way to a member array (D-004), each a CLI
' test.
OPTION BASE 1
COMMON SHARED cs() AS LONG
DIM SHARED sh(5) AS LONG
DIM i AS LONG, j AS LONG, k AS LONG
DIM s(3 TO 8) AS LONG, d(-2 TO 12) AS LONG
FOR i = 3 TO 8: s(i) = i * 10: NEXT
PRINT "-- forms"
GOSUB clr: _ARRAYCOPY s() TO d(): GOSUB show
GOSUB clr: _ARRAYCOPY s(4 TO 6) TO d(-1): GOSUB show
GOSUB clr: _ARRAYCOPY s(4 TO 6) TO d(): GOSUB show
GOSUB clr: _ARRAYCOPY s() TO d(0): GOSUB show
GOSUB clr: _ARRAYCOPY s(5 TO 5) TO d(12): GOSUB show
GOSUB fill: _ARRAYCOPY s(3 TO 6) TO s(5): GOSUB shows
GOSUB fill: _ARRAYCOPY s(5 TO 8) TO s(3): GOSUB shows
GOSUB fill: _ARRAYCOPY s() TO s(): GOSUB shows
GOSUB fill: _arraycopy   s( 3  to  4 )   to   s( 7 ): GOSUB shows

PRINT "-- kinds"
DIM b1(1 TO 3) AS _BYTE, b2(0 TO 3) AS _BYTE
FOR i = 1 TO 3: b1(i) = -i: NEXT: _ARRAYCOPY b1(2 TO 3) TO b2(1)
FOR i = 0 TO 3: PRINT b2(i);: NEXT: PRINT
DIM ub1(1 TO 3) AS _UNSIGNED _BYTE, ub2(0 TO 3) AS _UNSIGNED _BYTE
FOR i = 1 TO 3: ub1(i) = 250 + i: NEXT: _ARRAYCOPY ub1(2 TO 3) TO ub2(1)
FOR i = 0 TO 3: PRINT ub2(i);: NEXT: PRINT
DIM n1(1 TO 3) AS INTEGER, n2(0 TO 3) AS INTEGER
FOR i = 1 TO 3: n1(i) = -1000 * i: NEXT: _ARRAYCOPY n1(2 TO 3) TO n2(1)
FOR i = 0 TO 3: PRINT n2(i);: NEXT: PRINT
DIM un1(1 TO 3) AS _UNSIGNED INTEGER, un2(0 TO 3) AS _UNSIGNED INTEGER
FOR i = 1 TO 3: un1(i) = 65530 + i: NEXT: _ARRAYCOPY un1(2 TO 3) TO un2(1)
FOR i = 0 TO 3: PRINT un2(i);: NEXT: PRINT
DIM ul1(1 TO 3) AS _UNSIGNED LONG, ul2(0 TO 3) AS _UNSIGNED LONG
FOR i = 1 TO 3: ul1(i) = 4294967290 + i: NEXT: _ARRAYCOPY ul1(2 TO 3) TO ul2(1)
FOR i = 0 TO 3: PRINT ul2(i);: NEXT: PRINT
DIM q1(1 TO 3) AS _INTEGER64, q2(0 TO 3) AS _INTEGER64
FOR i = 1 TO 3: q1(i) = -9000000000000 * i: NEXT: _ARRAYCOPY q1(2 TO 3) TO q2(1)
FOR i = 0 TO 3: PRINT q2(i);: NEXT: PRINT
DIM uq1(1 TO 3) AS _UNSIGNED _INTEGER64, uq2(0 TO 3) AS _UNSIGNED _INTEGER64
FOR i = 1 TO 3: uq1(i) = 18446744073709551610~&& + i: NEXT: _ARRAYCOPY uq1(2 TO 3) TO uq2(1)
FOR i = 0 TO 3: PRINT uq2(i);: NEXT: PRINT
DIM f1(1 TO 3) AS SINGLE, f2(0 TO 3) AS SINGLE
FOR i = 1 TO 3: f1(i) = i / 3: NEXT: _ARRAYCOPY f1(2 TO 3) TO f2(1)
FOR i = 0 TO 3: PRINT f2(i);: NEXT: PRINT
DIM d1(1 TO 3) AS DOUBLE, d2(0 TO 3) AS DOUBLE
FOR i = 1 TO 3: d1(i) = i / 3: NEXT: _ARRAYCOPY d1(2 TO 3) TO d2(1)
FOR i = 0 TO 3: PRINT d2(i);: NEXT: PRINT
DIM o1(1 TO 3) AS _OFFSET, o2(0 TO 3) AS _OFFSET
FOR i = 1 TO 3: o1(i) = i * 7: NEXT: _ARRAYCOPY o1(2 TO 3) TO o2(1)
FOR i = 0 TO 3: PRINT o2(i);: NEXT: PRINT
DIM uo1(1 TO 3) AS _UNSIGNED _OFFSET, uo2(0 TO 3) AS _UNSIGNED _OFFSET
FOR i = 1 TO 3: uo1(i) = i * 9: NEXT: _ARRAYCOPY uo1() TO uo2()
FOR i = 0 TO 3: PRINT uo2(i);: NEXT: PRINT
DIM fs1(1 TO 3) AS STRING * 3, fs2(0 TO 3) AS STRING * 3
FOR i = 1 TO 3: fs1(i) = CHR$(64 + i) + "x": NEXT: _ARRAYCOPY fs1(2 TO 3) TO fs2(1)
FOR i = 0 TO 3: PRINT "["; fs2(i); "]";: NEXT: PRINT
DIM vs1(1 TO 3) AS STRING, vs2(0 TO 3) AS STRING
FOR i = 1 TO 3: vs1(i) = STRING$(i, 96 + i): NEXT: vs2(0) = "keep": _ARRAYCOPY vs1(2 TO 3) TO vs2(1)
FOR i = 0 TO 3: PRINT "["; vs2(i); "]";: NEXT: PRINT
vs1(2) = "changed": PRINT "["; vs2(1); "]"
FOR i = 1 TO 3: vs1(i) = STRING$(i, 65): NEXT: _ARRAYCOPY vs1(1 TO 2) TO vs1(2)
FOR i = 1 TO 3: PRINT "["; vs1(i); "]";: NEXT: PRINT

PRINT "-- dimensions"
DIM m(1 TO 2, 0 TO 2) AS LONG, n(1 TO 2, 0 TO 2) AS LONG, big(0 TO 4, 0 TO 5) AS LONG
FOR i = 1 TO 2: FOR j = 0 TO 2: m(i, j) = i * 10 + j: NEXT: NEXT
_ARRAYCOPY m() TO n()
FOR i = 1 TO 2: FOR j = 0 TO 2: PRINT n(i, j);: NEXT: NEXT: PRINT
GOSUB bclr: _ARRAYCOPY m() TO big(1, 2): GOSUB bshow
FOR i = 0 TO 4: FOR j = 0 TO 5: big(i, j) = i * 10 + j: NEXT: NEXT
DIM small(0 TO 2, 0 TO 3) AS LONG, other(0 TO 4, 0 TO 5) AS LONG
FOR i = 0 TO 2: FOR j = 0 TO 3: small(i, j) = -1: NEXT: NEXT
_ARRAYCOPY big(1 TO 2, 2 TO 4) TO small(1, 0)
FOR i = 0 TO 2: FOR j = 0 TO 3: PRINT small(i, j);: NEXT: PRINT: NEXT
_ARRAYCOPY big(1 TO 2, 2 TO 4) TO other()
FOR i = 0 TO 4: FOR j = 0 TO 5: PRINT other(i, j);: NEXT: PRINT: NEXT
DIM c3(1 TO 2, 1 TO 2, 1 TO 2) AS INTEGER, e3(1 TO 2, 1 TO 2, 1 TO 2) AS INTEGER
FOR i = 1 TO 2: FOR j = 1 TO 2: FOR k = 1 TO 2: c3(i, j, k) = i * 100 + j * 10 + k: NEXT: NEXT: NEXT
_ARRAYCOPY c3(2 TO 2, 1 TO 2, 2 TO 2) TO e3(1, 1, 1)
FOR i = 1 TO 2: FOR j = 1 TO 2: FOR k = 1 TO 2: PRINT e3(i, j, k);: NEXT: NEXT: NEXT: PRINT
DIM sm(1 TO 2, 1 TO 3) AS STRING, sb(0 TO 3, 0 TO 4) AS STRING
FOR i = 1 TO 2: FOR j = 1 TO 3: sm(i, j) = CHR$(64 + i) + CHR$(48 + j): NEXT: NEXT
GOSUB sclr: _ARRAYCOPY sm() TO sb(1, 1): GOSUB sshow
GOSUB sclr: _ARRAYCOPY sm(1 TO 2, 2 TO 3) TO sb(0, 0): GOSUB sshow
GOSUB sclr: _ARRAYCOPY sm(2 TO 2, 1 TO 3) TO sb(): GOSUB sshow
DIM fx(1 TO 2, 1 TO 2) AS STRING * 2, fy(1 TO 2, 1 TO 2) AS STRING * 2
fx(1, 1) = "aa": fx(2, 1) = "bb": fx(1, 2) = "cc": fx(2, 2) = "dd"
_ARRAYCOPY fx() TO fy(): PRINT fy(1, 1); fy(2, 1); fy(1, 2); fy(2, 2)

PRINT "-- places"
REDIM cs(5) AS LONG
REDIM r(3) AS LONG
FOR i = 1 TO 3: r(i) = i: NEXT
_ARRAYCOPY r() TO sh(2)
FOR i = 1 TO 5: PRINT sh(i);: NEXT: PRINT
REDIM _PRESERVE r(6) AS LONG
_ARRAYCOPY sh() TO r(2)
FOR i = 1 TO 6: PRINT r(i);: NEXT: PRINT
'$DYNAMIC
DIM dz(4) AS LONG
'$STATIC
dz(2) = 22: _ARRAYCOPY dz() TO cs()
FOR i = 1 TO 5: PRINT cs(i);: NEXT: PRINT
inside
FOR i = 1 TO 5: PRINT sh(i);: NEXT: PRINT
DIM p(1 TO 3) AS LONG, q(1 TO 5) AS LONG, p2(1 TO 2, 1 TO 2) AS LONG, r2(1 TO 2, 1 TO 2) AS LONG
p(1) = 7: p(2) = 8: p(3) = 9
param p(), q()
FOR i = 1 TO 5: PRINT q(i);: NEXT: PRINT
p2(1, 1) = 1: p2(2, 1) = 2: p2(1, 2) = 3: p2(2, 2) = 4
param2 p2(), r2()
PRINT r2(1, 1); r2(2, 1); r2(1, 2); r2(2, 2)
u(1) = 5: w(1) = 6
_ARRAYCOPY u() TO w(): PRINT w(1); w(10)

PRINT "-- errors"
ON ERROR GOTO h
DIM a(1 TO 5) AS LONG, e(0 TO 5) AS LONG, z5(2 TO 4) AS LONG
FOR i = 1 TO 5: a(i) = i: NEXT
GOSUB eclr: _ARRAYCOPY a(3 TO 2) TO e(1): GOSUB eshow
GOSUB eclr: _ARRAYCOPY a(0 TO 2) TO e(1): GOSUB eshow
GOSUB eclr: _ARRAYCOPY a(4 TO 6) TO e(1): GOSUB eshow
GOSUB eclr: _ARRAYCOPY a(1 TO 2) TO e(-1): GOSUB eshow
GOSUB eclr: _ARRAYCOPY a(1 TO 2) TO e(6): GOSUB eshow
GOSUB eclr: _ARRAYCOPY a(1 TO 3) TO e(4): GOSUB eshow
GOSUB eclr: _ARRAYCOPY a() TO e(2): GOSUB eshow
GOSUB eclr: _ARRAYCOPY a() TO e(): GOSUB eshow
_ARRAYCOPY a() TO z5(): PRINT z5(2); z5(3); z5(4)
_ARRAYCOPY a(1 TO 2) TO z5(): PRINT z5(2); z5(3); z5(4)
REDIM zz(3) AS LONG
ERASE zz
GOSUB eclr: _ARRAYCOPY zz() TO e(): GOSUB eshow
_ARRAYCOPY a() TO zz(): PRINT " erased destination"
'$DYNAMIC
DIM nv(1 TO 5) AS LONG
'$STATIC
ERASE nv
_ARRAYCOPY a(1 TO 2) TO nv(1): PRINT " erased $DYNAMIC"
DIM st(1 TO 5) AS LONG
FOR i = 1 TO 5: st(i) = 9: NEXT
ERASE st
_ARRAYCOPY a() TO st(): PRINT st(1); st(5)
DIM mm(1 TO 2, 1 TO 3) AS LONG, nn(1 TO 3, 1 TO 2) AS LONG, kk(0 TO 2, 1 TO 3) AS LONG
_ARRAYCOPY mm() TO nn(): PRINT " other shape"
_ARRAYCOPY mm() TO kk(): PRINT " other lower"
_ARRAYCOPY mm() TO nn(2, 1): PRINT " does not fit"
_ARRAYCOPY mm(1 TO 3, 1 TO 1) TO nn(1, 1): PRINT " range outside"
_ARRAYCOPY mm(1 TO 2, 2 TO 3) TO nn(): PRINT " range to whole outside"
DIM t1(1 TO 3) AS STRING, t2(1 TO 2) AS STRING
t1(1) = "a": t1(2) = "b": t1(3) = "c": t2(1) = "keep"
_ARRAYCOPY t1() TO t2(1): PRINT " "; t2(1)
_ARRAYCOPY t1(2 TO 1) TO t2(1): PRINT " "; t2(1)
_ARRAYCOPY a(1 TO 9223372036854775807) TO e(0): PRINT " huge"
GOSUB eclr: _ARRAYCOPY a(1 TO bad&) TO e(0): GOSUB eshow

PRINT "-- order"
DIM g(0 TO 9) AS LONG, gd(0 TO 9) AS LONG, gm(0 TO 3, 0 TO 3) AS LONG, gn(0 TO 3, 0 TO 3) AS LONG
FOR i = 0 TO 9: g(i) = i: NEXT
_ARRAYCOPY g(f(2) TO f(4)) TO gd(f(6)): PRINT
FOR i = 0 TO 9: PRINT gd(i);: NEXT: PRINT
_ARRAYCOPY gm(f(1) TO f(2), f(3) TO f(3)) TO gn(f(0), f(1)): PRINT
GOSUB gclr: _ARRAYCOPY g(1.5 TO 3.5) TO gd(0.5): GOSUB gshow
GOSUB gclr: _ARRAYCOPY g(2.4 TO 2.6) TO gd(4.49): GOSUB gshow
DIM b8 AS _BYTE, u64 AS _UNSIGNED _INTEGER64, sg AS SINGLE
b8 = 2: u64 = 3: sg = 4.5
GOSUB gclr: _ARRAYCOPY g(b8 TO u64) TO gd(sg): GOSUB gshow
GOSUB gclr: _ARRAYCOPY g(-1 + 2 TO 2 ^ 2) TO gd(): GOSUB gshow
GOSUB gclr: _ARRAYCOPY g(1 TO 2) TO gd(bad&): GOSUB gshow
SYSTEM

clr:
FOR i = -2 TO 12: d(i) = -1: NEXT
RETURN
show:
FOR i = -2 TO 12: PRINT d(i);: NEXT: PRINT
RETURN
fill:
FOR i = 3 TO 8: s(i) = i * 10: NEXT
RETURN
shows:
FOR i = 3 TO 8: PRINT s(i);: NEXT: PRINT
RETURN
bclr:
FOR i = 0 TO 4: FOR j = 0 TO 5: big(i, j) = -1: NEXT: NEXT
RETURN
bshow:
FOR i = 0 TO 4: FOR j = 0 TO 5: PRINT big(i, j);: NEXT: PRINT: NEXT
RETURN
sclr:
FOR i = 0 TO 3: FOR j = 0 TO 4: sb(i, j) = "..": NEXT: NEXT
RETURN
sshow:
FOR i = 0 TO 3: FOR j = 0 TO 4: PRINT " "; sb(i, j);: NEXT: PRINT: NEXT
RETURN
eclr:
FOR i = 0 TO 5: e(i) = -1: NEXT
RETURN
eshow:
FOR i = 0 TO 5: PRINT e(i);: NEXT: PRINT
RETURN
gclr:
FOR i = 0 TO 9: gd(i) = -1: NEXT
RETURN
gshow:
FOR i = 0 TO 9: PRINT gd(i);: NEXT: PRINT
RETURN
h:
PRINT " error"; ERR;
RESUME NEXT

SUB inside
    DIM lc(3) AS LONG
    STATIC st(3) AS LONG
    lc(1) = 31: lc(2) = 32: lc(3) = 33
    _ARRAYCOPY lc() TO st()
    _ARRAYCOPY st(1 TO 2) TO sh(4)
END SUB

SUB param (x() AS LONG, y() AS LONG)
    _ARRAYCOPY x() TO y(LBOUND(y) + 1)
END SUB

SUB param2 (x() AS LONG, y() AS LONG)
    _ARRAYCOPY x(1 TO 2, 1 TO 2) TO y(1, 1)
END SUB

FUNCTION f& (v AS LONG)
    PRINT " f"; LTRIM$(STR$(v));
    f& = v
END FUNCTION

FUNCTION bad&
    ERROR 5
    bad& = 2
END FUNCTION
