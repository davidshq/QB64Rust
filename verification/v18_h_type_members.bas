$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): members of each type, nesting, layout (LEN), strings.
TYPE inner
    v AS LONG
    w AS INTEGER
END TYPE
TYPE all
    i AS INTEGER
    l AS LONG
    q AS _INTEGER64
    s AS SINGLE
    d AS DOUBLE
    f AS _FLOAT
    n AS inner
END TYPE
TYPE withstr
    a AS LONG
    t AS STRING
    u AS STRING * 5
END TYPE
DIM p AS all
DIM w AS withstr
PRINT p.i; p.l; p.q; p.s; p.d; p.f; p.n.v; p.n.w
p.i = 32767: p.l = 2147483647: p.q = 9223372036854775807
p.s = 1 / 3: p.d = 1 / 3: p.f = 1 / 3
p.n.v = 7: p.n.w = -2
PRINT p.i; p.l; p.q
PRINT p.s; p.d; p.f
PRINT p.n.v; p.n.w
p.i = 2.5: p.l = 3.5
PRINT p.i; p.l
PRINT "LEN(p); LEN(p.n):"; LEN(p); LEN(p.n)
PRINT "["; w.t; "] ["; w.u; "]"; LEN(w.t); LEN(w.u)
w.t = "hello": w.u = "abcdefg"
PRINT "["; w.t; "] ["; w.u; "]"
PRINT "p.l / 3:"; p.l / 3
SYSTEM
