$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Slice (m3-typefields tasks 4.2 to 4.4): _DYNAMIC member arrays of every kind of element, whole-value copies and
' the statements that read into a place through a member (verification\v35_c, v35_x56, x57, x61, x62).
' 1.-10.: each numeric type, STRING * n, STRING, user types without and with a string and with a _DYNAMIC member
' of their own; _RETAIN, _PRESERVE, plain REDIM and ERASE of them, and a copy of a type that holds them all.
' 11.-16.: how often a copy finds its places (an index FUNCTION that prints), for a type without and with a
' STRING, for owner types nested as a member and as an inline member array; an index outside; what a copy does
' when the source's member is not allocated (a type without a STRING keeps the target's member, one with a STRING
' leaves it unallocated); a copy to itself.
' 17.-48.: READ, INPUT # and LINE INPUT # from a file, console INPUT and LINE INPUT, and the MID$ statement into a
' member of an element and into an element of a member array (fixed and _DYNAMIC): a number is read before the
' indexes, a string after them; an index outside raises error 9. Where the new compiler differs on purpose
' (QB64PE-DIVERGENCES.md D-004: a number read through an index outside is stored nowhere; the old compiler stores it into
' position 0) items 23, 38 and 39 were recorded from a program that reads the item into q$ and raises ERROR 9.
TYPE inner
    k AS INTEGER
    f AS STRING * 2
END TYPE
TYPE innerstr
    k AS INTEGER
    s AS STRING
END TYPE
TYPE innerdyn
    k AS INTEGER
    d(1) _DYNAMIC AS INTEGER
END TYPE
TYPE kinds
    b(1) _DYNAMIC AS _BYTE
    ub(1) _DYNAMIC AS _UNSIGNED _BYTE
    i(1) _DYNAMIC AS INTEGER
    l(1) _DYNAMIC AS LONG
    q(1) _DYNAMIC AS _INTEGER64
    uq(1) _DYNAMIC AS _UNSIGNED _INTEGER64
    s(1) _DYNAMIC AS SINGLE
    d(1) _DYNAMIC AS DOUBLE
    f(1) _DYNAMIC AS _FLOAT
    o(1) _DYNAMIC AS _OFFSET
    fs(1) _DYNAMIC AS STRING * 3
    vs(1) _DYNAMIC AS STRING
    u(1) _DYNAMIC AS inner
    us(1) _DYNAMIC AS innerstr
    ud(1) _DYNAMIC AS innerdyn
END TYPE
TYPE cflat
    n AS LONG
    v(1) _DYNAMIC AS LONG
    m AS LONG
END TYPE
TYPE cstr
    n AS LONG
    s AS STRING
    v(1) _DYNAMIC AS LONG
    w(1) _DYNAMIC AS STRING
END TYPE
TYPE nin
    k AS LONG
    d(1) _DYNAMIC AS LONG
END TYPE
TYPE nout
    a AS LONG
    i AS nin
    r(1) AS nin
END TYPE
TYPE nins
    k AS LONG
    s AS STRING
    d(1) _DYNAMIC AS LONG
END TYPE
TYPE nouts
    a AS LONG
    i AS nins
    r(1) AS nins
    t(1) AS STRING
END TYPE
TYPE flat
    d(1) _DYNAMIC AS LONG
END TYPE
TYPE owner
    s AS STRING
    d(1) _DYNAMIC AS LONG
END TYPE
TYPE tg
    m AS LONG
    s AS STRING
    v(1) AS LONG
    w(1) AS STRING
    d(1) _DYNAMIC AS LONG
    e(1) _DYNAMIC AS STRING
END TYPE
DIM SHARED q$, z$
DIM SHARED tgt(2) AS tg
ON ERROR GOTO h
DIM x AS kinds, y AS kinds
x.b(1) = -5: x.ub(1) = 250: x.i(1) = -30000: x.l(1) = 100000: x.q(1) = 2 ^ 40: x.uq(1) = 18446744073709551614~&&
x.s(1) = 1.5: x.d(1) = 2.25: x.f(1) = 3.125: x.o(1) = 77
x.fs(0) = "abc": x.fs(1) = "xy"
x.vs(0) = "first": x.vs(1) = "second string"
x.u(1).k = 9: x.u(1).f = "zz"
x.us(1).k = 8: x.us(1).s = "inner string"
x.ud(1).k = 7: x.ud(1).d(1) = 70
REDIM x.ud(1).d(2 TO 4) AS INTEGER: x.ud(1).d(4) = 44
PRINT "1."; x.b(1); x.ub(1); x.i(1); x.l(1); x.q(1); x.uq(1); x.s(1); x.d(1); x.f(1); x.o(1)
PRINT "2. ["; x.fs(0); "] ["; x.fs(1); "] ["; x.vs(0); "] ["; x.vs(1); "]"
PRINT "3."; x.u(1).k; x.u(1).f; x.us(1).k; x.us(1).s; x.ud(1).k; LBOUND(x.ud(1).d); UBOUND(x.ud(1).d); x.ud(1).d(4)
y = x
x.vs(1) = "changed": x.us(1).s = "changed": x.ud(1).d(4) = -1: x.fs(0) = "QQQ"
PRINT "4. copy: ["; y.vs(1); "] ["; y.us(1).s; "]"; y.ud(1).d(4); "["; y.fs(0); "]"
REDIM _RETAIN x.vs(1 TO 3) AS STRING
PRINT "5. _RETAIN vs(1 TO 3): ["; x.vs(1); "] ["; x.vs(2); "] ["; x.vs(3); "]"
REDIM _RETAIN x.fs(-1 TO 1) AS STRING * 3
PRINT "6. _RETAIN fs(-1 TO 1): ["; x.fs(-1); "] ["; x.fs(0); "] ["; x.fs(1); "]"
REDIM _PRESERVE x.us(0 TO 2) AS innerstr
PRINT "7. _PRESERVE us(0 TO 2):"; x.us(1).k; "["; x.us(1).s; "]"; x.us(2).k; "["; x.us(2).s; "]"
REDIM _RETAIN x.ud(1 TO 2) AS innerdyn
PRINT "8. _RETAIN ud(1 TO 2):"; x.ud(1).k; LBOUND(x.ud(1).d); UBOUND(x.ud(1).d); x.ud(1).d(4); LBOUND(x.ud(2).d); UBOUND(x.ud(2).d)
REDIM x.vs(0 TO 1) AS STRING
PRINT "9. plain REDIM vs: ["; x.vs(0); "] ["; x.vs(1); "]"; LEN(x.vs(1))
ERASE x.us
PRINT "10. ERASE us, then y.us(1).s: ["; y.us(1).s; "]"
REDIM ca(2) AS cflat, cb(2) AS cflat, cc(2) AS cstr, cd(2) AS cstr
cb(1).v(1) = 5: cd(1).s = "s": cd(1).w(1) = "w"
PRINT "11.";
ca(one(1)) = cb(one(2))
PRINT
PRINT "12.";
cc(one(1)) = cd(one(2))
PRINT
PRINT "   "; ca(1).v(1); cc(1).s; cc(1).w(1)
DIM cp AS cflat
REDIM cp.v(4 TO 5) AS LONG: cp.v(5) = 55: cp.n = 6
ca(9) = cp
PRINT "13."; ca(0).n; LBOUND(ca(0).v); UBOUND(ca(0).v)
cp = ca(9)
PRINT "   "; cp.n
REDIM na(2) AS nout, nb(2) AS nout, nc(2) AS nouts, ne(2) AS nouts
nb(1).a = 1: nb(1).i.k = 2: nb(1).i.d(1) = 3: nb(1).r(1).k = 4: nb(1).r(1).d(0) = 5
ne(1).a = 1: ne(1).i.s = "s": ne(1).i.d(1) = 3: ne(1).r(1).s = "r": ne(1).r(1).d(0) = 5: ne(1).t(1) = "t"
PRINT "14.";
na(one(1)) = nb(one(2))
PRINT
PRINT "   "; na(1).a; na(1).i.k; na(1).i.d(1); na(1).r(1).k; na(1).r(1).d(0)
nb(1).i.d(1) = 30: nb(1).r(1).d(0) = 50
PRINT "   after a change of nb(1):"; na(1).i.d(1); na(1).r(1).d(0)
PRINT "15.";
nc(one(1)) = ne(one(2))
PRINT
PRINT "   "; nc(1).a; " "; nc(1).i.s; nc(1).i.d(1); " "; nc(1).r(1).s; nc(1).r(1).d(0); " "; nc(1).t(1)
DIM ff AS flat, fg AS flat, oo AS owner, op AS owner
ff.d(1) = 7: oo.d(1) = 7
ERASE fg.d, op.d
ff = fg
PRINT "16. flat, source erased:";
PRINT LBOUND(ff.d); UBOUND(ff.d);
PRINT ff.d(1)
oo = op
PRINT "    owner, source erased:";
PRINT LBOUND(oo.d); UBOUND(oo.d);
PRINT oo.d(1)
REDIM op.d(2 TO 3) AS LONG: op.d(3) = 9
oo = op
PRINT "    owner, source redimmed:"; LBOUND(oo.d); UBOUND(oo.d); oo.d(3)
oo = oo
PRINT "    copied to itself:"; LBOUND(oo.d); UBOUND(oo.d); oo.d(3)
READ tgt(rd(1)).m: PRINT "17."; tgt(1).m
READ tgt(rd(1)).s: PRINT "18. "; tgt(1).s
READ tgt(1).v(rd(1)): PRINT "19."; tgt(1).v(1)
READ tgt(1).w(rd(1)): PRINT "20. "; tgt(1).w(1)
READ tgt(1).d(rd(1)): PRINT "21."; tgt(1).d(1)
READ tgt(1).e(rd(1)): PRINT "22. "; tgt(1).e(1)
READ tgt(9).m: PRINT "23."; tgt(0).m
READ tgt(9).s: PRINT "24. "; tgt(0).s
READ tgt(1).v(9): PRINT "25."; tgt(1).v(0)
READ tgt(1).d(9): PRINT "26."; tgt(1).d(0)
READ tgt(1).e(9): PRINT "27. "; tgt(1).e(0)
READ q$: PRINT "28. next item: "; q$
OPEN "s117.tmp" FOR OUTPUT AS #1
FOR i = 1 TO 20: PRINT #1, "L"; LTRIM$(STR$(100 + i)): NEXT
CLOSE #1
OPEN "s117.tmp" FOR INPUT AS #1
INPUT #1, tgt(rf(1)).s: PRINT "29. "; tgt(1).s
INPUT #1, tgt(1).e(rf(1)): PRINT "30. "; tgt(1).e(1)
LINE INPUT #1, tgt(rf(1)).s: PRINT "31. "; tgt(1).s
LINE INPUT #1, tgt(1).w(rf(1)): PRINT "32. "; tgt(1).w(1)
INPUT #1, tgt(9).s: PRINT "33. "; tgt(0).s
LINE INPUT #1, tgt(1).e(9): PRINT "34. "; tgt(1).e(0)
INPUT #1, q$: PRINT "35. next line: "; q$
CLOSE #1
OPEN "s117.tmp" FOR OUTPUT AS #1
FOR i = 1 TO 20: PRINT #1, 200 + i: NEXT
CLOSE #1
OPEN "s117.tmp" FOR INPUT AS #1
INPUT #1, tgt(rf(1)).m: PRINT "36."; tgt(1).m
INPUT #1, tgt(1).d(rf(1)): PRINT "37."; tgt(1).d(1)
INPUT #1, tgt(9).m: PRINT "38."; tgt(0).m
INPUT #1, tgt(1).v(9): PRINT "39."; tgt(1).v(0)
INPUT #1, q$: PRINT "40. next line: "; q$
CLOSE #1
KILL "s117.tmp"
INPUT tgt(ci(1)).m, tgt(1).d(ci(1)): PRINT "41."; tgt(1).m; tgt(1).d(1)
LINE INPUT tgt(1).e(ci(1)): PRINT "42. "; tgt(1).e(1)
INPUT tgt(9).m: PRINT "43."; tgt(0).m
LINE INPUT tgt(1).e(9): PRINT "44. "; tgt(1).e(0)
tgt(1).s = "abcdef": tgt(1).e(1) = "uvwxyz": tgt(0).s = "000000": tgt(1).w(0) = "nnnnnn"
z$ = "A": MID$(tgt(mf(1)).s, 1, 1) = z$: PRINT "45. "; tgt(1).s
z$ = "A": MID$(tgt(1).e(mf(1)), 2) = z$: PRINT "46. "; tgt(1).e(1)
MID$(tgt(9).s, 3, 1) = "Z": PRINT "47. "; tgt(0).s
MID$(tgt(1).w(9), 4, 1) = "Z": PRINT "48. "; tgt(1).w(0)
SYSTEM
h:
PRINT "[error"; ERR; "]";
RESUME NEXT
DATA 10,11,x,y,20,21,p,q,30,31,r,s,40,u,50,v,60,w,70,z,80,y,90
FUNCTION one& (k AS LONG)
    PRINT k;
    one& = 1
END FUNCTION
FUNCTION rd (k AS LONG)
    READ q$
    PRINT "[rd "; q$; "]";
    rd = k
END FUNCTION
FUNCTION rf (k AS LONG)
    INPUT #1, q$
    PRINT "[rf "; q$; "]";
    rf = k
END FUNCTION
FUNCTION ci (k AS LONG)
    PRINT "[ci]";
    ci = k
END FUNCTION
FUNCTION mf (k AS LONG)
    z$ = "B"
    PRINT "[mf]";
    mf = k
END FUNCTION
