$CONSOLE:ONLY
' The CONST evaluator's floats and lanes (m3-constant-evaluation; verification\v45_a_precision, v45_a_lanes,
' v45_a_auto, v45_b_conv, v45_b_asc_suffix): _FLOAT precision and the type a float constant takes from its text,
' the unsigned lane and the clamp of an unsuffixed literal beyond _INTEGER64, a float in the integer lanes by qbr,
' integer-valued floats near 2^63, suffixed constants inside a CONST, ASC, and the auto-included constants that
' were not computed before (_E, the limits of the types, clamped as the old compiler has them).
CONST third## = 1## / 3##
CONST c = 1E+30 / 3
CONST d = 1 / 3
CONST e = 1E-10 / 3
CONST t = 0.1 * 3
CONST t2## = 0.1 * 3
CONST big = 1E+19 / 1
PRINT _TOSTR$(third##); " "; _TOSTR$(c); " "; _TOSTR$(d); " "; _TOSTR$(e)
PRINT _TOSTR$(t); " "; _TOSTR$(t2##); " "; _TOSTR$(big)
PRINT c; d; big

CONST a = 18446744073709551615~&&
CONST b = a + 1
CONST u = a - 1
CONST big2~&& = 18446744073709551615
CONST plain = 18446744073709551615
CONST c8% = 4 + 4
CONST d8 = c8% * 2
PRINT a; b; u
PRINT big2~&&; plain
PRINT d8

CONST i1 = 2.5 \ 1, i2 = 3.5 \ 1, i3 = -2.5 \ 1, i4 = 2.4999 \ 1, i5 = 7.5 MOD 4, i6 = -7.5 MOD 4
CONST u1 = 2.5 AND 255~&&, u2 = 3.5 OR 0~&&, u3 = 1E+19 OR 0~&&, u4 = -1.5 OR 0~&&
CONST s1 = 1E+19 OR 0, s2 = -1E+19 OR 0, s3 = 1E+30 OR 0
CONST n1 = -18446744073709551615~&&, n2 = NOT 0~&&, n3 = NOT 2.5
CONST g = 9.3E+18 / 1, g2 = 9.2E+18 / 1, g3 = 9.9E+18 / 1
CONST h = 1E+18 / 1, k = 1E-5 / 1, k2 = 0.0001 / 1, k3 = 123456789012345678 / 10
PRINT i1; i2; i3; i4; i5; i6
PRINT u1; u2; u3; u4
PRINT s1; s2; s3
PRINT n1; n2; n3
PRINT g; g2; g3
PRINT h; _TOSTR$(k); " "; _TOSTR$(k2); " "; _TOSTR$(k3)

CONST a1 = ASC("A"), a2 = ASC("abc", 2), s = "xyz", a3 = ASC(s), a4 = ASC(s, 3)
CONST a5 = ASC("\"), a6 = CHR$(ASC(CHR$(ASC("\")))), a7 = ASC("abc", 2.5)
CONST cc% = 8
CONST dd = cc% * 2, ee = cc& + 1, ff = cc! / 2, gg = cc## * 1, hh = cc~&& - 9, kk = cc&& + cc
CONST uu~& = 4294967295
CONST vv = uu% + 1, ww = uu~& + 1
CONST tt$ = "ab"
CONST tt2 = tt$ + "c", tt3 = tt + "d"
CONST fl# = 0.1
CONST fl2 = fl# * 3, fl3 = fl% * 3
PRINT a1; a2; a3; a4; a5; a6; a7
PRINT dd; ee; ff; gg; hh; kk
PRINT vv; ww; tt2; " "; tt3
PRINT _TOSTR$(fl2); " "; _TOSTR$(fl3)

PRINT _TOSTR$(_E); _E
PRINT _INTEGER64_MIN; _INTEGER64_MIN + 0
PRINT _UINTEGER64_MAX; _UINTEGER64_MAX + 0
PRINT _OFFSET_MIN; _UOFFSET_MAX
PRINT _TOSTR$(_DOUBLE_MAX); " "; _TOSTR$(_DOUBLE_MIN)
PRINT _TOSTR$(_FLOAT_MIN_FRAC); " "; _TOSTR$(_FLOAT_MAX); " "; _TOSTR$(_FLOAT_MIN)
SYSTEM
