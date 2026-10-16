$CONSOLE:ONLY
' Verification (m3-mem, task 1.1): the members of the block of each kind of place. Each line prints the name of
' the place, then OFFSET less the address of the variable (or of the array's first element), SIZE, TYPE,
' ELEMENTSIZE, IMAGE and SOUND.
TYPE t
    a AS INTEGER
    s AS STRING * 5
    v(0 TO 3) AS LONG
    d AS DOUBLE
END TYPE
DIM SHARED g AS LONG, ga(5) AS DOUBLE
DIM a AS LONG, m AS _MEM, m2 AS _MEM, z(3) AS _MEM
DIM f AS STRING * 6, u AS t, b(10) AS INTEGER, ut(3) AS t, fs(4) AS STRING * 3, b2(3, 4) AS LONG
DIM s1 AS SINGLE, d1 AS DOUBLE, i64 AS _INTEGER64, by AS _BYTE, ub AS _UNSIGNED _BYTE, o AS _OFFSET
DIM uo AS _UNSIGNED _OFFSET, fl AS _FLOAT, ul AS _UNSIGNED LONG, i AS INTEGER, ui AS _UNSIGNED INTEGER
DIM ui64 AS _UNSIGNED _INTEGER64
REDIM dy(5) AS LONG
m = _MEM(a): show "a", m, _OFFSET(a)
m = _MEM(f): show "f", m, _OFFSET(f)
m = _MEM(u): show "u", m, _OFFSET(u)
m = _MEM(u.a): show "u.a", m, _OFFSET(u)
m = _MEM(u.s): show "u.s", m, _OFFSET(u)
m = _MEM(u.d): show "u.d", m, _OFFSET(u)
m = _MEM(u.v(2)): show "u.v(2)", m, _OFFSET(u)
m = _MEM(u.v()): show "u.v()", m, _OFFSET(u)
m = _MEM(b()): show "b()", m, _OFFSET(b())
m = _MEM(b(4)): show "b(4)", m, _OFFSET(b())
m = _MEM(ut()): show "ut()", m, _OFFSET(ut())
m = _MEM(ut(1)): show "ut(1)", m, _OFFSET(ut())
m = _MEM(ut(1).d): show "ut(1).d", m, _OFFSET(ut())
m = _MEM(ut(1).v()): show "ut(1).v()", m, _OFFSET(ut())
m = _MEM(ut(1).v(2)): show "ut(1).v(2)", m, _OFFSET(ut())
m = _MEM(fs()): show "fs()", m, _OFFSET(fs())
m = _MEM(fs(2)): show "fs(2)", m, _OFFSET(fs())
m = _MEM(b2()): show "b2()", m, _OFFSET(b2())
m = _MEM(b2(1, 2)): show "b2(1, 2)", m, _OFFSET(b2())
m = _MEM(dy()): show "dy()", m, _OFFSET(dy())
m = _MEM(dy(2)): show "dy(2)", m, _OFFSET(dy())
m = _MEM(m2): show "m2", m, _OFFSET(m2)
m = _MEM(z(1)): show "z(1)", m, _OFFSET(z())
m = _MEM(s1): show "s1", m, _OFFSET(s1)
m = _MEM(d1): show "d1", m, _OFFSET(d1)
m = _MEM(i64): show "i64", m, _OFFSET(i64)
m = _MEM(by): show "by", m, _OFFSET(by)
m = _MEM(ub): show "ub", m, _OFFSET(ub)
m = _MEM(o): show "o", m, _OFFSET(o)
m = _MEM(uo): show "uo", m, _OFFSET(uo)
m = _MEM(fl): show "fl", m, _OFFSET(fl)
m = _MEM(ul): show "ul", m, _OFFSET(ul)
m = _MEM(i): show "i", m, _OFFSET(i)
m = _MEM(ui): show "ui", m, _OFFSET(ui)
m = _MEM(ui64): show "ui64", m, _OFFSET(ui64)
PRINT "_MEMELEMENT"
m = _MEMELEMENT(a): show "a", m, _OFFSET(a)
m = _MEMELEMENT(f): show "f", m, _OFFSET(f)
m = _MEMELEMENT(u): show "u", m, _OFFSET(u)
m = _MEMELEMENT(u.a): show "u.a", m, _OFFSET(u)
m = _MEMELEMENT(u.d): show "u.d", m, _OFFSET(u)
m = _MEMELEMENT(u.v(1)): show "u.v(1)", m, _OFFSET(u)
m = _MEMELEMENT(b()): show "b()", m, _OFFSET(b())
m = _MEMELEMENT(b(4)): show "b(4)", m, _OFFSET(b())
m = _MEMELEMENT(ut(1)): show "ut(1)", m, _OFFSET(ut())
m = _MEMELEMENT(fs(2)): show "fs(2)", m, _OFFSET(fs())
m = _MEMELEMENT(b2(1, 2)): show "b2(1, 2)", m, _OFFSET(b2())
PRINT "an address, new memory"
o = _OFFSET(a)
m = _MEM(o, 10): show "_MEM(o, 10)", m, o
m = _MEMNEW(16)
show "_MEMNEW(16)", m, m.OFFSET
_MEMFREE m
m = _MEMNEW(0)
show "_MEMNEW(0)", m, 0
PRINT "LEN"; LEN(m); LEN(u); LEN(z(1))
PRINT "procedures"
CALL x(ga())
PRINT "an array parameter whose first use is _MEM(q()), called only from below"
CALL zz
SYSTEM

SUB show (l$, m AS _MEM, bas AS _OFFSET)
    PRINT l$; ":"; m.OFFSET - bas; m.SIZE; m.TYPE; m.ELEMENTSIZE; m.IMAGE; m.SOUND
END SUB

SUB x (p() AS DOUBLE)
    DIM l AS INTEGER
    STATIC st AS LONG
    DIM m AS _MEM
    m = _MEM(l): show "l", m, _OFFSET(l)
    m = _MEM(st): show "st", m, _OFFSET(st)
    m = _MEM(g): show "g", m, _OFFSET(g)
    m = _MEM(ga(1)): show "ga(1)", m, _OFFSET(ga())
    m = _MEM(p()): show "p()", m, _OFFSET(p())
    m = _MEM(p(2)): show "p(2)", m, _OFFSET(p())
    m = _MEMELEMENT(p(2)): show "p(2) element", m, _OFFSET(p())
END SUB

SUB y (q() AS LONG)
    DIM m AS _MEM
    m = _MEM(q())
    PRINT "q() before an index:"; _MEMEXISTS(m); m.SIZE; m.TYPE; m.ELEMENTSIZE
    q(0) = 6
    PRINT "after q(0) = 6:"; _MEMEXISTS(m)
    m = _MEM(q())
    PRINT "q() after an index:"; _MEMEXISTS(m)
    q(0) = 7
    PRINT "after q(0) = 7:"; _MEMEXISTS(m)
END SUB

SUB zz
    DIM q2(3) AS LONG
    q2(0) = 5
    CALL y(q2())
END SUB
