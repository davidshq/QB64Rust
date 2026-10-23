' TEST: typed
$CONSOLE:ONLY
' VARPTR, VARSEG, SADD and VARPTR$ of every kind of place of the main module the old compiler takes
' (m4-memory-records design D2, D3; verification\v56_*): a scalar of a number, _BIT, a string and a fixed-length
' string, a member through member arrays and nested types, an element of an array of numbers, strings,
' fixed-length strings and a TYPE, a member of one, VARSEG of a TYPE variable, SADD and VARPTR$ of s$(), and the
' width-0 radix forms. Each address is _INTEGER64 (VARPTR$ a string); VARSEG names the whole array or the variable
TYPE t
    a AS LONG
    m(3) AS INTEGER
END TYPE
TYPE o
    z AS INTEGER
    n AS t
END TYPE
DIM i AS INTEGER, d AS DOUBLE, bt AS _BIT, s AS STRING, fx AS STRING * 4, r AS t, ro AS o, l AS _INTEGER64
DIM na(5) AS LONG, sa(1 TO 3) AS STRING, fa(2) AS STRING * 2, ra(1 TO 4) AS t, v AS STRING
l = VARPTR(i) + VARSEG(d) + VARPTR(bt)
l = VARPTR(s) + VARPTR(fx) + VARSEG(fx)
l = VARPTR(r.a) + VARPTR(r.m(2)) + VARPTR(ro.n.m(1)) + VARSEG(r) + VARSEG(ro.n.a)
l = VARPTR(na(3)) + VARSEG(na(3)) + VARPTR(na())
l = VARPTR(sa(2)) + VARSEG(sa(2)) + VARPTR(fa(1)) + VARSEG(fa(1))
l = VARPTR(ra(2)) + VARPTR(ra(3).m(1)) + VARSEG(ra(4).a)
l = SADD(s) + SADD(sa(1)) + SADD(sa())
v = VARPTR$(i) + VARPTR$(bt) + VARPTR$(s) + VARPTR$(fx) + VARPTR$(r) + VARPTR$(r.a) + VARPTR$(sa(3)) + VARPTR$(sa())
v = HEX$(VARPTR(i)) + OCT$(VARSEG(i)) + _BIN$(SADD(s)) + HEX$(_SCREENEXISTS)
SYSTEM
