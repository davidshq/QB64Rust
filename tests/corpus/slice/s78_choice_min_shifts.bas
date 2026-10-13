$CONSOLE:ONLY
' Slice (m3-console-files, task 2.5; from verification\v30_g_iif_min_max, v30_h_shifts): _IIF, _MIN, _MAX, _CLAMP,
' _SHL, _SHR, _ROL and _ROR, 24 numbered lines. _IIF evaluates only the part it chooses (a FUNCTION that prints, an
' element outside its array, a division by zero); its condition is rounded; the result of the four is typed by a
' rule of their own (a SINGLE beside an _INTEGER64, signed beside unsigned of one width); the rotations turn within
' the width of their first argument's type.
ON ERROR GOTO handler
TYPE rec
    s AS SINGLE
    q AS _INTEGER64
END TYPE
DIM b AS _BYTE, ub AS _UNSIGNED _BYTE, i AS INTEGER, ui AS _UNSIGNED INTEGER, l AS LONG, ul AS _UNSIGNED LONG
DIM q AS _INTEGER64, uq AS _UNSIGNED _INTEGER64, s AS SINGLE, d AS DOUBLE, f AS _FLOAT, x AS STRING * 5
DIM v AS STRING, a(3) AS LONG, n AS LONG, r AS rec, bt AS _BIT, nan AS SINGLE
PRINT "1:"; _IIF(1, 10, 20); _IIF(0, 10, 20); _IIF(-1, "yes", "no"); _IIF(0, "yes", "no")
PRINT "2:"; _IIF(1, pa&, pb&); _IIF(0, pa&, pb&)
n = 9: a(1) = 5
PRINT "3:"; _IIF(n <= 3, a(n), -1); _IIF(n > 3, -1, a(n))
n = 1
PRINT "4:"; _IIF(n <= 3, a(n), -1); _IIF(n, 100 \ n, 0)
n = 0
PRINT "5:"; _IIF(n, 100 \ n, 0); _IIF(n = 0, 0, 100 MOD n)
PRINT "6: an element outside in the part chosen:";: l = _IIF(1, a(9), 2): PRINT l
PRINT "7:"; _IIF(0.5, "t", "f"); _IIF(0.6, "t", "f"); _IIF(0.4, "t", "f"); _IIF(-0.5, "t", "f"); _IIF(1.5, "t", "f");
d = 1E-30: s = 2.5: q = 4294967296
PRINT _IIF(d, "t", "f"); _IIF(s, "t", "f"); _IIF(q, "t", "f"); _IIF(2.5 - s, "t", "f")
x = "ab": v = "abc"
PRINT "8:"; LEN(_IIF(0, x, v)); LEN(_IIF(1, x, v)); "["; _IIF(1, x, v); "]["; _IIF(0, x + "!", MID$(v, 2)); "]"
i = 7: l = 100000: s = 1.5
PRINT "9:"; _IIF(1, i, l); _IIF(0, i, l); _IIF(1, i, s); _IIF(0, i, s) * 2; _IIF(1, 1, 2.5) / 2
PRINT "10:"; _IIF(_IIF(n, 0, 1), "inner 1", "inner 0"); _IIF(1, _IIF(0, 1, 2), 3)
v = _IIF(LEN(v) > 2, v + v, "")
PRINT "11: "; v; _IIF(pa& > 0 AND pb& > 0, " both", " not both")
PRINT "12:"; _MIN(3, 7); _MAX(3, 7); _MIN(-3, -7); _MAX(-3, -7); _MIN(2.5, 2); _MAX(2.5, 2); _MIN(1E+30, 1D+300)
b = -1: ub = 255: i = -1: ui = 65535: l = -1: ul = 4294967295
PRINT "13:"; _MIN(b, ub); _MAX(b, ub); _MIN(ub, b); _MIN(i, ui); _MAX(l, ul); _MIN(i, ub); _MAX(b, ui)
s = 16777217: q = 16777217: d = 16777217
PRINT "14:"; _MAX(s, q); _MAX(q, s); _MIN(q, 16777218); _MAX(d, q); _MAX(q, 0.5)
uq = 0: uq = uq - 1: q = -1
PRINT "15:"; _MIN(uq, q); _MAX(q, uq); _MIN(q, 5); _MAX(uq, 5)
r.s = 3.5: r.q = 3
PRINT "16:"; _MAX(r.s, r.q); _MIN(r.s, r.q); _MIN(a(1), 4.5); _MAX(bt, 1); _MIN(bt, bt)
PRINT "17:"; _MIN(pa&, pb&); _MAX(pb&, pa&); _CLAMP(pa&, pb&, pc&)
PRINT "18:"; _CLAMP(15, 0, 10); _CLAMP(-5, 0, 10); _CLAMP(5, 0, 10); _CLAMP(5, 10, 0); _CLAMP(2.75, 1, 2.5);
PRINT _CLAMP(i, ub, 300); _CLAMP(ub, 1, 10)
nan = 0: nan = nan / nan
PRINT "19:"; _MIN(nan, 1) = _MIN(nan, 1); _MIN(1, nan); _MAX(1, nan)
l = _MIN(i, b) + _MAX(l, 5) * 2
PRINT "20:"; l; HEX$(_MIN(i, b)); " "; HEX$(_MAX(ub, 1)); STR$(_MIN(2, 3)); _MIN(_MAX(1, 2), _MIN(3, 4))
PRINT "21:"; _SHL(1, 4); _SHR(256, 4); _SHL(1, 63); _SHR(-1, 1); _SHL(1, 64); _SHL(1, 65); _SHR(8, -1); _SHL(2.5, 1.5);
i = -2: PRINT _SHR(i, 1); _SHL(i, 1)
b = -127: ub = 129: i = 1: l = 1: q = 1
PRINT "22:"; _ROL(ub, 1); _ROR(ub, 1); _ROL(b, 1); _ROR(i, 1); _ROL(i, 15); _ROR(l, 1); _ROR(q, 1)
PRINT "23:"; _ROL(1, 16); _ROR(1, 1); _ROL(70000, 31); _ROR(70000, 32); _ROL(5000000000, 1); _ROL(-1, 3); _ROL(i + 0, 63)
PRINT "24:"; _ROL(ub, 9); _ROL(ub, -1); _ROR(ub, 8); _ROL(i, q + 16); HEX$(_ROL(b, 4)); " "; _ROL(_ROL(ub, 4), 4)
SYSTEM
handler:
PRINT " <error"; ERR; ">";
RESUME NEXT
FUNCTION pa&
    PRINT " (a)";
    pa& = 5
END FUNCTION
FUNCTION pb&
    PRINT " (b)";
    pb& = 1
END FUNCTION
FUNCTION pc&
    PRINT " (c)";
    pc& = 3
END FUNCTION
