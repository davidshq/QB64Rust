$CONSOLE:ONLY
' Verification (m3-get-put-bload, task 1.2): forms of TAB, SPC, graphics GET and PUT, DEF SEG, BLOAD, BSAVE,
' PEEK, POKE, VARPTR, VARSEG, _RGB32, _RGBA32 and _FULLSCREEN the old compiler accepts, as code that is not
' reached.
TYPE rec
    x AS INTEGER
    y AS LONG
END TYPE
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, s AS SINGLE, d AS DOUBLE, t AS STRING, u AS _UNSIGNED LONG
DIM a(100) AS INTEGER, b(1 TO 10, 1 TO 20) AS LONG, c(100) AS SINGLE, e(100) AS _UNSIGNED _BYTE
DIM r(10) AS rec, fx AS STRING * 8
REDIM rd(1 TO 30) AS LONG
PRINT "ok"
SYSTEM
PRINT TAB(5); "a"; SPC(5); "b"
PRINT TAB(q); TAB(s); TAB(d); SPC(q); SPC(s); SPC(d)
PRINT TAB(5)
PRINT TAB(5),
PRINT TAB(5) "a"
PRINT "a" TAB(5) "b" SPC(2)
PRINT TAB(3); TAB(9); SPC(1);
PRINT #1, TAB(5); "a"; SPC(3)
WRITE TAB(5)
t = TAB(5) + SPC(i)
l = LEN(TAB(l))
GET (1, 2)-(3, 4), a
GET (1, 2)-(3, 4), a()
GET (1, 2)-(3, 4), a(5)
GET (i, l)-(s, d), b
GET (1, 2)-(3, 4), b(2, 3)
GET STEP(1, 2)-(3, 4), c
GET (1, 2)-STEP(3, 4), e
GET STEP(1, 2)-STEP(3, 4), a
GET (1, 2)-(3, 4), r
GET (1, 2)-(3, 4), r(2)
GET (1, 2)-(3, 4), rd(5)
GET (1, 2)-(3, 4), i
GET (1, 2)-(3, 4), fx
PUT (1, 2), a
PUT (1, 2), a()
PUT (1, 2), a(5)
PUT (i, l), b, PSET
PUT (s, d), b, PRESET
PUT (1, 2), c, AND
PUT (1, 2), e, OR
PUT (1, 2), a(i), XOR
PUT STEP(1, 2), a
PUT STEP(1, 2), a, PSET
PUT (1, 2), a, _CLIP
PUT (1, 2), a, _CLIP PSET
PUT (1, 2), a, _CLIP PSET, 5
PUT (1, 2), a, _CLIP , 5
PUT (1, 2), a, PSET, 5
PUT (1, 2), a, , 5
PUT (1, 2), a,
PUT (1.5, 2.5), rd
PUT (1, 2), l
DEF SEG
DEF SEG = 0
DEF SEG = i
DEF SEG = s
DEF SEG = &HA000
DEF SEG = VARSEG(a(1))
BLOAD "x.vga"
BLOAD "x.vga", VARPTR(a(1))
BLOAD t, l
BLOAD t + ".vga", s
BLOAD fx, i
BSAVE "y.vga", VARPTR(a(0)), 100
BSAVE t, i, s
l = VARSEG(a(1)) + VARPTR(a(1))
l = VARSEG(a) + VARPTR(a)
l = VARSEG(i) + VARPTR(i)
l = VARPTR(b(2, 3)) + VARSEG(b(2, 3))
l = VARPTR(rd(1)) + VARSEG(rd(1))
l = VARPTR(t) + VARPTR(fx) + VARSEG(d)
l = VARPTR(r(1)) + VARSEG(r(2))
l = SADD(t)
t = VARPTR$(t)
i = PEEK(l)
i = PEEK(s)
POKE l, i
POKE s, d
u = _RGB32(1, 2, 3)
u = _RGB32(1, 2, 3, 4)
u = _RGB32(1)
u = _RGB32(1, 2)
u = _RGB32(i, l, s)
u = _RGB32(d, q, u, i)
u = _RGBA32(1, 2, 3, 4)
_FULLSCREEN
_FULLSCREEN _OFF
_FULLSCREEN _STRETCH
_FULLSCREEN _SQUAREPIXELS
_FULLSCREEN OFF
_FULLSCREEN , _SMOOTH
_FULLSCREEN _STRETCH, _SMOOTH
l = _FULLSCREEN
