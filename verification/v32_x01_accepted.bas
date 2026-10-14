$CONSOLE:ONLY
' Verification (m3-mem, task 1.1): forms of _MEM, its functions and statements, _OFFSET, MKI$ ... _CV that the
' old compiler accepts, as code that is not reached.
TYPE t
    a AS INTEGER
    s AS STRING * 5
    v(0 TO 3) AS LONG
    d AS DOUBLE
END TYPE
TYPE w
    a AS INTEGER
    mm AS _MEM
    v(1 TO 2) AS LONG
END TYPE
DIM SHARED g AS LONG, ga(5) AS DOUBLE, gm AS _MEM
DIM a AS LONG, m AS _MEM, n AS _MEM, m2 AS _MEM, z(3) AS _MEM
DIM f AS STRING * 6, s AS STRING, u AS t, uw AS w, b(10) AS INTEGER, ut(3) AS t, fs(4) AS STRING * 3
DIM b2(3, 4) AS LONG
DIM s1 AS SINGLE, d1 AS DOUBLE, i64 AS _INTEGER64, by AS _BYTE, ub AS _UNSIGNED _BYTE, o AS _OFFSET
DIM uo AS _UNSIGNED _OFFSET, fl AS _FLOAT, ul AS _UNSIGNED LONG, i AS INTEGER, k AS LONG
REDIM dy(5) AS LONG
REDIM rm(3) AS _MEM
PRINT "ok"
SYSTEM
' a01 the block of each kind of place in the main module
m = _MEM(a)
m = _MEM(f)
m = _MEM(u)
m = _MEM(u.a)
m = _MEM(u.s)
m = _MEM(u.d)
m = _MEM(u.v(2))
m = _MEM(u.v())
m = _MEM(b())
m = _MEM(b(4))
m = _MEM(ut())
m = _MEM(ut(1))
m = _MEM(ut(1).d)
m = _MEM(ut(1).v())
m = _MEM(ut(1).v(2))
m = _MEM(fs())
m = _MEM(fs(2))
m = _MEM(b2())
m = _MEM(b2(1, 2))
m = _MEM(dy())
m = _MEM(dy(2))
m = _MEM(m2)
m = _MEM(z(1))
m = _MEM(s1): m = _MEM(d1): m = _MEM(i64): m = _MEM(by): m = _MEM(ub): m = _MEM(o): m = _MEM(uo)
m = _MEM(fl): m = _MEM(ul): m = _MEM(i)
' a02 the block of one element or value
m = _MEMELEMENT(a)
m = _MEMELEMENT(b(4))
m = _MEMELEMENT(b())
m = _MEMELEMENT(ut(1))
m = _MEMELEMENT(u)
m = _MEMELEMENT(u.a)
m = _MEMELEMENT(u.v(1))
m = _MEMELEMENT(fs(2))
m = _MEMELEMENT(b2(1, 2))
m = _MEMELEMENT(f)
' a03 the block of an address
m = _MEM(o, 10)
m = _MEM(_OFFSET(a), 4)
m = _MEM(o, k)
m = _MEM(o, 1.5)
' a04 blocks that are calls
m = _MEMNEW(64)
m = _MEMNEW(k)
m = _MEMNEW(1.5)
m = _MEMIMAGE
m = _MEMIMAGE(k)
m = _MEMIMAGE(1.5)
m = _MEMSOUND(k, 1)
m = _MEMSOUND(k)
z(2) = _MEMNEW(8)
uw.mm = _MEMNEW(8)
rm(1) = _MEMIMAGE
' a05 members, whole values, LEN
PRINT m.OFFSET; m.SIZE; m.TYPE; m.ELEMENTSIZE; m.IMAGE; m.SOUND
k = m.SIZE: o = m.OFFSET: k = m.TYPE: k = m.IMAGE
PRINT uw.mm.SIZE; LEN(m); LEN(uw); z(1).OFFSET; rm(2).ELEMENTSIZE
PRINT m.OFFSET + 1
n = m
SWAP m, n
gm = m
' a06 freeing and checking
PRINT _MEMEXISTS(m); _MEMEXISTS(z(1)); _MEMEXISTS(uw.mm); _MEMEXISTS(rm(1))
_MEMFREE m
_MEMFREE z(1)
_MEMFREE uw.mm
_MEMFREE rm(1)
CALL _MEMFREE(m)
' a07 _MEMGET as a function, by each type
k = _MEMGET(m, m.OFFSET, LONG)
d1 = _MEMGET(m, m.OFFSET + 8, DOUBLE)
PRINT _MEMGET(m, m.OFFSET, _UNSIGNED _BYTE)
PRINT _MEMGET(m, m.OFFSET, _INTEGER64); _MEMGET(m, m.OFFSET, INTEGER); _MEMGET(m, m.OFFSET, _BYTE)
PRINT _MEMGET(m, m.OFFSET, SINGLE); _MEMGET(m, m.OFFSET, _UNSIGNED INTEGER); _MEMGET(m, m.OFFSET, _UNSIGNED LONG)
PRINT _MEMGET(m, m.OFFSET, _UNSIGNED _INTEGER64); _MEMGET(m, m.OFFSET, _UNSIGNED _OFFSET)
f = _MEMGET(m, m.OFFSET, STRING * 4)
PRINT _MEMGET(m, m.OFFSET, STRING * 2)
u = _MEMGET(m, m.OFFSET, t)
o = _MEMGET(m, m.OFFSET, _OFFSET)
fl = _MEMGET(m, m.OFFSET, _FLOAT)
k = _MEMGET(m, 5, LONG)
k = _MEMGET(uw.mm, uw.mm.OFFSET, LONG)
' a08 the _MEMGET statement, by each kind of variable
_MEMGET m, m.OFFSET, k
_MEMGET m, m.OFFSET, d1
_MEMGET m, m.OFFSET, u
_MEMGET m, m.OFFSET, f
_MEMGET m, m.OFFSET, s
_MEMGET m, m.OFFSET, b(2)
_MEMGET m, m.OFFSET, fl
_MEMGET m, m.OFFSET, b()
_MEMGET m, m.OFFSET, u.d
_MEMGET m, m.OFFSET, i
_MEMGET m, m.OFFSET, by
' a09 _MEMPUT, by each kind of variable and AS each type
_MEMPUT m, m.OFFSET, k
_MEMPUT m, m.OFFSET, u
_MEMPUT m, m.OFFSET, f
_MEMPUT m, m.OFFSET, s
_MEMPUT m, m.OFFSET, "abc"
_MEMPUT m, m.OFFSET, s + "x"
_MEMPUT m, m.OFFSET, b(1)
_MEMPUT m, m.OFFSET, b()
_MEMPUT m, m.OFFSET, u.v()
_MEMPUT m, m.OFFSET, fl
_MEMPUT m, m.OFFSET, 7 AS LONG
_MEMPUT m, m.OFFSET, 1.5 AS SINGLE
_MEMPUT m, m.OFFSET, k AS _UNSIGNED _BYTE
_MEMPUT m, m.OFFSET, d1 AS _FLOAT
_MEMPUT m, m.OFFSET, 3 AS _OFFSET
_MEMPUT m, m.OFFSET, k + 1 AS INTEGER
_MEMPUT m, m.OFFSET, d1 AS _INTEGER64
_MEMPUT m, m.OFFSET, 2 AS _BYTE
_MEMPUT m, m.OFFSET, 2 AS DOUBLE
_MEMPUT m, m.OFFSET, 2 AS _UNSIGNED _OFFSET
_MEMPUT m, 5, 1 AS LONG
_MEMPUT uw.mm, uw.mm.OFFSET, 1 AS LONG
' a10 _MEMFILL
_MEMFILL m, m.OFFSET, 8, 3 AS INTEGER
_MEMFILL m, m.OFFSET, 8, 3 AS _BYTE
_MEMFILL m, m.OFFSET, 8, 3 AS LONG
_MEMFILL m, m.OFFSET, 8, 3 AS _FLOAT
_MEMFILL m, m.OFFSET, 8, 3 AS _OFFSET
_MEMFILL m, m.OFFSET, 8, 3 AS SINGLE
_MEMFILL m, m.OFFSET, 8, 3 AS DOUBLE
_MEMFILL m, m.OFFSET, 8, 3 AS _UNSIGNED _INTEGER64
_MEMFILL m, m.OFFSET, 8, 3 AS _UNSIGNED _OFFSET
_MEMFILL m, m.OFFSET, 8, k
_MEMFILL m, m.OFFSET, 8, "ab"
_MEMFILL m, m.OFFSET, 8, u
_MEMFILL m, m.OFFSET, 8, b()
_MEMFILL m, m.OFFSET, 8, f
' a11 _MEMCOPY
_MEMCOPY m, m.OFFSET, 8 TO n, n.OFFSET
_MEMCOPY uw.mm, uw.mm.OFFSET, 4 TO rm(2), rm(2).OFFSET
CALL _MEMCOPY(m, 0, 8 TO n, 0)
' a12 _OFFSET of each kind of place
o = _OFFSET(a)
o = _OFFSET(f)
o = _OFFSET(s)
o = _OFFSET(u)
o = _OFFSET(u.a)
o = _OFFSET(u.v(1))
o = _OFFSET(u.v())
o = _OFFSET(ut())
o = _OFFSET(ut(1))
o = _OFFSET(ut(1).v(2))
o = _OFFSET(b())
o = _OFFSET(b(2))
o = _OFFSET(fs())
o = _OFFSET(fs(1))
o = _OFFSET(b2())
o = _OFFSET(b2(1, 2))
o = _OFFSET(m)
PRINT _OFFSET(a) + 1
k = _OFFSET(a)
' a13 MKI$ ... CVD
s = MKI$(i): s = MKI$(k): s = MKI$(1.7)
s = MKL$(k): s = MKL$(d1)
s = MKS$(s1): s = MKS$(k)
s = MKD$(d1): s = MKD$(1)
i = CVI(s): k = CVL(s): s1 = CVS(s): d1 = CVD(s)
k = CVL(f)
PRINT CVI(MKI$(3)); LEN(MKL$(65))
' a14 _MK$ and _CV with every type word
s = _MK$(INTEGER, k): s = _MK$(LONG, k): s = _MK$(SINGLE, k): s = _MK$(DOUBLE, k): s = _MK$(_FLOAT, k)
s = _MK$(_BYTE, k): s = _MK$(_UNSIGNED _BYTE, k): s = _MK$(_UNSIGNED INTEGER, k): s = _MK$(_UNSIGNED LONG, k)
s = _MK$(_INTEGER64, k): s = _MK$(_UNSIGNED _INTEGER64, k): s = _MK$(_OFFSET, k): s = _MK$(_UNSIGNED _OFFSET, k)
s = _MK$(_BIT, k): s = _MK$(_UNSIGNED _BIT, k): s = _MK$(INTEGER, 1.7)
i = _CV(INTEGER, s): k = _CV(LONG, s): s1 = _CV(SINGLE, s): d1 = _CV(DOUBLE, s): fl = _CV(_FLOAT, s)
i = _CV(_BYTE, s): ub = _CV(_UNSIGNED _BYTE, s): k = _CV(_UNSIGNED INTEGER, s): d1 = _CV(_UNSIGNED LONG, s)
i64 = _CV(_INTEGER64, s): d1 = _CV(_UNSIGNED _INTEGER64, s): o = _CV(_OFFSET, s): o = _CV(_UNSIGNED _OFFSET, s)
k = _CV(_BIT, s): k = _CV(_UNSIGNED _BIT, s)
PRINT _CV(LONG, f); LEN(_MK$(DOUBLE, 1))
' a15 procedures
CALL x(ga())
PRINT fm(m)
SUB x (p() AS DOUBLE)
    DIM l AS INTEGER, q AS _MEM
    STATIC st AS LONG, sm AS _MEM
    q = _MEM(l)
    q = _MEM(st)
    q = _MEM(g)
    q = _MEM(ga(1))
    q = _MEM(p())
    q = _MEM(p(2))
    q = _MEMELEMENT(p(2))
    q = _MEM(p())
    sm = q
    gm = sm
    y q
END SUB
SUB y (p AS _MEM)
    _MEMPUT p, p.OFFSET, 2 AS INTEGER
    p = _MEM(p)
    gm = p
END SUB
FUNCTION fm (q AS _MEM)
    fm = q.SIZE
END FUNCTION
